//! Guard the checked-in bindings crates against drift: regenerate each from its
//! DAR and assert it matches the committed `src/lib.rs`.
//!
//! The table of crates, DARs and external packages is shared with the
//! `regenerate_splice` example — see `examples/shared/splice_crates.rs`. The
//! external-package map is part of the output, so a guard with a map of its own
//! would check the committed files against something nobody generates.
//!
//! Each crate is gated on its DAR being available and skips otherwise, which is
//! reported as a pass. CI closes that hole by asserting the per-crate marker
//! this prints; see the `bindings-drift` job.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use canton_codegen::{generate_crate, lower_dar};
use canton_lf::Dar;

include!("../examples/shared/splice_crates.rs");

#[test]
fn every_committed_bindings_crate_is_up_to_date() {
    for (crate_name, source, externals, emits) in CRATES {
        let Some(path) = dar_path(*source) else {
            // stdout, not stderr: CI pipes this into a log and asserts that
            // nothing skipped. On stderr the assertion could never fire.
            println!("skipping {crate_name}: its DAR is not configured");
            continue;
        };
        let dar = Dar::open(&path).unwrap_or_else(|e| panic!("{path}: {e}"));
        let (krate, _) = canton_codegen::lower_dar_selecting(
            &dar,
            &selection_for(*emits),
            &externals_for(externals, *emits),
        )
        .expect("lower");
        let regenerated = generate_crate(&krate).expect("generate");

        let committed_path = format!("{}/../{crate_name}/src/lib.rs", env!("CARGO_MANIFEST_DIR"));
        let committed = std::fs::read_to_string(&committed_path)
            .unwrap_or_else(|e| panic!("{committed_path}: {e}"));

        // The committed file is formatted by the repository's rustfmt while the
        // generator emits prettyplease, so compare canonical forms: that sees
        // the generated API rather than the formatting.
        assert_eq!(
            normalize(&regenerated),
            normalize(&committed),
            "{crate_name} is stale — regenerate every bindings crate together \
             (see the header of examples/regenerate_splice.rs)"
        );
        // Printed so CI can assert the guard ran rather than skipped. Cargo's
        // own per-test output is not a contract; this marker is.
        println!("bindings agreement: {crate_name} ok");
    }
}

/// The one guard that needs nothing external, so it runs everywhere — CI
/// included. It does not cover the published crates directly, but it catches
/// the thing that breaks them: an emitter change nobody meant to make. A
/// deliberate change fails it too, which is the point — regenerating the
/// fixture is the step that reminds you to regenerate the crates.
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
         intended, regenerate it — and regenerate the bindings crates with it."
    );
    println!("emitter fixture agreement: ok");
}

/// Reduce Rust source to a canonical form (parse + re-emit).
fn normalize(source: &str) -> String {
    prettyplease::unparse(&syn::parse_file(source).expect("parse generated source"))
}
