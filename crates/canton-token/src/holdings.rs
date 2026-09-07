//! Which holdings a party has, read from the ledger.
//!
//! Every transfer and allocation starts with this question, and the standard
//! lets a registry answer it itself — a request may name no input holdings.
//! Splice's reference registry does not: it refuses an empty list, and the
//! refusal arrives from the Daml interpreter after the whole registry
//! round-trip succeeded. Worse, naming a holding that is **locked** (an
//! allocation in flight, an expiry not yet reached) fails the same late way,
//! with `Lock.expiresAt` in the message. So a wallet needs the list, and it
//! needs to know which entries are spendable.
//!
//! This reads the `Holding` interface views from the active contract set:
//! one request, filtered to the interface, with the view included. The V1 and
//! V2 `Holding` interfaces name the same contracts, so the ids serve either
//! workflow; [`HoldingSummary::typed`] hands one out as whichever the caller
//! wants.

use canton_core::{Error, Result};
use canton_daml as rt;
use canton_daml::Contract as _;
use canton_ledger::CantonClient;
use canton_ledger::request::ActiveContractsRequest;
use canton_splice_api_token_holding_v1::splice_api_token_holding_v1::Splice_Api_Token_HoldingV1::HoldingView;
use futures_util::StreamExt as _;

use crate::Holding;

/// One holding, as the ledger reports it.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct HoldingSummary {
    /// The contract id, untyped: the same contract is a V1 and a V2 `Holding`.
    pub contract_id: String,
    /// The interface view — owner, instrument, amount, lock, meta.
    pub view: HoldingView,
}

impl HoldingSummary {
    /// Locked holdings cannot be spent until the lock expires or is released;
    /// naming one as an input fails at the interpreter.
    #[must_use]
    pub fn is_locked(&self) -> bool {
        self.view.lock.is_some()
    }

    /// The instrument this holding is of.
    #[must_use]
    pub fn instrument_id(&self) -> &str {
        &self.view.instrument_id.id
    }

    /// The amount held.
    #[must_use]
    pub fn amount(&self) -> &rt::Numeric {
        &self.view.amount
    }

    /// The contract id as a `ContractId` of whichever `Holding` the caller's
    /// workflow wants — `canton_token::Holding` for V1, the V2 crate's for V2.
    #[must_use]
    pub fn typed<T>(&self) -> rt::ContractId<T> {
        rt::ContractId::new(self.contract_id.clone())
    }
}

/// The holdings `owner` has, optionally of one instrument, as of the current
/// ledger end.
///
/// # Errors
/// As any ledger read, plus [`Error::UnexpectedResponse`] if a `Holding` view
/// the participant returned does not decode as one.
pub async fn holdings(
    ledger: &CantonClient,
    owner: &str,
    instrument_id: Option<&str>,
) -> Result<Vec<HoldingSummary>> {
    let end = ledger.ledger_end().await?;
    let id = Holding::template_id();
    let interface = format!("{}:{}:{}", id.package_id, id.module_name, id.entity_name);
    let request =
        ActiveContractsRequest::new(vec![owner.to_string()], end).for_interfaces([interface])?;
    let stream = ledger.active_contracts_with(request).await?;
    futures_util::pin_mut!(stream);

    let mut out = Vec::new();
    while let Some(item) = stream.next().await {
        let active = item?;
        let Some(event) = active.created_event else {
            continue;
        };
        let Some(record) = event
            .interface_views
            .iter()
            .find_map(|v| v.view_value.as_ref())
        else {
            continue;
        };
        let view: HoldingView = rt::from_record(record).map_err(|e| {
            Error::UnexpectedResponse(format!(
                "holding {} carries a view that is not a HoldingView: {e}",
                event.contract_id
            ))
        })?;
        if instrument_id.is_some_and(|want| view.instrument_id.id != want) {
            continue;
        }
        out.push(HoldingSummary {
            contract_id: event.contract_id,
            view,
        });
    }
    Ok(out)
}

/// The spendable subset: every holding of `instrument_id` that is not locked.
/// What a transfer or allocation should name as its inputs.
///
/// # Errors
/// As [`holdings`].
pub async fn spendable(
    ledger: &CantonClient,
    owner: &str,
    instrument_id: &str,
) -> Result<Vec<HoldingSummary>> {
    Ok(holdings(ledger, owner, Some(instrument_id))
        .await?
        .into_iter()
        .filter(|holding| !holding.is_locked())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn summary(locked: bool, instrument: &str) -> HoldingSummary {
        use canton_splice_api_token_holding_v1::splice_api_token_holding_v1::Splice_Api_Token_HoldingV1 as h;
        use canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1 as md;
        HoldingSummary {
            contract_id: "00abc".to_string(),
            view: HoldingView {
                owner: rt::Party::parse("alice::1220aa").unwrap(),
                instrument_id: h::InstrumentId {
                    admin: rt::Party::parse("dso::1220bb").unwrap(),
                    id: instrument.to_string(),
                },
                amount: "1.5".parse().unwrap(),
                lock: locked.then(|| h::Lock {
                    holders: vec![],
                    expires_at: None,
                    expires_after: None,
                    context: None,
                }),
                meta: md::Metadata {
                    values: rt::TextMap::new(),
                },
            },
        }
    }

    #[test]
    fn a_lock_is_what_makes_a_holding_unspendable() {
        assert!(summary(true, "Amulet").is_locked());
        assert!(!summary(false, "Amulet").is_locked());
        assert_eq!(summary(false, "Amulet").instrument_id(), "Amulet");
    }

    #[test]
    fn the_same_id_serves_either_holding_interface() {
        let s = summary(false, "Amulet");
        let v1: rt::ContractId<crate::Holding> = s.typed();
        assert_eq!(v1.as_str(), "00abc");
    }
}
