//! Live integration tests against a running participant node.
//!
//! Gated on `CANTON_TEST_ENDPOINT` so `cargo test` stays green without a node.
//! Run against LocalNet's App Provider:
//!
//! ```sh
//! CANTON_TEST_ENDPOINT=http://localhost:3901 \
//!   cargo test -p canton-ledger --test live -- --nocapture
//! ```
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::large_futures)]

use canton_auth::{OidcConfig, TokenProvider};
use canton_ledger::{
    CantonClient, Config, JsonClient, JsonCommands, RetryConfig, Submit, create, exercise,
    identifier, record, value,
};

fn endpoint() -> Option<String> {
    std::env::var("CANTON_TEST_ENDPOINT").ok()
}

fn oidc() -> Option<OidcConfig> {
    Some(OidcConfig::new(
        std::env::var("CANTON_TEST_TOKEN_URL").ok()?,
        std::env::var("CANTON_TEST_CLIENT_ID").ok()?,
        std::env::var("CANTON_TEST_CLIENT_SECRET").ok()?,
    ))
}

/// Full authenticated setup: `(client, party, pkg)`, or `None` to skip.
fn full_setup() -> Option<(CantonClient, String, String)> {
    let client =
        CantonClient::connect_lazy(Config::new(endpoint()?).with_oidc(TokenProvider::new(oidc()?)))
            .ok()?;
    let party = std::env::var("CANTON_TEST_PARTY").ok()?;
    let pkg = std::env::var("CANTON_TEST_LICENSING_PKG").ok()?;
    Some((client, party, pkg))
}

/// The offset carried by a paged update response item.
fn update_page_offset(item: &canton_ledger::proto::GetUpdateResponse) -> Option<i64> {
    use canton_ledger::proto::get_update_response::Update;
    match &item.update {
        Some(Update::Transaction(t)) => Some(t.offset),
        Some(Update::Reassignment(r)) => Some(r.offset),
        Some(Update::TopologyTransaction(t)) => Some(t.offset),
        None => None,
    }
}

/// The contract id of the first created event in a transaction, if any.
fn created_contract_id(tx: &canton_ledger::proto::Transaction) -> Option<String> {
    use canton_ledger::proto::event::Event;
    tx.events.iter().find_map(|e| match &e.event {
        Some(Event::Created(created)) => Some(created.contract_id.clone()),
        _ => None,
    })
}

/// Build an `AppInstallRequest` create acting as `party` (empty metadata).
fn app_install(party: &str, pkg: &str) -> canton_ledger::proto::Command {
    let template = identifier(pkg, "Licensing.AppInstall", "AppInstallRequest");
    let arguments = record(vec![
        ("provider", value::party(party)),
        ("user", value::party(party)),
        (
            "meta",
            value::record(record(vec![("values", value::empty_text_map())])),
        ),
    ]);
    create(template, arguments)
}

#[tokio::test]
async fn version_returns_from_live_node() {
    let Some(ep) = endpoint() else {
        eprintln!("skipping version_returns_from_live_node: set CANTON_TEST_ENDPOINT");
        return;
    };

    let client = CantonClient::connect_lazy(Config::new(ep)).expect("valid config");
    let version = client.version().await.expect("version rpc should succeed");

    assert!(
        !version.is_empty(),
        "ledger api version should be non-empty"
    );
    println!("live ledger api version: {version}");
}

#[tokio::test]
async fn health_check_reports_serving() {
    use canton_ledger::ServingStatus;

    let Some(ep) = endpoint() else {
        eprintln!("skipping health_check_reports_serving: set CANTON_TEST_ENDPOINT");
        return;
    };

    // Health is unauthenticated and served on the Ledger API port.
    let client = CantonClient::connect_lazy(Config::new(ep)).expect("valid config");
    let status = client
        .health_check()
        .await
        .expect("health rpc should succeed");

    assert_eq!(
        status,
        ServingStatus::Serving,
        "a running participant should report SERVING"
    );
    println!("live health status: {status:?}");
}

#[tokio::test]
async fn ledger_end_with_oidc_auth() {
    let (Some(ep), Some(oidc_config)) = (endpoint(), oidc()) else {
        eprintln!(
            "skipping ledger_end_with_oidc_auth: set CANTON_TEST_ENDPOINT + \
             CANTON_TEST_TOKEN_URL/CLIENT_ID/CLIENT_SECRET"
        );
        return;
    };

    let provider = TokenProvider::new(oidc_config);
    let client =
        CantonClient::connect_lazy(Config::new(ep).with_oidc(provider)).expect("valid config");
    let offset = client
        .ledger_end()
        .await
        .expect("ledger_end rpc should succeed");

    assert!(offset >= 0, "ledger end offset should be non-negative");
    println!("live ledger end offset: {offset}");
}

#[tokio::test]
async fn create_contract_and_read_transaction() {
    let (Some(ep), Some(oidc_config)) = (endpoint(), oidc()) else {
        eprintln!("skipping create_contract_and_read_transaction: auth env not set");
        return;
    };
    let (Ok(party), Ok(pkg)) = (
        std::env::var("CANTON_TEST_PARTY"),
        std::env::var("CANTON_TEST_LICENSING_PKG"),
    ) else {
        eprintln!("skipping: set CANTON_TEST_PARTY + CANTON_TEST_LICENSING_PKG");
        return;
    };

    let provider = TokenProvider::new(oidc_config);
    let client =
        CantonClient::connect_lazy(Config::new(ep).with_oidc(provider)).expect("valid config");

    // Create an AppInstallRequest with the acting party as both provider and
    // user, and empty metadata. signatory = user, so acting-as `party` suffices.
    let template = identifier(pkg, "Licensing.AppInstall", "AppInstallRequest");
    let arguments = record(vec![
        ("provider", value::party(&party)),
        ("user", value::party(&party)),
        (
            "meta",
            value::record(record(vec![("values", value::empty_text_map())])),
        ),
    ]);
    let command = create(template, arguments);

    let transaction = client
        .submit_and_wait_for_transaction(Submit::new(&party).add_command(command))
        .await
        .expect("submit_and_wait_for_transaction should succeed");

    assert!(!transaction.update_id.is_empty(), "update_id should be set");
    assert!(
        !transaction.events.is_empty(),
        "transaction should contain the created event"
    );
    println!(
        "created — update_id={} events={} offset={}",
        transaction.update_id,
        transaction.events.len(),
        transaction.offset
    );
}

#[tokio::test]
async fn duplicate_command_id_is_deduplicated() {
    let (Some(ep), Some(oidc_config)) = (endpoint(), oidc()) else {
        eprintln!("skipping duplicate_command_id_is_deduplicated: auth env not set");
        return;
    };
    let (Ok(party), Ok(pkg)) = (
        std::env::var("CANTON_TEST_PARTY"),
        std::env::var("CANTON_TEST_LICENSING_PKG"),
    ) else {
        eprintln!("skipping: set CANTON_TEST_PARTY + CANTON_TEST_LICENSING_PKG");
        return;
    };

    let provider = TokenProvider::new(oidc_config);
    let client =
        CantonClient::connect_lazy(Config::new(ep).with_oidc(provider)).expect("valid config");

    // A fixed command id makes the change ID stable across both submissions.
    let command_id = format!("sdk-dedup-{}", uuid::Uuid::new_v4());
    let build = || {
        let template = identifier(&pkg, "Licensing.AppInstall", "AppInstallRequest");
        let arguments = record(vec![
            ("provider", value::party(&party)),
            ("user", value::party(&party)),
            (
                "meta",
                value::record(record(vec![("values", value::empty_text_map())])),
            ),
        ]);
        Submit::new(&party)
            .with_command_id(&command_id)
            .add_command(create(template, arguments))
    };

    let first = client.submit_and_wait_for_transaction(build()).await;
    assert!(first.is_ok(), "first submission should succeed: {first:?}");

    let second = client.submit_and_wait_for_transaction(build()).await;
    assert!(
        second.is_err(),
        "second submission with the same command_id should be de-duplicated, got {second:?}"
    );
    println!("dedup — first ok, second rejected: {}", second.unwrap_err());
}

#[tokio::test]
async fn await_completion_recovers_a_submitted_command() {
    let (Some(ep), Some(oidc_config)) = (endpoint(), oidc()) else {
        eprintln!("skipping await_completion_recovers_a_submitted_command: auth env not set");
        return;
    };
    let (Ok(party), Ok(pkg)) = (
        std::env::var("CANTON_TEST_PARTY"),
        std::env::var("CANTON_TEST_LICENSING_PKG"),
    ) else {
        eprintln!("skipping: set CANTON_TEST_PARTY + CANTON_TEST_LICENSING_PKG");
        return;
    };

    let provider = TokenProvider::new(oidc_config);
    let client =
        CantonClient::connect_lazy(Config::new(ep).with_oidc(provider)).expect("valid config");

    // Remember the offset before submitting, so recovery can scan from there.
    let begin = client.ledger_end().await.expect("ledger_end");

    let command_id = format!("sdk-compl-{}", uuid::Uuid::new_v4());
    let template = identifier(&pkg, "Licensing.AppInstall", "AppInstallRequest");
    let arguments = record(vec![
        ("provider", value::party(&party)),
        ("user", value::party(&party)),
        (
            "meta",
            value::record(record(vec![("values", value::empty_text_map())])),
        ),
    ]);
    client
        .submit_and_wait_for_transaction(
            Submit::new(&party)
                .with_command_id(&command_id)
                .add_command(create(template, arguments)),
        )
        .await
        .expect("submit should succeed");

    // Recover the command's completion from the stream (the method bounds the
    // wait internally so a missing completion can't hang).
    let completion = client
        .await_completion(
            &command_id,
            vec![party.clone()],
            begin,
            std::time::Duration::from_secs(15),
        )
        .await
        .expect("completion should be found");

    assert_eq!(completion.command_id, command_id);
    assert!(
        !completion.update_id.is_empty(),
        "completed command has an update id"
    );
    println!(
        "recovered completion — command_id={} offset={} update_id={}",
        completion.command_id, completion.offset, completion.update_id
    );
}

#[tokio::test]
async fn active_contract_set_snapshot() {
    use tokio_stream::StreamExt as _;
    let Some((client, party, pkg)) = full_setup() else {
        eprintln!("skipping active_contract_set_snapshot: env not set");
        return;
    };

    // Ensure at least one active contract exists.
    client
        .submit_and_wait_for_transaction(Submit::new(&party).add_command(app_install(&party, &pkg)))
        .await
        .expect("seed create should succeed");

    let offset = client.ledger_end().await.expect("ledger_end");
    let stream = client
        .active_contracts(vec![party.clone()], offset)
        .await
        .expect("open acs stream");
    tokio::pin!(stream);

    let count = tokio::time::timeout(std::time::Duration::from_secs(15), async {
        let mut n = 0usize;
        while let Some(item) = stream.next().await {
            let active = item?;
            assert!(
                active.created_event.is_some(),
                "active contract has a created event"
            );
            n += 1;
        }
        Ok::<usize, canton_ledger::Error>(n)
    })
    .await
    .expect("acs snapshot timed out")
    .expect("acs stream error");

    assert!(
        count >= 1,
        "expected at least one active contract, got {count}"
    );
    println!("ACS snapshot — {count} active contract(s)");
}

#[tokio::test]
async fn updates_stream_replays_a_created_transaction() {
    use tokio_stream::StreamExt as _;
    let Some((client, party, pkg)) = full_setup() else {
        eprintln!("skipping updates_stream_replays_a_created_transaction: env not set");
        return;
    };

    let begin = client.ledger_end().await.expect("ledger_end");
    client
        .submit_and_wait_for_transaction(Submit::new(&party).add_command(app_install(&party, &pkg)))
        .await
        .expect("create should succeed");

    let stream = client
        .updates(vec![party.clone()], begin)
        .await
        .expect("open updates stream");
    tokio::pin!(stream);

    let first = tokio::time::timeout(std::time::Duration::from_secs(15), stream.next())
        .await
        .expect("updates timed out")
        .expect("stream ended unexpectedly")
        .expect("update stream error");

    match first {
        canton_ledger::proto::get_updates_response::Update::Transaction(tx) => {
            assert!(!tx.update_id.is_empty());
            assert!(
                tx.offset > begin,
                "replayed tx should be after begin offset"
            );
            println!(
                "update — transaction offset={} events={}",
                tx.offset,
                tx.events.len()
            );
        }
        other => panic!("expected a transaction update, got {other:?}"),
    }
}

#[tokio::test]
async fn resumable_updates_yield_a_transaction() {
    use tokio_stream::StreamExt as _;
    let Some((client, party, pkg)) = full_setup() else {
        eprintln!("skipping resumable_updates_yield_a_transaction: env not set");
        return;
    };

    let begin = client.ledger_end().await.expect("ledger_end");
    client
        .submit_and_wait_for_transaction(Submit::new(&party).add_command(app_install(&party, &pkg)))
        .await
        .expect("create should succeed");

    let stream = client.updates_resumable(vec![party.clone()], begin);
    tokio::pin!(stream);

    let first = tokio::time::timeout(std::time::Duration::from_secs(15), stream.next())
        .await
        .expect("resumable updates timed out")
        .expect("stream ended unexpectedly")
        .expect("update stream error");

    assert!(matches!(
        first,
        canton_ledger::proto::get_updates_response::Update::Transaction(_)
    ));
    println!("resumable — first update received");
}

#[tokio::test]
async fn json_transport_version_and_ledger_end() {
    let Some(json_url) = std::env::var("CANTON_TEST_JSON_ENDPOINT").ok() else {
        eprintln!("skipping json_transport_version_and_ledger_end: set CANTON_TEST_JSON_ENDPOINT");
        return;
    };

    // version is unauthenticated
    let version = JsonClient::new(&json_url)
        .version()
        .await
        .expect("json version should succeed");
    assert!(!version.is_empty());

    // ledger end is authenticated
    let Some(oidc_config) = oidc() else {
        eprintln!("skipping json ledger-end: auth env not set");
        return;
    };
    let offset = JsonClient::new(&json_url)
        .with_oidc(TokenProvider::new(oidc_config))
        .ledger_end()
        .await
        .expect("json ledger_end should succeed");
    assert!(offset >= 0);

    println!("JSON transport — version={version} ledger_end={offset}");
}

#[tokio::test]
async fn retry_enabled_client_works_on_the_happy_path() {
    let (Some(ep), Some(oidc_config)) = (endpoint(), oidc()) else {
        eprintln!("skipping retry_enabled_client_works_on_the_happy_path: auth env not set");
        return;
    };

    let client = CantonClient::connect_lazy(
        Config::new(ep)
            .with_oidc(TokenProvider::new(oidc_config))
            .with_retry(RetryConfig::default()),
    )
    .expect("valid config");

    let offset = client
        .ledger_end()
        .await
        .expect("ledger_end under retry should succeed");
    assert!(offset >= 0);
    println!("retry-enabled — ledger_end={offset}");
}

#[tokio::test]
async fn rejected_command_surfaces_a_non_retriable_error() {
    let Some((client, party, pkg)) = full_setup() else {
        eprintln!("skipping rejected_command_surfaces_a_non_retriable_error: env not set");
        return;
    };

    // A create against a template that does not exist: the participant rejects
    // it during preprocessing. Exercises the error-classification path.
    let bad = create(
        identifier(&pkg, "Licensing.AppInstall", "NoSuchTemplate"),
        record(vec![]),
    );
    let error = client
        .submit_and_wait_for_transaction(Submit::new(&party).add_command(bad))
        .await
        .expect_err("submitting an unknown template must fail");

    assert!(
        !error.is_retriable(),
        "a rejected command must not be classified retriable: {error}"
    );
    assert!(
        error.code().is_some(),
        "a gRPC rejection should carry a status code: {error}"
    );
    println!(
        "rejection — code={:?} is_retriable={} err={error}",
        error.code(),
        error.is_retriable()
    );
}

#[tokio::test]
async fn exercise_choice_on_a_created_contract() {
    let Some((client, party, pkg)) = full_setup() else {
        eprintln!("skipping exercise_choice_on_a_created_contract: env not set");
        return;
    };

    // Create an AppInstallRequest, then take the created contract id.
    let create_tx = client
        .submit_and_wait_for_transaction(Submit::new(&party).add_command(app_install(&party, &pkg)))
        .await
        .expect("create should succeed");
    let contract_id = created_contract_id(&create_tx).expect("create yields a contract id");

    // Exercise AppInstallRequest_Reject { meta = {} } — controller is the
    // provider, which is our acting party; this consumes (archives) the request.
    let template = identifier(&pkg, "Licensing.AppInstall", "AppInstallRequest");
    let argument = value::record(record(vec![(
        "meta",
        value::record(record(vec![("values", value::empty_text_map())])),
    )]));
    let exercise_tx = client
        .submit_and_wait_for_transaction(Submit::new(&party).add_command(exercise(
            template,
            &contract_id,
            "AppInstallRequest_Reject",
            argument,
        )))
        .await
        .expect("exercise should succeed");

    assert!(
        !exercise_tx.events.is_empty(),
        "exercise transaction should contain events"
    );
    println!(
        "exercise — rejected {contract_id}: events={} offset={}",
        exercise_tx.events.len(),
        exercise_tx.offset
    );
}

#[tokio::test]
async fn multiple_commands_submit_atomically() {
    let Some((client, party, pkg)) = full_setup() else {
        eprintln!("skipping multiple_commands_submit_atomically: env not set");
        return;
    };

    // Two creates in one Submit → one atomic transaction with two created events.
    let tx = client
        .submit_and_wait_for_transaction(
            Submit::new(&party)
                .add_command(app_install(&party, &pkg))
                .add_command(app_install(&party, &pkg)),
        )
        .await
        .expect("multi-command submit should succeed");

    let created = tx
        .events
        .iter()
        .filter(|e| {
            matches!(
                &e.event,
                Some(canton_ledger::proto::event::Event::Created(_))
            )
        })
        .count();
    assert_eq!(created, 2, "both creates should land in one transaction");
    assert!(!tx.update_id.is_empty());
    println!(
        "multi-command — {created} creates in update {}",
        tx.update_id
    );
}

#[tokio::test]
async fn json_bad_token_is_an_http_error() {
    let Some(json_url) = std::env::var("CANTON_TEST_JSON_ENDPOINT").ok() else {
        eprintln!("skipping json_bad_token_is_an_http_error: set CANTON_TEST_JSON_ENDPOINT");
        return;
    };

    // ledger_end is authenticated; a bogus token must yield a classified HTTP
    // error (not a vague InvalidRequest), and a 4xx must be non-retriable.
    let error = JsonClient::new(&json_url)
        .with_token("not-a-real-token")
        .ledger_end()
        .await
        .expect_err("a bad token should be rejected");

    match &error {
        canton_ledger::Error::Http { status, .. } => {
            assert!((400..500).contains(status), "expected a 4xx, got {status}");
            assert!(!error.is_retriable(), "a 4xx must not be retriable");
        }
        other => panic!("expected Error::Http, got {other:?}"),
    }
    println!("json bad-token — {error}");
}

#[tokio::test]
async fn grpc_and_json_transports_agree() {
    let (Some((client, _, _)), Ok(json_url)) =
        (full_setup(), std::env::var("CANTON_TEST_JSON_ENDPOINT"))
    else {
        eprintln!("skipping grpc_and_json_transports_agree: env not set");
        return;
    };
    let Some(oidc_config) = oidc() else { return };
    let json = JsonClient::new(&json_url).with_oidc(TokenProvider::new(oidc_config));

    // version must match exactly across transports.
    let grpc_version = client.version().await.expect("grpc version");
    let json_version = json.version().await.expect("json version");
    assert_eq!(
        grpc_version, json_version,
        "gRPC and JSON must report the same version"
    );

    // ledger end can advance between the two reads; require JSON >= gRPC taken first.
    let grpc_end = client.ledger_end().await.expect("grpc ledger_end");
    let json_end = json.ledger_end().await.expect("json ledger_end");
    assert!(
        json_end >= grpc_end,
        "JSON ledger_end ({json_end}) should be >= the earlier gRPC read ({grpc_end})"
    );
    println!("parity — version={grpc_version} grpc_end={grpc_end} json_end={json_end}");
}

#[tokio::test]
async fn event_query_returns_the_created_event() {
    let Some((client, party, pkg)) = full_setup() else {
        eprintln!("skipping event_query_returns_the_created_event: env not set");
        return;
    };

    let tx = client
        .submit_and_wait_for_transaction(Submit::new(&party).add_command(app_install(&party, &pkg)))
        .await
        .expect("create should succeed");
    let contract_id = created_contract_id(&tx).expect("create yields a contract id");

    let events = client
        .events_by_contract_id(&contract_id, vec![party.clone()])
        .await
        .expect("event query should succeed");
    assert!(
        events.created.is_some(),
        "event query should report the created event"
    );
    println!("event-query — created present for {contract_id}");
}

#[tokio::test]
async fn acs_paging_walks_pages_via_token() {
    let Some((client, party, pkg)) = full_setup() else {
        eprintln!("skipping acs_paging_walks_pages_via_token: env not set");
        return;
    };

    // Seed a couple of contracts so there is > 1 page at page size 1.
    for _ in 0..2 {
        client
            .submit_and_wait_for_transaction(
                Submit::new(&party).add_command(app_install(&party, &pkg)),
            )
            .await
            .expect("seed create");
    }

    let offset = client.ledger_end().await.expect("ledger_end");
    let mut token = None;
    let mut pages = 0usize;
    let mut total = 0usize;
    loop {
        let (contracts, next) = client
            .active_contracts_page(vec![party.clone()], offset, 1, token)
            .await
            .expect("acs page");
        total += contracts.len();
        pages += 1;
        match next {
            Some(t) => token = Some(t),
            None => break,
        }
        assert!(pages <= 500, "paging did not terminate");
    }
    assert!(
        pages >= 2,
        "expected multiple pages at page size 1, got {pages}"
    );
    assert!(total >= 2, "expected to page through several contracts");
    println!("acs-paging — {total} contracts across {pages} pages");
}

#[tokio::test]
async fn updates_page_reverse_order_is_newest_first() {
    let Some((client, party, pkg)) = full_setup() else {
        eprintln!("skipping updates_page_reverse_order_is_newest_first: env not set");
        return;
    };

    let begin = client.ledger_end().await.expect("ledger_end");
    // Produce two updates after `begin`.
    for _ in 0..2 {
        client
            .submit_and_wait_for_transaction(
                Submit::new(&party).add_command(app_install(&party, &pkg)),
            )
            .await
            .expect("create");
    }
    let end = client.ledger_end().await.expect("ledger_end");

    let (items, _next) = client
        .updates_page(vec![party.clone()], begin, end, 100, true, None)
        .await
        .expect("updates page (reverse)");

    let offsets: Vec<i64> = items.iter().filter_map(update_page_offset).collect();
    assert!(offsets.len() >= 2, "expected at least two updates in range");
    assert!(
        offsets.first() >= offsets.last(),
        "descending order: first offset {:?} should be >= last {:?}",
        offsets.first(),
        offsets.last()
    );
    println!(
        "updates-page reverse — {} items, offsets {}..={}",
        offsets.len(),
        offsets.last().copied().unwrap_or_default(),
        offsets.first().copied().unwrap_or_default()
    );
}

#[tokio::test]
async fn json_submit_and_read_back() {
    let (Some(json_url), Some(oidc_config), Ok(party), Ok(pkg)) = (
        std::env::var("CANTON_TEST_JSON_ENDPOINT").ok(),
        oidc(),
        std::env::var("CANTON_TEST_PARTY"),
        std::env::var("CANTON_TEST_LICENSING_PKG"),
    ) else {
        eprintln!(
            "skipping json_submit_and_read_back: set CANTON_TEST_JSON_ENDPOINT + \
             CANTON_TEST_TOKEN_URL/CLIENT_ID/CLIENT_SECRET + CANTON_TEST_PARTY + \
             CANTON_TEST_LICENSING_PKG"
        );
        return;
    };

    let json = JsonClient::new(json_url).with_oidc(TokenProvider::new(oidc_config));

    // Submit an AppInstallRequest create over the JSON transport.
    let template_id = format!("{pkg}:Licensing.AppInstall:AppInstallRequest");
    let arguments = serde_json::json!({
        "provider": party,
        "user": party,
        "meta": { "values": {} },
    });
    let commands = JsonCommands::new(vec![party.clone()]).add_create(template_id, arguments);

    let response = json
        .submit_and_wait_for_transaction(&commands)
        .await
        .expect("json submit should commit");
    assert!(
        !response.transaction.update_id.is_empty(),
        "committed transaction should have an update id"
    );
    assert!(
        !response.transaction.events.is_empty(),
        "the create should yield at least one event"
    );
    let offset = response.transaction.offset;
    println!(
        "json submit — update_id={} offset={} events={}",
        response.transaction.update_id,
        offset,
        response.transaction.events.len()
    );

    // The ACS snapshot at that offset is a non-empty bounded read.
    let acs = json
        .active_contracts(vec![party.clone()], offset, Some(200))
        .await
        .expect("json active-contracts");
    assert!(!acs.is_empty(), "ACS snapshot should be non-empty");
    println!("json active-contracts — {} contracts", acs.len());

    // The bounded update range (offset-1, offset] must contain our create.
    let updates = json
        .updates(vec![party.clone()], offset - 1, Some(offset), Some(50))
        .await
        .expect("json updates");
    assert!(!updates.is_empty(), "expected the create update in range");
    let has_app_install = updates.iter().any(|u| {
        u.to_string()
            .contains("Licensing.AppInstall:AppInstallRequest")
    });
    assert!(
        has_app_install,
        "the created AppInstallRequest should appear in the update range"
    );
    println!(
        "json updates — {} items in (offset-1, offset]",
        updates.len()
    );
}

#[tokio::test]
async fn json_updates_too_large_is_a_413() {
    let (Some(json_url), Some(oidc_config), Ok(party)) = (
        std::env::var("CANTON_TEST_JSON_ENDPOINT").ok(),
        oidc(),
        std::env::var("CANTON_TEST_PARTY"),
    ) else {
        eprintln!("skipping json_updates_too_large_is_a_413: set JSON endpoint + token + party");
        return;
    };

    let json = JsonClient::new(json_url).with_oidc(TokenProvider::new(oidc_config));
    let end = json.ledger_end().await.expect("ledger end");

    // Unbounded (0, end] with no limit exceeds the node's list cap → 413.
    match json.updates(vec![party], 0, Some(end), None).await {
        Ok(items) => {
            // A tiny ledger might fit under the cap; then it must just succeed.
            println!(
                "json updates (unbounded) fit under the cap: {} items",
                items.len()
            );
        }
        Err(canton_ledger::Error::Http { status, .. }) => {
            assert_eq!(status, 413, "over-large result should be 413");
            println!("json updates (unbounded) — 413 as expected");
        }
        Err(other) => panic!("expected Ok or Http 413, got {other:?}"),
    }
}

#[cfg(feature = "ws")]
#[tokio::test]
async fn ws_streams_active_contracts_and_updates() {
    use tokio_stream::StreamExt as _;

    let (Some(json_url), Some(oidc_config), Ok(party)) = (
        std::env::var("CANTON_TEST_JSON_ENDPOINT").ok(),
        oidc(),
        std::env::var("CANTON_TEST_PARTY"),
    ) else {
        eprintln!(
            "skipping ws_streams_active_contracts_and_updates: set JSON endpoint + token + party"
        );
        return;
    };

    let json = JsonClient::new(json_url).with_oidc(TokenProvider::new(oidc_config));
    let end = json.ledger_end().await.expect("ledger end");

    // The ACS snapshot over WebSocket closes itself when fully delivered.
    let acs = json
        .ws_active_contracts(vec![party.clone()], end)
        .await
        .expect("ws active-contracts handshake");
    let mut acs = Box::pin(acs);
    let mut acs_count = 0usize;
    while let Some(item) = acs.next().await {
        item.expect("ws acs frame");
        acs_count += 1;
    }
    assert!(acs_count > 0, "ws ACS snapshot should yield contracts");
    println!("ws active-contracts — {acs_count} frames then close");

    // Bounded updates over WebSocket: prove frames arrive (don't drain the ledger).
    let updates = json
        .ws_updates(vec![party.clone()], 0, Some(end))
        .await
        .expect("ws updates handshake");
    let mut updates = Box::pin(updates);
    let mut got = 0usize;
    while let Some(item) = updates.next().await {
        item.expect("ws update frame");
        got += 1;
        if got >= 3 {
            break; // enough to prove streaming; dropping closes the socket
        }
    }
    assert!(got > 0, "ws updates should yield at least one frame");
    println!("ws updates — {got} frame(s)");
}

#[tokio::test]
async fn submit_fire_and_forget_then_recover() {
    let Some((client, party, pkg)) = full_setup() else {
        eprintln!("skipping submit_fire_and_forget_then_recover: full setup env not set");
        return;
    };

    let begin = client.ledger_end().await.expect("ledger_end");

    // Fire-and-forget submit returns the change-ID command_id used.
    let command_id = client
        .submit(Submit::new(&party).add_command(app_install(&party, &pkg)))
        .await
        .expect("submit should be accepted");
    assert!(
        command_id.starts_with("sdk-"),
        "expected a generated command id, got {command_id}"
    );

    // The outcome is recoverable via the completion stream.
    let completion = client
        .await_completion(
            &command_id,
            vec![party.clone()],
            begin,
            std::time::Duration::from_secs(15),
        )
        .await
        .expect("the submitted command's completion should be found");
    assert_eq!(
        completion.command_id, command_id,
        "recovered completion should match the submitted command"
    );
    println!("fire-and-forget submit — command_id={command_id} recovered");
}

#[tokio::test]
async fn submit_and_wait_returns_the_update_id() {
    let Some((client, party, pkg)) = full_setup() else {
        eprintln!("skipping submit_and_wait_returns_the_update_id: full setup env not set");
        return;
    };

    let response = client
        .submit_and_wait(
            Submit::new(&party)
                .with_workflow_id("sdk-wf-live")
                .with_deduplication_duration(std::time::Duration::from_secs(60))
                .add_command(app_install(&party, &pkg)),
        )
        .await
        .expect("submit_and_wait should commit");

    assert!(!response.update_id.is_empty(), "update id should be set");
    assert!(response.completion_offset > 0, "offset should be positive");
    println!(
        "submit-and-wait — update_id={} offset={}",
        response.update_id, response.completion_offset
    );
}
