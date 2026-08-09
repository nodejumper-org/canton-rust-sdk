//! Guard the checked-in generated bindings crates against drift: regenerate
//! each from its DAR and assert it matches the committed `src/lib.rs`. Env-gated
//! per crate on the DAR path.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use canton_codegen::{generate_crate, lower_dar};
use canton_lf::Dar;

#[test]
fn canton_splice_amulet_bindings_are_up_to_date() {
    check(
        "CANTON_SPLICE_AMULET_DAR",
        "splice-amulet-0.1.14.dar",
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../canton-splice-amulet/src/lib.rs"
        ),
    );
}

#[test]
fn canton_quickstart_licensing_bindings_are_up_to_date() {
    check(
        "CANTON_LICENSING_DAR",
        "quickstart-licensing-0.0.1.dar",
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../canton-quickstart-licensing/src/lib.rs"
        ),
    );
}

#[test]
fn canton_splice_wallet_bindings_are_up_to_date() {
    check(
        "CANTON_SPLICE_WALLET_DAR",
        "splice-wallet-0.1.14.dar",
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../canton-splice-wallet/src/lib.rs"
        ),
    );
}

#[test]
fn canton_splice_wallet_payments_bindings_are_up_to_date() {
    check(
        "CANTON_SPLICE_WALLET_PAYMENTS_DAR",
        "splice-wallet-payments-0.1.14.dar",
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../canton-splice-wallet-payments/src/lib.rs"
        ),
    );
}

/// The same guard, on the one DAR that lives in this repository.
///
/// The four above need a DAR from a Splice or cn-quickstart checkout, so on a
/// machine without one they skip — and a skipped test is reported as a passing
/// one. CI has no such checkout, which means the property they exist to
/// protect has never actually been enforced anywhere but a developer's laptop:
/// three of those four crates are published, and nothing said the emitter still
/// produces what was published.
///
/// This one needs nothing external and therefore runs everywhere, including
/// CI. It does not cover the published crates directly, but it catches the
/// thing that breaks them: an emitter change nobody meant to make. A deliberate
/// change fails it too, which is the point — regenerating the fixture is the
/// step that reminds you to regenerate the crates.
///
/// Regenerate with:
/// `cargo run -p canton-codegen-cli -- --dar testdata/splice-api-token-holding-v1-1.0.0.dar
///  --out /tmp/b --runtime-path crates/canton-daml`
/// then copy `/tmp/b/src/lib.rs` over the fixture.
#[test]
fn the_emitter_still_produces_the_committed_fixture() {
    let dar = Dar::open(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../testdata/splice-api-token-holding-v1-1.0.0.dar"
    ))
    .expect("the fixture DAR is committed to this repository");
    let (krate, _errors) = lower_dar(&dar).expect("lower DAR");
    let regenerated = generate_crate(&krate).expect("generate crate");
    let committed = include_str!("fixtures/holding_v1.rs");

    assert_eq!(
        normalize(&regenerated),
        normalize(committed),
        "the emitter no longer produces the committed fixture. If that was \
         intended, regenerate it — and regenerate the four bindings crates with \
         it, because they are published and this is the only guard that runs in CI."
    );
    // Printed so CI can assert the test ran rather than skipped, the way the
    // conformance-oracle job already does.
    println!("emitter fixture agreement: ok");
}

/// Regenerate the bindings from the DAR named by `dar_env` and compare — at the
/// AST level, so formatting is ignored — against the committed `src/lib.rs`.
fn check(dar_env: &str, dar_name: &str, committed_path: &str) {
    let Ok(dar_path) = std::env::var(dar_env) else {
        eprintln!("skipping: set {dar_env}=/path/to/{dar_name}");
        return;
    };

    let dar = Dar::open(&dar_path).expect("open DAR");
    let (krate, _errors) = lower_dar(&dar).expect("lower DAR");
    let regenerated = generate_crate(&krate).expect("generate crate");
    let committed = std::fs::read_to_string(committed_path).expect("read committed lib.rs");

    // The committed file is formatted by the repo's rustfmt while the generator
    // emits prettyplease, so compare canonical (parsed + re-emitted) forms — that
    // sees the generated API, not formatting.
    assert_eq!(
        normalize(&regenerated),
        normalize(&committed),
        "{committed_path} is stale — regenerate it from the DAR (see the crate README)"
    );
}

/// Reduce Rust source to a canonical form (parse + re-emit).
fn normalize(source: &str) -> String {
    prettyplease::unparse(&syn::parse_file(source).expect("parse generated source"))
}
