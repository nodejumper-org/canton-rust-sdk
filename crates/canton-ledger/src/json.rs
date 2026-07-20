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
#[derive(Clone, Debug)]
pub struct JsonClient {
    base_url: String,
    http: reqwest::Client,
    auth: Auth,
    /// Kept for the WebSocket handshake (feature `ws`); the HTTP client bakes
    /// its TLS settings into `http` at `with_tls` time.
    tls: Option<canton_core::TlsConfig>,
}

#[derive(Deserialize)]
struct VersionResponse {
    version: String,
}

#[derive(Deserialize)]
struct LedgerEndResponse {
    offset: i64,
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
        }
    }

    /// Set an explicit change-ID `command_id` (for exactly-once / de-duplication).
    #[must_use]
    pub fn with_command_id(mut self, command_id: impl Into<String>) -> Self {
        self.command_id = command_id.into();
        self
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
fn active_contracts_request(parties: &[String], active_at_offset: i64) -> Value {
    json!({
        "activeAtOffset": active_at_offset,
        "eventFormat": wildcard_event_format(parties),
    })
}

/// The request body for updates over `(begin_exclusive, end_inclusive]` (POST
/// and WS); omit `end_inclusive` for an unbounded tail. Uses the
/// `LEDGER_EFFECTS` transaction shape — the same as the gRPC lane — so both
/// transports yield the same event set for the same query.
fn updates_request(parties: &[String], begin_exclusive: i64, end_inclusive: Option<i64>) -> Value {
    let mut body = json!({
        "beginExclusive": begin_exclusive,
        "updateFormat": {
            "includeTransactions": {
                "eventFormat": wildcard_event_format(parties),
                "transactionShape": "TRANSACTION_SHAPE_LEDGER_EFFECTS",
            }
        }
    });
    if let Some(end) = end_inclusive {
        body["endInclusive"] = json!(end);
    }
    body
}

/// The request body for command completions from `begin_exclusive` (WS).
#[cfg(feature = "ws")]
fn completions_request(parties: &[String], begin_exclusive: i64) -> Value {
    json!({ "parties": parties, "beginExclusive": begin_exclusive })
}

/// An `EventFormat` (JSON) with a wildcard (all-templates) filter per party,
/// mirroring the gRPC client's `wildcard_event_format`.
fn wildcard_event_format(parties: &[String]) -> Value {
    let filters_by_party: serde_json::Map<String, Value> = parties
        .iter()
        .map(|party| {
            (
                party.clone(),
                json!({
                    "cumulative": [{
                        "identifierFilter": {
                            "WildcardFilter": { "value": { "includeCreatedEventBlob": false } }
                        }
                    }]
                }),
            )
        })
        .collect();
    json!({ "filtersByParty": filters_by_party, "verbose": true })
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
        }
    }

    /// Use TLS for the HTTP connection: a custom CA (server-side TLS against a
    /// private/self-signed server) and/or a client identity (mutual TLS). This
    /// is a terminal builder step — call it last, after [`Self::with_token`] /
    /// [`Self::with_oidc`].
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

    async fn get<T: for<'de> Deserialize<'de>>(&self, path: &str) -> Result<T> {
        let mut request = self.http.get(format!("{}{path}", self.base_url));
        if let Some(token) = self.auth.bearer().await? {
            request = request.bearer_auth(token);
        }
        request = with_trace_context(request);
        let response = request
            .send()
            .await
            .map_err(|e| Error::Connection(format!("json request to {path} failed: {e}")))?;
        read_json(response, path).await
    }

    async fn post<B: Serialize, T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let mut request = self
            .http
            .post(format!("{}{path}", self.base_url))
            .json(body);
        if let Some(token) = self.auth.bearer().await? {
            request = request.bearer_auth(token);
        }
        request = with_trace_context(request);
        let response = request
            .send()
            .await
            .map_err(|e| Error::Connection(format!("json request to {path} failed: {e}")))?;
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
    pub async fn ws_updates(
        &self,
        parties: Vec<String>,
        begin_exclusive: i64,
        end_inclusive: Option<i64>,
    ) -> Result<impl futures_core::Stream<Item = Result<Value>> + Send + use<>> {
        telemetry::instrument("ws_updates", TRANSPORT_JSON, async move {
            let request = updates_request(&parties, begin_exclusive, end_inclusive);
            let inner = crate::ws::subscribe(
                &self.base_url,
                &self.auth,
                self.tls.as_ref(),
                "/v2/updates",
                request,
            )
            .await?;
            Ok(crate::ws::filter_checkpoints(inner))
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
    pub async fn ws_active_contracts(
        &self,
        parties: Vec<String>,
        active_at_offset: i64,
    ) -> Result<impl futures_core::Stream<Item = Result<Value>> + Send + use<>> {
        telemetry::instrument("ws_active_contracts", TRANSPORT_JSON, async move {
            let request = active_contracts_request(&parties, active_at_offset);
            crate::ws::subscribe(
                &self.base_url,
                &self.auth,
                self.tls.as_ref(),
                "/v2/state/active-contracts",
                request,
            )
            .await
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
    pub async fn ws_completions(
        &self,
        parties: Vec<String>,
        begin_exclusive: i64,
    ) -> Result<impl futures_core::Stream<Item = Result<Value>> + Send + use<>> {
        telemetry::instrument("ws_completions", TRANSPORT_JSON, async move {
            let request = completions_request(&parties, begin_exclusive);
            let inner = crate::ws::subscribe(
                &self.base_url,
                &self.auth,
                self.tls.as_ref(),
                "/v2/commands/command-completions",
                request,
            )
            .await?;
            Ok(crate::ws::filter_checkpoints(inner))
        })
        .await
    }

    /// Like [`Self::ws_updates`] (unbounded tail), but **resumable**: on a
    /// retriable disconnect it reconnects from the last offset it observed
    /// (tracked via `OffsetCheckpoint` heartbeats and update offsets), with a
    /// short backoff and a bounded number of consecutive reconnects. Mirrors the
    /// gRPC [`CantonClient::updates_resumable`]. Checkpoints are consumed for
    /// position tracking and not yielded.
    ///
    /// [`CantonClient::updates_resumable`]: crate::CantonClient::updates_resumable
    pub fn ws_updates_resumable(
        &self,
        parties: Vec<String>,
        begin_exclusive: i64,
    ) -> impl futures_core::Stream<Item = Result<Value>> + Send + use<> {
        const MAX_RECONNECTS: u32 = 5;
        let base_url = self.base_url.clone();
        let auth = self.auth.clone();
        let tls = self.tls.clone();
        async_stream::stream! {
            let mut offset = begin_exclusive;
            let mut reconnects = 0u32;
            loop {
                // Unbounded tail (no end): a close means the connection dropped.
                let request = updates_request(&parties, offset, None);
                match crate::ws::subscribe(&base_url, &auth, tls.as_ref(), "/v2/updates", request).await {
                    Ok(inner) => {
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
                                Some(Err(err)) if err.is_retriable() => break,
                                Some(Err(err)) => {
                                    yield Err(err);
                                    return;
                                }
                                None => break, // WS closed → reconnect from `offset`
                            }
                        }
                    }
                    Err(err) if err.is_retriable() => {}
                    Err(err) => {
                        yield Err(err);
                        return;
                    }
                }

                reconnects += 1;
                if reconnects > MAX_RECONNECTS {
                    yield Err(Error::UnexpectedResponse(format!(
                        "ws update stream failed to resume after {MAX_RECONNECTS} reconnects"
                    )));
                    return;
                }
                tokio::time::sleep(std::time::Duration::from_millis(250 * u64::from(reconnects)))
                    .await;
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use super::*;

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
            .add_create("pkg:Mod:Ent", json!({ "owner": "alice::1" }))
            .add_command(json!({ "ExerciseCommand": { "contractId": "c1" } }));
        let value = serde_json::to_value(&commands).unwrap();

        assert_eq!(value["userId"], "user-1");
        assert_eq!(value["readAs"][0], "bob::2");
        assert_eq!(value["workflowId"], "wf-1");
        assert_eq!(value["synchronizerId"], "sync-1");
        // Both the convenience create and the raw command are present, in order.
        assert!(value["commands"][0]["CreateCommand"].is_object());
        assert_eq!(value["commands"][1]["ExerciseCommand"]["contractId"], "c1");
    }

    #[test]
    fn wildcard_event_format_filters_each_party() {
        let format = wildcard_event_format(&["alice::1".to_string(), "bob::2".to_string()]);
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
}
