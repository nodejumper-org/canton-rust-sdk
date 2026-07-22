//! Guard the checked-in `canton-splice-amulet` bindings against drift: regenerate
//! `src/lib.rs` from the DAR and assert it byte-matches the committed file.
//! Env-gated on `CANTON_SPLICE_AMULET_DAR` (path to `splice-amulet-0.1.14.dar`).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::PathBuf;

use canton_codegen::{generate_crate, lower_dar};
use canton_lf::Dar;

#[test]
fn canton_splice_amulet_bindings_are_up_to_date() {
    let Ok(dar_path) = std::env::var("CANTON_SPLICE_AMULET_DAR") else {
        eprintln!("skipping: set CANTON_SPLICE_AMULET_DAR=/path/to/splice-amulet-0.1.14.dar");
        return;
    };

    let dar = Dar::open(&dar_path).expect("open DAR");
    let (krate, _errors) = lower_dar(&dar).expect("lower DAR");
    let regenerated = generate_crate(&krate).expect("generate crate");

    let committed_path = PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../canton-splice-amulet/src/lib.rs"
    ));
    let committed = std::fs::read_to_string(&committed_path).expect("read committed lib.rs");

    // Compare at the AST level: the committed file is formatted by the repo's
    // rustfmt while the generator emits prettyplease, so a byte comparison would
    // flag formatting, not drift. Normalising both through `syn` + `prettyplease`
    // erases formatting and compares the actual generated bindings.
    assert_eq!(
        normalize(&regenerated),
        normalize(&committed),
        "canton-splice-amulet/src/lib.rs is stale — regenerate it from the DAR \
         (see crates/canton-splice-amulet/README.md)"
    );
}

/// Reduce Rust source to a canonical form (parse + re-emit), so the comparison
/// ignores formatting and sees only the generated API.
fn normalize(source: &str) -> String {
    prettyplease::unparse(&syn::parse_file(source).expect("parse generated source"))
}
