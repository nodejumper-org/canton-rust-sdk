//! Settle V2 allocations — the executor's move, and the step that makes an
//! allocation a delivery. `v2_allocate` reserves the sender's holdings for a
//! settlement; this finds every allocation naming the caller as executor and
//! settles each settlement as one batch through the registry's settlement
//! factory, so all of its legs move in one transaction or none do.
//!
//! ```sh
//! CANTON_TOKEN_REGISTRY_URL=http://localhost:5012 \
//! CANTON_TEST_ENDPOINT=http://localhost:3901 \
//! CANTON_TOKEN_EXECUTOR='sv::1220…' \
//!   cargo run -p canton-token --example v2_settle
//! ```
//!
//! `CANTON_TOKEN_SETTLEMENT` restricts the run to one settlement id.
//! Authentication as in `v1_transfer`, for the executor's user;
//! `CANTON_TOKEN_DRY_RUN=1` builds without submitting.

use std::collections::BTreeMap;

use canton_auth::{OidcConfig, TokenProvider};
use canton_daml as rt;
use canton_daml::Contract as _;
use canton_ledger::request::ActiveContractsRequest;
use canton_ledger::{CantonClient, Config};
use canton_token::RegistryClient;
use canton_token::types::metadata as md;
use canton_token::types::v2::allocation as al;
use futures_util::StreamExt as _;

fn var(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|_| format!("set {name}"))
}

/// Attach whatever credentials the environment offers: a bearer token as
/// issued (`CANTON_TOKEN`), or OIDC client credentials to obtain one, with
/// `CANTON_TEST_AUDIENCE` where the issuer wants it. An unauthenticated
/// LocalNet needs neither.
fn authenticate(config: Config) -> Config {
    match (
        std::env::var("CANTON_TOKEN"),
        std::env::var("CANTON_TEST_TOKEN_URL"),
        std::env::var("CANTON_TEST_CLIENT_ID"),
        std::env::var("CANTON_TEST_CLIENT_SECRET"),
    ) {
        (Ok(token), ..) => config.with_token(token),
        (_, Ok(url), Ok(id), Ok(secret)) => {
            let mut oidc = OidcConfig::new(url, id, secret);
            if let Ok(audience) = std::env::var("CANTON_TEST_AUDIENCE") {
                oidc = oidc.with_audience(audience);
            }
            config.with_oidc(TokenProvider::new(oidc))
        }
        _ => config,
    }
}

/// The allocations visible to `party` that name it as an executor, read from
/// the active contract set through the `Allocation` interface.
async fn allocations_to_settle(
    client: &CantonClient,
    party: &rt::Party,
) -> Result<Vec<(String, al::AllocationView)>, Box<dyn std::error::Error>> {
    let end = client.ledger_end().await?;
    let id = al::Allocation::template_id();
    let interface = format!("{}:{}:{}", id.package_id, id.module_name, id.entity_name);
    let request =
        ActiveContractsRequest::new(vec![party.to_string()], end).for_interfaces([interface])?;
    let stream = client.active_contracts_with(request).await?;
    futures_util::pin_mut!(stream);
    let mut out = Vec::new();
    while let Some(item) = stream.next().await {
        let Some(event) = item?.created_event else {
            continue;
        };
        let Some(record) = event
            .interface_views
            .iter()
            .find_map(|v| v.view_value.as_ref())
        else {
            continue;
        };
        let view: al::AllocationView = rt::from_record(record)?;
        if view.settlement.executors.contains(party) {
            out.push((event.contract_id, view));
        }
    }
    Ok(out)
}

/// The transfer leg an allocation's sender side describes: from the
/// authorizer's account to the other side's, for the amount reserved.
fn legs_of(view: &al::AllocationView) -> Vec<al::TransferLeg> {
    view.allocation
        .transfer_leg_sides
        .iter()
        .map(|side| {
            let (sender, receiver) = match side.side {
                al::TransferSide::SenderSide => {
                    (view.allocation.authorizer.clone(), side.otherside.clone())
                }
                al::TransferSide::ReceiverSide => {
                    (side.otherside.clone(), view.allocation.authorizer.clone())
                }
            };
            al::TransferLeg {
                transfer_leg_id: side.transfer_leg_id.clone(),
                sender,
                receiver,
                amount: side.amount.clone(),
                instrument_id: side.instrument_id.clone(),
                meta: md::Metadata {
                    values: rt::TextMap::new(),
                },
            }
        })
        .collect()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = RegistryClient::new(&var("CANTON_TOKEN_REGISTRY_URL")?)?;
    let executor = rt::Party::parse(&var("CANTON_TOKEN_EXECUTOR")?)?;
    let client =
        CantonClient::connect_lazy(authenticate(Config::new(var("CANTON_TEST_ENDPOINT")?)))?;

    let mut found = allocations_to_settle(&client, &executor).await?;
    if let Ok(only) = std::env::var("CANTON_TOKEN_SETTLEMENT") {
        found.retain(|(_, view)| view.settlement.id == only);
    }
    println!("executor:    {executor}");
    println!("allocations: {} naming it as executor", found.len());

    // One batch per settlement: every allocation made for it settles together.
    let mut batches: BTreeMap<String, Vec<(String, al::AllocationView)>> = BTreeMap::new();
    for (cid, view) in found {
        batches
            .entry(view.settlement.id.clone())
            .or_default()
            .push((cid, view));
    }

    for (settlement_id, allocations) in batches {
        let settlement = allocations[0].1.settlement.clone();
        // Both sides of a leg describe the same leg; the batch names it once.
        let mut by_id: BTreeMap<String, al::TransferLeg> = BTreeMap::new();
        for leg in allocations.iter().flat_map(|(_, v)| legs_of(v)) {
            by_id.entry(leg.transfer_leg_id.clone()).or_insert(leg);
        }
        let legs: Vec<al::TransferLeg> = by_id.into_values().collect();
        let finalized: Vec<al::FinalizedAllocation> = allocations
            .iter()
            .map(|(cid, _)| al::FinalizedAllocation {
                allocation_cid: rt::ContractId::new(cid.clone()),
                extra_transfer_leg_sides: Vec::new(),
                next_iteration_funding: None,
            })
            .collect();
        println!(
            "settlement {settlement_id}: {} allocation(s), {} leg(s)",
            allocations.len(),
            legs.len()
        );
        for leg in &legs {
            println!(
                "  leg {}: {} -> {} : {} {}",
                leg.transfer_leg_id,
                leg.sender.owner.as_ref().map_or("?", |p| p.as_str()),
                leg.receiver.owner.as_ref().map_or("?", |p| p.as_str()),
                leg.amount,
                leg.instrument_id
            );
        }

        let command = canton_token::v2::settle_batch(
            &registry,
            settlement,
            legs,
            finalized,
            vec![executor.clone()],
        )
        .await?;
        println!(
            "  disclosing {} contract(s) the registry named",
            command.disclosed_contracts().len()
        );
        if std::env::var_os("CANTON_TOKEN_DRY_RUN").is_some() {
            println!("  dry run: built and not submitted");
            continue;
        }
        let transaction = client
            .submit_and_wait_for_transaction(command.into_submit(executor.as_str()))
            .await?;
        println!(
            "  settled: {} at offset {} with {} event(s)",
            transaction.update_id,
            transaction.offset,
            transaction.events.len()
        );
    }
    Ok(())
}
