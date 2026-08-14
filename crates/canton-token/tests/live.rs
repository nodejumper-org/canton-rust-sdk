//! Against a running token-standard registry.
//!
//! `tests/inprocess.rs` covers what the client *sends*, which is the half a
//! reply-only fixture misses. What only a real registry answers is whether the
//! paths exist where the specification says, whether the shapes it returns
//! decode into the generated types, and whether the fields the standard marks
//! optional are actually optional in the wild. A stub agrees with whatever it
//! was written to agree with.
//!
//! A LocalNet's registry is the **scan**, which the super-validator runs — a
//! deployment with `SV_PROFILE=on` has one, on port 5012 by default, though
//! cn-quickstart does not publish that port to the host.
//!
//! ```sh
//! CANTON_TOKEN_REGISTRY_URL=http://localhost:5012 \
//!   cargo test -p canton-token --test live -- --nocapture
//! ```
#![allow(clippy::unwrap_used, clippy::expect_used)]

use canton_token::RegistryClient;

fn client() -> Option<RegistryClient> {
    let url = std::env::var("CANTON_TOKEN_REGISTRY_URL").ok()?;
    // Configured but unreachable is a failure, not a skip — the same rule the
    // PQS and interactive suites follow. A run against a dead registry that
    // reported passes would be worse than no suite at all.
    Some(RegistryClient::new(&url).expect("CANTON_TOKEN_REGISTRY_URL must be a URL"))
}

/// The registry names the party that administers its instruments. Every factory
/// call is exercised against this party, so getting it wrong fails everything
/// downstream — and it is the first thing a client reads.
#[tokio::test]
async fn the_registry_says_who_administers_its_instruments() {
    let Some(client) = client() else { return };

    let info = client
        .info()
        .await
        .expect("the registry answers /registry/metadata/v1/info");

    // A party id, not a display name: `<hint>::<fingerprint>`.
    let (hint, fingerprint) = info
        .admin_id
        .split_once("::")
        .expect("an admin id is a party id");
    assert!(
        !hint.is_empty(),
        "the admin id has no hint: {}",
        info.admin_id
    );
    assert!(
        fingerprint.starts_with("1220") && fingerprint.len() > 8,
        "the fingerprint is not a Canton multihash: {fingerprint}"
    );

    // And it parses as one, which is what the workflow will do with it.
    canton_daml::Party::parse(&info.admin_id).expect("the admin id parses as a party");

    assert!(
        info.supported_apis
            .contains_key("splice-api-token-metadata-v1"),
        "a registry serving this API must say so: {:?}",
        info.supported_apis
    );
    println!("admin: {}  apis: {:?}", info.admin_id, info.supported_apis);
}

/// The instruments decode, and the fields the standard marks optional really do
/// arrive absent — which is where a stub cannot be trusted, because it returns
/// whatever it was written to return.
#[tokio::test]
async fn the_instruments_decode_as_the_standard_describes_them() {
    let Some(client) = client() else { return };

    let (instruments, _) = client
        .list_instruments(None, None)
        .await
        .expect("the registry lists instruments");
    let instrument = instruments.into_iter().next().expect(
        "this registry issues nothing, so these tests would assert nothing — point \
         CANTON_TOKEN_REGISTRY_URL at a registry with at least one instrument",
    );

    assert!(!instrument.id.is_empty(), "an instrument needs an id");
    assert!(!instrument.name.is_empty(), "and a name");
    assert!(!instrument.symbol.is_empty(), "and a symbol");
    // The standard defaults this to 10 and a registry may omit it. Whether it
    // is sent or defaulted, a caller displaying money needs a real number.
    assert!(
        (0..=38).contains(&instrument.decimals),
        "decimals outside anything a Numeric can carry: {}",
        instrument.decimals
    );
    println!(
        "instrument: {} ({}) {} decimals, apis: {:?}",
        instrument.name, instrument.symbol, instrument.decimals, instrument.supported_apis
    );

    // Fetching it by id gives the same instrument back.
    let same = client
        .instrument(&instrument.id)
        .await
        .expect("the registry answers for an instrument it just listed")
        .expect("and it is issued here");
    assert_eq!(same.id, instrument.id);
    assert_eq!(same.decimals, instrument.decimals);
}

/// An id the registry does not issue is an answer — `None` — not an error.
///
/// This is the branch that used to swallow a misconfigured base URL: any 404
/// became `Ok(None)`, so a URL one path component off answered a polling client
/// with a steady, quiet "not issued" while every other call failed loudly. A
/// real registry is the only place to check that a *genuine* not-found still
/// reads as `None` now that a non-JSON 404 is an error.
#[tokio::test]
async fn an_instrument_this_registry_does_not_issue_is_none_rather_than_an_error() {
    let Some(client) = client() else { return };

    let missing = client
        .instrument("NotAnInstrumentThisRegistryIssues")
        .await
        .expect("a genuine not-found is an answer, not a failure");
    assert!(missing.is_none(), "the registry claims to issue it");
}

/// The one that matters for the milestone: which token-standard APIs this
/// registry's instruments actually implement.
///
/// The proposal's verification clause asks for a V2 `Account`-based flow
/// "against the V2 reference token", and this is where a run learns whether the
/// network it is pointed at has one. Printed rather than asserted for V2,
/// because a registry serving only V1 is a legitimate deployment — what is
/// asserted is that the question can be answered at all.
#[tokio::test]
async fn the_registry_declares_which_token_standard_versions_it_serves() {
    let Some(client) = client() else { return };

    let (instruments, _) = client
        .list_instruments(None, None)
        .await
        .expect("the registry lists instruments");
    let instrument = instruments
        .into_iter()
        .next()
        .expect("this registry issues nothing");

    let v1 = instrument
        .supported_apis
        .contains_key("splice-api-token-transfer-instruction-v1");
    let v2 = instrument
        .supported_apis
        .contains_key("splice-api-token-transfer-instruction-v2");
    println!("{}: transfer-instruction v1={v1} v2={v2}", instrument.id);

    assert!(
        v1 || v2,
        "an instrument implementing neither transfer-instruction API cannot be transferred \
         through the standard at all: {:?}",
        instrument.supported_apis
    );
}

/// Page tokens round-trip. A page token is an instrument id by the standard's
/// own definition, so it can contain anything an id can — and formatting one
/// into the query string split it on `&`. The hermetic test proves the
/// encoding; this proves the registry accepts what the encoding produces.
#[tokio::test]
async fn paging_through_instruments_works_against_a_real_registry() {
    let Some(client) = client() else { return };

    let (first, next) = client
        .list_instruments(Some(1), None)
        .await
        .expect("a page of one");
    assert!(
        first.len() <= 1,
        "page size was not honoured: {}",
        first.len()
    );

    if let Some(token) = next {
        let (second, _) = client
            .list_instruments(Some(1), Some(&token))
            .await
            .expect("the registry accepts the token it just issued");
        assert!(
            second.first().map(|i| &i.id) != first.first().map(|i| &i.id),
            "the second page repeated the first"
        );
    } else {
        println!("one page of instruments; no token to follow");
    }
}
