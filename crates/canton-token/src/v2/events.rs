//! Reading V2 transfer events off a transaction.
//!
//! A V2 registry records what moved by exercising `EventLog_HoldingsChange` on
//! the `EventLog` interface: which account, which holdings went in, which came
//! out, and the legs they settled. That is the standard's answer to "what
//! actually happened", and it is the only place the answer is complete — the
//! created and archived events show holdings appearing and disappearing without
//! saying which transfer they belonged to.
//!
//! ```no_run
//! # async fn f(client: &canton_ledger::CantonClient, submit: canton_ledger::Submit)
//! # -> Result<(), Box<dyn std::error::Error>> {
//! let transaction = client.submit_and_wait_for_transaction(submit).await?;
//! for change in canton_token::v2::events::holdings_changes(&transaction.events)? {
//!     println!(
//!         "{} spent {} holding(s), produced {}",
//!         change.contract_id,
//!         change.change.input_holding_cids.len(),
//!         change.change.output_holding_cids.len(),
//!     );
//! }
//! # Ok(()) }
//! ```

use canton_core::{Error, Result};
use canton_daml as rt;
use canton_proto::com::daml::ledger::api::v2 as pb;
use canton_splice_api_token_transfer_events_v2::splice_api_token_transfer_events_v2::Splice_Api_Token_TransferEventsV2 as ev;

use rt::Contract as _;

/// One `EventLog_HoldingsChange`, with where on the transaction it was found.
///
/// The position is kept because a transaction can carry several — one per
/// account touched — and "which leg was this" is answered by the node id, not
/// by the order a caller happens to iterate in.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HoldingsChange {
    /// The offset of the transaction this was read from.
    pub offset: i64,
    /// Where in that transaction the event sits.
    pub node_id: i32,
    /// The `EventLog` contract the choice was exercised on.
    pub contract_id: String,
    /// The decoded choice argument — what moved.
    pub change: ev::EventLog_HoldingsChange,
}

/// Every V2 holdings change in `events`, in the order the transaction carries
/// them.
///
/// Events that are not holdings changes are skipped, not refused: a transaction
/// carries the creates and archives of the holdings themselves, and a
/// registry's own bookkeeping besides.
///
/// # Errors
/// [`Error::UnexpectedResponse`] if an event *is* a holdings change and its
/// argument does not decode — a participant serving a shape this build cannot
/// read is worth reporting rather than silently dropping, since the alternative
/// is a caller concluding that nothing moved.
pub fn holdings_changes(events: &[pb::Event]) -> Result<Vec<HoldingsChange>> {
    let mut changes = Vec::new();
    for event in events {
        let Some(pb::event::Event::Exercised(exercised)) = &event.event else {
            continue;
        };
        if !is_holdings_change(exercised) {
            continue;
        }
        let argument = exercised.choice_argument.as_ref().ok_or_else(|| {
            Error::UnexpectedResponse(format!(
                "the holdings change on {} carries no argument",
                exercised.contract_id
            ))
        })?;
        let change =
            <ev::EventLog_HoldingsChange as rt::FromValue>::from_value(argument).map_err(|e| {
                Error::UnexpectedResponse(format!(
                    "the holdings change on {} does not decode: {e}",
                    exercised.contract_id
                ))
            })?;
        changes.push(HoldingsChange {
            offset: exercised.offset,
            node_id: exercised.node_id,
            contract_id: exercised.contract_id.clone(),
            change,
        });
    }
    Ok(changes)
}

/// Whether an exercised event is the standard's holdings change.
///
/// Matched on the interface's **qualified name** rather than its package id.
/// The choice is defined on `EventLog`, so any template implementing it reports
/// the interface here — but the package id changes on every upgrade of the
/// standard, and a client that pinned one would stop seeing events the day a
/// network upgraded. The module and entity name are what stay put.
///
/// The choice name alone would not do: it is not reserved, and another package
/// is free to define a choice spelled the same way.
fn is_holdings_change(exercised: &pb::ExercisedEvent) -> bool {
    if exercised.choice != <ev::EventLog_HoldingsChange as rt::Choice<ev::EventLog>>::NAME {
        return false;
    }
    let named = |id: &pb::Identifier| {
        id.module_name == ev::EventLog::MODULE_NAME && id.entity_name == ev::EventLog::ENTITY_NAME
    };
    exercised.interface_id.as_ref().is_some_and(named)
        || exercised.implemented_interfaces.iter().any(named)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use canton_splice_api_token_holding_v2::splice_api_token_holding_v2::Splice_Api_Token_HoldingV2 as h;
    use canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1 as md;

    fn party(s: &str) -> rt::Party {
        rt::Party::parse(s).expect("a party")
    }

    fn a_change() -> ev::EventLog_HoldingsChange {
        ev::EventLog_HoldingsChange {
            admin: party("dso::1220ef"),
            account: h::Account {
                owner: Some(party("alice::1220ab")),
                provider: None,
                id: "main".to_string(),
            },
            input_holding_cids: vec![rt::ContractId::new("00in".to_string())],
            transfer_leg_sides: Vec::new(),
            output_holding_cids: vec![
                rt::ContractId::new("00out1".to_string()),
                rt::ContractId::new("00out2".to_string()),
            ],
            observers: Vec::new(),
            extra_args: md::ExtraArgs {
                context: md::ChoiceContext {
                    values: rt::TextMap::new(),
                },
                meta: md::Metadata {
                    values: rt::TextMap::new(),
                },
            },
        }
    }

    fn interface_id() -> pb::Identifier {
        pb::Identifier {
            package_id: ev::EventLog::PACKAGE_ID.to_string(),
            module_name: ev::EventLog::MODULE_NAME.to_string(),
            entity_name: ev::EventLog::ENTITY_NAME.to_string(),
        }
    }

    fn exercised(choice: &str, interface: Option<pb::Identifier>) -> pb::Event {
        pb::Event {
            event: Some(pb::event::Event::Exercised(pb::ExercisedEvent {
                offset: 42,
                node_id: 7,
                contract_id: "00log".to_string(),
                choice: choice.to_string(),
                choice_argument: Some(rt::ToValue::to_value(&a_change())),
                interface_id: interface,
                ..Default::default()
            })),
        }
    }

    #[test]
    fn a_holdings_change_round_trips_through_the_ledger_encoding() {
        let changes =
            holdings_changes(&[exercised("EventLog_HoldingsChange", Some(interface_id()))])
                .expect("decodes");

        assert_eq!(changes.len(), 1);
        assert_eq!(changes[0].offset, 42);
        assert_eq!(changes[0].node_id, 7);
        assert_eq!(changes[0].contract_id, "00log");
        assert_eq!(changes[0].change, a_change());
    }

    /// The package id moves on every upgrade of the standard. Matching it would
    /// make a client stop seeing events the day a network upgraded, which is
    /// the failure that looks like "nothing moved" rather than like an error.
    #[test]
    fn an_upgraded_package_id_is_still_the_same_interface() {
        let mut upgraded = interface_id();
        upgraded.package_id = "0".repeat(64);

        let changes = holdings_changes(&[exercised("EventLog_HoldingsChange", Some(upgraded))])
            .expect("decodes");
        assert_eq!(changes.len(), 1, "an upgrade must not hide the event");
    }

    /// A transaction carries the holdings' own creates and archives, and a
    /// registry's bookkeeping. None of it is a holdings change.
    #[test]
    fn events_that_are_not_holdings_changes_are_skipped() {
        let created = pb::Event {
            event: Some(pb::event::Event::Created(pb::CreatedEvent::default())),
        };
        let other_choice = exercised("EventLog_SomethingElse", Some(interface_id()));

        let changes = holdings_changes(&[created, other_choice]).expect("decodes");
        assert!(changes.is_empty());
    }

    /// The choice name is not reserved: another package may spell a choice the
    /// same way, and reading its argument as a holdings change would report a
    /// transfer that did not happen.
    #[test]
    fn the_choice_name_alone_is_not_enough_to_claim_an_event() {
        let impostor = exercised(
            "EventLog_HoldingsChange",
            Some(pb::Identifier {
                package_id: ev::EventLog::PACKAGE_ID.to_string(),
                module_name: "Some.Other.Module".to_string(),
                entity_name: "EventLog".to_string(),
            }),
        );
        assert!(holdings_changes(&[impostor]).expect("decodes").is_empty());

        // And an exercise reporting no interface at all is a choice on a
        // template, not on the standard's interface.
        let bare = exercised("EventLog_HoldingsChange", None);
        assert!(holdings_changes(&[bare]).expect("decodes").is_empty());
    }

    /// The interface can arrive in `implemented_interfaces` rather than
    /// `interface_id` — the latter is set only when the choice was exercised
    /// *through* the interface, and a registry exercising it on the template
    /// reports the implementation instead.
    #[test]
    fn the_interface_is_recognised_wherever_the_event_reports_it() {
        let mut event = exercised("EventLog_HoldingsChange", None);
        let Some(pb::event::Event::Exercised(ref mut ex)) = event.event else {
            unreachable!("built as an exercise")
        };
        ex.implemented_interfaces = vec![interface_id()];

        assert_eq!(holdings_changes(&[event]).expect("decodes").len(), 1);
    }

    /// A shape this build cannot read must not read as "nothing moved".
    #[test]
    fn an_argument_that_does_not_decode_is_reported_rather_than_dropped() {
        let mut event = exercised("EventLog_HoldingsChange", Some(interface_id()));
        let Some(pb::event::Event::Exercised(ref mut ex)) = event.event else {
            unreachable!("built as an exercise")
        };
        ex.choice_argument = Some(rt::ToValue::to_value(&"not a record".to_string()));

        let error = holdings_changes(&[event]).expect_err("must not be silently skipped");
        assert!(
            matches!(error, Error::UnexpectedResponse(_)),
            "expected an unexpected-response error, got {error:?}"
        );
        assert!(
            error.to_string().contains("00log"),
            "the message must name the contract: {error}"
        );
    }
}
