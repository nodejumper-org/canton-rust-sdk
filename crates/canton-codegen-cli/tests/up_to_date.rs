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
