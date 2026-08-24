//! A package published as its own crate is referenced, not copied.
//!
//! A DAR's dependency closure is shared: `splice-api-token-holding-v1` sits
//! under amulet, wallet and wallet-payments alike. Generating it into each
//! crate gives each crate its own `Holding`, and Rust treats those as unrelated
//! types — so a `ContractId<Holding>` read through one crate does not typecheck
//! against the other, though both name the same interface in the same package.
//! That was demonstrably true of the three crates this repository publishes:
//! a program depending on two of them failed with "expected `Holding`, found a
//! different `Holding`".
//!
//! These tests use the DAR committed to this repository, so they run
//! everywhere, including CI.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use canton_codegen::{ExternalPackages, lower_dar, lower_dar_with};
use canton_lf::Dar;

fn fixture() -> Dar {
    Dar::open(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../testdata/splice-api-token-holding-v1-1.0.0.dar"
    ))
    .expect("the fixture DAR is committed to this repository")
}

/// The package named as external disappears from the output, and every
/// reference to it becomes an absolute path into the other crate.
#[test]
fn an_external_package_is_referenced_instead_of_emitted() {
    let dar = fixture();

    let (plain, _) = lower_dar(&dar).expect("lower");
    let external = ExternalPackages::new().with(
        "splice-api-token-metadata-v1",
        "canton_splice_api_token_metadata_v1",
    );
    let (with_external, _) = lower_dar_with(&dar, &external).expect("lower");

    let names = |krate: &canton_codegen::ir::Crate| -> Vec<String> {
        krate.packages.iter().map(|p| p.name.clone()).collect()
    };
    let before = names(&plain);
    let after = names(&with_external);

    assert!(
        before.iter().any(|n| n.contains("metadata")),
        "the fixture must contain the package this test externalises: {before:?}"
    );
    assert!(
        !after.iter().any(|n| n.contains("metadata")),
        "an external package must not be emitted: {after:?}"
    );
    assert_eq!(
        after.len(),
        before.len() - 1,
        "exactly one package should have gone"
    );

    // And what is left points at the other crate, absolutely — a relative path
    // would be shadowed by a local module of the same name.
    let source = canton_codegen::generate_crate(&with_external).expect("generate");
    assert!(
        source.contains("::canton_splice_api_token_metadata_v1::"),
        "references must resolve into the external crate"
    );
}

/// The path into the other crate keeps the **package** segment.
///
/// The other crate wraps its packages in a module exactly as this one does, so
/// the path there is `::<crate>::<package>::<module>::<Type>`. Emitting three
/// segments instead of four compiles here and fails in the consumer with
/// "could not find `<Module>` in `<crate>`" — which is what happened the first
/// time this was written.
#[test]
fn the_external_path_carries_the_package_segment() {
    let external = ExternalPackages::new().with(
        "splice-api-token-metadata-v1",
        "canton_splice_api_token_metadata_v1",
    );
    let (krate, _) = lower_dar_with(&fixture(), &external).expect("lower");
    let source = canton_codegen::generate_crate(&krate).expect("generate");

    let referenced = source
        .lines()
        .find(|line| line.contains("::canton_splice_api_token_metadata_v1::"))
        .expect("at least one reference");
    assert!(
        referenced
            .contains("::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::"),
        "the package segment must survive: {}",
        referenced.trim()
    );
}

/// A package may be named by its **name** or by its id hash. The name is the
/// one to prefer — an id is the hash of one build, so it stops matching the
/// moment the dependency is rebuilt, while the name survives a version bump.
#[test]
fn a_package_can_be_named_by_id_as_well_as_by_name() {
    let dar = fixture();
    let by_name = ExternalPackages::new().with(
        "splice-api-token-metadata-v1",
        "canton_splice_api_token_metadata_v1",
    );
    let (from_name, _) = lower_dar_with(&dar, &by_name).expect("lower");

    // The id of that same package, read out of the DAR.
    let id = canton_lf::decode_all(&dar)
        .expect("decode")
        .into_iter()
        .find(|(_, package)| {
            canton_lf::package_name(package) == Some("splice-api-token-metadata-v1")
        })
        .map(|(id, _)| id)
        .expect("the fixture contains that package");

    let by_id = ExternalPackages::new().with(id, "canton_splice_api_token_metadata_v1");
    let (from_id, _) = lower_dar_with(&dar, &by_id).expect("lower");

    assert_eq!(
        from_name.packages.len(),
        from_id.packages.len(),
        "both keys must resolve to the same package"
    );
}

/// Naming nothing changes nothing — the default path stays exactly as it was.
#[test]
fn no_externals_generates_what_it_always_did() {
    let dar = fixture();
    let (plain, _) = lower_dar(&dar).expect("lower");
    let (empty, _) = lower_dar_with(&dar, &ExternalPackages::new()).expect("lower");
    assert_eq!(
        canton_codegen::generate_crate(&plain).expect("generate"),
        canton_codegen::generate_crate(&empty).expect("generate")
    );
}
