//! The choice context a registry returns, and what it becomes on the ledger.
//!
//! A token-standard choice is exercised through an interface, and the registry
//! — not the caller — knows what reference data that choice needs. So the
//! caller asks for a *choice context* and gets two things back: some data to
//! pass into the choice, and the contracts the participant must be shown for
//! the choice to resolve. The second half is why explicit disclosure exists:
//! those contracts are typically not visible to the submitting party at all.
//!
//! The wire types here are exactly the token-standard OpenAPI's, so the
//! translation into the Ledger API happens in one place with the differences
//! stated: a JSON `createdEventBlob` is base64 and a gRPC one is bytes, and a
//! template id is one string on the wire and three fields in the API.

use base64::Engine as _;
use canton_core::{Error, Result};
use canton_proto::com::daml::ledger::api::v2 as pb;
use serde::{Deserialize, Serialize};

/// A request for a choice context.
///
/// `meta` is passed to the choice and folded into the context by the registry;
/// the standard provides it for extensibility and most callers send none.
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChoiceContextRequest {
    /// Left out entirely when empty — the field is optional, and an empty
    /// object is a different thing to say than nothing.
    #[serde(skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub meta: std::collections::BTreeMap<String, String>,
    /// Ask the registry to leave out the debug fields.
    ///
    /// Added by the V2 specifications; V1's request has no such field, and a
    /// registry serving V1 ignores it. Sent only when set, so a V1 request is
    /// byte-identical to what it was.
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub exclude_debug_fields: bool,
}

/// A contract the participant must be shown for a choice to resolve.
///
/// The `debug*` fields the standard also defines are deliberately not read:
/// the specification says to use them only if the provider is trusted, since
/// they need not match the `createdEventBlob` — and nothing here needs them.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WireDisclosedContract {
    /// `<package>:<Module>:<Entity>`.
    pub template_id: String,
    /// The contract id.
    pub contract_id: String,
    /// The created-event blob, base64 as JSON requires.
    pub created_event_blob: String,
    /// The synchronizer the contract is currently assigned to.
    pub synchronizer_id: String,
}

/// What a registry returns for a choice.
#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WireChoiceContext {
    /// The data to pass into the choice, as Daml JSON.
    pub choice_context_data: serde_json::Value,
    /// The contracts to disclose.
    pub disclosed_contracts: Vec<WireDisclosedContract>,
}

/// A choice context, ready to be used against either transport.
#[derive(Clone, Debug)]
pub struct ChoiceContext {
    data: serde_json::Value,
    disclosed: Vec<pb::DisclosedContract>,
    disclosed_json: Vec<serde_json::Value>,
}

impl ChoiceContext {
    /// Translate what the registry returned.
    ///
    /// # Errors
    /// [`Error::UnexpectedResponse`] if a template id is not
    /// `<package>:<Module>:<Entity>`, or a blob is not valid base64. Both are
    /// the registry's mistake rather than the caller's, and both would
    /// otherwise reach the participant as something it cannot parse.
    pub fn from_wire(wire: WireChoiceContext) -> Result<Self> {
        let mut disclosed = Vec::with_capacity(wire.disclosed_contracts.len());
        let mut disclosed_json = Vec::with_capacity(wire.disclosed_contracts.len());
        for contract in wire.disclosed_contracts {
            disclosed.push(pb::DisclosedContract {
                template_id: Some(identifier(&contract.template_id)?),
                contract_id: contract.contract_id.clone(),
                created_event_blob: base64::engine::general_purpose::STANDARD
                    .decode(&contract.created_event_blob)
                    .map_err(|e| {
                        Error::UnexpectedResponse(format!(
                            "the registry disclosed {} with a createdEventBlob that is not \
                             base64: {e}",
                            contract.contract_id
                        ))
                    })?,
                synchronizer_id: contract.synchronizer_id.clone(),
            });
            // The JSON transport takes the wire form unchanged — the blob is
            // already base64 there, so decoding and re-encoding it would only
            // be a chance to get it wrong.
            disclosed_json.push(serde_json::json!({
                "templateId": contract.template_id,
                "contractId": contract.contract_id,
                "createdEventBlob": contract.created_event_blob,
                "synchronizerId": contract.synchronizer_id,
            }));
        }
        Ok(Self {
            data: wire.choice_context_data,
            disclosed,
            disclosed_json,
        })
    }

    /// The context data, as the registry sent it.
    #[must_use]
    pub fn data(&self) -> &serde_json::Value {
        &self.data
    }

    /// Decode the context data into a generated type.
    ///
    /// The registry encodes it with the Daml JSON API's rules, which is what
    /// `canton-daml` implements, so the generated `ChoiceContext` deserializes
    /// straight from it — no hand-written mapping.
    ///
    /// # Errors
    /// [`Error::UnexpectedResponse`] if the data does not match `T`.
    pub fn decode<T: serde::de::DeserializeOwned>(&self) -> Result<T> {
        serde_json::from_value(self.data.clone()).map_err(|e| {
            Error::UnexpectedResponse(format!(
                "the registry's choice context does not decode as {}: {e}",
                std::any::type_name::<T>()
            ))
        })
    }

    /// The contracts to disclose, for a gRPC submission.
    #[must_use]
    pub fn disclosed_contracts(&self) -> &[pb::DisclosedContract] {
        &self.disclosed
    }

    /// The contracts to disclose, for the JSON transport.
    #[must_use]
    pub fn disclosed_contracts_json(&self) -> &[serde_json::Value] {
        &self.disclosed_json
    }

    /// Take ownership of the disclosed contracts, for attaching to a
    /// submission.
    #[must_use]
    pub fn into_disclosed_contracts(self) -> Vec<pb::DisclosedContract> {
        self.disclosed
    }
}

/// `<package>:<Module>:<Entity>` — the Ledger API's three fields in one string.
///
/// A module name may itself contain dots (`Splice.Api.Token.HoldingV1`) but
/// never colons, so exactly three parts is the only well-formed shape. A fourth
/// is rejected rather than folded into the entity name, where it would reach
/// the participant as a template that does not exist and come back as an
/// unrelated interpretation error.
fn identifier(template_id: &str) -> Result<pb::Identifier> {
    let mut parts = template_id.split(':');
    match (parts.next(), parts.next(), parts.next(), parts.next()) {
        (Some(package_id), Some(module_name), Some(entity_name), None)
            if !package_id.is_empty() && !module_name.is_empty() && !entity_name.is_empty() =>
        {
            Ok(pb::Identifier {
                package_id: package_id.to_string(),
                module_name: module_name.to_string(),
                entity_name: entity_name.to_string(),
            })
        }
        _ => Err(Error::UnexpectedResponse(format!(
            "the registry disclosed a contract with template id `{template_id}`, which is not \
             `<package>:<Module>:<Entity>`"
        ))),
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    fn wire(template_id: &str, blob: &str) -> WireChoiceContext {
        WireChoiceContext {
            choice_context_data: serde_json::json!({ "values": {} }),
            disclosed_contracts: vec![WireDisclosedContract {
                template_id: template_id.to_string(),
                contract_id: "00abc".to_string(),
                created_event_blob: blob.to_string(),
                synchronizer_id: "sync::1220ab".to_string(),
            }],
        }
    }

    /// The exact JSON a registry sends, decoded by the field names the
    /// specification uses. A `serde` rename slip here is a field silently left
    /// at its default, which the participant reports much later as a missing
    /// disclosure.
    #[test]
    fn the_wire_shape_is_the_one_the_standard_specifies() {
        let json = serde_json::json!({
            "choiceContextData": { "values": { "amulet-rules": { "tag": "AV_ContractId", "value": "001" } } },
            "disclosedContracts": [{
                "templateId": "pkg:Splice.AmuletRules:AmuletRules",
                "contractId": "001",
                "createdEventBlob": "AQI=",
                "synchronizerId": "sync::1220ab",
                "debugPackageName": "splice-amulet",
                "debugPayload": { "ignored": true }
            }]
        });
        let wire: WireChoiceContext = serde_json::from_value(json).expect("decodes");
        assert_eq!(wire.disclosed_contracts.len(), 1);
        assert_eq!(wire.disclosed_contracts[0].contract_id, "001");
        assert_eq!(wire.disclosed_contracts[0].created_event_blob, "AQI=");
        assert!(wire.choice_context_data.get("values").is_some());
    }

    /// gRPC takes bytes and JSON takes base64. Sending the base64 *text* as the
    /// gRPC blob is a disclosure the participant cannot read, and it fails as
    /// an unrelated interpretation error.
    #[test]
    fn the_blob_is_bytes_for_grpc_and_stays_base64_for_json() {
        let context =
            ChoiceContext::from_wire(wire("pkg:Splice.AmuletRules:AmuletRules", "AQI=")).unwrap();

        assert_eq!(context.disclosed_contracts()[0].created_event_blob, [1, 2]);
        assert_eq!(
            context.disclosed_contracts_json()[0]["createdEventBlob"],
            "AQI="
        );
    }

    /// A module name has dots in it and an entity name does not have colons, so
    /// the split is on colons and the module keeps its dots.
    #[test]
    fn a_dotted_module_name_survives_the_split() {
        let context =
            ChoiceContext::from_wire(wire("abc123:Splice.Api.Token.HoldingV1:Holding", "AQI="))
                .unwrap();
        let id = context.disclosed_contracts()[0]
            .template_id
            .as_ref()
            .unwrap();
        assert_eq!(id.package_id, "abc123");
        assert_eq!(id.module_name, "Splice.Api.Token.HoldingV1");
        assert_eq!(id.entity_name, "Holding");
    }

    #[test]
    fn a_template_id_that_is_not_three_parts_is_refused() {
        let err = ChoiceContext::from_wire(wire("pkg:OnlyTwo", "AQI=")).expect_err("two parts");
        assert!(err.to_string().contains("OnlyTwo"), "{err}");

        let err = ChoiceContext::from_wire(wire("pkg::Entity", "AQI=")).expect_err("empty module");
        assert!(err.to_string().contains("Module"), "{err}");
    }

    #[test]
    fn a_blob_that_is_not_base64_is_refused_naming_the_contract() {
        let err =
            ChoiceContext::from_wire(wire("pkg:M:E", "not base64!!")).expect_err("bad base64");
        assert!(err.to_string().contains("00abc"), "{err}");
        assert!(err.to_string().contains("base64"), "{err}");
    }

    /// The context decodes into the generated type, because the registry uses
    /// the Daml JSON encoding that `canton-daml` implements. This is what makes
    /// the crate a workflow over generated types rather than a re-declaration
    /// of them.
    #[test]
    fn the_context_decodes_into_the_generated_choice_context() {
        use canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1 as md;

        let wire = WireChoiceContext {
            choice_context_data: serde_json::json!({
                "values": { "amulet-rules": { "tag": "AV_ContractId", "value": "001" } }
            }),
            disclosed_contracts: Vec::new(),
        };
        let context = ChoiceContext::from_wire(wire).unwrap();
        let decoded: md::ChoiceContext = context.decode().expect("the generated type");
        assert_eq!(decoded.values.len(), 1);
        assert!(decoded.values.contains_key("amulet-rules"));
    }

    /// An empty `meta` is left off the request rather than sent as `{}`: the
    /// field is optional, and saying nothing is not the same as saying empty.
    #[test]
    fn an_empty_meta_is_not_sent() {
        let request = ChoiceContextRequest::default();
        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            serde_json::json!({})
        );

        let mut request = ChoiceContextRequest::default();
        request.meta.insert("k".to_string(), "v".to_string());
        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            serde_json::json!({ "meta": { "k": "v" } })
        );

        // `excludeDebugFields` is a V2 addition, so a request that does not set
        // it stays byte-identical to what a V1 registry has always been sent.
        let request = ChoiceContextRequest {
            exclude_debug_fields: true,
            ..ChoiceContextRequest::default()
        };
        assert_eq!(
            serde_json::to_value(&request).unwrap(),
            serde_json::json!({ "excludeDebugFields": true })
        );
    }
}
