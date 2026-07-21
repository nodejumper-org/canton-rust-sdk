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

/// Decode the DAR's **main** package into the Daml-LF 2 AST, together with its
/// **package id** (the `Archive` hash — the input to SCU/PackageMap and to
/// qualifying cross-package references).
///
/// # Errors
/// Returns [`DecodeError`] if the container or protobuf bytes are malformed, or
/// the package is not Daml-LF 2.x.
pub fn decode_main_package(dar: &Dar) -> Result<(Package, String), DecodeError> {
    decode_package(dar.main_package_bytes()?)
}

/// Decode a single package's `.dalf` bytes (an `Archive`) into the LF 2 AST and
/// its package id (the archive hash).
///
/// # Errors
/// See [`decode_main_package`].
pub fn decode_package(archive_bytes: &[u8]) -> Result<(Package, String), DecodeError> {
    let archive = Archive::decode(archive_bytes)?;
    let package_id = archive.hash;
    let payload = ArchivePayload::decode(archive.payload.as_slice())?;
    match payload.sum {
        Some(archive_payload::Sum::DamlLf2(package)) => Ok((package, package_id)),
        _ => Err(DecodeError::UnsupportedVersion),
    }
}

/// Decode **every** package in a DAR, each paired with its package id (archive
/// hash) — the PackageMap for resolving cross-package references.
///
/// # Errors
/// Returns [`DecodeError`] if any package's bytes are malformed or not LF 2.x.
pub fn decode_all(dar: &Dar) -> Result<Vec<(String, Package)>, DecodeError> {
    dar.package_bytes()
        .map(|bytes| {
            let (package, id) = decode_package(bytes)?;
            Ok((id, package))
        })
        .collect()
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

/// Resolve an interned dotted name (a module or type name) to its `.`-joined
/// form, e.g. `"Licensing.AppInstall"`.
#[must_use]
pub fn interned_dotted_name(package: &Package, index: i32) -> Option<String> {
    let dotted = usize::try_from(index)
        .ok()
        .and_then(|i| package.interned_dotted_names.get(i))?;
    let segments = dotted
        .segments_interned_str
        .iter()
        .map(|&segment| interned_str(package, segment))
        .collect::<Option<Vec<_>>>()?;
    Some(segments.join("."))
}

/// Resolve an interned type by its `i32` index into a package's `interned_types`
/// table (LF 2.x stores types once and references them by index).
#[must_use]
pub fn interned_type(package: &Package, index: i32) -> Option<&crate::pb::daml_lf_2::Type> {
    usize::try_from(index)
        .ok()
        .and_then(|i| package.interned_types.get(i))
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
        let (package, package_id) = decode_main_package(&dar).expect("decode LF 2.x package");
        assert!(
            !package_id.is_empty(),
            "package id (archive hash) should be set"
        );

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
