//! The vendored schemas are still the ones that were vendored.
//!
//! These files are third-party: Canton's Ledger API, its admin API, gRPC's
//! health service. A local edit to one of them is the kind of change that
//! compiles, passes every test, and quietly makes this client speak a wire
//! format the participant does not — so what needs guarding is not that they
//! came from upstream (a checksum cannot say that) but that nobody has touched
//! them since.
//!
//! `crates/canton-proto/proto/PROVENANCE.md` records where they come from and
//! how to re-hash after a deliberate re-vendor.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use sha2::{Digest as _, Sha256};

fn proto_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("proto")
}

fn sha256(path: &Path) -> String {
    let bytes = std::fs::read(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()));
    let mut hasher = Sha256::new();
    hasher.update(&bytes);
    format!("{:x}", hasher.finalize())
}

/// Every `.proto` under the vendored tree, relative to it.
fn vendored_files(dir: &Path, root: &Path, into: &mut Vec<PathBuf>) {
    for entry in std::fs::read_dir(dir).expect("read the vendored proto tree") {
        let path = entry.expect("a directory entry").path();
        if path.is_dir() {
            vendored_files(&path, root, into);
        } else if path.extension().is_some_and(|ext| ext == "proto") {
            into.push(
                path.strip_prefix(root)
                    .expect("a path under the tree")
                    .to_path_buf(),
            );
        }
    }
}

#[test]
fn the_vendored_protos_match_their_recorded_checksums() {
    let root = proto_dir();
    let sums = std::fs::read_to_string(root.join("SHA256SUMS")).expect("read SHA256SUMS");
    let recorded: BTreeMap<&str, &str> = sums
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (hash, file) = line
                .split_once("  ")
                .unwrap_or_else(|| panic!("malformed SHA256SUMS line: {line}"));
            (file, hash)
        })
        .collect();

    let mut present = Vec::new();
    vendored_files(&root, &root, &mut present);
    present.sort();

    // Same set of files, so a vendored schema cannot be added or removed
    // without the record following it.
    let present_names: Vec<String> = present
        .iter()
        .map(|p| p.to_string_lossy().replace('\\', "/"))
        .collect();
    let recorded_names: Vec<String> = recorded.keys().map(|k| (*k).to_string()).collect();
    assert_eq!(
        present_names, recorded_names,
        "SHA256SUMS does not list the same files as the tree; \
         re-run tools/vendor-protos.sh --rehash after a deliberate re-vendor"
    );

    // These hashes are over the bytes on disk, which makes them a hostage to
    // line-ending conversion: git on Windows defaults to `core.autocrlf=true`
    // and would hand this test CRLF copies of files vendored with LF, failing
    // every one of them while the tree is untouched. `.gitattributes` marks the
    // vendored tree `-text` to prevent that, and this test is the reason it is
    // there.
    for (name, path) in present_names.iter().zip(present.iter()) {
        let expected = recorded[name.as_str()];
        let actual = sha256(&root.join(path));
        assert_eq!(
            actual, expected,
            "{name} has been modified since it was vendored"
        );
    }
}
