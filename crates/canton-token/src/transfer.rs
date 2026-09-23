//! CIP-56 transfers: resolve the factory, take its context, exercise the
//! choice.
//!
//! A transfer is not a command a client can build on its own. The factory that
//! implements it is the registry's contract, and the choice needs reference
//! data only the registry has. So it takes two steps, and the first one is an
//! HTTP call:
//!
//! 1. Ask the registry for the transfer factory, passing the transfer as the
//!    choice arguments it *will* be exercised with. The registry answers with
//!    the factory's contract id, how the transfer will be carried out, and the
//!    choice context.
//! 2. Exercise `TransferFactory_Transfer` on that factory with the context
//!    filled in, disclosing the contracts the registry named.
//!
//! The transfer passed in step 1 carries empty `extraArgs` — the standard says
//! so — because the context does not exist yet. It is filled in for step 2.

use canton_core::Result;
use canton_daml as rt;
use canton_proto::com::daml::ledger::api::v2 as pb;
use canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1 as md;
use canton_splice_api_token_transfer_instruction_v1::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1 as ti;

use crate::registry::{RegistryClient, TransferInstructionChoice, TransferKind};

/// A command that carries the contracts it needs disclosed.
///
/// A token-standard choice is meaningless without them: the participant cannot
/// resolve the factory's reference data otherwise, and the submission fails
/// during interpretation. Keeping the two together means they cannot be
/// separated by accident.
#[derive(Clone, Debug)]
pub struct TokenCommand {
    command: pb::Command,
    disclosed_contracts: Vec<pb::DisclosedContract>,
    transfer_kind: Option<TransferKind>,
}

impl TokenCommand {
    pub(crate) fn new(
        command: pb::Command,
        disclosed_contracts: Vec<pb::DisclosedContract>,
        transfer_kind: Option<TransferKind>,
    ) -> Self {
        Self {
            command,
            disclosed_contracts,
            transfer_kind,
        }
    }

    /// The command itself.
    #[must_use]
    pub fn command(&self) -> &pb::Command {
        &self.command
    }

    /// The contracts the registry says must be disclosed.
    #[must_use]
    pub fn disclosed_contracts(&self) -> &[pb::DisclosedContract] {
        &self.disclosed_contracts
    }

    /// How the registry decided to carry the transfer out.
    ///
    /// Worth branching on: [`TransferKind::Offer`] means the transfer is
    /// pending the receiver's acceptance rather than done, so a caller that
    /// treats every transfer as settled is wrong about that one.
    #[must_use]
    pub fn transfer_kind(&self) -> Option<TransferKind> {
        self.transfer_kind
    }

    /// An ordinary submission, with the disclosures attached.
    #[must_use]
    pub fn into_submit(self, act_as: impl Into<String>) -> canton_ledger::Submit {
        self.disclosed_contracts.into_iter().fold(
            canton_ledger::Submit::new(act_as).add_command(self.command),
            canton_ledger::Submit::add_disclosed_contract,
        )
    }

    /// An *interactive* submission, with the disclosures attached — for a party
    /// whose key the participant does not hold.
    #[must_use]
    pub fn into_prepare(self, act_as: impl Into<String>) -> canton_ledger::Prepare {
        canton_ledger::Prepare::new(act_as)
            .add_command(self.command)
            .with_disclosed_contracts(self.disclosed_contracts)
    }

    /// The parts, for a caller assembling a multi-command submission.
    #[must_use]
    pub fn into_parts(self) -> (pb::Command, Vec<pb::DisclosedContract>) {
        (self.command, self.disclosed_contracts)
    }
}

/// `ExtraArgs` with nothing in it — what the standard says to send when asking
/// for a factory, since the context being asked for does not exist yet.
#[must_use]
pub fn empty_extra_args() -> md::ExtraArgs {
    md::ExtraArgs {
        context: md::ChoiceContext {
            values: rt::TextMap::new(),
        },
        meta: md::Metadata {
            values: rt::TextMap::new(),
        },
    }
}

/// Resolve the transfer factory and build the exercise that performs
/// `transfer`.
///
/// `expected_admin` is the administrator the caller expects — see
/// [`RegistryClient::info`] for what a registry claims. Naming it in the choice
/// is what stops a *substituted* administrator from being accepted, so it is a
/// required argument: passing a value the same registry just returned proves
/// only that it did not change its answer between two calls. A caller who wants
/// the protection takes it from configuration.
///
/// # Errors
/// As any registry call, plus [`canton_core::Error::UnexpectedResponse`] if the
/// context does not decode.
pub async fn transfer(
    registry: &RegistryClient,
    expected_admin: &rt::Party,
    transfer: ti::Transfer,
) -> Result<TokenCommand> {
    // Step one: the choice as it will be exercised, but with empty extraArgs —
    // the registry uses the arguments to decide which context to return, so
    // they have to be the real ones.
    let probe = ti::TransferFactory_Transfer {
        expected_admin: expected_admin.clone(),
        transfer,
        extra_args: empty_extra_args(),
    };
    let arguments = serde_json::to_value(&probe).map_err(|e| {
        canton_core::Error::InvalidRequest(format!("the transfer does not serialize: {e}"))
    })?;

    let factory = registry.transfer_factory(&arguments).await?;
    let context: md::ChoiceContext = factory.context.decode()?;

    // Step two: the same choice, now with the context the registry supplied.
    let choice = ti::TransferFactory_Transfer {
        extra_args: md::ExtraArgs {
            context,
            meta: probe.extra_args.meta,
        },
        ..probe
    };
    let factory_id = rt::ContractId::<ti::TransferFactory>::new(factory.factory_id);

    Ok(TokenCommand::new(
        rt::exercise_command(&factory_id, &choice),
        factory.context.into_disclosed_contracts(),
        factory.transfer_kind,
    ))
}

/// Accept an offered transfer — the receiver's move.
///
/// A [`TransferKind::Offer`] transfer does not settle on submission: it creates
/// a `TransferInstruction` that waits for this. Without it the crate could tell
/// a caller their transfer was pending and give them no way to finish it.
///
/// # Errors
/// As any registry call.
pub async fn accept(
    registry: &RegistryClient,
    instruction_id: &rt::ContractId<ti::TransferInstruction>,
) -> Result<TokenCommand> {
    on_instruction(
        registry,
        instruction_id,
        TransferInstructionChoice::Accept,
        |extra_args| ti::TransferInstruction_Accept { extra_args },
    )
    .await
}

/// Reject an offered transfer — the receiver's move.
///
/// # Errors
/// As any registry call.
pub async fn reject(
    registry: &RegistryClient,
    instruction_id: &rt::ContractId<ti::TransferInstruction>,
) -> Result<TokenCommand> {
    on_instruction(
        registry,
        instruction_id,
        TransferInstructionChoice::Reject,
        |extra_args| ti::TransferInstruction_Reject { extra_args },
    )
    .await
}

/// Withdraw an offered transfer — the sender's move, before it is accepted.
///
/// # Errors
/// As any registry call.
pub async fn withdraw(
    registry: &RegistryClient,
    instruction_id: &rt::ContractId<ti::TransferInstruction>,
) -> Result<TokenCommand> {
    on_instruction(
        registry,
        instruction_id,
        TransferInstructionChoice::Withdraw,
        |extra_args| ti::TransferInstruction_Withdraw { extra_args },
    )
    .await
}

/// The shape the three choices on an existing instruction share.
///
/// One context per choice, never shared: the standard says a context may be
/// specific to the choice being exercised.
async fn on_instruction<C>(
    registry: &RegistryClient,
    instruction_id: &rt::ContractId<ti::TransferInstruction>,
    choice: TransferInstructionChoice,
    build: impl FnOnce(md::ExtraArgs) -> C,
) -> Result<TokenCommand>
where
    C: rt::Choice<ti::TransferInstruction> + rt::ToValue,
{
    let context = registry
        .transfer_instruction_context(
            instruction_id.as_str(),
            choice,
            &crate::ChoiceContextRequest::default(),
        )
        .await?;
    let argument = build(md::ExtraArgs {
        context: context.decode()?,
        meta: md::Metadata {
            values: rt::TextMap::new(),
        },
    });
    Ok(TokenCommand::new(
        rt::exercise_command(instruction_id, &argument),
        context.into_disclosed_contracts(),
        None,
    ))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    fn a_transfer() -> ti::Transfer {
        use canton_splice_api_token_holding_v1::splice_api_token_holding_v1::Splice_Api_Token_HoldingV1 as h;
        ti::Transfer {
            sender: rt::Party::parse("alice::1220ab").expect("a party"),
            receiver: rt::Party::parse("bob::1220cd").expect("a party"),
            amount: "10.0".parse().expect("a numeric"),
            instrument_id: h::InstrumentId {
                admin: rt::Party::parse("dso::1220ef").expect("a party"),
                id: "Amulet".to_string(),
            },
            requested_at: rt::Timestamp(0),
            execute_before: rt::Timestamp(1_000_000),
            input_holding_cids: Vec::new(),
            meta: md::Metadata {
                values: rt::TextMap::new(),
            },
        }
    }

    /// The standard is explicit that the probe carries empty `extraArgs`. A
    /// registry uses the arguments to pick a context, so sending a context in
    /// the request would be asking it to answer a question with the answer.
    #[test]
    fn the_factory_probe_carries_the_real_transfer_and_empty_extra_args() {
        let probe = ti::TransferFactory_Transfer {
            expected_admin: rt::Party::parse("dso::1220ef").expect("a party"),
            transfer: a_transfer(),
            extra_args: empty_extra_args(),
        };
        let json = serde_json::to_value(&probe).unwrap();

        assert_eq!(
            json["extraArgs"]["context"]["values"],
            serde_json::json!({})
        );
        assert_eq!(json["extraArgs"]["meta"]["values"], serde_json::json!({}));
        assert_eq!(json["transfer"]["sender"], "alice::1220ab");
        assert_eq!(json["transfer"]["receiver"], "bob::1220cd");
        assert_eq!(json["expectedAdmin"], "dso::1220ef");
    }

    /// LF-JSON carries a `Numeric` as a string, and the registry parses it with
    /// the Daml JSON API's rules. A bare number would be a different encoding.
    #[test]
    fn the_amount_is_encoded_as_the_daml_json_api_encodes_a_numeric() {
        let json = serde_json::to_value(a_transfer()).unwrap();
        assert_eq!(json["amount"], "10.0");
    }

    /// The context the registry returns must reach the choice — that is the
    /// entire point of the round trip. If it were dropped, the submission would
    /// fail during interpretation with a message about a missing contract
    /// rather than about a missing context.
    #[test]
    fn the_returned_context_is_what_gets_exercised() {
        let wire = crate::WireChoiceContext {
            choice_context_data: serde_json::json!({
                "values": { "amulet-rules": { "tag": "AV_ContractId", "value": "00rules" } }
            }),
            disclosed_contracts: vec![crate::WireDisclosedContract {
                template_id: "pkg:Splice.AmuletRules:AmuletRules".to_string(),
                contract_id: "00rules".to_string(),
                created_event_blob: "AQI=".to_string(),
                synchronizer_id: "sync::1220ab".to_string(),
            }],
        };
        let context = crate::ChoiceContext::from_wire(wire).unwrap();
        let decoded: md::ChoiceContext = context.decode().unwrap();

        let choice = ti::TransferFactory_Transfer {
            expected_admin: rt::Party::parse("dso::1220ef").expect("a party"),
            transfer: a_transfer(),
            extra_args: md::ExtraArgs {
                context: decoded,
                meta: md::Metadata {
                    values: rt::TextMap::new(),
                },
            },
        };
        let json = serde_json::to_value(&choice).unwrap();
        assert_eq!(
            json["extraArgs"]["context"]["values"]["amulet-rules"],
            serde_json::json!({ "tag": "AV_ContractId", "value": "00rules" })
        );
    }

    /// A command and the contracts it needs disclosed travel together, and both
    /// reach either kind of submission. Attaching one without the other is a
    /// submission that fails at interpretation.
    #[test]
    fn a_token_command_carries_its_disclosures_into_both_submission_paths() {
        let disclosed = vec![pb::DisclosedContract {
            template_id: None,
            contract_id: "00rules".to_string(),
            created_event_blob: vec![1, 2],
            synchronizer_id: "sync".to_string(),
        }];
        let command = TokenCommand::new(
            pb::Command { command: None },
            disclosed.clone(),
            Some(TransferKind::Direct),
        );

        assert_eq!(command.transfer_kind(), Some(TransferKind::Direct));
        assert_eq!(command.disclosed_contracts().len(), 1);

        let (_, taken) = command.clone().into_parts();
        assert_eq!(taken, disclosed);

        // Both paths keep them: the interactive one is what an externally
        // signed party uses, and it would be easy to wire only the ordinary
        // one.
        let prepared = command.clone().into_prepare("alice::1220ab");
        assert_eq!(prepared.disclosed_contracts().len(), 1);
    }
}
