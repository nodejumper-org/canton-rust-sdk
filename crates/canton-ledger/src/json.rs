//! JSON Ledger API client (HTTP).
//!
//! The JSON transport mirrors the gRPC client over Canton's HTTP JSON Ledger
//! API v2: read the version/offset, **submit commands**, and read the **active
//! contract set** and **updates** as bounded JSON arrays. It shares the SDK
//! error model and the same [`Auth`] as the gRPC client.
//!
//! Values use the Daml-LF JSON encoding: a record is a JSON object keyed by
//! field name, a party is a string, a `TextMap` is a JSON object. Reads return
//! `serde_json::Value` (the M1 dynamic path); typed bindings land in M2.
//!
//! The blocking read endpoints are capped by the node's
//! `http-list-max-elements-limit` and return `413` past it — pass a `limit` (or
//! a bounded offset range). WebSocket streaming for unbounded tails is a
//! separate transport.

use std::sync::Arc;

use canton_auth::TokenProvider;
use canton_core::telemetry::{self, TRANSPORT_JSON};
use canton_core::{Auth, Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// A client for the Canton **JSON** Ledger API over HTTP.
#[derive(Clone)]
pub struct JsonClient {
    base_url: String,
    http: reqwest::Client,
    auth: Auth,
    /// Kept for the WebSocket handshake (feature `ws`); the HTTP client bakes
    /// its TLS settings into `http` at `with_tls` time.
    tls: Option<canton_core::TlsConfig>,
    retry: Option<canton_core::RetryConfig>,
    /// The largest WebSocket frame this client will accept, in bytes. Only the
    /// WS lane has a ceiling to raise: `reqwest` puts no limit on an HTTP
    /// response body, so the `POST` lane reads whatever the participant sends.
    max_decoding_message_size: usize,
    /// How long one HTTP attempt may take. Applied per request rather than on
    /// the `reqwest` client, so it holds however the client was built and
    /// whatever order the builders were called in.
    timeout: std::time::Duration,
}

/// How long one JSON request may take before it is abandoned.
///
/// The same 30 seconds the gRPC channel uses, and for the same reason: without
/// it there is no bound at all. `reqwest` applies no timeout unless asked, so a
/// participant that accepts the connection and then stops answering holds the
/// caller's task open for as long as the process runs.
const DEFAULT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);

/// Hand-written for the same reason [`Config`]'s is: a base URL can carry
/// credentials in its userinfo (`https://user:secret@host`), and the derived
/// `Debug` printed them verbatim — one `tracing` field holding a client, or one
/// `{:?}` in a log line, was enough.
///
/// The `reqwest` client is left out entirely — `finish_non_exhaustive` says so
/// rather than pretending otherwise — because its internals describe a
/// connection pool, not this client.
///
/// [`Config`]: canton_core::Config
impl std::fmt::Debug for JsonClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JsonClient")
            .field("base_url", &canton_core::redact_url(&self.base_url))
            .field("auth", &self.auth)
            .field("tls", &self.tls)
            .field("retry", &self.retry)
            .field("max_decoding_message_size", &self.max_decoding_message_size)
            .finish_non_exhaustive()
    }
}

#[derive(Deserialize)]
struct VersionResponse {
    version: String,
}

#[derive(Deserialize)]
struct LedgerEndResponse {
    offset: i64,
}

#[derive(Deserialize)]
struct PackagesResponse {
    #[serde(rename = "packageIds", default)]
    package_ids: Vec<String>,
}

#[derive(Deserialize)]
struct PackageStatusResponse {
    #[serde(rename = "packageStatus")]
    package_status: String,
}

/// A set of commands to submit over the JSON transport (dynamic path).
///
/// Build with [`JsonCommands::new`] then add commands ([`JsonCommands::add_create`]
/// or [`JsonCommands::add_command`]) and optional metadata. `command_id`
/// defaults to a fresh UUID so ledger-side de-duplication behaves correctly.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JsonCommands {
    command_id: String,
    act_as: Vec<String>,
    commands: Vec<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    read_as: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    workflow_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    synchronizer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    submission_id: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    disclosed_contracts: Vec<Value>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    package_id_selection_preference: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deduplication_period: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_ledger_time_abs: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_ledger_time_rel: Option<Value>,
}

impl JsonCommands {
    /// Start a command set acting as `act_as`, with a generated `command_id`
    /// and no commands yet.
    #[must_use]
    pub fn new(act_as: Vec<String>) -> Self {
        Self {
            command_id: format!("sdk-{}", uuid::Uuid::new_v4()),
            act_as,
            commands: Vec::new(),
            user_id: None,
            read_as: Vec::new(),
            workflow_id: None,
            synchronizer_id: None,
            submission_id: None,
            disclosed_contracts: Vec::new(),
            package_id_selection_preference: Vec::new(),
            deduplication_period: None,
            min_ledger_time_abs: None,
            min_ledger_time_rel: None,
        }
    }

    /// Set an explicit change-ID `command_id` (for exactly-once / de-duplication).
    #[must_use]
    pub fn with_command_id(mut self, command_id: impl Into<String>) -> Self {
        self.command_id = command_id.into();
        self
    }

    /// This command set's complete identity — available before it is sent, and
    /// the only way back to the outcome of a submission whose result was lost.
    #[must_use]
    pub fn change_id(&self) -> crate::ChangeId {
        crate::ChangeId::new(
            self.user_id.clone().unwrap_or_default(),
            self.act_as.clone(),
            self.command_id.clone(),
        )
    }

    /// Set the acting user id (defaults to the one derived from the token).
    #[must_use]
    pub fn with_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// Add read-as parties.
    #[must_use]
    pub fn with_read_as(mut self, read_as: Vec<String>) -> Self {
        self.read_as = read_as;
        self
    }

    /// Set the workflow id.
    #[must_use]
    pub fn with_workflow_id(mut self, workflow_id: impl Into<String>) -> Self {
        self.workflow_id = Some(workflow_id.into());
        self
    }

    /// Pin the submission to a specific synchronizer.
    #[must_use]
    pub fn with_synchronizer_id(mut self, synchronizer_id: impl Into<String>) -> Self {
        self.synchronizer_id = Some(synchronizer_id.into());
        self
    }

    /// Add a `CreateCommand` for `template_id` (`"<pkg>:<Module>:<Entity>"`) with
    /// `create_arguments` in Daml-LF JSON (a record is an object keyed by field).
    #[must_use]
    pub fn add_create(mut self, template_id: impl Into<String>, create_arguments: Value) -> Self {
        // Build the object directly (rather than `json!`) so `create_arguments`
        // is moved in, not cloned.
        let mut create = serde_json::Map::new();
        create.insert("templateId".to_string(), Value::String(template_id.into()));
        create.insert("createArguments".to_string(), create_arguments);
        let mut command = serde_json::Map::new();
        command.insert("CreateCommand".to_string(), Value::Object(create));
        self.commands.push(Value::Object(command));
        self
    }

    /// Add a raw command value (e.g. an `ExerciseCommand`), for shapes the
    /// convenience builders don't cover.
    #[must_use]
    pub fn add_command(mut self, command: Value) -> Self {
        self.commands.push(command);
        self
    }

    /// Set an explicit submission id, to correlate this particular submission
    /// attempt in completions. Defaults to participant-generated.
    #[must_use]
    pub fn with_submission_id(mut self, submission_id: impl Into<String>) -> Self {
        self.submission_id = Some(submission_id.into());
        self
    }

    /// Attach a disclosed contract (raw JSON: `{"templateId": …,
    /// "contractId": …, "createdEventBlob": …, "synchronizerId": …}`, with the
    /// blob obtained from a read with created-event blobs enabled). May be
    /// called repeatedly.
    #[must_use]
    pub fn add_disclosed_contract(mut self, contract: Value) -> Self {
        self.disclosed_contracts.push(contract);
        self
    }

    /// Restrict package selection for interpretation to these package ids
    /// (at most one preference per package name) — the SCU upgrade pin.
    #[must_use]
    pub fn with_package_id_selection_preference(mut self, package_ids: Vec<String>) -> Self {
        self.package_id_selection_preference = package_ids;
        self
    }

    /// Set the de-duplication period (raw JSON, e.g.
    /// `{"DeduplicationDuration": {"value": {"duration": "5s"}}}`), matching
    /// the JSON API's `deduplicationPeriod` encoding.
    #[must_use]
    pub fn with_deduplication_period(mut self, period: Value) -> Self {
        self.deduplication_period = Some(period);
        self
    }

    /// Set the absolute lower bound for the ledger-effective time (raw JSON,
    /// an ISO-8601 timestamp string). Mutually exclusive with
    /// [`Self::with_min_ledger_time_rel`].
    #[must_use]
    pub fn with_min_ledger_time_abs(mut self, time: Value) -> Self {
        self.min_ledger_time_abs = Some(time);
        self
    }

    /// Set the relative lower bound for the ledger-effective time (raw JSON,
    /// a proto duration like `"5s"`). Mutually exclusive with
    /// [`Self::with_min_ledger_time_abs`].
    #[must_use]
    pub fn with_min_ledger_time_rel(mut self, duration: Value) -> Self {
        self.min_ledger_time_rel = Some(duration);
        self
    }
}

/// The response to a successful `submit-and-wait` — the lighter of the two
/// waiting submissions, which reports where the command landed without
/// returning the transaction itself.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct JsonSubmitAndWaitResponse {
    /// The id of the transaction the command produced.
    pub update_id: String,
    /// The offset of the completion.
    pub completion_offset: i64,
}

/// The response to a successful `submit-and-wait-for-transaction`.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct JsonSubmitResponse {
    /// The committed transaction.
    pub transaction: JsonTransaction,
}

/// A committed transaction from the JSON transport. Top-level fields are typed;
/// `events` stay as raw JSON (the M1 dynamic path).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub struct JsonTransaction {
    /// The update id (globally unique).
    pub update_id: String,
    /// The submitter-provided command id (empty if not echoed).
    #[serde(default)]
    pub command_id: String,
    /// The workflow id (empty if unset).
    #[serde(default)]
    pub workflow_id: String,
    /// The ledger offset at which this transaction was committed.
    pub offset: i64,
    /// The synchronizer that sequenced the transaction.
    #[serde(default)]
    pub synchronizer_id: String,
    /// Ledger-effective time (ISO-8601).
    #[serde(default)]
    pub effective_at: String,
    /// Record time (ISO-8601).
    #[serde(default)]
    pub record_time: String,
    /// The events, each a tagged object (`{"CreatedEvent": …}` / `{"ArchivedEvent": …}`).
    #[serde(default)]
    pub events: Vec<Value>,
}

/// The request body for an ACS snapshot at `active_at_offset` (POST and WS).
/// Built through [`crate::request::ActiveContractsRequest`], so the plain and
/// builder-driven methods share one body producer.
fn active_contracts_request(parties: &[String], active_at_offset: i64) -> Value {
    crate::request::ActiveContractsRequest::new(parties.to_vec(), active_at_offset).json_body()
}

/// The request body for updates over `(begin_exclusive, end_inclusive]` (POST
/// and WS); omit `end_inclusive` for an unbounded tail. Built through
/// [`crate::request::UpdatesRequest`] — `LEDGER_EFFECTS`, wildcard filters,
/// reassignments included — the same defaults as the gRPC lane, so both
/// transports yield the same event set for the same query.
fn updates_request(parties: &[String], begin_exclusive: i64, end_inclusive: Option<i64>) -> Value {
    let mut request = crate::request::UpdatesRequest::new(parties.to_vec(), begin_exclusive);
    if let Some(end) = end_inclusive {
        request = request.until(end);
    }
    request.json_body()
}

/// The request body for command completions from `begin_exclusive` (WS).
#[cfg(feature = "ws")]
fn completions_request(parties: &[String], begin_exclusive: i64) -> Value {
    crate::request::CompletionsRequest::new(parties.to_vec(), begin_exclusive).json_body()
}

/// Whether a failed submission is the participant refusing a command it already
/// has. Canton's JSON lane maps `ALREADY_EXISTS` to HTTP 409, and names the
/// error in the body — either signal is enough, and a body that names
/// `DUPLICATE_COMMAND` under some other status still means the same thing.
fn is_duplicate_submission(error: &Error) -> bool {
    match error {
        Error::Http { status, body } => *status == 409 || body.contains("DUPLICATE_COMMAND"),
        _ => false,
    }
}

/// Add W3C trace-context headers to an outgoing request (a no-op without the
/// `otel` feature, or when no OpenTelemetry context is active).
fn with_trace_context(request: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
    #[cfg(feature = "otel")]
    {
        let mut headers = reqwest::header::HeaderMap::new();
        canton_core::telemetry::otel::inject_trace_context(&mut headers);
        if !headers.is_empty() {
            return request.headers(headers);
        }
    }
    request
}

/// Validate an HTTP response and deserialize its JSON body.
async fn read_json<T: for<'de> Deserialize<'de>>(
    response: reqwest::Response,
    path: &str,
) -> Result<T> {
    // Non-2xx carries its status (e.g. `413` past the node's list cap, `401`
    // for a bad token), so callers can branch and retry 5xx/429.
    if !response.status().is_success() {
        let status = response.status().as_u16();
        let body = response.text().await.unwrap_or_default();
        return Err(Error::Http { status, body });
    }
    let body = response
        .text()
        .await
        .map_err(|e| Error::Connection(format!("reading json body from {path} failed: {e}")))?;
    // A malformed body is a deserialization error (Error::Json), not a bad request.
    serde_json::from_str::<T>(&body).map_err(Error::from)
}

/// Upgrade an `http://` base URL to `https://` when TLS is configured.
///
/// Both JSON lanes select TLS by URL scheme: `reqwest` for HTTP, and
/// [`ws_url`](crate::ws) (`http`→`ws`, `https`→`wss`) for WebSocket. So a
/// `JsonClient` given an `http://` base URL together with `with_tls` would send
/// plaintext HTTP with the certificates unused and open a `ws://` socket. This
/// normalises the scheme so TLS is actually applied, matching the gRPC channel
/// builder. Detection is case-insensitive; anything not `http://` (already
/// `https://`, or another scheme) is left unchanged.
fn upgrade_base_url_for_tls(base_url: &str) -> String {
    if base_url
        .get(..7)
        .is_some_and(|s| s.eq_ignore_ascii_case("http://"))
    {
        format!("https://{}", &base_url[7..])
    } else {
        base_url.to_string()
    }
}

impl JsonClient {
    /// Create a JSON client for `base_url` (e.g. `http://localhost:3975`), with
    /// no authentication. A trailing slash on `base_url` is tolerated.
    #[must_use]
    pub fn new(base_url: impl Into<String>) -> Self {
        let mut base_url = base_url.into();
        while base_url.ends_with('/') {
            base_url.pop();
        }
        Self {
            base_url,
            http: reqwest::Client::new(),
            auth: Auth::None,
            tls: None,
            retry: None,
            max_decoding_message_size: canton_core::DEFAULT_MAX_DECODING_MESSAGE_SIZE,
            timeout: DEFAULT_TIMEOUT,
        }
    }

    /// Build a client from a local development network exported into the
    /// environment — the JSON counterpart to [`Config::from_env`].
    ///
    /// ```text
    /// eval "$(canton-devkit localnet env demo)"
    /// ```
    ///
    /// reads `CANTON_JSON_LEDGER_API_URL` and the default participant's JWT.
    /// See [`canton_core::localnet`] for the full contract.
    ///
    /// # Errors
    /// Returns [`Error::InvalidRequest`] when no JSON endpoint is exported,
    /// naming the variable and the command that produces it. A missing token is
    /// not an error: an unauthenticated LocalNet is a normal target.
    ///
    /// [`Config::from_env`]: canton_core::Config::from_env
    pub fn from_env() -> Result<Self> {
        Self::for_role(None)
    }

    /// The same, for a participant other than the default: `"app-user"`,
    /// `"sv"`, or any role the exporter knows.
    ///
    /// # Errors
    /// As [`Self::from_env`], naming that role's variable.
    pub fn from_env_for(role: &str) -> Result<Self> {
        Self::for_role(Some(role))
    }

    fn for_role(role: Option<&str>) -> Result<Self> {
        use canton_core::localnet;

        let base_url = localnet::json_endpoint(role).ok_or_else(|| {
            let variable = match role {
                None => "CANTON_JSON_LEDGER_API_URL".to_string(),
                Some(role) => format!(
                    "CANTON_{}_JSON_LEDGER_API_URL",
                    role.to_uppercase().replace('-', "_")
                ),
            };
            Error::InvalidRequest(format!(
                "no JSON ledger endpoint in the environment: set {variable}. \
                 A local network exports it with `canton-devkit localnet env <instance>`; \
                 run that through `eval` first."
            ))
        })?;
        let client = Self::new(base_url);
        Ok(match localnet::token(role) {
            Some(token) => client.with_token(token),
            None => client,
        })
    }

    /// How long one HTTP attempt may take (default 30s), the JSON lane's
    /// counterpart to [`Config::with_timeout`].
    ///
    /// `reqwest` imposes no timeout of its own, so without this a request to a
    /// participant that accepts the connection and then goes quiet never
    /// returns. Applied per attempt: under [`Self::with_retry`] each try gets
    /// the full budget, matching how the gRPC channel's timeout composes with
    /// its retries.
    ///
    /// The clock covers the whole exchange — connecting, sending, and reading
    /// the response body — so it is a real bound rather than a bound on the
    /// first byte.
    ///
    /// [`Config::with_timeout`]: canton_core::Config::with_timeout
    #[must_use]
    pub fn with_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// The largest WebSocket message this client will accept, in bytes —
    /// the JSON lane's counterpart to [`Config::with_max_decoding_message_size`],
    /// and the same default.
    ///
    /// A WS stream carries the same payloads the gRPC one does, so it needs the
    /// same ceiling. Left alone, `tungstenite` applies its own: 64 MiB per
    /// message and **16 MiB per frame**, the second of which is the one a large
    /// update meets first. Both are set from this value, so a frame is capped
    /// only by what the caller asked for.
    ///
    /// Does not affect the HTTP (`POST`) lane, which has no ceiling to raise.
    ///
    /// [`Config::with_max_decoding_message_size`]: canton_core::Config::with_max_decoding_message_size
    #[must_use]
    pub fn with_max_decoding_message_size(mut self, bytes: usize) -> Self {
        self.max_decoding_message_size = bytes;
        self
    }

    /// Retry requests on retriable errors (category-first classification of
    /// the participant's error body, transient HTTP statuses, connection
    /// failures) with exponential backoff, honouring a server-recommended
    /// retry delay — the same policy as the gRPC client's unary retries.
    /// Off by default. Safe for command submission too: the command id in
    /// the body stays fixed across attempts, so the participant de-duplicates.
    /// Streaming (the WS lane) resumes via its own reconnect policy instead.
    #[must_use]
    pub fn with_retry(mut self, retry: canton_core::RetryConfig) -> Self {
        self.retry = Some(retry);
        self
    }

    /// Use TLS for the HTTP connection: a custom CA (server-side TLS against a
    /// private/self-signed server) and/or a client identity (mutual TLS). This
    /// is a terminal builder step — call it last, after [`Self::with_token`] /
    /// [`Self::with_oidc`].
    ///
    /// An `http://` base URL is normalised to `https://` so TLS is never
    /// silently downgraded: `reqwest` selects TLS from the URL scheme (not from
    /// the configured certificates), and the WebSocket lane maps `http`→`ws` /
    /// `https`→`wss` the same way, so an `http://` base URL with `with_tls`
    /// would otherwise send plaintext HTTP and open a `ws://` socket with the
    /// certificates unused. Detection is case-insensitive, mirroring the gRPC
    /// channel builder (`canton-core`'s `resolve_endpoint`).
    ///
    /// `TlsConfig::domain_name` is not applied here: `reqwest` derives SNI
    /// from the request URL (it is a gRPC/`tonic` knob).
    ///
    /// # Errors
    /// Returns [`Error::InvalidRequest`] if a certificate/identity PEM is
    /// invalid or the HTTPS client cannot be built.
    pub fn with_tls(mut self, tls: &canton_core::TlsConfig) -> Result<Self> {
        let mut builder = reqwest::Client::builder();
        if let Some(ca) = &tls.ca_certificate_pem {
            let cert = reqwest::Certificate::from_pem(ca)
                .map_err(|e| Error::InvalidRequest(format!("invalid CA certificate: {e}")))?;
            builder = builder.add_root_certificate(cert);
        }
        if let Some((cert, key)) = &tls.client_identity_pem {
            // reqwest/rustls expects one PEM blob: certificate chain then key.
            let mut pem = cert.clone();
            pem.push(b'\n');
            pem.extend_from_slice(key);
            let identity = reqwest::Identity::from_pem(&pem)
                .map_err(|e| Error::InvalidRequest(format!("invalid client identity: {e}")))?;
            builder = builder.identity(identity);
        }
        self.http = builder
            .build()
            .map_err(|e| Error::InvalidRequest(format!("building the HTTPS client failed: {e}")))?;
        self.base_url = upgrade_base_url_for_tls(&self.base_url);
        self.tls = Some(tls.clone());
        Ok(self)
    }

    /// Authenticate with a fixed bearer token.
    #[must_use]
    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.auth = Auth::Static(token.into());
        self
    }

    /// Authenticate with an OIDC token provider (client-credentials, auto-refresh).
    #[must_use]
    pub fn with_oidc(mut self, provider: TokenProvider) -> Self {
        self.auth = Auth::Dynamic(Arc::new(provider));
        self
    }

    /// The WS lane's view of this client. One place, so a setting added to the
    /// transport reaches every subscription rather than the ones someone
    /// remembered.
    #[cfg(feature = "ws")]
    fn ws_transport(&self) -> crate::ws::WsTransport<'_> {
        crate::ws::WsTransport {
            base_url: &self.base_url,
            auth: &self.auth,
            tls: self.tls.as_ref(),
            max_decoding_message_size: self.max_decoding_message_size,
            timeout: self.timeout,
        }
    }

    async fn get<T: for<'de> Deserialize<'de>>(&self, path: &str) -> Result<T> {
        canton_core::retry::run_with_retry(self.retry.as_ref(), || async {
            let mut request = self
                .http
                .get(format!("{}{path}", self.base_url))
                .timeout(self.timeout);
            if let Some(token) = self.auth.bearer().await? {
                request = request.bearer_auth(token);
            }
            request = with_trace_context(request);
            let response = request
                .send()
                .await
                .map_err(|e| transport_error(path, &e))?;
            read_json(response, path).await
        })
        .await
    }

    async fn post<B: Serialize, T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        canton_core::retry::run_with_retry(self.retry.as_ref(), || self.post_once(path, body)).await
    }

    /// One POST, no retry. The retrying wrappers are built on this so that a
    /// submission can tell its own retries apart from its first attempt.
    async fn post_once<B: Serialize, T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let mut request = self
            .http
            .post(format!("{}{path}", self.base_url))
            .timeout(self.timeout)
            .json(body);
        if let Some(token) = self.auth.bearer().await? {
            request = request.bearer_auth(token);
        }
        request = with_trace_context(request);
        // main's structure (one-shot core under the retrying wrapper) with this
        // branch's transport classification instead of a blanket Connection.
        let response = request
            .send()
            .await
            .map_err(|e| transport_error(path, &e))?;
        read_json(response, path).await
    }

    /// The participant's Ledger API version (`GET /v2/version`, unauthenticated).
    ///
    /// # Errors
    /// Returns an [`Error`] if the request fails or the response is malformed.
    pub async fn version(&self) -> Result<String> {
        telemetry::instrument("version", TRANSPORT_JSON, async {
            Ok(self.get::<VersionResponse>("/v2/version").await?.version)
        })
        .await
    }

    /// The current ledger end offset (`GET /v2/state/ledger-end`, authenticated).
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication or the request fails.
    pub async fn ledger_end(&self) -> Result<i64> {
        telemetry::instrument("ledger_end", TRANSPORT_JSON, async {
            Ok(self
                .get::<LedgerEndResponse>("/v2/state/ledger-end")
                .await?
                .offset)
        })
        .await
    }

    /// The ids of every package the participant knows (`GET /v2/packages`).
    ///
    /// The JSON twin of [`AdminClient::list_packages`](canton_admin::AdminClient::list_packages),
    /// for a deployment that exposes only the JSON Ledger API. What a package
    /// *is* to the ledger — uploaded, vetted, both — is answered by
    /// [`package_status`](Self::package_status), not by presence in this list.
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication or the request fails.
    pub async fn list_packages(&self) -> Result<Vec<String>> {
        telemetry::instrument("list_packages", TRANSPORT_JSON, async {
            Ok(self
                .get::<PackagesResponse>("/v2/packages")
                .await?
                .package_ids)
        })
        .await
    }

    /// Whether the participant has a package registered
    /// (`GET /v2/packages/{package-id}/status`).
    ///
    /// Answers with the same [`PackageStatus`](canton_proto::com::daml::ledger::api::v2::PackageStatus)
    /// the gRPC path returns, so a caller can switch transports without
    /// re-learning the vocabulary. A status this build does not know is an
    /// [`Error::UnexpectedResponse`] rather than a silent
    /// `PACKAGE_STATUS_UNSPECIFIED`: the participant said something definite,
    /// and guessing would hide it.
    ///
    /// # Errors
    /// Returns [`Error::InvalidRequest`] if `package_id` is empty or could not
    /// form a path segment, and an [`Error`] if authentication or the request
    /// fails or the status is not one this build knows.
    pub async fn package_status(
        &self,
        package_id: &str,
    ) -> Result<canton_proto::com::daml::ledger::api::v2::PackageStatus> {
        use canton_proto::com::daml::ledger::api::v2::PackageStatus;

        // The id becomes a path segment. A package id is a hex hash, so
        // anything that would change the path's shape is a caller mistake,
        // not something to escape and send.
        if package_id.is_empty()
            || package_id
                .chars()
                .any(|c| matches!(c, '/' | '?' | '#' | '%') || c.is_whitespace())
        {
            return Err(Error::InvalidRequest(format!(
                "package id {package_id:?} cannot form a path segment"
            )));
        }
        let path = format!("/v2/packages/{package_id}/status");
        telemetry::instrument("package_status", TRANSPORT_JSON, async {
            let status = self
                .get::<PackageStatusResponse>(&path)
                .await?
                .package_status;
            PackageStatus::from_str_name(&status).ok_or_else(|| {
                Error::UnexpectedResponse(format!(
                    "package status {status:?} is not one this build knows"
                ))
            })
        })
        .await
    }

    /// Submit commands and wait for the resulting transaction
    /// (`POST /v2/commands/submit-and-wait-for-transaction`).
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication fails, the command is rejected
    /// (surfaced as [`Error::Http`] carrying the participant's error body), or
    /// the response is malformed.
    pub async fn submit_and_wait_for_transaction(
        &self,
        commands: &JsonCommands,
    ) -> Result<JsonSubmitResponse> {
        telemetry::instrument("submit_and_wait_for_transaction", TRANSPORT_JSON, async {
            let body = json!({ "commands": commands });
            self.post("/v2/commands/submit-and-wait-for-transaction", &body)
                .await
        })
        .await
    }

    /// Submit **without waiting** (`POST /v2/commands/async/submit`): the
    /// participant accepts the command and the outcome arrives on the
    /// completion stream.
    ///
    /// The gRPC lane has had this since M1. Note what a failure here means: the
    /// command may still have committed, so reach for
    /// [`Self::submission`] rather than this method when losing the outcome is
    /// not acceptable.
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication fails or the participant rejects
    /// the submission ([`Error::Http`] with its error body).
    pub async fn submit(&self, commands: &JsonCommands) -> Result<()> {
        telemetry::instrument("submit", TRANSPORT_JSON, async {
            // These two endpoints take the command set *itself* as the body,
            // where `submit-and-wait-for-transaction` takes a request object
            // wrapping it. The participant rejects the wrong one with a 400
            // naming the fields it could not find.
            // The response body is an empty object; accepting it is the answer.
            self.post_submission("/v2/commands/async/submit", commands)
                .await
        })
        .await
    }

    /// `post`, with the one rule a submission needs that a read does not: a
    /// retry the participant refuses as a duplicate means our *own* earlier
    /// attempt was accepted and its response was lost, so the command
    /// succeeded. Reporting the rejection would tell the caller their command
    /// failed when it did the opposite — which is the failure this change ID
    /// exists to prevent, and it is not the JSON lane's to have differently
    /// from gRPC.
    async fn post_submission<B: Serialize>(&self, path: &str, body: &B) -> Result<()> {
        let attempt = std::sync::atomic::AtomicU32::new(0);
        canton_core::retry::run_with_retry(self.retry.as_ref(), || async {
            let retry = attempt.fetch_add(1, std::sync::atomic::Ordering::Relaxed) > 0;
            match self.post_once::<B, serde_json::Value>(path, body).await {
                Ok(_) => Ok(()),
                Err(error) if retry && is_duplicate_submission(&error) => {
                    tracing::debug!(
                        "submission retry was de-duplicated; the earlier attempt is the one that landed"
                    );
                    Ok(())
                }
                Err(error) => Err(error),
            }
        })
        .await
    }

    /// Submit and wait for the completion (`POST /v2/commands/submit-and-wait`)
    /// without fetching the transaction — the update id and completion offset.
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication fails or the command is rejected.
    pub async fn submit_and_wait(
        &self,
        commands: &JsonCommands,
    ) -> Result<JsonSubmitAndWaitResponse> {
        telemetry::instrument("submit_and_wait", TRANSPORT_JSON, async {
            self.post("/v2/commands/submit-and-wait", commands).await
        })
        .await
    }

    /// The create and consuming-exercise events of one contract
    /// (`POST /v2/events/events-by-contract-id`), as seen by `parties`.
    ///
    /// Returns the raw response object; a contract that has been pruned or is
    /// invisible to `parties` comes back as a `CONTRACT_EVENTS_NOT_FOUND`
    /// error rather than an empty result.
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication or the request fails, or the
    /// contract has no events visible to `parties`.
    pub async fn events_by_contract_id(
        &self,
        contract_id: impl Into<String>,
        parties: Vec<String>,
    ) -> Result<Value> {
        telemetry::instrument("events_by_contract_id", TRANSPORT_JSON, async {
            let request = crate::request::ActiveContractsRequest::new(parties, 0);
            let body = json!({
                "contractId": contract_id.into(),
                "eventFormat": request.json_body()["eventFormat"],
            });
            self.post("/v2/events/events-by-contract-id", &body).await
        })
        .await
    }

    /// Fix a submission's identity **before** sending it, returning a
    /// [`JsonSubmission`](crate::JsonSubmission) that carries its
    /// [`ChangeId`](crate::ChangeId) — the JSON lane's
    /// [`CantonClient::submission`](crate::CantonClient::submission).
    #[must_use]
    pub fn submission(&self, commands: JsonCommands) -> crate::submission::JsonSubmission {
        crate::submission::JsonSubmission::new(self.clone(), commands)
    }

    /// The active contract set snapshot at `active_at_offset`, wildcard-filtered
    /// to `parties` (`POST /v2/state/active-contracts`).
    ///
    /// This is a **bounded** read: the node caps results at
    /// `http-list-max-elements-limit` and returns [`Error::Http`] `413` past it,
    /// so pass a `limit` for large sets (or use the streaming transport).
    /// Each element is raw JSON (`{"workflowId": …, "contractEntry": …}`).
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication or the request fails, or the
    /// result set exceeds the node limit (`413`).
    pub async fn active_contracts(
        &self,
        parties: Vec<String>,
        active_at_offset: i64,
        limit: Option<i64>,
    ) -> Result<Vec<Value>> {
        telemetry::instrument("active_contracts", TRANSPORT_JSON, async {
            let body = active_contracts_request(&parties, active_at_offset);
            let path = with_limit("/v2/state/active-contracts", limit);
            self.post(&path, &body).await
        })
        .await
    }

    /// Like [`Self::active_contracts`], with the full request surface of an
    /// [`ActiveContractsRequest`](crate::request::ActiveContractsRequest)
    /// (template/interface filters, created-event blobs, non-verbose records)
    /// — the same builder the gRPC lane takes.
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication or the request fails, or the
    /// result set exceeds the node limit (`413`).
    pub async fn active_contracts_with(
        &self,
        request: &crate::request::ActiveContractsRequest,
        limit: Option<i64>,
    ) -> Result<Vec<Value>> {
        telemetry::instrument("active_contracts", TRANSPORT_JSON, async {
            let path = with_limit("/v2/state/active-contracts", limit);
            self.post(&path, &request.json_body()).await
        })
        .await
    }

    /// Updates (transactions/reassignments) for `parties` in the offset range
    /// `(begin_exclusive, end_inclusive]` (`POST /v2/updates`).
    ///
    /// A **bounded** read like [`Self::active_contracts`]: bound it with
    /// `end_inclusive` and/or `limit`, or the node returns [`Error::Http`]
    /// `413`. Each element is raw JSON (`{"update": …}`), including
    /// `OffsetCheckpoint` heartbeats.
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication or the request fails, or the
    /// result set exceeds the node limit (`413`).
    pub async fn updates(
        &self,
        parties: Vec<String>,
        begin_exclusive: i64,
        end_inclusive: Option<i64>,
        limit: Option<i64>,
    ) -> Result<Vec<Value>> {
        telemetry::instrument("updates", TRANSPORT_JSON, async {
            let body = updates_request(&parties, begin_exclusive, end_inclusive);
            let path = with_limit("/v2/updates", limit);
            self.post(&path, &body).await
        })
        .await
    }

    /// Like [`Self::updates`], with the full request surface of an
    /// [`UpdatesRequest`](crate::request::UpdatesRequest) (bounds, template/
    /// interface filters, transaction shape, created-event blobs, topology
    /// events, non-verbose records) — the same builder the gRPC lane takes.
    ///
    /// # Errors
    /// Returns an [`Error`] if authentication or the request fails, or the
    /// result set exceeds the node limit (`413`).
    pub async fn updates_with(
        &self,
        request: &crate::request::UpdatesRequest,
        limit: Option<i64>,
    ) -> Result<Vec<Value>> {
        telemetry::instrument("updates", TRANSPORT_JSON, async {
            let path = with_limit("/v2/updates", limit);
            self.post(&path, &request.json_body()).await
        })
        .await
    }
}

/// Append a `?limit=<n>` query when a limit is set.
fn with_limit(path: &str, limit: Option<i64>) -> String {
    match limit {
        Some(limit) => format!("{path}?limit={limit}"),
        None => path.to_string(),
    }
}

#[cfg(feature = "ws")]
use futures_util::StreamExt as _;

#[cfg(feature = "ws")]
impl JsonClient {
    /// Stream updates over WebSocket (feature `ws`) for `parties`, starting after
    /// `begin_exclusive`. With `end_inclusive` the stream is bounded and closes
    /// once the range is exhausted; without it the stream tails live. Each item
    /// is a raw JSON update (`{"update": …}`); `OffsetCheckpoint` heartbeats are
    /// filtered out (as in the gRPC [`CantonClient::updates`]).
    ///
    /// Unlike [`Self::updates`], this is not capped by the node's list limit. For
    /// automatic reconnection use [`Self::ws_updates_resumable`].
    ///
    /// [`CantonClient::updates`]: crate::CantonClient::updates
    ///
    /// # Errors
    /// Returns an [`Error`] if the handshake fails; the stream yields `Err` on a
    /// participant error frame or a transport failure.
    #[allow(clippy::large_futures)] // the WS handshake state is inherently large; awaited once.
    #[cfg_attr(docsrs, doc(cfg(feature = "ws")))]
    pub async fn ws_updates(
        &self,
        parties: Vec<String>,
        begin_exclusive: i64,
        end_inclusive: Option<i64>,
    ) -> Result<impl futures_core::Stream<Item = Result<Value>> + Send + use<>> {
        telemetry::instrument("ws_updates", TRANSPORT_JSON, async move {
            let request = updates_request(&parties, begin_exclusive, end_inclusive);
            let inner = crate::ws::subscribe(&self.ws_transport(), "/v2/updates", request).await?;
            Ok(telemetry::instrument_stream(
                "ws_updates",
                TRANSPORT_JSON,
                crate::ws::filter_checkpoints(inner),
            ))
        })
        .await
    }

    /// Like [`Self::ws_updates`], with the full request surface of an
    /// [`UpdatesRequest`](crate::request::UpdatesRequest) — the same builder
    /// the gRPC lane takes (bounds, filters, shape, blobs, topology events,
    /// non-verbose records).
    ///
    /// # Errors
    /// Returns an [`Error`] if the handshake fails; the stream yields `Err` on
    /// a participant error frame or a transport failure.
    #[allow(clippy::large_futures)] // the WS handshake state is inherently large; awaited once.
    #[cfg_attr(docsrs, doc(cfg(feature = "ws")))]
    pub async fn ws_updates_with(
        &self,
        request: &crate::request::UpdatesRequest,
    ) -> Result<impl futures_core::Stream<Item = Result<Value>> + Send + use<>> {
        telemetry::instrument("ws_updates", TRANSPORT_JSON, async move {
            let inner =
                crate::ws::subscribe(&self.ws_transport(), "/v2/updates", request.json_body())
                    .await?;
            Ok(telemetry::instrument_stream(
                "ws_updates",
                TRANSPORT_JSON,
                crate::ws::filter_checkpoints(inner),
            ))
        })
        .await
    }

    /// Stream the active contract set snapshot at `active_at_offset` over
    /// WebSocket (feature `ws`), wildcard-filtered to `parties`. The stream
    /// closes when the snapshot is fully delivered. Each item is raw JSON
    /// (`{"workflowId": …, "contractEntry": …}`).
    ///
    /// Unlike [`Self::active_contracts`], this is not capped by the node's list
    /// limit.
    ///
    /// # Errors
    /// Returns an [`Error`] if the handshake fails; the stream yields `Err` on a
    /// participant error frame or a transport failure.
    #[allow(clippy::large_futures)] // the WS handshake state is inherently large; awaited once.
    #[cfg_attr(docsrs, doc(cfg(feature = "ws")))]
    pub async fn ws_active_contracts(
        &self,
        parties: Vec<String>,
        active_at_offset: i64,
    ) -> Result<impl futures_core::Stream<Item = Result<Value>> + Send + use<>> {
        telemetry::instrument("ws_active_contracts", TRANSPORT_JSON, async move {
            let request = active_contracts_request(&parties, active_at_offset);
            let inner =
                crate::ws::subscribe(&self.ws_transport(), "/v2/state/active-contracts", request)
                    .await?;
            Ok(telemetry::instrument_stream(
                "ws_active_contracts",
                TRANSPORT_JSON,
                inner,
            ))
        })
        .await
    }

    /// Like [`Self::ws_active_contracts`], with the full request surface of an
    /// [`ActiveContractsRequest`](crate::request::ActiveContractsRequest) —
    /// the same builder the gRPC lane takes.
    ///
    /// # Errors
    /// Returns an [`Error`] if the handshake fails; the stream yields `Err` on
    /// a participant error frame or a transport failure.
    #[allow(clippy::large_futures)] // the WS handshake state is inherently large; awaited once.
    #[cfg_attr(docsrs, doc(cfg(feature = "ws")))]
    pub async fn ws_active_contracts_with(
        &self,
        request: &crate::request::ActiveContractsRequest,
    ) -> Result<impl futures_core::Stream<Item = Result<Value>> + Send + use<>> {
        telemetry::instrument("ws_active_contracts", TRANSPORT_JSON, async move {
            let inner = crate::ws::subscribe(
                &self.ws_transport(),
                "/v2/state/active-contracts",
                request.json_body(),
            )
            .await?;
            Ok(telemetry::instrument_stream(
                "ws_active_contracts",
                TRANSPORT_JSON,
                inner,
            ))
        })
        .await
    }

    /// Stream command completions over WebSocket (feature `ws`) for `parties`,
    /// starting after `begin_exclusive`. Each item is a raw JSON completion;
    /// `OffsetCheckpoint` heartbeats are filtered out.
    ///
    /// # Errors
    /// Returns an [`Error`] if the handshake fails; the stream yields `Err` on a
    /// participant error frame or a transport failure.
    #[allow(clippy::large_futures)] // the WS handshake state is inherently large; awaited once.
    #[cfg_attr(docsrs, doc(cfg(feature = "ws")))]
    pub async fn ws_completions(
        &self,
        parties: Vec<String>,
        begin_exclusive: i64,
    ) -> Result<impl futures_core::Stream<Item = Result<Value>> + Send + use<>> {
        telemetry::instrument("ws_completions", TRANSPORT_JSON, async move {
            let request = completions_request(&parties, begin_exclusive);
            let inner = crate::ws::subscribe(
                &self.ws_transport(),
                "/v2/commands/command-completions",
                request,
            )
            .await?;
            Ok(telemetry::instrument_stream(
                "ws_completions",
                TRANSPORT_JSON,
                crate::ws::filter_checkpoints(inner),
            ))
        })
        .await
    }

    /// Like [`Self::ws_completions`], with the full request surface of a
    /// [`CompletionsRequest`](crate::request::CompletionsRequest) — including
    /// the submitting `user_id` to scope the stream to (the same builder the
    /// gRPC lane takes).
    ///
    /// # Errors
    /// Returns an [`Error`] if the handshake fails; the stream yields `Err` on
    /// a participant error frame or a transport failure.
    #[allow(clippy::large_futures)] // the WS handshake state is inherently large; awaited once.
    #[cfg_attr(docsrs, doc(cfg(feature = "ws")))]
    pub async fn ws_completions_with(
        &self,
        request: &crate::request::CompletionsRequest,
    ) -> Result<impl futures_core::Stream<Item = Result<Value>> + Send + use<>> {
        telemetry::instrument("ws_completions", TRANSPORT_JSON, async move {
            let inner = crate::ws::subscribe(
                &self.ws_transport(),
                "/v2/commands/command-completions",
                request.json_body(),
            )
            .await?;
            Ok(telemetry::instrument_stream(
                "ws_completions",
                TRANSPORT_JSON,
                crate::ws::filter_checkpoints(inner),
            ))
        })
        .await
    }

    /// The reconnect policy for the WebSocket streams: `(max_reconnects,
    /// backoff_unit)`, taken from this client's
    /// [`RetryConfig`](canton_core::RetryConfig) when one is configured — the
    /// same derivation the gRPC client makes, so configuring retries once
    /// governs both lanes rather than only the unary one.
    #[cfg(feature = "ws")]
    fn reconnect_policy(&self) -> (u32, std::time::Duration) {
        match &self.retry {
            Some(retry) => (retry.max_attempts, retry.initial_backoff),
            None => (5, std::time::Duration::from_millis(250)),
        }
    }

    /// Like [`Self::ws_active_contracts`], but **resumable**: on a retriable
    /// disconnect it resubscribes from the last continuation token the
    /// participant sent, rather than starting the snapshot again.
    ///
    /// An ACS snapshot has no offsets to resume from — it is a position in a
    /// stream of entries, which is what `streamContinuationToken` names. The
    /// gRPC lane has had a resumable ACS read since M1; this is its counterpart,
    /// and without it a WebSocket consumer of a large snapshot had to start over
    /// on any blip.
    ///
    /// The token is only valid against the same participant, the same
    /// `active_at_offset` and the same filters — all of which are fixed for the
    /// life of this stream — and while the snapshot's offset has not been
    /// pruned.
    #[cfg(feature = "ws")]
    #[cfg_attr(docsrs, doc(cfg(feature = "ws")))]
    pub fn ws_active_contracts_resumable(
        &self,
        parties: Vec<String>,
        active_at_offset: i64,
    ) -> impl futures_core::Stream<Item = Result<Value>> + Send + use<> {
        let (max_reconnects, backoff_unit) = self.reconnect_policy();
        let base_url = self.base_url.clone();
        let auth = self.auth.clone();
        let tls = self.tls.clone();
        let max_decoding_message_size = self.max_decoding_message_size;
        let timeout = self.timeout;
        async_stream::stream! {
            let mut token: Option<String> = None;
            let mut reconnects = 0u32;
            loop {
                let mut request = active_contracts_request(&parties, active_at_offset);
                if let Some(token) = &token {
                    request["streamContinuationToken"] = Value::String(token.clone());
                }
                let transport = crate::ws::WsTransport {
                    base_url: &base_url,
                    auth: &auth,
                    tls: tls.as_ref(),
                    max_decoding_message_size,
                    timeout,
                };
                // What made this reconnect necessary, carried out of the inner
                // loop for the reason the gRPC streams carry it: giving up must
                // report the participant's own failure, not an error of ours.
                let cause = match crate::ws::subscribe(&transport, "/v2/state/active-contracts", request).await {
                    Ok(inner) => {
                        // Each connection is instrumented for its own life, so a
                        // subscription that dies mid-snapshot is counted rather
                        // than disappearing into the reconnect loop.
                        let inner = telemetry::instrument_stream(
                            "ws_active_contracts",
                            TRANSPORT_JSON,
                            inner,
                        );
                        tokio::pin!(inner);
                        loop {
                            match inner.next().await {
                                Some(Ok(frame)) => {
                                    if let Some(next) = frame
                                        .get("streamContinuationToken")
                                        .and_then(Value::as_str)
                                        .filter(|next| !next.is_empty())
                                    {
                                        token = Some(next.to_string());
                                    }
                                    reconnects = 0;
                                    yield Ok(frame);
                                }
                                Some(Err(err)) if err.is_retriable() => break err,
                                Some(Err(err)) => {
                                    yield Err(err);
                                    return;
                                }
                                // A bounded read: the participant closes the
                                // socket when the snapshot is delivered, and
                                // there is no token left to resume from.
                                None => return,
                            }
                        }
                    }
                    Err(err) if err.is_retriable() => err,
                    Err(err) => {
                        yield Err(err);
                        return;
                    }
                };

                reconnects += 1;
                if reconnects > max_reconnects {
                    tracing::warn!(
                        max_reconnects,
                        "ws acs stream gave up resuming; reporting the failure that caused it"
                    );
                    yield Err(cause);
                    return;
                }
                tokio::time::sleep(backoff_unit * reconnects).await;
            }
        }
    }

    /// Like [`Self::ws_updates`] (unbounded tail), but **resumable**: on a
    /// retriable disconnect it reconnects from the last offset it observed
    /// (tracked via `OffsetCheckpoint` heartbeats and update offsets), with a
    /// short backoff and a bounded number of consecutive reconnects. Mirrors the
    /// gRPC [`CantonClient::updates_resumable`]. Checkpoints are consumed for
    /// position tracking and not yielded.
    ///
    /// [`CantonClient::updates_resumable`]: crate::CantonClient::updates_resumable
    #[cfg_attr(docsrs, doc(cfg(feature = "ws")))]
    pub fn ws_updates_resumable(
        &self,
        parties: Vec<String>,
        begin_exclusive: i64,
    ) -> impl futures_core::Stream<Item = Result<Value>> + Send + use<> {
        let (max_reconnects, backoff_unit) = self.reconnect_policy();
        let base_url = self.base_url.clone();
        let auth = self.auth.clone();
        let tls = self.tls.clone();
        let max_decoding_message_size = self.max_decoding_message_size;
        let timeout = self.timeout;
        async_stream::stream! {
            let mut offset = begin_exclusive;
            let mut reconnects = 0u32;
            loop {
                // Unbounded tail (no end): a close means the connection dropped.
                let request = updates_request(&parties, offset, None);
                let transport = crate::ws::WsTransport {
                    base_url: &base_url,
                    auth: &auth,
                    tls: tls.as_ref(),
                    max_decoding_message_size,
                    timeout,
                };
                // `Option`, unlike the other resumable streams: here a clean
                // WS close is also a reason to reconnect, and it carries no
                // failure to report. Everything else does.
                let cause = match crate::ws::subscribe(&transport, "/v2/updates", request).await {
                    Ok(inner) => {
                        let inner =
                            telemetry::instrument_stream("ws_updates", TRANSPORT_JSON, inner);
                        tokio::pin!(inner);
                        loop {
                            match inner.next().await {
                                Some(Ok(frame)) => {
                                    if let Some(o) = crate::ws::update_offset(&frame) {
                                        offset = o;
                                    }
                                    reconnects = 0;
                                    if !crate::ws::is_offset_checkpoint(&frame) {
                                        yield Ok(frame);
                                    }
                                }
                                Some(Err(err)) if err.is_retriable() => break Some(err),
                                Some(Err(err)) => {
                                    yield Err(err);
                                    return;
                                }
                                None => break None, // WS closed → reconnect from `offset`
                            }
                        }
                    }
                    Err(err) if err.is_retriable() => Some(err),
                    Err(err) => {
                        yield Err(err);
                        return;
                    }
                };

                reconnects += 1;
                if reconnects > max_reconnects {
                    tracing::warn!(
                        max_reconnects,
                        offset,
                        "ws update stream gave up resuming; reporting the failure that caused it"
                    );
                    // No cause means the participant kept closing the socket
                    // cleanly and the stream never got anywhere — which is not
                    // a participant error, so it is described as what it is.
                    yield Err(cause.unwrap_or_else(|| Error::UnexpectedResponse(format!(
                        "ws update stream was closed and reopened {max_reconnects} times \
                         without delivering an update"
                    ))));
                    return;
                }
                tokio::time::sleep(backoff_unit * reconnects).await;
            }
        }
    }
}

/// Classify a failure to *send* a JSON request.
///
/// This sits inside [`canton_core::retry::run_with_retry`], which loops while
/// `is_retriable()` — and `Error::Connection` always is. Reporting every
/// transport failure that way meant an unverifiable certificate, a malformed
/// URL and a redirect loop each ran the full retry schedule and then reported
/// `error sending request for url (…)`, with the word *certificate* left in the
/// source chain nobody read.
fn transport_error(path: &str, e: &reqwest::Error) -> Error {
    let detail = canton_core::chain(e);
    if e.is_timeout() {
        return Error::Timeout;
    }
    if e.is_builder() || e.is_redirect() {
        return Error::InvalidRequest(format!("the request to {path} could not be made: {detail}"));
    }
    if e.is_decode() {
        return Error::UnexpectedResponse(format!(
            "{path} returned a body this cannot read: {detail}"
        ));
    }
    // A TLS failure arrives as a connect error, indistinguishable by type from
    // a refused connection — reqwest exposes no typed TLS error — so the chain
    // is the only place it shows. Permanent, so not retriable.
    if detail.to_ascii_lowercase().contains("certificate") {
        return Error::InvalidRequest(format!(
            "the certificate presented for {path} could not be verified: {detail}"
        ));
    }
    Error::Connection(format!("json request to {path} failed: {detail}"))
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

    #[test]
    fn updates_request_matches_grpc_and_includes_reassignments() {
        let parties = vec!["alice::1".to_string()];
        let body = updates_request(&parties, 10, Some(20));
        assert_eq!(body["beginExclusive"], 10);
        assert_eq!(body["endInclusive"], 20);
        let fmt = &body["updateFormat"];
        // Both sub-formats present — same event set as the gRPC lane, which sets
        // include_transactions AND include_reassignments.
        assert!(fmt["includeTransactions"].is_object(), "{body}");
        assert!(
            fmt["includeReassignments"].is_object(),
            "reassignments must be requested, or the JSON lane drops them: {body}"
        );
        assert_eq!(
            fmt["includeTransactions"]["transactionShape"],
            "TRANSACTION_SHAPE_LEDGER_EFFECTS"
        );
    }

    #[test]
    fn commands_serialize_to_the_json_api_shape() {
        let commands = JsonCommands::new(vec!["alice::1".to_string()])
            .with_command_id("cmd-1")
            .add_create("pkg:Mod:Ent", json!({ "owner": "alice::1" }));
        let value = serde_json::to_value(&commands).unwrap();

        assert_eq!(value["commandId"], "cmd-1");
        assert_eq!(value["actAs"][0], "alice::1");
        // Tagged CreateCommand with a Daml-LF-JSON record argument.
        assert_eq!(
            value["commands"][0]["CreateCommand"]["templateId"],
            "pkg:Mod:Ent"
        );
        assert_eq!(
            value["commands"][0]["CreateCommand"]["createArguments"]["owner"],
            "alice::1"
        );
        // Optional fields are omitted, not null.
        assert!(value.get("userId").is_none());
        assert!(value.get("readAs").is_none());
    }

    #[test]
    fn all_command_options_serialize_to_camel_case() {
        let commands = JsonCommands::new(vec!["alice::1".to_string()])
            .with_command_id("cmd-1")
            .with_user_id("user-1")
            .with_read_as(vec!["bob::2".to_string()])
            .with_workflow_id("wf-1")
            .with_synchronizer_id("sync-1")
            .with_submission_id("sub-1")
            .add_disclosed_contract(json!({ "contractId": "c9", "createdEventBlob": "AQI=" }))
            .with_package_id_selection_preference(vec!["pkg-9".to_string()])
            .with_deduplication_period(
                json!({ "DeduplicationDuration": { "value": { "duration": "30s" } } }),
            )
            .with_min_ledger_time_rel(json!("5s"))
            .add_create("pkg:Mod:Ent", json!({ "owner": "alice::1" }))
            .add_command(json!({ "ExerciseCommand": { "contractId": "c1" } }));
        let value = serde_json::to_value(&commands).unwrap();

        assert_eq!(value["userId"], "user-1");
        assert_eq!(value["readAs"][0], "bob::2");
        assert_eq!(value["workflowId"], "wf-1");
        assert_eq!(value["synchronizerId"], "sync-1");
        assert_eq!(value["submissionId"], "sub-1");
        assert_eq!(value["disclosedContracts"][0]["contractId"], "c9");
        assert_eq!(value["packageIdSelectionPreference"][0], "pkg-9");
        assert_eq!(
            value["deduplicationPeriod"]["DeduplicationDuration"]["value"]["duration"],
            "30s"
        );
        assert_eq!(value["minLedgerTimeRel"], "5s");
        assert!(value.get("minLedgerTimeAbs").is_none());
        // Both the convenience create and the raw command are present, in order.
        assert!(value["commands"][0]["CreateCommand"].is_object());
        assert_eq!(value["commands"][1]["ExerciseCommand"]["contractId"], "c1");
    }

    #[test]
    fn wildcard_event_format_filters_each_party() {
        let format = &active_contracts_request(&["alice::1".to_string(), "bob::2".to_string()], 0)
            ["eventFormat"];
        assert_eq!(format["verbose"], true);
        assert!(format["filtersByParty"]["alice::1"]["cumulative"][0]["identifierFilter"]
            ["WildcardFilter"]
            .is_object());
        assert!(format["filtersByParty"]["bob::2"].is_object());
    }

    #[test]
    fn with_limit_appends_only_when_set() {
        assert_eq!(with_limit("/v2/updates", None), "/v2/updates");
        assert_eq!(with_limit("/v2/updates", Some(5)), "/v2/updates?limit=5");
    }

    /// A participant that accepts the connection and then says nothing is the
    /// case no status code covers, and `reqwest` waits for it forever unless
    /// told not to. The gRPC lane has bounded this at 30s since M1; this one
    /// had no bound and no knob.
    ///
    /// The listener here accepts and never answers, which is the only way to
    /// tell a timeout that works from a request that happened to be fast.
    #[tokio::test]
    async fn a_request_to_a_silent_participant_gives_up_instead_of_hanging() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        // Hold every accepted connection open, answering nothing.
        tokio::spawn(async move {
            let mut held = Vec::new();
            while let Ok((socket, _)) = listener.accept().await {
                held.push(socket);
            }
        });

        let client = JsonClient::new(format!("http://{addr}"))
            .with_timeout(std::time::Duration::from_millis(250));

        // The outer bound is what makes this a test rather than a hang: with no
        // per-request timeout the call never returns at all, and a CI job that
        // stops responding says less than one that fails.
        let outcome =
            tokio::time::timeout(std::time::Duration::from_secs(5), client.version()).await;
        let Ok(result) = outcome else {
            panic!("the request never returned — the per-request timeout is not being applied");
        };

        let error = result.unwrap_err();
        assert!(
            error.is_retriable(),
            "a timeout is transient and should be retriable: {error}"
        );

        // And the default is a real bound, not `None` dressed up as one.
        assert_eq!(
            JsonClient::new("http://localhost:3975").timeout,
            DEFAULT_TIMEOUT
        );
    }

    /// A client is the thing a caller is most likely to put in a `tracing`
    /// field or a `{:?}`, and its base URL is the one place a credential can
    /// hide in plain sight. `Config` has redacted its endpoint since the
    /// mutual-TLS fix; this type held the same secret behind a derived `Debug`.
    #[test]
    fn debug_does_not_print_credentials_carried_in_the_base_url() {
        let secret = "s3cr3t-p@ssw0rd";
        let client = JsonClient::new(format!("https://svc-account:{secret}@ledger.example:3975"))
            .with_token("eyJhbGciOiJSUzI1NiJ9.PAYLOAD.SIG");
        let rendered = format!("{client:?}");

        assert!(
            !rendered.contains(secret),
            "leaked the password: {rendered}"
        );
        assert!(
            !rendered.contains("svc-account"),
            "leaked the user: {rendered}"
        );
        assert!(
            !rendered.contains("PAYLOAD"),
            "leaked the token: {rendered}"
        );
        // Still useful: the host has to survive, or the output tells a reader
        // nothing about which participant this client talks to.
        assert!(
            rendered.contains("ledger.example:3975"),
            "should keep the host: {rendered}"
        );
    }

    /// The WS lane must not silently inherit `tungstenite`'s ceiling: 64 MiB
    /// per message and 16 MiB per *frame*, neither of them anything the caller
    /// asked for. A JSON client carries the same limit the gRPC one does, and
    /// raising it is the caller's to do.
    #[test]
    fn the_ws_lane_starts_at_the_sdk_size_limit_not_tungstenites() {
        let client = JsonClient::new("http://localhost:3975");
        assert_eq!(
            client.max_decoding_message_size,
            canton_core::DEFAULT_MAX_DECODING_MESSAGE_SIZE,
        );
        // Well clear of both tungstenite defaults, which is the whole point.
        assert!(client.max_decoding_message_size > 64 << 20);

        let raised = client.with_max_decoding_message_size(256 << 20);
        assert_eq!(raised.max_decoding_message_size, 256 << 20);
    }

    #[test]
    fn command_id_defaults_to_a_generated_uuid() {
        let commands = JsonCommands::new(vec!["alice::1".to_string()]);
        let value = serde_json::to_value(&commands).unwrap();
        let id = value["commandId"].as_str().unwrap();
        assert!(id.starts_with("sdk-"), "got {id}");
        assert!(id.len() > 10, "expected a uuid suffix, got {id}");
    }

    #[test]
    fn with_tls_threads_a_ca_and_client_identity() {
        let ck = rcgen::generate_simple_self_signed(vec!["localhost".to_string()]).unwrap();
        let cert_pem = ck.cert.pem().into_bytes();
        let key_pem = ck.key_pair.serialize_pem().into_bytes();

        // A valid CA + client identity (mTLS) builds an HTTPS client.
        let tls = canton_core::TlsConfig::new()
            .with_ca_certificate(cert_pem.clone())
            .with_client_identity(cert_pem, key_pem);
        assert!(
            JsonClient::new("https://localhost:3975")
                .with_token("t")
                .with_tls(&tls)
                .is_ok()
        );

        // A malformed client-identity PEM is rejected as an InvalidRequest.
        let bad = canton_core::TlsConfig::new()
            .with_client_identity(b"not a pem".to_vec(), b"nor this".to_vec());
        assert!(matches!(
            JsonClient::new("https://localhost:3975").with_tls(&bad),
            Err(Error::InvalidRequest(_))
        ));
    }

    #[test]
    fn with_tls_upgrades_an_http_base_url_to_https() {
        // The security bug: `with_tls` on an http:// base URL would otherwise
        // send plaintext HTTP (certs unused) and open a ws:// socket. The scheme
        // must become https so both the reqwest and WebSocket lanes use TLS.
        let client = JsonClient::new("http://localhost:3975")
            .with_tls(&canton_core::TlsConfig::new())
            .unwrap();
        assert_eq!(client.base_url, "https://localhost:3975");

        // Already-https is left untouched, and the check is case-insensitive.
        assert_eq!(
            upgrade_base_url_for_tls("https://host:443"),
            "https://host:443"
        );
        assert_eq!(
            upgrade_base_url_for_tls("HTTP://host:80"),
            "https://host:80"
        );
        // Without TLS, plain http is preserved (no normalisation on `new`).
        assert_eq!(
            JsonClient::new("http://localhost:3975").base_url,
            "http://localhost:3975"
        );
    }
}
