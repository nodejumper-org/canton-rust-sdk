//! Interactive submission against a running Canton node.
//!
//! The whole point of the flow is that the participant never holds the key, so
//! there is nothing to prove in-process: a hermetic test can check that the
//! right bytes are assembled, but only a node can say whether the signature it
//! is handed actually authorizes the transaction. That is what these do.
//!
//! Gated on env vars, so `cargo test` stays green without a node. Allocating an
//! external party needs a token with the `ParticipantAdmin` right (LocalNet's
//! `app-provider-validator`); everything after that acts as the external party
//! itself.
//!
//! ```sh
//! CANTON_TEST_ENDPOINT=http://localhost:3901 \
//! CANTON_TEST_TOKEN_URL=http://keycloak.localhost:8082/realms/AppProvider/protocol/openid-connect/token \
//! CANTON_TEST_ADMIN_CLIENT_ID=app-provider-validator CANTON_TEST_ADMIN_CLIENT_SECRET=… \
//! CANTON_TEST_CLIENT_ID=app-provider-backend CANTON_TEST_CLIENT_SECRET=… \
//!   cargo test -p canton-ledger --test interactive_live -- --nocapture
//! ```
//!
//! Test names carry the Ledger Client Standard prefixes the conformance suite
//! collects: `signing__*` and `external_commands__*`.
#![allow(clippy::unwrap_used, clippy::expect_used)]
// `signing__…` / `external_commands__…`: the double underscore separates the
// Ledger Client Standard capability from the description, so a conformance
// harness can split on it. A single one would be ambiguous — every name here
// has underscores in its description too.
#![allow(non_snake_case)]

use canton_admin::AdminClient;
use canton_auth::{OidcConfig, TokenProvider};
use canton_ledger::{CantonClient, Config, Prepare, identifier, record, value};
// `Signer` is in scope for `fingerprint()`, which the trait provides.
use canton_signer::{Ed25519Key, Ed25519Signer, Signer as _};

fn endpoint() -> Option<String> {
    std::env::var("CANTON_TEST_ENDPOINT")
        .ok()
        .or_else(|| canton_core::localnet::grpc_endpoint(None))
}

/// The admin credentials — allocating a party needs `ParticipantAdmin`.
///
/// Deliberately separate from the ordinary ones: a LocalNet's exported token
/// carries no promise of that right, and a test that runs and fails with
/// `PermissionDenied` is worse than one that says it is skipping.
fn admin_oidc() -> Option<OidcConfig> {
    Some(OidcConfig::new(
        std::env::var("CANTON_TEST_TOKEN_URL").ok()?,
        std::env::var("CANTON_TEST_ADMIN_CLIENT_ID").ok()?,
        std::env::var("CANTON_TEST_ADMIN_CLIENT_SECRET").ok()?,
    ))
}

fn ledger_oidc() -> Option<OidcConfig> {
    Some(OidcConfig::new(
        std::env::var("CANTON_TEST_TOKEN_URL").ok()?,
        std::env::var("CANTON_TEST_CLIENT_ID").ok()?,
        std::env::var("CANTON_TEST_CLIENT_SECRET").ok()?,
    ))
}

fn admin_client() -> Option<AdminClient> {
    AdminClient::connect_lazy(
        canton_admin::Config::new(endpoint()?).with_oidc(TokenProvider::new(admin_oidc()?)),
    )
    .ok()
}

fn ledger_client() -> Option<CantonClient> {
    let config = Config::new(endpoint()?);
    let config = match ledger_oidc() {
        Some(oidc) => config.with_oidc(TokenProvider::new(oidc)),
        None => config.with_token(canton_core::localnet::token(None)?),
    };
    CantonClient::connect_lazy(config).ok()
}

/// The user id the ledger token authenticates as, asked of the participant
/// rather than guessed.
///
/// The new party is given `act_as` rights for this user, or nothing could
/// submit on its behalf. It is **not** the OIDC client id: Canton derives the
/// ledger user from the token's `sub`, which in a client-credentials flow is a
/// service-account uuid. Guessing the client id gets `USER_NOT_FOUND`.
async fn ledger_user_id(ledger_admin: &AdminClient) -> String {
    ledger_admin
        .current_user()
        .await
        .expect("the token authenticates as some ledger user")
        .id
}

/// Allocate a fresh external party, holding its key in this process.
///
/// This is the two-step onboarding in full: generate the topology from a public
/// key, sign the multi-hash with the private one, submit. The fingerprint the
/// participant computes in step one is what makes the key a `Signer` — it
/// cannot be known before asking.
async fn external_party(hint: &str) -> Option<(String, Ed25519Signer)> {
    let admin = admin_client()?;
    let ledger = ledger_client()?;
    // Self-inspect under the *ledger* token: the rights are granted to the user
    // that will submit, not to the admin that allocates.
    let as_ledger_user = AdminClient::connect_lazy(match ledger_oidc() {
        Some(oidc) => canton_admin::Config::new(endpoint()?).with_oidc(TokenProvider::new(oidc)),
        None => {
            canton_admin::Config::new(endpoint()?).with_token(canton_core::localnet::token(None)?)
        }
    })
    .ok()?;
    let user_id = ledger_user_id(&as_ledger_user).await;

    // Any party the participant already hosts will do to ask which
    // synchronizers it is connected to.
    let synchronizers = ledger.connected_synchronizers("").await.ok()?;
    let synchronizer = synchronizers.first()?.synchronizer_id.clone();

    let key = Ed25519Key::generate().expect("a random source");
    let topology = admin
        .generate_external_party_topology(&synchronizer, hint, &key.public_key())
        .await
        .expect("the participant describes the onboarding");

    assert!(
        topology.party_id().starts_with(hint),
        "the party id is built from the hint: {}",
        topology.party_id()
    );
    assert!(
        !topology.transactions().is_empty(),
        "onboarding needs at least one topology transaction"
    );

    let signer = key.into_signer(topology.public_key_fingerprint());
    let party = admin
        .allocate_external_party(&synchronizer, &topology, &signer, &user_id)
        .await
        .expect("the signed onboarding is accepted");

    assert_eq!(
        party,
        topology.party_id(),
        "the allocated party must be the one that was described"
    );
    Some((party, signer))
}

/// The participant computes a fingerprint the caller cannot, and a signature
/// over the multi-hash is what proves the party controls the key. If either
/// half were wrong the allocation would be refused — so getting a party id back
/// *is* the assertion.
#[tokio::test]
async fn signing__an_external_party_is_onboarded_by_signing_its_own_topology() {
    let Some((party, signer)) = external_party("rust-sdk-onboard").await else {
        eprintln!("skipping: set CANTON_TEST_ENDPOINT and the admin credentials");
        return;
    };
    println!("allocated external party {party}");
    assert!(
        party.contains(signer.fingerprint()),
        "the party's namespace is its key's fingerprint: {party} / {}",
        signer.fingerprint()
    );
}

/// The round trip the whole crate exists for: prepare, sign off the
/// participant, execute, and read the transaction back.
#[tokio::test]
async fn external_commands__prepare_sign_execute_commits_a_transaction() {
    let Some(ledger) = ledger_client() else {
        eprintln!("skipping: set CANTON_TEST_ENDPOINT");
        return;
    };
    let Some((party, signer)) = external_party("rust-sdk-submit").await else {
        eprintln!("skipping: set the admin credentials");
        return;
    };
    let Some(package_id) = std::env::var("CANTON_TEST_LICENSING_PACKAGE_ID").ok() else {
        eprintln!("skipping: set CANTON_TEST_LICENSING_PACKAGE_ID");
        return;
    };

    // `AppInstallRequest` has a single signatory — the user — so the external
    // party can create it on its own authority.
    let command = canton_ledger::create(
        identifier(&package_id, "Licensing.AppInstall", "AppInstallRequest"),
        record(vec![
            ("provider", value::party(&party)),
            ("user", value::party(&party)),
            (
                "meta",
                value::record(record(vec![("values", value::empty_text_map())])),
            ),
        ]),
    );

    let prepared = ledger
        .prepare_submission(Prepare::new(&party).add_command(command))
        .await
        .expect("the participant interprets the command");

    assert!(
        !prepared.hash().is_empty(),
        "there must be something to sign"
    );
    assert_eq!(prepared.act_as(), [party.as_str()].map(String::from));

    let executable = prepared
        .sign_with(&signer)
        .await
        .expect("the in-memory key signs");
    assert!(
        executable.unsigned_parties().is_empty(),
        "the only acting party has signed"
    );

    // Boxed: the response carries a whole transaction, so the future is large
    // enough that clippy flags holding it on the stack.
    let transaction = Box::pin(ledger.execute_submission_and_wait_for_transaction(executable))
        .await
        .expect("the participant accepts the signature and commits");

    println!(
        "committed {} at offset {} with {} event(s)",
        transaction.update_id,
        transaction.offset,
        transaction.events.len()
    );
    assert!(!transaction.update_id.is_empty());
    assert!(
        !transaction.events.is_empty(),
        "creating a contract produces an event"
    );
}

/// A signature by the wrong key must be refused. Without this the round-trip
/// test above proves only that *something* was accepted — not that the
/// signature is what carried it.
#[tokio::test]
async fn signing__a_signature_from_another_key_is_rejected() {
    let Some(ledger) = ledger_client() else {
        eprintln!("skipping: set CANTON_TEST_ENDPOINT");
        return;
    };
    let Some((party, real)) = external_party("rust-sdk-badsig").await else {
        eprintln!("skipping: set the admin credentials");
        return;
    };
    let Some(package_id) = std::env::var("CANTON_TEST_LICENSING_PACKAGE_ID").ok() else {
        eprintln!("skipping: set CANTON_TEST_LICENSING_PACKAGE_ID");
        return;
    };

    let command = canton_ledger::create(
        identifier(&package_id, "Licensing.AppInstall", "AppInstallRequest"),
        record(vec![
            ("provider", value::party(&party)),
            ("user", value::party(&party)),
            (
                "meta",
                value::record(record(vec![("values", value::empty_text_map())])),
            ),
        ]),
    );
    let prepared = ledger
        .prepare_submission(Prepare::new(&party).add_command(command))
        .await
        .expect("prepare");

    // A different key, presented under the real party's fingerprint: the
    // signature is well-formed and claims the right signer, and only actually
    // verifying it catches that it is not.
    let impostor = Ed25519Key::generate()
        .expect("a random source")
        .into_signer(real.fingerprint());
    let executable = prepared.sign_with(&impostor).await.expect("signing itself");

    let err = ledger
        .execute_submission_and_wait(executable)
        .await
        .expect_err("a signature from the wrong key must not authorize anything");
    println!("rejected as expected: {err}");
}
