//! Against a running PQS.
//!
//! The query compiler is checked without a database — that is what its unit
//! tests do. What only a real store can answer is whether the statements it
//! produces are ones Postgres accepts against the schema Scribe actually
//! creates, and whether a payload read out of it deserializes into the same
//! generated type the Ledger API path yields. Both are the point of the crate.
//!
//! ```sh
//! CANTON_PQS_URL='host=localhost port=5432 user=cnadmin password=… dbname=pqs-app-provider' \
//!   cargo test -p canton-pqs --test live -- --nocapture
//! ```
#![allow(clippy::unwrap_used, clippy::expect_used)]

use canton_pqs::{PqsClient, Predicate, Query};
use canton_quickstart_licensing::quickstart_licensing::Licensing_AppInstall::AppInstallRequest;

async fn client() -> Option<PqsClient> {
    let url = std::env::var("CANTON_PQS_URL").ok()?;
    // Configured but unreachable is a failure, not a skip. Treating it as a
    // skip meant a run against a broken store reported six passes.
    Some(
        PqsClient::connect(&url)
            .await
            .expect("CANTON_PQS_URL is set, so PQS must be reachable"),
    )
}

/// The store must hold the contracts these tests read, or they assert nothing.
///
/// Returned rather than skipped: "no rows" and "nothing was checked" look
/// identical in a passing run, and five of these tests used to end that way.
async fn require_sample(client: &PqsClient) -> canton_pqs::Contract<AppInstallRequest> {
    let contracts = client
        .active::<AppInstallRequest>()
        .await
        .expect("the query runs");
    contracts.into_iter().next().expect(
        "this store holds no AppInstallRequest, so these tests would assert nothing — \
         point CANTON_PQS_URL at a store with the reference app's contracts",
    )
}

/// The store says how far it has ingested. Everything else reads against it.
#[tokio::test]
async fn pqs_reports_how_far_it_has_ingested() {
    let Some(client) = client().await else {
        eprintln!("skipping: set CANTON_PQS_URL");
        return;
    };
    let offset = client.latest_offset().await.expect("an offset");
    println!("latest offset: {offset}");
    assert!(offset > 0, "a store with contracts has a positive offset");
}

/// The whole point: a contract read from Postgres is the same generated type a
/// transaction stream yields. If the payload encoding were anything but Daml
/// JSON, this would not compile away — it would fail here.
#[tokio::test]
async fn a_contract_read_from_postgres_is_the_generated_type() {
    let Some(client) = client().await else {
        eprintln!("skipping: set CANTON_PQS_URL");
        return;
    };
    let contracts = client
        .active::<AppInstallRequest>()
        .await
        .expect("the query runs");

    println!("active AppInstallRequest: {}", contracts.len());
    let contract = contracts
        .first()
        .expect("this store holds no AppInstallRequest, so nothing would be asserted");

    // The payload is typed, not a JSON blob.
    let payload: &AppInstallRequest = contract.payload();
    println!(
        "  {} provider={} user={}",
        contract.contract_id().as_str(),
        payload.provider.as_str(),
        payload.user.as_str()
    );

    assert!(!contract.contract_id().as_str().is_empty());
    assert_eq!(
        contract.archived_at_offset(),
        None,
        "read at the latest offset"
    );
    assert!(contract.created_at_offset() > 0);
    assert!(
        contract.created_effective_at().is_some(),
        "a created contract has a ledger effective time"
    );
    assert_eq!(contract.package_name(), "quickstart-licensing");
    assert!(
        contract.signatories().contains(&payload.user.to_string()),
        "the user signs an AppInstallRequest: {:?}",
        contract.signatories()
    );
}

/// A predicate on the payload reaches the database as parameters and filters
/// there — not in Rust after fetching everything.
#[tokio::test]
async fn a_payload_predicate_filters_in_the_database() {
    let Some(client) = client().await else {
        eprintln!("skipping: set CANTON_PQS_URL");
        return;
    };
    let sample = require_sample(&client).await;
    let user = sample.payload().user.to_string();

    let matching = client
        .run(&Query::<AppInstallRequest>::active().filter(Predicate::eq("user", user.clone())))
        .await
        .expect("the filtered query runs");
    assert!(
        !matching.is_empty(),
        "the contract that supplied the value must match it"
    );
    assert!(
        matching
            .iter()
            .all(|c| c.payload().user.to_string() == user)
    );

    // And a value nothing has returns nothing, rather than everything.
    let none = client
        .run(
            &Query::<AppInstallRequest>::active()
                .filter(Predicate::eq("user", "nobody::1220deadbeef")),
        )
        .await
        .expect("the query runs");
    assert!(none.is_empty(), "an unmatched filter must match nothing");
}

/// Containment is the predicate an index can serve, so it has to work against
/// the real column type.
#[tokio::test]
async fn containment_and_party_columns_work_against_the_real_schema() {
    let Some(client) = client().await else {
        eprintln!("skipping: set CANTON_PQS_URL");
        return;
    };
    let sample = require_sample(&client).await;
    let user = sample.payload().user.to_string();

    let by_containment = client
        .run(
            &Query::<AppInstallRequest>::active()
                .filter(Predicate::contains(serde_json::json!({ "user": user }))),
        )
        .await
        .expect("containment runs");
    assert!(!by_containment.is_empty());

    let by_signatory = client
        .run(&canton_pqs::active_signed_by::<AppInstallRequest>(&user))
        .await
        .expect("the signatory query runs");
    assert!(
        !by_signatory.is_empty(),
        "the user signs, so a signatory filter must find it"
    );
}

/// An ordered comparison has to be accepted by Postgres, cast and all. A
/// lexical comparison of an LF-JSON number is the bug this guards.
#[tokio::test]
async fn an_ordered_comparison_is_a_statement_postgres_accepts() {
    let Some(client) = client().await else {
        eprintln!("skipping: set CANTON_PQS_URL");
        return;
    };
    // Against a real numeric field, on contracts that exist. Pointed at a path
    // no contract has, this returned zero rows whether the comparison was
    // numeric or lexical — it could not tell the bug it names from correct
    // behaviour.
    //
    // Amulet amounts are LF-JSON strings like "504680.1600000000", which is
    // exactly where a lexical comparison goes wrong.
    let amounts = client
        .query(&canton_pqs::Sql {
            text: "SELECT payload #>> ARRAY['amount','initialAmount'] AS a \
                   FROM active('splice-amulet:Splice.Amulet:Amulet') LIMIT 1"
                .to_string(),
            params: Vec::new(),
        })
        .await
        .expect("the store is readable");
    let Some(amount) = amounts
        .first()
        .and_then(|row| row.try_get::<_, Option<String>>("a").ok().flatten())
    else {
        eprintln!("skipping: this store holds no Amulet to compare against");
        return;
    };
    let amount: f64 = amount.parse().expect("an amulet amount is a number");
    println!("comparing against an amulet amount of {amount}");

    let matching = |sql: String, threshold: String| {
        let client = client.clone();
        async move {
            let rows = client
                .query(&canton_pqs::Sql {
                    text: sql,
                    params: vec![canton_pqs::Param::Text(threshold)],
                })
                .await
                .expect("Postgres accepts the statement");
            rows[0].try_get::<_, i64>("n").expect("a count")
        }
    };

    // The threshold is a single digit larger than the amount's first: "9".
    // Numerically 504680.16 > 9; as text, "504680.16" < "9" because '5' < '9'.
    // That is the whole bug in one comparison — the reason "9" sorts after "10"
    // — so the two must disagree here or the cast is not doing anything.
    let numeric = matching(
        "SELECT count(*) AS n FROM active('splice-amulet:Splice.Amulet:Amulet') \
         WHERE (payload #>> ARRAY['amount','initialAmount'])::numeric > $1::text::numeric"
            .to_string(),
        "9".to_string(),
    )
    .await;
    let lexical = matching(
        "SELECT count(*) AS n FROM active('splice-amulet:Splice.Amulet:Amulet') \
         WHERE payload #>> ARRAY['amount','initialAmount'] > $1"
            .to_string(),
        "9".to_string(),
    )
    .await;

    assert!(numeric > 0, "{amount} compared numerically must exceed 9");
    println!("numeric matched {numeric}, lexical matched {lexical}");
    assert_ne!(
        numeric, lexical,
        "if the two agree on this store the test proves nothing about the cast — \
         pick a threshold where they differ"
    );
}

/// `lookup_contract` finds a contract by id whether or not it is still active,
/// which a filter on `active()` cannot do.
#[tokio::test]
async fn a_contract_is_found_by_id() {
    let Some(client) = client().await else {
        eprintln!("skipping: set CANTON_PQS_URL");
        return;
    };
    let sample = require_sample(&client).await;

    let found = client
        .lookup::<AppInstallRequest>(sample.contract_id().as_str())
        .await
        .expect("the lookup runs")
        .expect("the contract is there");
    assert_eq!(found.contract_id().as_str(), sample.contract_id().as_str());

    let missing = client
        .lookup::<AppInstallRequest>("00deadbeef")
        .await
        .expect("the lookup runs");
    assert!(missing.is_none(), "an unknown id is None, not an error");
}

/// Reading at an offset is what makes a paged or repeated read consistent:
/// the ACS as of a point, rather than a moving target.
#[tokio::test]
async fn the_acs_can_be_read_as_of_an_offset() {
    let Some(client) = client().await else {
        eprintln!("skipping: set CANTON_PQS_URL");
        return;
    };
    let offset = client.latest_offset().await.expect("an offset");
    let now = client
        .active::<AppInstallRequest>()
        .await
        .expect("the query runs");
    let pinned = client
        .run(&Query::<AppInstallRequest>::active_at(offset))
        .await
        .expect("the pinned query runs");

    assert_eq!(
        now.len(),
        pinned.len(),
        "the latest offset is what active() defaults to"
    );

    // An offset the store no longer holds is refused, and says so. That is
    // the right answer rather than an empty result: "nothing was active then"
    // and "I cannot tell you what was active then" are different facts, and a
    // caller paging backwards needs to know which it got.
    let err = client
        .run(&Query::<AppInstallRequest>::active_at(1))
        .await
        .expect_err("an offset before the oldest is refused");
    let message = err.to_string();
    assert!(
        message.contains("oldest known offset"),
        "the database's own explanation must survive: {message}"
    );
}
