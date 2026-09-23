//! CIP-0112 — the V2 token standard.
//!
//! The shape is the one V1 established: ask the registry for a factory, take
//! the choice context it returns, exercise the choice with the contracts it
//! said to disclose. What changed is worth stating, because a caller porting
//! from V1 meets all of it at once.
//!
//! * **Accounts, not parties.** A V2 [`Transfer`](ti::Transfer) moves between
//!   two [`Account`](h::Account)s. A party can hold several, so "who owns this"
//!   and "where does it live" stopped being the same question. Both the owner
//!   and the provider of an account are *optional* — an account held at a
//!   provider can be named by its `id` alone, which is what lets a registry
//!   route to an account whose owning party it does not need to state.
//! * **Every choice names its actors.** V2 choices carry `actors: Vec<Party>` —
//!   who is exercising, which V1 left implicit in the submitting party.
//! * **Settlement moved to a factory.** A V1 allocation is executed through a
//!   choice on itself; a V2 one is settled as part of a *batch*, through
//!   [`settle_batch`]. That is what lets both legs of a delivery-versus-payment
//!   settle together instead of one at a time.
//! * **Allocation instructions have their own choices.** V1 drove them entirely
//!   through the factory.
//!
//! Every path and payload here comes from the V2 OpenAPI documents in the
//! Splice repository, not from V1 with a digit changed — the two differ in ways
//! a search-and-replace would get wrong.

pub mod events;

use canton_core::Result;
use canton_daml as rt;
use canton_splice_api_token_allocation_instruction_v2::splice_api_token_allocation_instruction_v2::Splice_Api_Token_AllocationInstructionV2 as ai;
use canton_splice_api_token_allocation_v2::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2 as al;
use canton_splice_api_token_holding_v2::splice_api_token_holding_v2::Splice_Api_Token_HoldingV2 as h;
use canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1 as md;
use canton_splice_api_token_transfer_instruction_v2::splice_api_token_transfer_instruction_v2::Splice_Api_Token_TransferInstructionV2 as ti;

use crate::registry::{
    AllocationChoice, AllocationInstructionChoice, RegistryClient, TransferInstructionChoice,
};
use crate::transfer::{TokenCommand, empty_extra_args};

/// Resolve the V2 transfer factory and build the exercise that performs
/// `transfer`.
///
/// `actors` are the parties exercising the choice — usually just the sender.
/// V1 left this implicit in the submitting party; V2 states it.
///
/// # Errors
/// As any registry call, plus [`canton_core::Error::UnexpectedResponse`] if the
/// context does not decode.
pub async fn transfer(
    registry: &RegistryClient,
    transfer: ti::Transfer,
    actors: Vec<rt::Party>,
) -> Result<TokenCommand> {
    let probe = ti::TransferFactory_Transfer {
        transfer,
        actors,
        extra_args: empty_extra_args(),
    };
    let arguments = serde_json::to_value(&probe).map_err(|e| {
        canton_core::Error::InvalidRequest(format!("the transfer does not serialize: {e}"))
    })?;

    let factory = registry.transfer_factory_v2(&arguments).await?;
    let choice = ti::TransferFactory_Transfer {
        extra_args: md::ExtraArgs {
            context: factory.context.decode()?,
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

/// Accept an offered V2 transfer — the receiver's move.
///
/// # Errors
/// As any registry call.
pub async fn accept(
    registry: &RegistryClient,
    instruction_id: &rt::ContractId<ti::TransferInstruction>,
    actors: Vec<rt::Party>,
) -> Result<TokenCommand> {
    on_instruction(
        registry,
        instruction_id,
        TransferInstructionChoice::Accept,
        |extra_args| ti::TransferInstruction_Accept { actors, extra_args },
    )
    .await
}

/// Reject an offered V2 transfer.
///
/// # Errors
/// As any registry call.
pub async fn reject(
    registry: &RegistryClient,
    instruction_id: &rt::ContractId<ti::TransferInstruction>,
    actors: Vec<rt::Party>,
) -> Result<TokenCommand> {
    on_instruction(
        registry,
        instruction_id,
        TransferInstructionChoice::Reject,
        |extra_args| ti::TransferInstruction_Reject { actors, extra_args },
    )
    .await
}

/// Withdraw an offered V2 transfer — the sender's move.
///
/// # Errors
/// As any registry call.
pub async fn withdraw(
    registry: &RegistryClient,
    instruction_id: &rt::ContractId<ti::TransferInstruction>,
    actors: Vec<rt::Party>,
) -> Result<TokenCommand> {
    on_instruction(
        registry,
        instruction_id,
        TransferInstructionChoice::Withdraw,
        |extra_args| ti::TransferInstruction_Withdraw { actors, extra_args },
    )
    .await
}

/// Resolve the V2 allocation factory and build the exercise that allocates.
///
/// A V2 allocation names the `settlement` it belongs to at the moment it is
/// created. V1 allocated first and settled against whatever came later; naming
/// it up front is what lets the settlement factory later check that every leg
/// of a batch was allocated for *this* settlement.
///
/// # Errors
/// As any registry call.
pub async fn allocate(
    registry: &RegistryClient,
    settlement: al::SettlementInfo,
    allocation: al::AllocationSpecification,
    requested_at: rt::Timestamp,
    input_holding_cids: Vec<rt::ContractId<h::Holding>>,
    actors: Vec<rt::Party>,
) -> Result<TokenCommand> {
    use canton_splice_api_token_allocation_instruction_v2::splice_api_token_allocation_instruction_v2::Splice_Api_Token_AllocationInstructionV2 as ai;

    let probe = ai::AllocationFactory_Allocate {
        settlement,
        allocation,
        requested_at,
        input_holding_cids,
        actors,
        extra_args: empty_extra_args(),
    };
    let arguments = serde_json::to_value(&probe).map_err(|e| {
        canton_core::Error::InvalidRequest(format!("the allocation does not serialize: {e}"))
    })?;

    let factory = registry.allocation_factory_v2(&arguments).await?;
    let choice = ai::AllocationFactory_Allocate {
        extra_args: md::ExtraArgs {
            context: factory.context.decode()?,
            meta: probe.extra_args.meta,
        },
        ..probe
    };
    let factory_id = rt::ContractId::<ai::AllocationFactory>::new(factory.factory_id);

    Ok(TokenCommand::new(
        rt::exercise_command(&factory_id, &choice),
        factory.context.into_disclosed_contracts(),
        None,
    ))
}

/// Settle a batch of allocations through the settlement factory — the
/// executor's move, and the V2 replacement for V1's execute-transfer.
///
/// Settling a *batch* is what makes delivery-versus-payment atomic: both legs
/// move in one transaction, or neither does. V1 could only execute one
/// allocation at a time.
///
/// # Errors
/// As any registry call.
pub async fn settle_batch(
    registry: &RegistryClient,
    settlement: al::SettlementInfo,
    transfer_legs: Vec<al::TransferLeg>,
    allocations: Vec<al::FinalizedAllocation>,
    actors: Vec<rt::Party>,
) -> Result<TokenCommand> {
    let probe = al::SettlementFactory_SettleBatch {
        settlement,
        transfer_legs,
        allocations,
        actors,
        extra_args: empty_extra_args(),
    };
    let arguments = serde_json::to_value(&probe).map_err(|e| {
        canton_core::Error::InvalidRequest(format!("the settlement does not serialize: {e}"))
    })?;

    let factory = registry.settlement_factory_v2(&arguments).await?;
    let choice = al::SettlementFactory_SettleBatch {
        extra_args: md::ExtraArgs {
            context: factory.context.decode()?,
            meta: probe.extra_args.meta,
        },
        ..probe
    };
    let factory_id = rt::ContractId::<al::SettlementFactory>::new(factory.factory_id);

    Ok(TokenCommand::new(
        rt::exercise_command(&factory_id, &choice),
        factory.context.into_disclosed_contracts(),
        None,
    ))
}

/// Withdraw a V2 allocation, releasing the holdings — the sender's move.
///
/// # Errors
/// As any registry call.
pub async fn withdraw_allocation(
    registry: &RegistryClient,
    allocation_id: &rt::ContractId<al::Allocation>,
    actors: Vec<rt::Party>,
) -> Result<TokenCommand> {
    on_allocation(
        registry,
        allocation_id,
        AllocationChoice::Withdraw,
        |extra_args| al::Allocation_Withdraw { actors, extra_args },
    )
    .await
}

/// Cancel a V2 allocation — the executor's move.
///
/// # Errors
/// As any registry call.
pub async fn cancel(
    registry: &RegistryClient,
    allocation_id: &rt::ContractId<al::Allocation>,
    actors: Vec<rt::Party>,
) -> Result<TokenCommand> {
    on_allocation(
        registry,
        allocation_id,
        AllocationChoice::Cancel,
        |extra_args| al::Allocation_Cancel { actors, extra_args },
    )
    .await
}

/// Accept a pending V2 allocation instruction — the registry asked the
/// allocating party to confirm, and this is the confirmation.
///
/// V1 has no equivalent: an allocation instruction there is driven entirely
/// through the factory, so there is nothing to accept.
///
/// # Errors
/// As any registry call.
pub async fn accept_allocation_instruction(
    registry: &RegistryClient,
    instruction_id: &rt::ContractId<ai::AllocationInstruction>,
    actors: Vec<rt::Party>,
) -> Result<TokenCommand> {
    on_allocation_instruction(
        registry,
        instruction_id,
        AllocationInstructionChoice::Accept,
        |extra_args| ai::AllocationInstruction_Accept { actors, extra_args },
    )
    .await
}

/// Withdraw a pending V2 allocation instruction.
///
/// # Errors
/// As any registry call.
pub async fn withdraw_allocation_instruction(
    registry: &RegistryClient,
    instruction_id: &rt::ContractId<ai::AllocationInstruction>,
    actors: Vec<rt::Party>,
) -> Result<TokenCommand> {
    on_allocation_instruction(
        registry,
        instruction_id,
        AllocationInstructionChoice::Withdraw,
        |extra_args| ai::AllocationInstruction_Withdraw { actors, extra_args },
    )
    .await
}

/// The shape every choice on an existing V2 transfer instruction shares.
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
        .transfer_instruction_context_v2(
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

/// And the shape the two choices on an existing V2 allocation instruction
/// share.
async fn on_allocation_instruction<C>(
    registry: &RegistryClient,
    instruction_id: &rt::ContractId<ai::AllocationInstruction>,
    choice: AllocationInstructionChoice,
    build: impl FnOnce(md::ExtraArgs) -> C,
) -> Result<TokenCommand>
where
    C: rt::Choice<ai::AllocationInstruction> + rt::ToValue,
{
    let context = registry
        .allocation_instruction_context_v2(
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

/// And the shape every choice on an existing V2 allocation shares.
///
/// `execute-transfer` is deliberately absent: V2 settles through
/// [`settle_batch`], and asking for that context is refused by the registry
/// client with a message naming the replacement.
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
        .allocation_context_v2(
            allocation_id.as_str(),
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

    fn account(owner: &str) -> h::Account {
        h::Account {
            owner: Some(party(owner)),
            provider: None,
            id: "main".to_string(),
        }
    }

    /// The Account model is the visible difference from V1: a transfer moves
    /// between accounts, and a party can hold more than one.
    #[test]
    fn a_v2_transfer_moves_between_accounts_rather_than_parties() {
        let transfer = ti::Transfer {
            sender: account("alice::1220ab"),
            receiver: account("bob::1220cd"),
            amount: "10.0".parse().expect("a numeric"),
            instrument_id: h::InstrumentId {
                admin: party("dso::1220ef"),
                id: "Amulet".to_string(),
            },
            requested_at: rt::Timestamp(0),
            execute_before: rt::Timestamp(1_000_000),
            input_holding_cids: Vec::new(),
            meta: md::Metadata {
                values: rt::TextMap::new(),
            },
        };
        let json = serde_json::to_value(&transfer).unwrap();

        // Not a bare party string, as V1 encodes it — an object with an owner
        // and an account id.
        assert_eq!(json["sender"]["owner"], "alice::1220ab");
        assert_eq!(json["sender"]["id"], "main");
        assert_eq!(json["receiver"]["owner"], "bob::1220cd");
        assert_eq!(json["amount"], "10.0");
    }

    /// Every V2 choice names who is exercising it. V1 left that implicit in the
    /// submitting party, so a caller porting across has to supply it.
    #[test]
    fn every_v2_choice_names_its_actors() {
        let probe = ti::TransferFactory_Transfer {
            transfer: ti::Transfer {
                sender: account("alice::1220ab"),
                receiver: account("bob::1220cd"),
                amount: "1.0".parse().expect("a numeric"),
                instrument_id: h::InstrumentId {
                    admin: party("dso::1220ef"),
                    id: "Amulet".to_string(),
                },
                requested_at: rt::Timestamp(0),
                execute_before: rt::Timestamp(1),
                input_holding_cids: Vec::new(),
                meta: md::Metadata {
                    values: rt::TextMap::new(),
                },
            },
            actors: vec![party("alice::1220ab")],
            extra_args: empty_extra_args(),
        };
        let json = serde_json::to_value(&probe).unwrap();
        assert_eq!(json["actors"], serde_json::json!(["alice::1220ab"]));
        // And the probe still carries empty extraArgs, as the standard says.
        assert_eq!(
            json["extraArgs"]["context"]["values"],
            serde_json::json!({})
        );
    }

    /// The V2 choices are the ones the standard names, and they are not V1's.
    #[test]
    fn the_v2_choices_are_named_as_the_ledger_knows_them() {
        assert_eq!(
            <al::Allocation_Settle as rt::Choice<al::Allocation>>::NAME,
            "Allocation_Settle"
        );
        assert_eq!(
            <al::SettlementFactory_SettleBatch as rt::Choice<al::SettlementFactory>>::NAME,
            "SettlementFactory_SettleBatch"
        );
        assert_eq!(
            <ti::TransferInstruction_Accept as rt::Choice<ti::TransferInstruction>>::NAME,
            "TransferInstruction_Accept"
        );
    }
}
