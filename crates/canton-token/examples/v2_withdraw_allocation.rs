//! Withdraw a V2 allocation — the sender's move, releasing the holdings it
//! reserved. The other half of `v2_allocate`: an allocation the executor
//! never settles would otherwise hold the sender's coin until its deadline.
//!
//! ```sh
//! CANTON_TOKEN_REGISTRY_URL=http://localhost:5012 \
//! CANTON_TEST_ENDPOINT=http://localhost:3901 \
//! CANTON_TOKEN_SENDER='app_provider…::1220…' \
//!   cargo run -p canton-token --example v2_withdraw_allocation
//! ```
//!
//! Without `CANTON_TOKEN_ALLOCATION` (a contract id) every active allocation
//! the sender authorised is withdrawn, one submission each. Authentication as
//! in `v1_transfer`; `CANTON_TOKEN_DRY_RUN=1` lists and builds without
//! submitting.

use canton_auth::{OidcConfig, TokenProvider};
use canton_daml as rt;
use canton_daml::Contract as _;
use canton_ledger::request::ActiveContractsRequest;
use canton_ledger::{CantonClient, Config};
use canton_token::RegistryClient;
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

/// The sender's active allocations: contract id and the view the interface
/// exposes, read from the active contract set.
async fn allocations(
    client: &CantonClient,
    sender: &str,
) -> Result<Vec<(String, al::AllocationView)>, Box<dyn std::error::Error>> {
    let end = client.ledger_end().await?;
    let id = al::Allocation::template_id();
    let interface = format!("{}:{}:{}", id.package_id, id.module_name, id.entity_name);
    let request =
        ActiveContractsRequest::new(vec![sender.to_string()], end).for_interfaces([interface])?;
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
        out.push((event.contract_id, view));
    }
    Ok(out)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let registry = RegistryClient::new(&var("CANTON_TOKEN_REGISTRY_URL")?)?;
    let sender = rt::Party::parse(&var("CANTON_TOKEN_SENDER")?)?;
    let client =
        CantonClient::connect_lazy(authenticate(Config::new(var("CANTON_TEST_ENDPOINT")?)))?;

    let mut active = allocations(&client, sender.as_str()).await?;
    if let Ok(only) = std::env::var("CANTON_TOKEN_ALLOCATION") {
        active.retain(|(cid, _)| *cid == only);
    }
    println!("sender:      {sender}");
    println!("allocations: {} active", active.len());
    for (cid, view) in &active {
        println!(
            "  {cid}\n    settlement {} by {}, {} holding(s) reserved",
            view.settlement.id,
            view.settlement
                .executors
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", "),
            view.holding_cids.len()
        );
    }

    for (cid, view) in active {
        let allocation = rt::ContractId::<al::Allocation>::new(cid.clone());
        let command =
            canton_token::v2::withdraw_allocation(&registry, &allocation, vec![sender.clone()])
                .await?;
        println!(
            "withdrawing {} (settlement {}): disclosing {} contract(s) the registry named",
            &cid[..16],
            view.settlement.id,
            command.disclosed_contracts().len()
        );
        if std::env::var_os("CANTON_TOKEN_DRY_RUN").is_some() {
            println!("dry run: built and not submitted");
            continue;
        }
        let transaction = client
            .submit_and_wait_for_transaction(command.into_submit(sender.as_str()))
            .await?;
        println!(
            "withdrawn:   {} at offset {} with {} event(s)",
            transaction.update_id,
            transaction.offset,
            transaction.events.len()
        );
    }
    Ok(())
}
