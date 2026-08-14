//! The registry's off-ledger HTTP API.
//!
//! Every path and payload here comes from the token standard's OpenAPI
//! documents — `token-metadata-v1`, `transfer-instruction-v1`,
//! `allocation-v1`, `allocation-instruction-v1` — and the method names follow
//! their `operationId`s, so a reader can match one to the other.
//!
//! # What the registry is for
//!
//! Two things. It *describes* the instruments it issues (the metadata API),
//! and it supplies the **choice context** for exercising a token-standard
//! choice: the reference data and the contracts a participant must be shown.
//! A client cannot assemble that itself — the contracts are the registry's own
//! and usually invisible to the submitting party.

use canton_core::{Error, Result};
use serde::{Deserialize, Serialize};

use crate::context::{ChoiceContext, ChoiceContextRequest, WireChoiceContext};

/// A client for one registry's off-ledger API.
/// How long to wait on a registry that accepts the connection and then says
/// nothing. `reqwest`'s own default is unbounded, which turns an unresponsive
/// registry into a transfer that never returns.
const DEFAULT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);

/// A client for one registry's off-ledger API.
#[derive(Clone, Debug)]
pub struct RegistryClient {
    base: reqwest::Url,
    http: reqwest::Client,
}

/// What a registry says about itself.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegistryInfo {
    /// The party that administers the instruments — the `expectedAdmin` a
    /// factory choice is exercised against.
    pub admin_id: String,
    /// Which token-standard APIs this registry implements.
    #[serde(default)]
    pub supported_apis: std::collections::BTreeMap<String, i32>,
}

/// An instrument the registry issues.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instrument {
    /// The instrument id, as it appears in a `Holding`.
    pub id: String,
    /// Human-readable name.
    pub name: String,
    /// Ticker-style symbol.
    pub symbol: String,
    /// Total supply, when the registry publishes one.
    #[serde(default)]
    pub total_supply: Option<String>,
    /// When `total_supply` was measured.
    #[serde(default)]
    pub total_supply_as_of: Option<String>,
    /// How many decimal places amounts of this instrument carry.
    ///
    /// The standard marks this required with a default of 10, so an absent
    /// field means ten places — not "unknown". Reporting it as unknown pushes a
    /// display-precision decision about money onto a caller who has no better
    /// information than this default.
    #[serde(default = "default_decimals")]
    pub decimals: i32,
    /// Which token-standard APIs apply to this instrument.
    #[serde(default)]
    pub supported_apis: std::collections::BTreeMap<String, i32>,
}

/// The standard's default for `decimals`.
fn default_decimals() -> i32 {
    10
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ListInstrumentsResponse {
    #[serde(default)]
    instruments: Vec<Instrument>,
    #[serde(default)]
    next_page_token: Option<String>,
}

/// How a transfer will be carried out, as the registry decides.
///
/// Not a caller's choice: it follows from whether the receiver has pre-approved
/// direct transfers, and whether sender and receiver are the same party. A
/// client that assumes `Direct` and gets `Offer` has a transfer that is
/// pending someone else's acceptance, which is a different workflow.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum TransferKind {
    /// Sender and receiver are the same party; no approval, usually immediate.
    #[serde(rename = "self")]
    SelfTransfer,
    /// The receiver has pre-approved direct transfers, so it completes at once.
    Direct,
    /// The receiver is offered the transfer and it completes only if accepted.
    Offer,
    /// A kind this build does not know.
    ///
    /// Without this, a registry that adds a fourth kind makes the whole factory
    /// response fail to decode and *every* transfer stop — including the ones
    /// whose kind is perfectly understood. `#[non_exhaustive]` promises a
    /// caller their match will keep compiling; this is what makes the promise
    /// reach runtime.
    #[serde(other)]
    Unknown,
}

/// A factory contract together with the context for exercising its choice.
#[derive(Clone, Debug)]
pub struct FactoryWithContext {
    /// The contract id of the contract implementing the factory interface.
    pub factory_id: String,
    /// How the transfer will be carried out. `None` for factories whose API
    /// does not classify the workflow — the allocation factory, for one.
    pub transfer_kind: Option<TransferKind>,
    /// The context for the factory's choice.
    pub context: ChoiceContext,
}

/// `TransferFactoryWithChoiceContext` — `transferKind` is `required` here.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WireTransferFactory {
    factory_id: String,
    transfer_kind: TransferKind,
    choice_context: WireChoiceContext,
}

/// `FactoryWithChoiceContext` — the allocation API has no `transferKind` at
/// all. Kept separate from the transfer factory so that "this API does not
/// classify the workflow" and "a registry omitted a required field" cannot
/// arrive as the same value.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WireFactory {
    factory_id: String,
    choice_context: WireChoiceContext,
}

/// A request for a factory.
///
/// `choice_arguments` is the choice as the Daml JSON API encodes it, **with
/// `extraArgs.context` and `extraArgs.meta` empty** — the standard says so
/// explicitly, and it is what lets a registry tailor the context to the actual
/// arguments (the configuration for one instrument id, say).
///
/// So the argument is a *generated* choice type, serialized. Nothing here
/// hand-writes the JSON.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct GetFactoryRequest<'a> {
    choice_arguments: &'a serde_json::Value,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    exclude_debug_fields: bool,
}

impl RegistryClient {
    /// A client for the registry served at `base_url`.
    ///
    /// The URL is parsed here rather than at the first request, so a typo is a
    /// configuration error the caller sees immediately — and not, as it was, a
    /// `Connection` failure that the SDK reports as retriable and an
    /// application retries forever.
    ///
    /// # Errors
    /// [`Error::InvalidRequest`] if `base_url` is not a URL, or an HTTP client
    /// cannot be built.
    pub fn new(base_url: &str) -> Result<Self> {
        let http = reqwest::Client::builder()
            .timeout(DEFAULT_TIMEOUT)
            .build()
            .map_err(|e| Error::InvalidRequest(format!("cannot build an HTTP client: {e}")))?;
        Self::with_http_client(base_url, http)
    }

    /// A client using an HTTP client the caller has already configured —
    /// timeouts, proxies, a custom TLS root store.
    ///
    /// # Errors
    /// [`Error::InvalidRequest`] if `base_url` is not a URL.
    pub fn with_http_client(base_url: &str, http: reqwest::Client) -> Result<Self> {
        // A trailing slash matters to `Url::join`, and the paths below are
        // relative, so it is normalised here once.
        let normalised = format!("{}/", base_url.trim_end_matches('/'));
        let base = reqwest::Url::parse(&normalised).map_err(|e| {
            Error::InvalidRequest(format!("`{base_url}` is not a registry URL: {e}"))
        })?;
        Ok(Self { base, http })
    }

    /// Build a URL from path segments.
    ///
    /// Every segment is **percent-encoded**. Interpolating them into a string
    /// let a caller-supplied id change which resource was addressed: an
    /// instrument id of `a?b` fetched instrument `a` and returned its name,
    /// symbol and decimals with no error at all, and `../../info` walked out of
    /// the collection entirely. Instrument ids are admin-assigned free-form
    /// strings, so nothing upstream rules that out.
    fn url(&self, segments: &[&str]) -> Result<reqwest::Url> {
        let mut url = self.base.clone();
        url.path_segments_mut()
            .map_err(|()| Error::InvalidRequest("the registry URL cannot have a path".to_string()))?
            .pop_if_empty()
            .extend(segments);
        Ok(url)
    }

    /// The registry's own description of itself — including `admin_id`, which
    /// is the `expectedAdmin` a factory choice names.
    ///
    /// # Errors
    /// As any HTTP call, plus [`Error::UnexpectedResponse`] on a body that does
    /// not match the standard.
    pub async fn info(&self) -> Result<RegistryInfo> {
        self.get(self.url(&["registry", "metadata", "v1", "info"])?)
            .await
    }

    /// One page of the instruments this registry issues.
    ///
    /// Returns the page and the token for the next one, or `None` at the end.
    ///
    /// # Errors
    /// As [`info`](Self::info).
    pub async fn list_instruments(
        &self,
        page_size: Option<u32>,
        page_token: Option<&str>,
    ) -> Result<(Vec<Instrument>, Option<String>)> {
        let mut url = self.url(&["registry", "metadata", "v1", "instruments"])?;
        {
            // Encoded, not formatted: a page token is an instrument id by the
            // standard's own definition, so one containing `&` would otherwise
            // split into extra query parameters.
            let mut query = url.query_pairs_mut();
            if let Some(size) = page_size {
                query.append_pair("pageSize", &size.to_string());
            }
            if let Some(token) = page_token {
                query.append_pair("pageToken", token);
            }
        }
        let response: ListInstrumentsResponse = self.get(url).await?;
        Ok((response.instruments, response.next_page_token))
    }

    /// One instrument by id, or `None` if this registry does not issue it.
    ///
    /// # Errors
    /// As [`info`](Self::info). A `404` is not an error here — not issuing an
    /// instrument is an answer — but only when the *registry* is the one
    /// answering it. A `404` whose body is not JSON did not come from a
    /// registry handler at all: it is a base URL one path component off, or a
    /// gateway in front of one, and reporting that as "this registry does not
    /// issue it" is the worst of both worlds. A wallet polling for an
    /// instrument would see a steady, quiet `None` while every other call on
    /// the same client failed loudly.
    pub async fn instrument(&self, instrument_id: &str) -> Result<Option<Instrument>> {
        let url = self.url(&["registry", "metadata", "v1", "instruments", instrument_id])?;
        let response = self
            .http
            .get(url.clone())
            .send()
            .await
            .map_err(|e| connection(&e))?;
        if response.status() == reqwest::StatusCode::NOT_FOUND {
            let body = response.text().await.unwrap_or_default();
            let trimmed = body.trim();
            if trimmed.is_empty() || serde_json::from_str::<serde_json::Value>(trimmed).is_ok() {
                return Ok(None);
            }
            return Err(Error::Http {
                status: 404,
                body: format!(
                    "{url}: a 404 that is not the registry's own — check the base URL: {}",
                    truncate(trimmed)
                ),
            });
        }
        Ok(Some(Self::decode(response, url.as_str()).await?))
    }

    /// Resolve the transfer factory for a transfer, with its choice context.
    ///
    /// `choice_arguments` is a serialized `TransferFactory_Transfer` with empty
    /// `extraArgs` — see [`crate::transfer()`], which builds it.
    ///
    /// # Errors
    /// As [`info`](Self::info).
    pub async fn transfer_factory(
        &self,
        choice_arguments: &serde_json::Value,
    ) -> Result<FactoryWithContext> {
        let url = self.url(&["registry", "transfer-instruction", "v1", "transfer-factory"])?;
        let wire: WireTransferFactory = self.post_factory(url, choice_arguments).await?;
        Ok(FactoryWithContext {
            factory_id: wire.factory_id,
            transfer_kind: Some(wire.transfer_kind),
            context: ChoiceContext::from_wire(wire.choice_context)?,
        })
    }

    /// Resolve the allocation factory, with its choice context.
    ///
    /// # Errors
    /// As [`info`](Self::info).
    pub async fn allocation_factory(
        &self,
        choice_arguments: &serde_json::Value,
    ) -> Result<FactoryWithContext> {
        let url = self.url(&[
            "registry",
            "allocation-instruction",
            "v1",
            "allocation-factory",
        ])?;
        let wire: WireFactory = self.post_factory(url, choice_arguments).await?;
        Ok(FactoryWithContext {
            factory_id: wire.factory_id,
            transfer_kind: None,
            context: ChoiceContext::from_wire(wire.choice_context)?,
        })
    }

    // ---- CIP-0112 (V2) ------------------------------------------------------
    //
    // Separate methods rather than a version flag on the ones above: the V2
    // paths are not the V1 paths with a digit changed, and the shapes differ
    // too. Allocation V2 has no `execute-transfer` context at all — settlement
    // moved to a factory — and allocation-instruction V2 gained `accept` and
    // `withdraw`, which V1 has not. A flag would have made those differences
    // silent.

    /// Resolve the **V2** transfer factory, with its choice context.
    ///
    /// `POST /registry/transfer-instruction/v2/transfer-factory`
    ///
    /// # Errors
    /// As [`info`](Self::info).
    pub async fn transfer_factory_v2(
        &self,
        choice_arguments: &serde_json::Value,
    ) -> Result<FactoryWithContext> {
        let url = self.url(&["registry", "transfer-instruction", "v2", "transfer-factory"])?;
        let wire: WireTransferFactory = self.post_factory(url, choice_arguments).await?;
        Ok(FactoryWithContext {
            factory_id: wire.factory_id,
            transfer_kind: Some(wire.transfer_kind),
            context: ChoiceContext::from_wire(wire.choice_context)?,
        })
    }

    /// Resolve the **V2** allocation factory.
    ///
    /// `POST /registry/allocation-instruction/v2/allocation-factory`
    ///
    /// # Errors
    /// As [`info`](Self::info).
    pub async fn allocation_factory_v2(
        &self,
        choice_arguments: &serde_json::Value,
    ) -> Result<FactoryWithContext> {
        let url = self.url(&[
            "registry",
            "allocation-instruction",
            "v2",
            "allocation-factory",
        ])?;
        let wire: WireFactory = self.post_factory(url, choice_arguments).await?;
        Ok(FactoryWithContext {
            factory_id: wire.factory_id,
            transfer_kind: None,
            context: ChoiceContext::from_wire(wire.choice_context)?,
        })
    }

    /// Resolve the **settlement factory** — how a V2 allocation is settled.
    ///
    /// `POST /registry/allocation/v2/settlement-factory`
    ///
    /// Note the **singular** `allocation` here against the plural
    /// `allocations` of the choice-context paths below. That is what the
    /// specification says, inconsistent as it looks, and a client that
    /// regularises it gets a 404 from a registry that is working correctly.
    ///
    /// V1 has no equivalent: there, an allocation is executed through a choice
    /// context on the allocation itself. V2 settles a *batch* through this
    /// factory instead, which is what lets both legs of a delivery-versus-
    /// payment settle together.
    ///
    /// # Errors
    /// As [`info`](Self::info).
    pub async fn settlement_factory_v2(
        &self,
        choice_arguments: &serde_json::Value,
    ) -> Result<FactoryWithContext> {
        let url = self.url(&["registry", "allocation", "v2", "settlement-factory"])?;
        let wire: WireFactory = self.post_factory(url, choice_arguments).await?;
        Ok(FactoryWithContext {
            factory_id: wire.factory_id,
            transfer_kind: None,
            context: ChoiceContext::from_wire(wire.choice_context)?,
        })
    }

    /// The **V2** context for a choice on an existing transfer instruction.
    ///
    /// `POST /registry/transfer-instruction/v2/{id}/choice-contexts/{choice}`
    ///
    /// # Errors
    /// As [`info`](Self::info).
    pub async fn transfer_instruction_context_v2(
        &self,
        transfer_instruction_id: &str,
        choice: TransferInstructionChoice,
        request: &ChoiceContextRequest,
    ) -> Result<ChoiceContext> {
        self.choice_context(
            self.url(&[
                "registry",
                "transfer-instruction",
                "v2",
                transfer_instruction_id,
                "choice-contexts",
                choice.path(),
            ])?,
            request,
        )
        .await
    }

    /// The **V2** context for a choice on an existing allocation.
    ///
    /// `POST /registry/allocations/v2/{id}/choice-contexts/{choice}` — plural,
    /// unlike the settlement factory above.
    ///
    /// # Errors
    /// As [`info`](Self::info), plus [`Error::InvalidRequest`] for
    /// [`AllocationChoice::ExecuteTransfer`], which V2 does not have: settling
    /// goes through [`settlement_factory_v2`](Self::settlement_factory_v2).
    /// Refusing locally names the reason; the registry would answer 404.
    pub async fn allocation_context_v2(
        &self,
        allocation_id: &str,
        choice: AllocationChoice,
        request: &ChoiceContextRequest,
    ) -> Result<ChoiceContext> {
        if choice == AllocationChoice::ExecuteTransfer {
            return Err(Error::InvalidRequest(
                "V2 allocations are settled through the settlement factory, not an \
                 execute-transfer context — see `settlement_factory_v2`"
                    .to_string(),
            ));
        }
        self.choice_context(
            self.url(&[
                "registry",
                "allocations",
                "v2",
                allocation_id,
                "choice-contexts",
                choice.path(),
            ])?,
            request,
        )
        .await
    }

    /// The **V2** context for a choice on an allocation *instruction*.
    ///
    /// `POST /registry/allocation-instruction/v2/{id}/choice-contexts/{choice}`
    ///
    /// V1 has no such endpoint — an allocation instruction there is driven
    /// entirely through the factory.
    ///
    /// # Errors
    /// As [`info`](Self::info).
    pub async fn allocation_instruction_context_v2(
        &self,
        allocation_instruction_id: &str,
        choice: AllocationInstructionChoice,
        request: &ChoiceContextRequest,
    ) -> Result<ChoiceContext> {
        self.choice_context(
            self.url(&[
                "registry",
                "allocation-instruction",
                "v2",
                allocation_instruction_id,
                "choice-contexts",
                choice.path(),
            ])?,
            request,
        )
        .await
    }

    /// The context for a choice on an existing transfer instruction.
    ///
    /// # Errors
    /// As [`info`](Self::info).
    pub async fn transfer_instruction_context(
        &self,
        transfer_instruction_id: &str,
        choice: TransferInstructionChoice,
        request: &ChoiceContextRequest,
    ) -> Result<ChoiceContext> {
        self.choice_context(
            self.url(&[
                "registry",
                "transfer-instruction",
                "v1",
                transfer_instruction_id,
                "choice-contexts",
                choice.path(),
            ])?,
            request,
        )
        .await
    }

    /// The context for a choice on an existing allocation.
    ///
    /// # Errors
    /// As [`info`](Self::info).
    pub async fn allocation_context(
        &self,
        allocation_id: &str,
        choice: AllocationChoice,
        request: &ChoiceContextRequest,
    ) -> Result<ChoiceContext> {
        self.choice_context(
            self.url(&[
                "registry",
                "allocations",
                "v1",
                allocation_id,
                "choice-contexts",
                choice.path(),
            ])?,
            request,
        )
        .await
    }

    async fn post_factory<T: serde::de::DeserializeOwned>(
        &self,
        url: reqwest::Url,
        choice_arguments: &serde_json::Value,
    ) -> Result<T> {
        let response = self
            .http
            .post(url.clone())
            .json(&GetFactoryRequest {
                choice_arguments,
                // The debug fields are explicitly untrustworthy unless the
                // provider is trusted, and nothing here reads them, so they are
                // bytes on the wire for no purpose.
                exclude_debug_fields: true,
            })
            .send()
            .await
            .map_err(|e| connection(&e))?;
        Self::decode(response, url.as_str()).await
    }

    async fn choice_context(
        &self,
        url: reqwest::Url,
        request: &ChoiceContextRequest,
    ) -> Result<ChoiceContext> {
        let response = self
            .http
            .post(url.clone())
            .json(request)
            .send()
            .await
            .map_err(|e| connection(&e))?;
        let wire: WireChoiceContext = Self::decode(response, url.as_str()).await?;
        ChoiceContext::from_wire(wire)
    }

    async fn get<T: serde::de::DeserializeOwned>(&self, url: reqwest::Url) -> Result<T> {
        let response = self
            .http
            .get(url.clone())
            .send()
            .await
            .map_err(|e| connection(&e))?;
        Self::decode(response, url.as_str()).await
    }

    /// One place where a non-success status becomes an error and a body becomes
    /// a value, so every call reports failure the same way.
    async fn decode<T: serde::de::DeserializeOwned>(
        response: reqwest::Response,
        url: &str,
    ) -> Result<T> {
        let status = response.status();
        let body = response.text().await.map_err(|e| connection(&e))?;
        if !status.is_success() {
            // The standard's error body is `{"error": "…"}`; anything else is
            // passed through, since a registry that is failing is exactly when
            // a truncated-to-nothing message helps least.
            let message = serde_json::from_str::<ErrorResponse>(&body)
                .map(|e| e.error)
                .unwrap_or(body);
            return Err(Error::Http {
                status: status.as_u16(),
                body: format!("{url}: {message}"),
            });
        }
        serde_json::from_str(&body).map_err(|e| {
            Error::UnexpectedResponse(format!(
                "{url} returned a body this does not understand: {e}"
            ))
        })
    }
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: String,
}

/// Turn a transport failure into an error a caller can act on.
///
/// Two things this must get right, and the first version got neither.
///
/// **The message.** A `reqwest::Error` prints only its outer sentence —
/// `error sending request for url (…)` — and keeps what actually went wrong in
/// its source chain. Reporting only the outer message hides the one line that
/// says *certificate*, or *dns*, or *connection refused*.
///
/// **The verdict.** [`Error::Connection`] is unconditionally retriable, so
/// mapping everything to it makes an application that loops on `is_retriable()`
/// retry forever on a condition no amount of waiting fixes — a certificate it
/// cannot verify, a URL it cannot build, a redirect loop. Those are reported as
/// [`Error::InvalidRequest`], which is not retriable.
fn connection(e: &reqwest::Error) -> Error {
    let detail = chain(e);
    if e.is_timeout() {
        // The 30s default this client sets. Retriable, and `Error::Timeout`
        // says why without the caller reading the message.
        return Error::Timeout;
    }
    if e.is_builder() || e.is_redirect() {
        return Error::InvalidRequest(format!("the registry request could not be made: {detail}"));
    }
    if e.is_decode() {
        return Error::UnexpectedResponse(format!(
            "the registry's reply could not be read: {detail}"
        ));
    }
    // A TLS failure arrives as a connect error, indistinguishable by type from
    // a refused connection — reqwest exposes no typed TLS error — so the chain
    // is the only place the difference shows. Worth the string match: a
    // certificate the client cannot verify is permanent, and retrying it
    // forever is the failure this whole function exists to avoid.
    let lower = detail.to_ascii_lowercase();
    if lower.contains("certificate") || lower.contains("tls handshake") {
        return Error::InvalidRequest(format!(
            "the registry's TLS certificate could not be verified: {detail}"
        ));
    }
    Error::Connection(format!("cannot reach the registry: {detail}"))
}

/// A `reqwest::Error` and everything under it, so the cause is not lost.
fn chain(e: &reqwest::Error) -> String {
    let mut message = e.to_string();
    let mut source = std::error::Error::source(e);
    while let Some(cause) = source {
        message.push_str(": ");
        message.push_str(&cause.to_string());
        source = cause.source();
    }
    message
}

/// The choices a transfer instruction offers, as the registry paths name them.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum TransferInstructionChoice {
    /// The receiver accepts an offered transfer.
    Accept,
    /// The receiver rejects it.
    Reject,
    /// The sender withdraws it.
    Withdraw,
}

impl TransferInstructionChoice {
    fn path(self) -> &'static str {
        match self {
            Self::Accept => "accept",
            Self::Reject => "reject",
            Self::Withdraw => "withdraw",
        }
    }
}

/// The choices a V2 allocation *instruction* offers.
///
/// V1 has no equivalent: an allocation instruction there is driven through the
/// factory and has no choice contexts of its own.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum AllocationInstructionChoice {
    /// The registry accepts a pending instruction.
    Accept,
    /// The sender withdraws it.
    Withdraw,
}

impl AllocationInstructionChoice {
    fn path(self) -> &'static str {
        match self {
            Self::Accept => "accept",
            Self::Withdraw => "withdraw",
        }
    }
}

/// The choices an allocation offers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum AllocationChoice {
    /// The executor settles the allocated transfer.
    ExecuteTransfer,
    /// The sender withdraws the allocation.
    Withdraw,
    /// The executor cancels it.
    Cancel,
}

impl AllocationChoice {
    fn path(self) -> &'static str {
        match self {
            Self::ExecuteTransfer => "execute-transfer",
            Self::Withdraw => "withdraw",
            Self::Cancel => "cancel",
        }
    }
}

/// A body excerpt short enough for an error message. An HTML error page is
/// otherwise long enough to bury the sentence that names the problem.
fn truncate(body: &str) -> String {
    const LIMIT: usize = 200;
    if body.len() <= LIMIT {
        return body.to_string();
    }
    let cut = body
        .char_indices()
        .map(|(i, _)| i)
        .take_while(|i| *i <= LIMIT)
        .last()
        .unwrap_or(0);
    format!("{}…", &body[..cut])
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    /// The paths are the standard's, spelled as its `operationId`s imply. A
    /// wrong one is a 404 from a registry that is working perfectly.
    #[test]
    fn the_choice_paths_are_the_ones_the_standard_publishes() {
        assert_eq!(TransferInstructionChoice::Accept.path(), "accept");
        assert_eq!(TransferInstructionChoice::Reject.path(), "reject");
        assert_eq!(TransferInstructionChoice::Withdraw.path(), "withdraw");
        assert_eq!(AllocationChoice::ExecuteTransfer.path(), "execute-transfer");
        assert_eq!(AllocationChoice::Withdraw.path(), "withdraw");
        assert_eq!(AllocationChoice::Cancel.path(), "cancel");
    }

    /// `self` is a Rust keyword and the wire spells it exactly that, so the
    /// rename is load-bearing: without it the variant would never match.
    #[test]
    fn the_transfer_kinds_decode_from_the_words_the_wire_uses() {
        let kinds: Vec<TransferKind> =
            serde_json::from_str(r#"["self", "direct", "offer"]"#).expect("decodes");
        assert_eq!(
            kinds,
            [
                TransferKind::SelfTransfer,
                TransferKind::Direct,
                TransferKind::Offer
            ]
        );
    }

    /// A trailing slash on the base URL would otherwise produce `//registry/…`,
    /// which some servers route and others do not.
    #[test]
    fn a_trailing_slash_on_the_base_url_does_not_double_up() {
        let client = RegistryClient::new("https://scan.example.com/").unwrap();
        assert_eq!(
            client
                .url(&["registry", "metadata", "v1", "info"])
                .unwrap()
                .as_str(),
            "https://scan.example.com/registry/metadata/v1/info"
        );
    }

    /// A path segment from outside must not be able to change which resource is
    /// addressed. Formatted into a string, an instrument id of `a?b` fetched
    /// instrument `a` and returned its name, symbol and decimals with no error
    /// — and `../../info` walked out of the collection entirely.
    #[test]
    fn a_hostile_path_segment_cannot_redirect_the_request() {
        let client = RegistryClient::new("https://scan.example.com").unwrap();
        for hostile in ["a?b", "a#b", "../../info", "a/b"] {
            let url = client
                .url(&["registry", "metadata", "v1", "instruments", hostile])
                .unwrap();
            assert_eq!(
                url.query(),
                None,
                "`{hostile}` must not become a query: {url}"
            );
            assert_eq!(
                url.path_segments().map(Iterator::count),
                Some(5),
                "`{hostile}` must stay one segment: {url}"
            );
            assert!(
                url.as_str()
                    .starts_with("https://scan.example.com/registry/metadata/v1/instruments/"),
                "`{hostile}` escaped the collection: {url}"
            );
        }
    }

    /// A base URL that is not a URL is a configuration mistake, and it has to
    /// arrive as one. Left to surface from the first request it became
    /// `Error::Connection`, which the SDK reports as retriable — so an
    /// application would retry a typo forever.
    #[test]
    fn a_base_url_that_is_not_a_url_is_refused_immediately() {
        let err = RegistryClient::new("not a url").expect_err("not a URL");
        assert!(!err.is_retriable(), "a typo is not worth retrying: {err}");
        assert!(err.to_string().contains("not a url"), "{err}");
    }

    #[test]
    fn the_factory_request_is_the_shape_the_standard_specifies() {
        let args = serde_json::json!({ "expectedAdmin": "dso::1220", "transfer": {} });
        let request = GetFactoryRequest {
            choice_arguments: &args,
            exclude_debug_fields: true,
        };
        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            serde_json::json!({
                "choiceArguments": { "expectedAdmin": "dso::1220", "transfer": {} },
                "excludeDebugFields": true
            })
        );
    }

    /// The response is read by the field names the standard uses, and the
    /// factory's context is translated on the way through.
    #[test]
    fn a_factory_response_decodes_and_its_context_is_translated() {
        let json = serde_json::json!({
            "factoryId": "00factory",
            "transferKind": "direct",
            "choiceContext": {
                "choiceContextData": { "values": {} },
                "disclosedContracts": [{
                    "templateId": "pkg:Splice.AmuletRules:AmuletRules",
                    "contractId": "001",
                    "createdEventBlob": "AQI=",
                    "synchronizerId": "sync::1220ab"
                }]
            }
        });
        let wire: WireTransferFactory = serde_json::from_value(json).expect("decodes");
        assert_eq!(wire.factory_id, "00factory");
        assert_eq!(wire.transfer_kind, TransferKind::Direct);

        let context = ChoiceContext::from_wire(wire.choice_context).expect("translates");
        assert_eq!(context.disclosed_contracts().len(), 1);
        assert_eq!(context.disclosed_contracts()[0].created_event_blob, [1, 2]);
    }

    /// The allocation factory's response has no `transferKind` at all, and it
    /// is a different type for that reason: "this API does not classify the
    /// workflow" and "a registry omitted a field its spec requires" must not
    /// arrive as the same value.
    #[test]
    fn the_allocation_factory_response_has_no_transfer_kind() {
        let json = serde_json::json!({
            "factoryId": "00factory",
            "choiceContext": { "choiceContextData": {}, "disclosedContracts": [] }
        });
        let wire: WireFactory = serde_json::from_value(json).expect("decodes");
        assert_eq!(wire.factory_id, "00factory");

        // And the transfer factory refuses the same body, because its spec
        // marks the field required.
        let json = serde_json::json!({
            "factoryId": "00factory",
            "choiceContext": { "choiceContextData": {}, "disclosedContracts": [] }
        });
        assert!(serde_json::from_value::<WireTransferFactory>(json).is_err());
    }

    /// A registry that adds a fourth kind must not break the transfers whose
    /// kind this build understands perfectly well.
    #[test]
    fn an_unknown_transfer_kind_decodes_instead_of_failing_the_response() {
        let json = serde_json::json!({
            "factoryId": "00factory",
            "transferKind": "escrow-something-new",
            "choiceContext": { "choiceContextData": {}, "disclosedContracts": [] }
        });
        let wire: WireTransferFactory =
            serde_json::from_value(json).expect("an unknown kind is a value, not a decode failure");
        assert_eq!(wire.transfer_kind, TransferKind::Unknown);
    }

    /// The standard marks `decimals` required with a default of ten, so an
    /// absent field means ten places — not "unknown". It is what a wallet
    /// formats an amount with.
    #[test]
    fn an_absent_decimals_is_the_standards_default_and_not_unknown() {
        let instrument: Instrument = serde_json::from_value(serde_json::json!({
            "id": "Amulet", "name": "Canton Coin", "symbol": "CC"
        }))
        .expect("decodes");
        assert_eq!(instrument.decimals, 10);
    }

    #[test]
    fn instruments_decode_with_the_optional_fields_absent() {
        let json = serde_json::json!({
            "instruments": [{ "id": "Amulet", "name": "Canton Coin", "symbol": "CC" }]
        });
        let response: ListInstrumentsResponse = serde_json::from_value(json).expect("decodes");
        assert_eq!(response.instruments.len(), 1);
        assert_eq!(response.instruments[0].id, "Amulet");
        assert_eq!(response.instruments[0].total_supply, None);
        assert_eq!(response.instruments[0].decimals, 10);
        assert_eq!(response.next_page_token, None);
    }
}
