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
#[derive(Clone, Debug)]
pub struct RegistryClient {
    base_url: String,
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
    #[serde(default)]
    pub decimals: Option<i32>,
    /// Which token-standard APIs apply to this instrument.
    #[serde(default)]
    pub supported_apis: std::collections::BTreeMap<String, i32>,
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WireFactory {
    factory_id: String,
    #[serde(default)]
    transfer_kind: Option<TransferKind>,
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
    /// # Errors
    /// [`Error::InvalidRequest`] if an HTTP client cannot be built.
    pub fn new(base_url: impl Into<String>) -> Result<Self> {
        Ok(Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            http: reqwest::Client::builder()
                .build()
                .map_err(|e| Error::InvalidRequest(format!("cannot build an HTTP client: {e}")))?,
        })
    }

    /// A client using an HTTP client the caller has already configured —
    /// timeouts, proxies, a custom TLS root store.
    #[must_use]
    pub fn with_http_client(base_url: impl Into<String>, http: reqwest::Client) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            http,
        }
    }

    /// The registry's own description of itself — including `admin_id`, which
    /// is the `expectedAdmin` a factory choice names.
    ///
    /// # Errors
    /// As any HTTP call, plus [`Error::UnexpectedResponse`] on a body that does
    /// not match the standard.
    pub async fn info(&self) -> Result<RegistryInfo> {
        self.get("/registry/metadata/v1/info").await
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
        let mut path = "/registry/metadata/v1/instruments".to_string();
        let mut query = Vec::new();
        if let Some(size) = page_size {
            query.push(format!("pageSize={size}"));
        }
        if let Some(token) = page_token {
            query.push(format!("pageToken={token}"));
        }
        if !query.is_empty() {
            path.push('?');
            path.push_str(&query.join("&"));
        }
        let response: ListInstrumentsResponse = self.get(&path).await?;
        Ok((response.instruments, response.next_page_token))
    }

    /// One instrument by id, or `None` if this registry does not issue it.
    ///
    /// # Errors
    /// As [`info`](Self::info). A `404` is not an error here: not issuing an
    /// instrument is an answer.
    pub async fn instrument(&self, instrument_id: &str) -> Result<Option<Instrument>> {
        let url = format!(
            "{}/registry/metadata/v1/instruments/{instrument_id}",
            self.base_url
        );
        let response = self
            .http
            .get(&url)
            .send()
            .await
            .map_err(|e| connection(&e))?;
        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }
        Ok(Some(Self::decode(response, &url).await?))
    }

    /// Resolve the transfer factory for a transfer, with its choice context.
    ///
    /// `choice_arguments` is a serialized `TransferFactory_Transfer` with empty
    /// `extraArgs` — see [`crate::transfer`], which builds it.
    ///
    /// # Errors
    /// As [`info`](Self::info).
    pub async fn transfer_factory(
        &self,
        choice_arguments: &serde_json::Value,
    ) -> Result<FactoryWithContext> {
        self.factory(
            "/registry/transfer-instruction/v1/transfer-factory",
            choice_arguments,
        )
        .await
    }

    /// Resolve the allocation factory, with its choice context.
    ///
    /// # Errors
    /// As [`info`](Self::info).
    pub async fn allocation_factory(
        &self,
        choice_arguments: &serde_json::Value,
    ) -> Result<FactoryWithContext> {
        self.factory(
            "/registry/allocation-instruction/v1/allocation-factory",
            choice_arguments,
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
            &format!(
                "/registry/transfer-instruction/v1/{transfer_instruction_id}/choice-contexts/{}",
                choice.path()
            ),
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
            &format!(
                "/registry/allocations/v1/{allocation_id}/choice-contexts/{}",
                choice.path()
            ),
            request,
        )
        .await
    }

    async fn factory(
        &self,
        path: &str,
        choice_arguments: &serde_json::Value,
    ) -> Result<FactoryWithContext> {
        let url = format!("{}{path}", self.base_url);
        let response = self
            .http
            .post(&url)
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
        let wire: WireFactory = Self::decode(response, &url).await?;
        Ok(FactoryWithContext {
            factory_id: wire.factory_id,
            transfer_kind: wire.transfer_kind,
            context: ChoiceContext::from_wire(wire.choice_context)?,
        })
    }

    async fn choice_context(
        &self,
        path: &str,
        request: &ChoiceContextRequest,
    ) -> Result<ChoiceContext> {
        let url = format!("{}{path}", self.base_url);
        let response = self
            .http
            .post(&url)
            .json(request)
            .send()
            .await
            .map_err(|e| connection(&e))?;
        let wire: WireChoiceContext = Self::decode(response, &url).await?;
        ChoiceContext::from_wire(wire)
    }

    async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}{path}", self.base_url);
        let response = self
            .http
            .get(&url)
            .send()
            .await
            .map_err(|e| connection(&e))?;
        Self::decode(response, &url).await
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

fn connection(e: &reqwest::Error) -> Error {
    Error::Connection(e.to_string())
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
        assert_eq!(client.base_url, "https://scan.example.com");
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
        let wire: WireFactory = serde_json::from_value(json).expect("decodes");
        assert_eq!(wire.factory_id, "00factory");
        assert_eq!(wire.transfer_kind, Some(TransferKind::Direct));

        let context = ChoiceContext::from_wire(wire.choice_context).expect("translates");
        assert_eq!(context.disclosed_contracts().len(), 1);
        assert_eq!(context.disclosed_contracts()[0].created_event_blob, [1, 2]);
    }

    /// The allocation factory's response carries no `transferKind` — its API
    /// does not classify a workflow — so the field must be optional rather than
    /// a decode failure.
    #[test]
    fn a_factory_without_a_transfer_kind_still_decodes() {
        let json = serde_json::json!({
            "factoryId": "00factory",
            "choiceContext": { "choiceContextData": {}, "disclosedContracts": [] }
        });
        let wire: WireFactory = serde_json::from_value(json).expect("decodes");
        assert_eq!(wire.transfer_kind, None);
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
        assert_eq!(response.next_page_token, None);
    }
}
