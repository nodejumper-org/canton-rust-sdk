//! Always-on decode tests over the checked-in fixture DAR — the container and
//! LF-decode layers run on a real archive in every `cargo test`, complementing
//! the env-gated corpus suites and the JVM conformance oracle.

#![allow(clippy::unwrap_used, clippy::expect_used)]

use canton_lf::{Dar, decode_all, decode_main_package, package_name, package_version};

const FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../testdata/splice-api-token-holding-v1-1.0.0.dar"
);

#[test]
fn fixture_dar_container_and_manifest_read() {
    let dar = Dar::open(FIXTURE).expect("open the fixture DAR");
    assert_eq!(dar.name(), Some("splice-api-token-holding-v1-1.0.0"));
    assert!(dar.sdk_version().is_some());
    assert!(
        dar.package_count() >= 20,
        "the DAR bundles its dependency closure, got {}",
        dar.package_count()
    );
}

#[test]
fn fixture_dar_decodes_fully_with_metadata_and_ids() {
    let dar = Dar::open(FIXTURE).expect("open the fixture DAR");

    let (main, main_id) = decode_main_package(&dar).expect("decode the main package");
    assert_eq!(package_name(&main), Some("splice-api-token-holding-v1"));
    assert_eq!(package_version(&main), Some("1.0.0"));
    assert_eq!(main_id.len(), 64, "package id is a sha256 hex hash");
    assert!(main_id.bytes().all(|b| b.is_ascii_hexdigit()));

    // The whole closure decodes; every package resolves its own metadata.
    let packages = decode_all(&dar).expect("decode the whole closure");
    assert_eq!(packages.len(), dar.package_count());
    for (id, package) in &packages {
        assert_eq!(id.len(), 64, "every package id is a hash");
        assert!(
            package_name(package).is_some(),
            "package {id} resolves a name from its interning tables"
        );
    }
}
