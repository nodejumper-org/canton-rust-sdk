//! Decode a package's Daml-LF bytes into the LF 2 AST ([`Package`]), and resolve
//! its interned package metadata — the name and version that drive SCU.
//!
//! LF 2.x stores every name and literal in the package's `interned_strings` /
//! `interned_dotted_names` tables and references them by `i32` index, so the raw
//! prost AST is index-heavy; [`interned_str`] resolves those indices.

use prost::Message;

use crate::dar::{Dar, DarError};
use crate::pb::daml_lf_2::Package;
use crate::pb::daml_lf_dev::{Archive, ArchivePayload, archive_payload};

/// An error decoding Daml-LF package bytes.
#[derive(Debug, thiserror::Error)]
pub enum DecodeError {
    /// Reading the DAR container failed.
    #[error(transparent)]
    Dar(#[from] DarError),
    /// The protobuf bytes were malformed.
    #[error("malformed Daml-LF archive: {0}")]
    Proto(#[from] prost::DecodeError),
    /// The package is not Daml-LF 2.x (this decoder only supports LF 2.x).
    #[error("unsupported Daml-LF version (only LF 2.x is supported)")]
    UnsupportedVersion,
}

/// Decode the DAR's **main** package into the Daml-LF 2 AST.
///
/// # Errors
/// Returns [`DecodeError`] if the container or protobuf bytes are malformed, or
/// the package is not Daml-LF 2.x.
pub fn decode_main_package(dar: &Dar) -> Result<Package, DecodeError> {
    decode_package(dar.main_package_bytes()?)
}

/// Decode a single package's `.dalf` bytes (an `Archive`) into the LF 2 AST.
///
/// # Errors
/// See [`decode_main_package`].
pub fn decode_package(archive_bytes: &[u8]) -> Result<Package, DecodeError> {
    let archive = Archive::decode(archive_bytes)?;
    let payload = ArchivePayload::decode(archive.payload.as_slice())?;
    match payload.sum {
        Some(archive_payload::Sum::DamlLf2(package)) => Ok(package),
        _ => Err(DecodeError::UnsupportedVersion),
    }
}

/// Resolve an interned string by its `i32` index into a package's
/// `interned_strings` table.
#[must_use]
pub fn interned_str(package: &Package, index: i32) -> Option<&str> {
    usize::try_from(index)
        .ok()
        .and_then(|i| package.interned_strings.get(i))
        .map(String::as_str)
}

/// The package name from its metadata (`PackageMetadata.name_interned_str`).
#[must_use]
pub fn package_name(package: &Package) -> Option<&str> {
    let index = package.metadata.as_ref()?.name_interned_str;
    interned_str(package, index)
}

/// The package version from its metadata (`PackageMetadata.version_interned_str`).
#[must_use]
pub fn package_version(package: &Package) -> Option<&str> {
    let index = package.metadata.as_ref()?.version_interned_str;
    interned_str(package, index)
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn decodes_a_real_dar_main_package() {
        let Ok(path) = std::env::var("CANTON_TEST_DAR") else {
            eprintln!("skipping LF-decode test: set CANTON_TEST_DAR=/path/to/x.dar");
            return;
        };

        let dar = Dar::open(&path).expect("open DAR");
        let package = decode_main_package(&dar).expect("decode LF 2.x package");

        assert!(!package.modules.is_empty(), "package should have modules");
        assert!(
            !package.interned_strings.is_empty(),
            "LF 2.x packages intern their strings"
        );
        // The SCU inputs — name + version — resolve from the interning table.
        let name = package_name(&package);
        let version = package_version(&package);
        assert!(name.is_some(), "package metadata should carry a name");

        println!(
            "decoded {} v{} — {} modules, {} interned strings, {} interned types",
            name.unwrap_or("?"),
            version.unwrap_or("?"),
            package.modules.len(),
            package.interned_strings.len(),
            package.interned_types.len(),
        );
    }
}
