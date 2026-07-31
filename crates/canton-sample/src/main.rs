//! Reference app for the Canton Rust SDK — the typed codegen (M2) and the Ledger
//! API client (M1) end to end, against the cn-quickstart licensing app.
//!
//! It builds a **typed** `AppInstallRequest` from the generated
//! `canton-quickstart-licensing` bindings, round-trips it through both codecs,
//! and then runs the full milestone-2 verification loop on **both transports**:
//! codegen → submit → observe the committed transaction → **decode it back into
//! the typed payload** (`from_created_event`) → query the ACS → **exercise a
//! typed choice** on the created contract, over gRPC and over JSON.
//!
//! Offline (no ledger) it demonstrates the codegen + codecs and always runs. To
//! run the live loop, set:
//!   - `LEDGER_ENDPOINT`      — gRPC Ledger API (e.g. http://localhost:3901)
//!   - `LEDGER_JSON_ENDPOINT` — JSON Ledger API (e.g. http://localhost:3975)
//!   - `LEDGER_TOKEN`         — a bearer token for the acting party
//!   - `LEDGER_PARTY`         — the acting party id

use canton_daml as rt;
use canton_ledger::{CantonClient, Config, JsonClient, JsonCommands, Submit};
use canton_quickstart_licensing::quickstart_licensing_0_0_1::Licensing_AppInstall::{
    AppInstallRequest, AppInstallRequest_Reject,
};
use canton_quickstart_licensing::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata;
use rt::Template as _;

type Error = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let party =
        std::env::var("LEDGER_PARTY").unwrap_or_else(|_| "app_provider::example".to_string());
    let request = AppInstallRequest {
        provider: rt::Party::new(party.clone()),
        user: rt::Party::new(party.clone()),
        meta: Metadata {
            values: rt::TextMap::new(),
        },
    };

    demonstrate_codecs(&request)?;

    let template_id = <AppInstallRequest as rt::Contract>::template_id();
    println!(
        "\ntemplate id {}:{}:{}",
        template_id.package_id, template_id.module_name, template_id.entity_name,
    );

    let token = std::env::var("LEDGER_TOKEN").ok();
    let mut ran_live = false;
    if let Ok(endpoint) = std::env::var("LEDGER_ENDPOINT") {
        Box::pin(run_grpc(endpoint, token.clone(), &party, &request)).await?;
        ran_live = true;
    }
    if let Ok(endpoint) = std::env::var("LEDGER_JSON_ENDPOINT") {
        Box::pin(run_json(endpoint, token, &party, &request)).await?;
        ran_live = true;
    }
    if !ran_live {
        println!(
            "\n(offline) set LEDGER_ENDPOINT and/or LEDGER_JSON_ENDPOINT (+ LEDGER_TOKEN, \
             LEDGER_PARTY) to run the live submit → transaction → ACS loop."
        );
    }
    Ok(())
}

/// Show that a generated type round-trips through both wire codecs.
fn demonstrate_codecs(request: &AppInstallRequest) -> Result<(), Error> {
    let json = serde_json::to_string_pretty(request)?;
    println!("AppInstallRequest as LF-JSON:\n{json}");
    let from_json: AppInstallRequest = serde_json::from_str(&json)?;
    assert_eq!(&from_json, request, "JSON round-trip");

    let value = rt::ToValue::to_value(request);
    let from_value = <AppInstallRequest as rt::FromValue>::from_value(&value)?;
    assert_eq!(&from_value, request, "gRPC Value round-trip");

    println!("codec round-trip OK (JSON + gRPC)");
    Ok(())
}

/// The gRPC loop: submit the typed create, observe the committed transaction,
/// then query the ACS and confirm the created contract is present.
async fn run_grpc(
    endpoint: String,
    token: Option<String>,
    party: &str,
    request: &AppInstallRequest,
) -> Result<(), Error> {
    use tokio_stream::StreamExt as _;

    println!("\n=== gRPC transport ===");
    let mut config = Config::new(endpoint);
    if let Some(token) = token {
        config = config.with_token(token);
    }
    let client = CantonClient::connect_lazy(config)?;

    // codegen → submit → observe transaction.
    let command = rt::create_command(request);
    let tx = client
        .submit_and_wait_for_transaction(Submit::new(party).add_command(command))
        .await?;
    let created = created_contract_ids(&tx.events);
    println!(
        "submitted — update id {}, {} event(s), offset {}; created {:?}",
        tx.update_id,
        tx.events.len(),
        tx.offset,
        created,
    );

    // The typed READ path: decode the committed event back into the payload.
    let created_event = first_created_event(&tx.events).ok_or("no created event")?;
    let read_back = AppInstallRequest::from_created_event(created_event)?;
    assert_eq!(&read_back, request, "typed read-back matches the submit");
    println!(
        "typed read-back — AppInstallRequest for user {}",
        read_back.user
    );

    // query ACS → confirm the created contract is active.
    let offset = client.ledger_end().await?;
    let stream = client
        .active_contracts(vec![party.to_string()], offset)
        .await?;
    tokio::pin!(stream);
    let mut total = 0usize;
    let mut found = false;
    while let Some(active) = stream.next().await {
        let active = active?;
        total += 1;
        if let Some(event) = &active.created_event
            && created.contains(&event.contract_id)
        {
            found = true;
        }
    }
    println!("ACS: {total} active contract(s); our create present: {found}");
    assert!(found, "the created contract should be in the ACS");

    // A typed EXERCISE: reject the install request (consuming), and observe
    // the archive in the committed transaction.
    let contract_id: rt::ContractId<AppInstallRequest> =
        rt::ContractId::new(created_event.contract_id.clone());
    let reject = rt::exercise_command(
        &contract_id,
        &AppInstallRequest_Reject {
            meta: Metadata {
                values: rt::TextMap::new(),
            },
        },
    );
    let tx = client
        .submit_and_wait_for_transaction(Submit::new(party).add_command(reject))
        .await?;
    println!(
        "exercised AppInstallRequest_Reject — update id {}, {} event(s)",
        tx.update_id,
        tx.events.len(),
    );
    Ok(())
}

/// The first `CreatedEvent` in a transaction's events.
fn first_created_event(
    events: &[canton_ledger::proto::Event],
) -> Option<&canton_ledger::proto::CreatedEvent> {
    use canton_ledger::proto::event::Event;
    events.iter().find_map(|event| match &event.event {
        Some(Event::Created(created)) => Some(created),
        _ => None,
    })
}

/// The JSON loop: the same submit → observe → ACS, over the JSON Ledger API.
async fn run_json(
    endpoint: String,
    token: Option<String>,
    party: &str,
    request: &AppInstallRequest,
) -> Result<(), Error> {
    println!("\n=== JSON transport ===");
    let mut client = JsonClient::new(endpoint);
    if let Some(token) = token {
        client = client.with_token(token);
    }

    let id = <AppInstallRequest as rt::Contract>::template_id();
    let template_id = format!("{}:{}:{}", id.package_id, id.module_name, id.entity_name);
    // codegen → submit (the payload's LF-JSON is the create argument).
    let arguments = serde_json::to_value(request)?;
    let commands = JsonCommands::new(vec![party.to_string()]).add_create(template_id, arguments);
    let response = client.submit_and_wait_for_transaction(&commands).await?;
    let created = json_created_contract_ids(&response.transaction.events);
    println!(
        "submitted — update id {}, {} event(s), offset {}; created {:?}",
        response.transaction.update_id,
        response.transaction.events.len(),
        response.transaction.offset,
        created,
    );

    // Confirm our specific contract, as the gRPC lane does. The JSON ACS is a
    // *bounded* read (a `limit`, or the node returns 413), so on a busy party our
    // fresh contract may fall outside the window; the committed transaction is
    // read back reliably from the bounded update range `(offset - 1, offset]`.
    let offset = response.transaction.offset;
    let updates = client
        .updates(vec![party.to_string()], offset - 1, Some(offset), Some(50))
        .await?;
    let found = created
        .iter()
        .any(|id| updates.iter().any(|update| update.to_string().contains(id)));
    println!("read back — our create present: {found}");
    assert!(found, "the committed transaction should contain our create");

    // …and the ACS snapshot is a non-empty bounded read.
    let acs = client
        .active_contracts(vec![party.to_string()], offset, Some(200))
        .await?;
    println!("ACS: {} active contract(s)", acs.len());
    assert!(!acs.is_empty(), "the ACS snapshot should be non-empty");
    Ok(())
}

/// The contract ids created by a JSON transaction's events
/// (`{"CreatedEvent": {"contractId": …}}`).
fn json_created_contract_ids(events: &[serde_json::Value]) -> Vec<String> {
    events
        .iter()
        .filter_map(|event| {
            event
                .get("CreatedEvent")?
                .get("contractId")?
                .as_str()
                .map(String::from)
        })
        .collect()
}

/// The contract ids created by a transaction's events.
fn created_contract_ids(events: &[canton_ledger::proto::Event]) -> Vec<String> {
    use canton_ledger::proto::event::Event;
    events
        .iter()
        .filter_map(|event| match &event.event {
            Some(Event::Created(created)) => Some(created.contract_id.clone()),
            _ => None,
        })
        .collect()
}
