//! A crate can be generated from part of a DAR.
//!
//! `daml-stdlib`, `daml-prim` and `ghc-stdlib` ship no DAR of their own — they
//! arrive only as somebody else's dependency — so publishing them as a crate
//! means selecting them out of a DAR that happens to carry them. Without that,
//! every generated crate keeps its own copy and the copies are unrelated types,
//! which is the defect external packages exist to remove, one level down.
//!
//! These tests use the DAR committed to this repository, so they run
//! everywhere, including CI.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use canton_codegen::{
    ExternalPackages, Selection, generate_crate, lower_dar_selecting, lower_dar_with,
};
use canton_lf::Dar;

fn fixture() -> Dar {
    Dar::open(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../testdata/splice-api-token-holding-v1-1.0.0.dar"
    ))
    .expect("the fixture DAR is committed to this repository")
}

fn names(krate: &canton_codegen::ir::Crate) -> Vec<String> {
    krate.packages.iter().map(|p| p.name.clone()).collect()
}

/// Selecting nothing in particular is the behaviour that already existed.
#[test]
fn selecting_everything_generates_what_it_always_did() {
    let dar = fixture();
    let external = ExternalPackages::new();
    let (whole, _) = lower_dar_with(&dar, &external).expect("lower");
    let (selected, _) =
        lower_dar_selecting(&dar, &Selection::everything(), &external).expect("lower");
    assert_eq!(
        generate_crate(&whole).expect("generate"),
        generate_crate(&selected).expect("generate")
    );
}

/// Only the named packages are emitted — and naming one package does not drag
/// its dependencies in behind it.
#[test]
fn a_selection_emits_exactly_what_it_names() {
    let dar = fixture();
    let (whole, _) = lower_dar_with(&dar, &ExternalPackages::new()).expect("lower");
    assert!(
        names(&whole).len() > 2,
        "the fixture must carry a closure worth selecting from: {:?}",
        names(&whole)
    );

    // The standard library arrives as ~30 packages named `daml-stdlib`,
    // `daml-stdlib-DA-Time-Types`, `daml-prim-GHC-Types`, … — the family, not
    // two packages.
    let selection = Selection::only(std::iter::empty::<String>()).and_prefixed([
        "daml-stdlib",
        "daml-prim",
        "ghc-stdlib",
    ]);
    let (part, _) = lower_dar_selecting(&dar, &selection, &ExternalPackages::new()).expect("lower");

    assert!(!names(&part).is_empty(), "the selection matched nothing");
    for name in names(&part) {
        assert!(
            name.starts_with("daml_stdlib")
                || name.starts_with("daml_prim")
                || name.starts_with("ghc_stdlib"),
            "emitted a package outside the selection: {name}"
        );
    }
    assert!(
        !names(&part).iter().any(|n| n.contains("splice")),
        "the main package is not in the selection and must not be emitted: {:?}",
        names(&part)
    );
}

/// A reference that leaves the selection is reported, not written out.
///
/// The path would be `crate::<package>::…` for a package this crate does not
/// emit — it compiles here and fails in the consumer, which is exactly the
/// failure mode the external-package work was introduced to end. So the type
/// carrying it is skipped and the reason says which package is missing.
#[test]
fn a_reference_out_of_the_selection_is_reported_rather_than_emitted() {
    let dar = fixture();
    // The main package alone: everything it depends on is now outside.
    let selection = Selection::only(["splice-api-token-holding-v1"]);
    let (krate, skipped) =
        lower_dar_selecting(&dar, &selection, &ExternalPackages::new()).expect("lower");

    assert_eq!(names(&krate).len(), 1, "one package: {:?}", names(&krate));
    assert!(
        skipped
            .iter()
            .any(|s| s.reason().contains("neither generates nor references")),
        "a dependency left outside the selection must be reported: {:?}",
        skipped
            .iter()
            .map(canton_codegen::SkippedType::reason)
            .collect::<Vec<_>>()
    );

    // And nothing that escaped was written out anyway.
    let source = generate_crate(&krate).expect("generate");
    for package in names(&krate) {
        // Every `crate::<package>::` path must name a package this crate has.
        for line in source.lines().filter(|l| l.contains("crate::")) {
            if let Some(rest) = line.split("crate::").nth(1)
                && let Some(segment) = rest.split("::").next()
            {
                let segment = segment.trim_end_matches(|c: char| !c.is_alphanumeric() && c != '_');
                assert!(
                    segment.is_empty() || segment == package,
                    "path into a package this crate does not emit: {}",
                    line.trim()
                );
            }
        }
    }
}

/// Selection and external packages compose: what is selected is emitted, what
/// is external is referenced, and a package can be neither.
#[test]
fn a_selection_composes_with_external_packages() {
    let dar = fixture();
    // The real configuration: the main package here, the standard library and
    // the metadata package each in a crate of its own.
    let selection = Selection::only(["splice-api-token-holding-v1"]);
    let external = ExternalPackages::new()
        .with(
            "splice-api-token-metadata-v1",
            "canton_splice_api_token_metadata_v1",
        )
        .with_prefixed("daml-stdlib", "canton_daml_stdlib")
        .with_prefixed("daml-prim", "canton_daml_stdlib")
        .with_prefixed("ghc-stdlib", "canton_daml_stdlib");
    let (krate, _) = lower_dar_selecting(&dar, &selection, &external).expect("lower");
    let source = generate_crate(&krate).expect("generate");

    assert_eq!(names(&krate).len(), 1, "one package: {:?}", names(&krate));
    assert!(
        source.contains("::canton_splice_api_token_metadata_v1::"),
        "the external package must still be referenced"
    );
    assert!(
        source.contains("::canton_daml_stdlib::"),
        "a family named by prefix must be referenced too"
    );
    assert_eq!(
        external.crate_names(),
        vec![
            "canton_daml_stdlib".to_string(),
            "canton_splice_api_token_metadata_v1".to_string()
        ],
        "a manifest must depend on the prefixed crate exactly once"
    );
}

/// A prefix matches the family, not everything that merely starts with the
/// letters. `daml-prim` must not swallow a `daml-primary`.
#[test]
fn a_prefix_stops_at_the_package_boundary() {
    let dar = fixture();
    let (whole, _) = lower_dar_with(&dar, &ExternalPackages::new()).expect("lower");
    let (narrow, _) = lower_dar_selecting(
        &dar,
        &Selection::only(std::iter::empty::<String>()).and_prefixed(["daml-pri"]),
        &ExternalPackages::new(),
    )
    .expect("lower");
    assert!(
        narrow.packages.is_empty(),
        "`daml-pri` is nobody's package name and must match nothing, \
         but matched: {:?}",
        names(&narrow)
    );
    assert!(
        names(&whole).iter().any(|n| n.starts_with("daml_prim")),
        "the fixture must carry the packages this would have wrongly matched"
    );
}
