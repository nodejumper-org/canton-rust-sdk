//! Live integration tests against a running Canton node.
//!
//! Gated on env vars so `cargo test` stays green without a node. Party-admin
//! RPCs need a token with the `ParticipantAdmin` right (LocalNet's
//! `app-provider-validator`); user self-inspect works with any token; topology
//! read hits the admin API port (unauthenticated on LocalNet).
//!
//! ```sh
//! CANTON_TEST_ENDPOINT=http://localhost:3901 \
//! CANTON_TEST_ADMIN_ENDPOINT=http://localhost:3902 \
//! CANTON_TEST_TOKEN_URL=http://keycloak.localhost:8082/realms/AppProvider/protocol/openid-connect/token \
//! CANTON_TEST_CLIENT_ID=app-provider-backend CANTON_TEST_CLIENT_SECRET=… \
//! CANTON_TEST_ADMIN_CLIENT_ID=app-provider-validator CANTON_TEST_ADMIN_CLIENT_SECRET=… \
//!   cargo test -p canton-admin --test live -- --nocapture
//! ```
#![allow(clippy::unwrap_used, clippy::expect_used)]

use canton_admin::{AdminClient, Config, Store, TopologyClient};
use canton_auth::{OidcConfig, TokenProvider};

fn endpoint() -> Option<String> {
    std::env::var("CANTON_TEST_ENDPOINT").ok()
}

fn admin_endpoint() -> Option<String> {
    std::env::var("CANTON_TEST_ADMIN_ENDPOINT").ok()
}

/// The standard (non-admin) OIDC token — enough for user self-inspect.
fn oidc() -> Option<OidcConfig> {
    Some(OidcConfig::new(
        std::env::var("CANTON_TEST_TOKEN_URL").ok()?,
        std::env::var("CANTON_TEST_CLIENT_ID").ok()?,
        std::env::var("CANTON_TEST_CLIENT_SECRET").ok()?,
    ))
}

/// A token carrying the `ParticipantAdmin` right, for party-management RPCs.
fn admin_oidc() -> Option<OidcConfig> {
    Some(OidcConfig::new(
        std::env::var("CANTON_TEST_TOKEN_URL").ok()?,
        std::env::var("CANTON_TEST_ADMIN_CLIENT_ID").ok()?,
        std::env::var("CANTON_TEST_ADMIN_CLIENT_SECRET").ok()?,
    ))
}

fn admin_client(config: OidcConfig) -> Option<AdminClient> {
    AdminClient::connect_lazy(Config::new(endpoint()?).with_oidc(TokenProvider::new(config))).ok()
}

#[tokio::test]
async fn user_self_inspect_reports_the_authenticated_user() {
    let (Some(oidc_config), Some(client)) = (oidc(), oidc().and_then(admin_client)) else {
        eprintln!(
            "skipping user_self_inspect: set CANTON_TEST_ENDPOINT + \
             CANTON_TEST_TOKEN_URL/CLIENT_ID/CLIENT_SECRET"
        );
        return;
    };
    let _ = oidc_config;

    let user = client.current_user().await.expect("current_user");
    assert!(!user.id.is_empty(), "authenticated user should have an id");

    let rights = client.current_user_rights().await.expect("rights");
    let acting = client.acting_parties().await.expect("acting parties");
    println!(
        "self-inspect — user={} rights={} acting_parties={:?}",
        user.id,
        rights.len(),
        acting
    );
}

#[tokio::test]
async fn participant_id_is_returned() {
    let Some(client) = oidc().and_then(admin_client) else {
        eprintln!("skipping participant_id_is_returned: set CANTON_TEST_ENDPOINT + token env");
        return;
    };

    let id = client.participant_id().await.expect("participant_id");
    assert!(id.starts_with("participant"), "got {id}");
    println!("participant id: {id}");
}

#[tokio::test]
async fn party_admin_allocate_list_and_get() {
    let Some(client) = admin_oidc().and_then(admin_client) else {
        eprintln!(
            "skipping party_admin_allocate_list_and_get: set CANTON_TEST_ENDPOINT + \
             CANTON_TEST_ADMIN_CLIENT_ID/CANTON_TEST_ADMIN_CLIENT_SECRET (ParticipantAdmin)"
        );
        return;
    };

    // Allocate a fresh party (unique hint — allocations are irreversible).
    let hint = format!("sdk-admin-test-{}", uuid_like());
    let allocated = client
        .allocate_party(Some(&hint))
        .await
        .expect("allocate_party");
    assert!(
        allocated.party.starts_with(&hint),
        "got {}",
        allocated.party
    );
    assert!(allocated.is_local, "allocated party should be local");
    println!("allocated party: {}", allocated.party);

    // It shows up in the full listing…
    let all = client
        .list_known_parties()
        .await
        .expect("list_known_parties");
    assert!(
        all.iter().any(|p| p.party == allocated.party),
        "allocated party should be listed among {} parties",
        all.len()
    );

    // …and paging returns a token when the page is smaller than the set.
    let (page, next) = client
        .list_known_parties_page(1, None)
        .await
        .expect("list_known_parties_page");
    assert_eq!(page.len(), 1, "page_size=1 yields one party");
    assert!(
        next.is_some(),
        "more parties remain, so a token is returned"
    );

    // get_parties round-trips the allocated party and drops unknown ones.
    let got = client
        .get_parties(vec![allocated.party.clone(), "unknown::0".to_string()])
        .await
        .expect("get_parties");
    assert!(got.iter().any(|p| p.party == allocated.party));
    assert!(
        !got.iter().any(|p| p.party == "unknown::0"),
        "unknown parties are silently dropped"
    );
    println!("list={} page_next={:?} got={}", all.len(), next, got.len());
}

#[tokio::test]
async fn topology_reads_return_mappings() {
    let Some(admin_ep) = admin_endpoint() else {
        eprintln!(
            "skipping topology_reads_return_mappings: set CANTON_TEST_ADMIN_ENDPOINT (:3902)"
        );
        return;
    };

    // LocalNet's admin API is unauthenticated.
    let client = TopologyClient::connect_lazy(Config::new(admin_ep)).expect("valid config");

    let p2p = client
        .list_party_to_participant(Store::Authorized, "", "")
        .await
        .expect("list_party_to_participant");
    assert!(
        !p2p.is_empty(),
        "a provisioned participant should have party→participant mappings"
    );

    let delegations = client
        .list_namespace_delegations(Store::Authorized, "", "")
        .await
        .expect("list_namespace_delegations");
    assert!(
        !delegations.is_empty(),
        "the participant should have at least its own namespace delegation"
    );

    // Vetted packages live on the synchronizer store; against the authorized
    // store the RPC still succeeds (typically empty).
    let vetted = client
        .list_vetted_packages(Store::Authorized, "")
        .await
        .expect("list_vetted_packages");

    println!(
        "topology — party→participant={} namespace_delegations={} vetted(authorized)={}",
        p2p.len(),
        delegations.len(),
        vetted.len()
    );

    // If a synchronizer physical id is provided, assert vetted packages there.
    if let Ok(sync_id) = std::env::var("CANTON_TEST_SYNC_ID") {
        let vetted_sync = client
            .list_vetted_packages(Store::Synchronizer(sync_id), "")
            .await
            .expect("list_vetted_packages(synchronizer)");
        assert!(
            !vetted_sync.is_empty(),
            "the synchronizer store should list vetted packages"
        );
        println!("topology — vetted(synchronizer)={}", vetted_sync.len());
    }
}

/// A short unique-ish suffix without pulling in the `uuid` crate as a dep.
fn uuid_like() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |d| d.as_nanos());
    format!("{nanos:x}")
}

#[tokio::test]
async fn packages_read_lists_and_reports_status() {
    use canton_admin::PackageStatus;

    let Some(client) = oidc().and_then(admin_client) else {
        eprintln!("skipping packages_read_lists_and_reports_status: set endpoint + token env");
        return;
    };

    let packages = client.list_packages().await.expect("list_packages");
    assert!(
        !packages.is_empty(),
        "a provisioned participant should know at least one package"
    );

    // A known package reports Registered; the licensing package when provided.
    let probe = std::env::var("CANTON_TEST_LICENSING_PKG")
        .ok()
        .filter(|pkg| packages.contains(pkg))
        .unwrap_or_else(|| packages[0].clone());
    let status = client
        .get_package_status(&probe)
        .await
        .expect("get_package_status");
    assert_eq!(
        status,
        PackageStatus::Registered,
        "known package is registered"
    );

    // An unknown package id is not an error — it reports back as not registered.
    let unknown = client
        .get_package_status("0000000000000000000000000000000000000000000000000000000000000000")
        .await
        .expect("get_package_status(unknown)");
    assert_ne!(unknown, PackageStatus::Registered);
    println!(
        "packages — {} known, probe {} registered, unknown → {:?}",
        packages.len(),
        &probe[..12.min(probe.len())],
        unknown
    );
}
