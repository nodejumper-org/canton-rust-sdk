//! Against a running Scan.
//!
//! `tests/scan.rs` covers what the client sends. What only a real Scan
//! answers is whether the paths are served where Splice's documents say,
//! whether what comes back decodes into this crate's types, and whether the
//! contracts it publishes are ones a participant accepts as disclosed.
//!
//! ```sh
//! # a Scan, asked directly
//! CANTON_SCAN_URL=https://scan.sv-1.example/api/scan \
//!   cargo test -p canton-token --test scan_live -- --nocapture
//!
//! # a validator's scan-proxy, which needs a token
//! CANTON_SCAN_PROXY_URL=https://validator.example/api/validator CANTON_TOKEN=… \
//!   cargo test -p canton-token --test scan_live -- --nocapture
//! ```
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)]

/// A skipped test and a passing test are the same line in cargo's output. Set
/// `CANTON_TEST_REQUIRE_LIVE=1`, as any run that claims to have exercised a
/// live environment should, and a missing one fails here instead of passing
/// quietly. Same contract as the other live suites.
macro_rules! skip {
    ($($arg:tt)*) => {{
        let reason = format!($($arg)*);
        assert!(
            std::env::var("CANTON_TEST_REQUIRE_LIVE").is_err(),
            "live test skipped while CANTON_TEST_REQUIRE_LIVE is set: {reason}"
        );
        eprintln!("SKIP (no live environment): {reason}");
    }};
}

use canton_token::scan::{ScanClient, ScanUpdate, SnapshotRequest};

/// A Scan asked directly, with the internal endpoints opted into: this suite
/// is where a change to one of them should show.
fn scan() -> Option<ScanClient> {
    let url = std::env::var("CANTON_SCAN_URL").ok()?;
    Some(
        ScanClient::new(&url)
            .expect("CANTON_SCAN_URL must be a URL")
            .with_internal_endpoints(),
    )
}

/// A validator's scan-proxy, which authenticates.
fn proxy() -> Option<ScanClient> {
    let url = std::env::var("CANTON_SCAN_PROXY_URL").ok()?;
    let token = std::env::var("CANTON_TOKEN").ok()?;
    Some(
        ScanClient::through_validator(&url)
            .expect("CANTON_SCAN_PROXY_URL must be a URL")
            .with_token(token),
    )
}

fn is_a_party(party: &str) -> bool {
    party
        .split_once("::")
        .is_some_and(|(hint, fingerprint)| !hint.is_empty() && fingerprint.starts_with("1220"))
}

/// The contracts an application discloses to use Amulet: the rules and the
/// open round, as a participant takes them.
async fn the_dso_contracts_can_be_disclosed(client: &ScanClient) {
    let dso_party = client.dso_party_id().await.expect("the DSO party");
    assert!(is_a_party(&dso_party), "{dso_party}");

    let dso = client.dso().await.expect("the DSO");
    assert_eq!(dso.dso_party_id, dso_party);
    assert!(dso.voting_threshold >= 1);
    assert!(!dso.sv_node_states.is_empty(), "a DSO has Super Validators");

    let rules = client.amulet_rules().await.expect("the Amulet rules");
    assert!(
        rules
            .contract
            .template_id
            .ends_with(":Splice.AmuletRules:AmuletRules"),
        "{}",
        rules.contract.template_id
    );
    assert_eq!(
        rules.contract.contract_id,
        dso.amulet_rules.contract.contract_id
    );
    // Without the opt-in the same contract comes from the external API.
    if let Ok(url) = std::env::var("CANTON_SCAN_URL") {
        let external = ScanClient::new(&url)
            .expect("CANTON_SCAN_URL must be a URL")
            .amulet_rules()
            .await
            .expect("the Amulet rules, from the DSO the external API publishes");
        assert_eq!(external.contract.contract_id, rules.contract.contract_id);
    }
    let disclosed = rules
        .disclosed()
        .expect("the rules as a disclosed contract");
    assert_ne!(disclosed.created_event_blob, [] as [u8; 0]);
    assert!(
        is_a_party(&disclosed.synchronizer_id),
        "{}",
        disclosed.synchronizer_id
    );

    let rounds = client
        .open_and_issuing_mining_rounds()
        .await
        .expect("the mining rounds");
    assert!(!rounds.open.is_empty(), "a running network has open rounds");
    let round = client
        .latest_open_mining_round()
        .await
        .expect("a round that has opened");
    assert!(
        round
            .contract
            .template_id
            .ends_with(":Splice.Round:OpenMiningRound"),
        "{}",
        round.contract.template_id
    );
    round
        .disclosed()
        .expect("the round as a disclosed contract");
    let number = |round: &canton_token::scan::ContractWithState| {
        round.contract.payload["round"]["number"]
            .as_str()
            .and_then(|number| number.parse::<i64>().ok())
            .expect("a round number")
    };
    let highest = rounds.open.iter().map(number).max().expect("a round");
    println!(
        "DSO {dso_party}: {} open rounds up to {highest}, using {}",
        rounds.open.len(),
        number(&round)
    );
    assert!(number(&round) <= highest);

    // Someone who is not a featured application has no right, and that is an
    // answer rather than an error.
    let nobody = format!("nobody::{}", dso_party.split_once("::").expect("a party").1);
    assert!(
        client
            .featured_app_right(&nobody)
            .await
            .expect("the lookup answers")
            .is_none()
    );
    assert!(
        client
            .transfer_preapproval_by_party(&nobody)
            .await
            .expect("the lookup answers")
            .is_none()
    );
    assert!(
        client
            .ans_entry_by_party(&nobody)
            .await
            .expect("the lookup answers")
            .is_none()
    );
    let entries = client.ans_entries(None, 5).await.expect("the name service");
    println!("{} name service entries on the first page", entries.len());
}

#[tokio::test]
async fn scan_publishes_the_contracts_an_application_discloses() {
    let Some(client) = scan() else {
        skip!("set CANTON_SCAN_URL to a Scan (https://<scan>/api/scan)");
        return;
    };
    the_dso_contracts_can_be_disclosed(&client).await;
}

#[tokio::test]
async fn a_validators_scan_proxy_publishes_the_same_contracts() {
    let Some(client) = proxy() else {
        skip!("set CANTON_SCAN_PROXY_URL (https://<validator>/api/validator) and CANTON_TOKEN");
        return;
    };
    the_dso_contracts_can_be_disclosed(&client).await;
}

/// The history reads page forward, and an update read by its id is the one
/// the page had.
#[tokio::test]
async fn the_update_history_pages_forward() {
    let Some(client) = scan() else {
        skip!("set CANTON_SCAN_URL to a Scan (https://<scan>/api/scan)");
        return;
    };
    let first = client.updates(None, 5).await.expect("the first page");
    assert_eq!(
        first.len(),
        5,
        "a running network has more than five updates"
    );
    let last = first.last().expect("an update");
    let second = client
        .updates(Some(&last.position()), 5)
        .await
        .expect("the second page");
    assert!(!second.is_empty());
    for update in &second {
        assert!(
            first
                .iter()
                .all(|seen| seen.update_id() != update.update_id()),
            "the second page repeats {}",
            update.update_id()
        );
    }

    let by_id = client
        .update(last.update_id())
        .await
        .expect("the lookup answers")
        .expect("an update the history just returned");
    assert_eq!(by_id.update_id(), last.update_id());
    assert_eq!(by_id.position(), last.position());
    assert!(
        client
            .update("1220000000000000000000000000000000000000000000000000000000000000dead")
            .await
            .expect("the lookup answers")
            .is_none(),
        "an update id nothing has"
    );

    let transactions = first
        .iter()
        .filter(|update| matches!(update, ScanUpdate::Transaction(_)))
        .count();
    println!(
        "{transactions} of the first 5 updates are transactions; history starts at {:?}",
        first[0].position()
    );

    let events = client.events(None, 3).await.expect("the event history");
    assert_eq!(events.len(), 3);
    assert!(
        events
            .iter()
            .any(|event| event.update.is_some() || event.verdict.is_some())
    );
}

/// The snapshot reads: the time of the latest snapshot, a page of it, and
/// what the DSO held at it.
#[tokio::test]
async fn the_latest_snapshot_can_be_read() {
    let Some(client) = scan() else {
        skip!("set CANTON_SCAN_URL to a Scan (https://<scan>/api/scan)");
        return;
    };
    // A date after any snapshot taken so far, and the newest migration that
    // has one.
    let before = "2100-01-01T00:00:00Z";
    let mut latest = None;
    for migration_id in (0..=20).rev() {
        if let Some(record_time) = client
            .acs_snapshot_timestamp(before, migration_id)
            .await
            .expect("the lookup answers")
        {
            latest = Some((migration_id, record_time));
            break;
        }
    }
    let Some((migration_id, record_time)) = latest else {
        skip!(
            "this Scan has not taken a snapshot yet: it takes one every few hours, so a \
             minutes-old LocalNet has none"
        );
        return;
    };
    println!("latest snapshot: migration {migration_id} at {record_time}");

    // What the DSO held at it. The summary is served from Splice 0.6.3.
    let dso_party = client.dso_party_id().await.expect("the DSO party");
    let owners = SnapshotRequest::at(migration_id, record_time.clone())
        .with_page_size(5)
        .for_parties([dso_party.clone()]);
    let summary = client
        .holdings_summary(&owners)
        .await
        .expect("the DSO's totals");
    assert_eq!(summary.migration_id, migration_id);
    let later = client
        .holdings_summary(
            &SnapshotRequest::at(migration_id, before)
                .at_or_before()
                .for_parties([dso_party.clone()]),
        )
        .await
        .expect("the totals at or before a later time");
    assert_eq!(later.record_time, summary.record_time);
    println!("{dso_party}: {} summary row(s)", summary.summaries.len());

    // The page reads are Splice 0.7.5's.
    let request = SnapshotRequest::at(migration_id, record_time).with_page_size(5);
    let page = match client.acs_snapshot(&request).await {
        Err(canton_core::Error::Http { status: 404, .. }) => {
            skip!(
                "this Scan predates /v2/state/acs (Splice 0.7.5), so the snapshot pages cannot be read"
            );
            return;
        }
        page => page.expect("a page of the snapshot"),
    };
    assert_eq!(page.migration_id, migration_id);
    assert_eq!(page.created_events.len(), 5);
    let token = page
        .next_page_token
        .expect("more than five contracts are active");
    let next = client
        .acs_snapshot(&request.clone().after(token))
        .await
        .expect("the next page");
    assert_ne!(
        next.created_events[0].contract_id,
        page.created_events[0].contract_id
    );

    // "At or before" a later moment finds the same snapshot.
    let found = client
        .acs_snapshot(
            &SnapshotRequest::at(migration_id, before)
                .at_or_before()
                .with_page_size(1),
        )
        .await
        .expect("the snapshot at or before a later time");
    assert_eq!(found.record_time, page.record_time);

    let holdings = client
        .holdings_state(&owners)
        .await
        .expect("the DSO's holdings");
    println!(
        "{dso_party} held {} contracts on the first page",
        holdings.created_events.len()
    );
    assert_eq!(holdings.migration_id, migration_id);
}

/// The rest of the external reads a Scan serves.
#[tokio::test]
async fn scan_lists_its_peers_its_closed_rounds_and_a_members_traffic() {
    let Some(client) = scan() else {
        skip!("set CANTON_SCAN_URL to a Scan (https://<scan>/api/scan)");
        return;
    };
    let scans = client.scans().await.expect("the Scans");
    let all: usize = scans
        .iter()
        .map(|synchronizer| synchronizer.scans.len())
        .sum();
    assert!(all >= 1, "at least the Scan that answered");
    assert!(scans.iter().all(|s| is_a_party(&s.synchronizer_id)));
    println!("{all} Scans across {} synchronizer(s)", scans.len());

    let closed = client.closed_rounds().await.expect("the closed rounds");
    println!("{} closed rounds not yet archived", closed.len());

    // A member's traffic, for a participant the caller names. Not every
    // environment has one to name, so this half is opt-in.
    if let Ok(member) = std::env::var("CANTON_SCAN_MEMBER_ID") {
        let synchronizer = &scans[0].synchronizer_id;
        let traffic = client
            .member_traffic_status(synchronizer, &member)
            .await
            .expect("the member's traffic");
        println!(
            "traffic: consumed {} of a limit of {}, purchased {}",
            traffic.total_consumed, traffic.total_limit, traffic.total_purchased
        );
        assert!(traffic.total_consumed >= 0 && traffic.total_limit >= 0);
        assert!(traffic.total_purchased >= 0);
    }
}
