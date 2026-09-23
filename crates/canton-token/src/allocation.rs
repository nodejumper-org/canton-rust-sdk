//! Allocations: reserving holdings now so a third party can settle later.
//!
//! A transfer moves an asset between two parties. An *allocation* is what a
//! settlement needs instead: the sender commits holdings to one leg of a
//! settlement, and an **executor** — neither sender nor receiver — settles it
//! afterwards. That is what makes delivery-versus-payment expressible, since
//! both legs can be allocated before either moves.
//!
//! It follows the same two steps as a transfer, and for the same reason: the
//! factory is the registry's contract and only the registry knows what the
//! choice needs.
//!
//! Once an allocation exists, the choices on it — execute, withdraw, cancel —
//! each need a context of their own, and it is fetched per choice. The standard
//! is explicit that a context must not be reused across choices.

use canton_core::Result;
use canton_daml as rt;
use canton_splice_api_token_allocation_instruction_v1::splice_api_token_allocation_instruction_v1::Splice_Api_Token_AllocationInstructionV1 as ai;
use canton_splice_api_token_allocation_v1::splice_api_token_allocation_v1::Splice_Api_Token_AllocationV1 as al;
use canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1 as md;

use crate::registry::{AllocationChoice, RegistryClient};
use crate::transfer::{TokenCommand, empty_extra_args};

/// Resolve the allocation factory and build the exercise that allocates
/// `allocation`.
///
/// `expected_admin` is the registry's `admin_id`, and it is required here for
/// the same reason as in a transfer: naming the administrator in the choice is
/// what stops one being substituted.
///
/// `input_holding_cids` may be empty, in which case the registry selects
/// holdings itself where its implementation allows it.
///
/// # Errors
/// As any registry call, plus [`canton_core::Error::UnexpectedResponse`] if the
/// context does not decode.
pub async fn allocate(
    registry: &RegistryClient,
    expected_admin: &rt::Party,
    allocation: al::AllocationSpecification,
    requested_at: rt::Timestamp,
    input_holding_cids: Vec<rt::ContractId<crate::Holding>>,
) -> Result<TokenCommand> {
    let probe = ai::AllocationFactory_Allocate {
        expected_admin: expected_admin.clone(),
        allocation,
        requested_at,
        input_holding_cids,
        extra_args: empty_extra_args(),
    };
    let arguments = serde_json::to_value(&probe).map_err(|e| {
        canton_core::Error::InvalidRequest(format!("the allocation does not serialize: {e}"))
    })?;

    let factory = registry.allocation_factory(&arguments).await?;
    let context: md::ChoiceContext = factory.context.decode()?;

    let choice = ai::AllocationFactory_Allocate {
        extra_args: md::ExtraArgs {
            context,
            meta: probe.extra_args.meta,
        },
        ..probe
    };
    let factory_id = rt::ContractId::<ai::AllocationFactory>::new(factory.factory_id);

    Ok(TokenCommand::new(
        rt::exercise_command(&factory_id, &choice),
        factory.context.into_disclosed_contracts(),
        factory.transfer_kind,
    ))
}

/// Build the exercise that settles an allocation's transfer.
///
/// This is the executor's move, not the sender's: the executor named in the
/// settlement is the party that acts.
///
/// # Errors
/// As any registry call.
pub async fn execute_transfer(
    registry: &RegistryClient,
    allocation_id: &rt::ContractId<al::Allocation>,
) -> Result<TokenCommand> {
    on_allocation(
        registry,
        allocation_id,
        AllocationChoice::ExecuteTransfer,
        |extra_args| al::Allocation_ExecuteTransfer { extra_args },
    )
    .await
}

/// Build the exercise that withdraws an allocation, releasing the holdings.
///
/// The sender's move, before settlement.
///
/// # Errors
/// As any registry call.
pub async fn withdraw(
    registry: &RegistryClient,
    allocation_id: &rt::ContractId<al::Allocation>,
) -> Result<TokenCommand> {
    on_allocation(
        registry,
        allocation_id,
        AllocationChoice::Withdraw,
        |extra_args| al::Allocation_Withdraw { extra_args },
    )
    .await
}

/// Build the exercise that cancels an allocation.
///
/// The executor's move, when the settlement will not happen.
///
/// # Errors
/// As any registry call.
pub async fn cancel(
    registry: &RegistryClient,
    allocation_id: &rt::ContractId<al::Allocation>,
) -> Result<TokenCommand> {
    on_allocation(
        registry,
        allocation_id,
        AllocationChoice::Cancel,
        |extra_args| al::Allocation_Cancel { extra_args },
    )
    .await
}

/// The shape every choice on an existing allocation shares: fetch the context
/// for *that* choice, build the argument, exercise it.
///
/// One context per choice, never shared — the standard says a context may be
/// specific to the choice being exercised, so reusing one is a bug that would
/// work until the day a registry starts distinguishing them.
async fn on_allocation<C>(
    registry: &RegistryClient,
    allocation_id: &rt::ContractId<al::Allocation>,
    choice: AllocationChoice,
    build: impl FnOnce(md::ExtraArgs) -> C,
) -> Result<TokenCommand>
where
    C: rt::Choice<al::Allocation> + rt::ToValue,
{
    let context = registry
        .allocation_context(
            allocation_id.as_str(),
            choice,
            &crate::ChoiceContextRequest::default(),
        )
        .await?;
    let decoded: md::ChoiceContext = context.decode()?;
    let argument = build(md::ExtraArgs {
        context: decoded,
        meta: md::Metadata {
            values: rt::TextMap::new(),
        },
    });
    Ok(TokenCommand::new(
        rt::exercise_command(allocation_id, &argument),
        context.into_disclosed_contracts(),
        None,
    ))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    fn party(s: &str) -> rt::Party {
        rt::Party::parse(s).expect("a party")
    }

    fn a_specification() -> al::AllocationSpecification {
        use canton_splice_api_token_holding_v1::splice_api_token_holding_v1::Splice_Api_Token_HoldingV1 as h;
        al::AllocationSpecification {
            settlement: al::SettlementInfo {
                executor: party("venue::1220ff"),
                settlement_ref: al::Reference {
                    id: "trade-1".to_string(),
                    cid: None,
                },
                requested_at: rt::Timestamp(0),
                allocate_before: rt::Timestamp(1_000_000),
                settle_before: rt::Timestamp(2_000_000),
                meta: md::Metadata {
                    values: rt::TextMap::new(),
                },
            },
            transfer_leg_id: "leg-1".to_string(),
            transfer_leg: al::TransferLeg {
                sender: party("alice::1220ab"),
                receiver: party("bob::1220cd"),
                amount: "10.0".parse().expect("a numeric"),
                instrument_id: h::InstrumentId {
                    admin: party("dso::1220ef"),
                    id: "Amulet".to_string(),
                },
                meta: md::Metadata {
                    values: rt::TextMap::new(),
                },
            },
        }
    }

    /// The executor is the point of an allocation: it is a third party, and the
    /// settlement names it rather than either side of the transfer.
    #[test]
    fn the_allocation_names_an_executor_that_is_neither_side() {
        let spec = a_specification();
        assert_ne!(spec.settlement.executor, spec.transfer_leg.sender);
        assert_ne!(spec.settlement.executor, spec.transfer_leg.receiver);

        let json = serde_json::to_value(&spec).unwrap();
        assert_eq!(json["settlement"]["executor"], "venue::1220ff");
        assert_eq!(json["transferLeg"]["sender"], "alice::1220ab");
    }

    /// As for a transfer, the probe carries the real allocation and empty
    /// `extraArgs` — the registry picks a context from the arguments.
    #[test]
    fn the_factory_probe_carries_the_real_allocation_and_empty_extra_args() {
        let probe = ai::AllocationFactory_Allocate {
            expected_admin: party("dso::1220ef"),
            allocation: a_specification(),
            requested_at: rt::Timestamp(0),
            input_holding_cids: Vec::new(),
            extra_args: empty_extra_args(),
        };
        let json = serde_json::to_value(&probe).unwrap();
        assert_eq!(
            json["extraArgs"]["context"]["values"],
            serde_json::json!({})
        );
        assert_eq!(json["expectedAdmin"], "dso::1220ef");
        assert_eq!(json["allocation"]["transferLegId"], "leg-1");
    }

    /// Each choice keeps its own name on the ledger. Exercising the wrong one
    /// with the right context is a rejection that says nothing useful.
    #[test]
    fn each_allocation_choice_is_the_one_it_names() {
        assert_eq!(
            <al::Allocation_ExecuteTransfer as rt::Choice<al::Allocation>>::NAME,
            "Allocation_ExecuteTransfer"
        );
        assert_eq!(
            <al::Allocation_Withdraw as rt::Choice<al::Allocation>>::NAME,
            "Allocation_Withdraw"
        );
        assert_eq!(
            <al::Allocation_Cancel as rt::Choice<al::Allocation>>::NAME,
            "Allocation_Cancel"
        );
    }
}
