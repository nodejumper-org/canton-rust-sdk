//! Decode a package's Daml-LF bytes into the LF 2 AST ([`Package`]), and resolve
//! its interned package metadata — the name and version that drive SCU.
//!
//! LF 2.x stores every name and literal in the package's `interned_strings` /
//! `interned_dotted_names` tables and references them by `i32` index, so the raw
//! prost AST is index-heavy; [`interned_str`] resolves those indices.

use prost::Message;

use crate::dar::{Dar, DarError};
use crate::pb::daml_lf_2::Package;
use crate::pb::daml_lf_dev::{Archive, ArchivePayload, HashFunction, archive_payload};
use sha2::{Digest as _, Sha256};

/// An error decoding Daml-LF package bytes.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
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
    /// The archive's payload does not hash to the package id it declares.
    /// Refused: that id is the package's identity — it is what generated
    /// bindings embed as `PACKAGE_ID` and what cross-package references resolve
    /// through — so accepting a mismatch means addressing a package by a name
    /// its contents do not answer to.
    #[error("archive payload does not match its declared package id {declared}")]
    PackageIdMismatch {
        /// The id the archive claims.
        declared: String,
        /// The SHA-256 of the payload actually present.
        computed: String,
    },
    /// The archive declares a hash function this build does not implement.
    #[error("unsupported archive hash function {0} (only SHA-256 is defined)")]
    UnsupportedHashFunction(i32),
    /// The package is LF 2.x, but a **minor** this decoder was not built
    /// against. Refused rather than decoded: prost drops fields it does not
    /// know, so a newer minor would otherwise yield silently incomplete
    /// bindings — a template missing a field decodes, compiles, and then fails
    /// on the wire.
    #[error(
        "Daml-LF 2.{minor} is newer than this SDK supports (2.{supported}); \
         the protos are pinned to a Canton release, so use a DAR built for it \
         or upgrade the SDK"
    )]
    UnsupportedMinor {
        /// The minor version the archive declares.
        minor: String,
        /// The minors this build accepts, comma-separated.
        supported: String,
    },
    /// A decode failure attributed to a specific package in the DAR.
    #[error("package `{name}`: {source}")]
    InPackage {
        /// The archive path of the `.dalf` that failed to decode.
        name: String,
        /// The underlying failure.
        source: Box<DecodeError>,
    },
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
    let package_id = verified_package_id(&archive)?;
    let payload = ArchivePayload::decode(archive.payload.as_slice())?;
    check_minor(&payload.minor)?;
    match payload.sum {
        Some(archive_payload::Sum::DamlLf2(package)) => Ok((package, package_id)),
        _ => Err(DecodeError::UnsupportedVersion),
    }
}

/// The package id of an archive, checked against its contents rather than
/// taken on trust.
///
/// The id is the SHA-256 of the payload, and it is load-bearing: generated
/// bindings embed it as `Contract::PACKAGE_ID`, and every cross-package
/// reference in the DAR resolves through it. A DAR whose declared id does not
/// match its bytes — corrupted in transit, or assembled by hand — would
/// otherwise produce bindings that name a package by an id its contents do not
/// answer to, and nothing downstream would notice.
fn verified_package_id(archive: &Archive) -> Result<String, DecodeError> {
    // The enum has exactly one member today; anything else is a scheme this
    // build cannot check, and an unchecked hash is the thing being fixed here.
    if archive.hash_function != HashFunction::Sha256 as i32 {
        return Err(DecodeError::UnsupportedHashFunction(archive.hash_function));
    }

    let computed = hex(&Sha256::digest(&archive.payload));
    if computed == archive.hash {
        Ok(computed)
    } else {
        Err(DecodeError::PackageIdMismatch {
            declared: archive.hash.clone(),
            computed,
        })
    }
}

/// Lower-case hex, the spelling Daml-LF uses for a package id.
fn hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .fold(String::with_capacity(bytes.len() * 2), |mut out, byte| {
            use std::fmt::Write as _;
            // Infallible: writing to a String cannot fail.
            let _ = write!(out, "{byte:02x}");
            out
        })
}

/// The Daml-LF 2.x minor versions this build decodes.
///
/// The list is evidence, not aspiration: a survey of every DAR available here —
/// 18 of them, 648 packages across the Splice amulet/wallet/token-standard and
/// quickstart-licensing sets — finds 617 packages at `2.1` and 31 at `2.2`, and
/// both spellings appear inside DARs this repo generates bindings from. Both
/// decode correctly against the vendored schema, which the conformance oracle
/// checks against the official JVM `daml-lf-archive` reader.
///
/// Adding a minor is a deliberate act with a procedure: re-vendor the protos if
/// the new minor changes them, then re-run the oracle. Guessing instead means
/// prost silently drops fields belonging to a schema it does not know, and the
/// bindings come out quietly incomplete.
const SUPPORTED_LF2_MINORS: &[&str] = &["1", "2"];

/// Refuse a minor we were not built against.
///
/// `"dev"` is refused with everything else on purpose — it is the unstable
/// spelling, and the whole point of this gate is that an unknown schema must
/// not decode silently.
fn check_minor(minor: &str) -> Result<(), DecodeError> {
    if SUPPORTED_LF2_MINORS.contains(&minor) {
        return Ok(());
    }
    Err(DecodeError::UnsupportedMinor {
        minor: minor.to_string(),
        supported: SUPPORTED_LF2_MINORS.join(", "),
    })
}

/// Decode **every** package in a DAR, each paired with its package id (archive
/// hash) — the PackageMap for resolving cross-package references.
///
/// # Errors
/// Returns [`DecodeError`] if any package's bytes are malformed or not LF 2.x.
pub fn decode_all(dar: &Dar) -> Result<Vec<(String, Package)>, DecodeError> {
    dar.package_entries()
        .map(|(name, bytes)| {
            let (package, id) = decode_package(bytes).map_err(|error| DecodeError::InPackage {
                name: name.to_string(),
                source: Box::new(error),
            })?;
            Ok((id, package))
        })
        .collect()
}

/// Resolve a `package_import_id` — an index into the package's explicit import
/// table (`package_imports.imported_packages`) — to the target package's id
/// hash. Newer LF (2.dev / SDK 3.5+) references imported packages this way
/// instead of by an interned id string.
#[must_use]
pub fn imported_package_id(package: &Package, index: i32) -> Option<&str> {
    let imports = package.package_imports.as_ref()?;
    let i = usize::try_from(index).ok()?;
    imports.imported_packages.get(i).map(String::as_str)
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
mod hash_tests {
    use super::*;

    /// An archive whose declared id is the real SHA-256 of its payload.
    fn honest_archive() -> Archive {
        let payload = ArchivePayload {
            minor: "1".to_string(),
            sum: Some(archive_payload::Sum::DamlLf2(Package::default())),
        }
        .encode_to_vec();
        let hash = hex(&Sha256::digest(&payload));
        Archive {
            hash_function: HashFunction::Sha256 as i32,
            payload,
            hash,
        }
    }

    #[test]
    fn an_archive_that_hashes_to_its_id_decodes_and_returns_that_id() {
        let archive = honest_archive();
        let expected = archive.hash.clone();
        let (_package, id) = decode_package(&archive.encode_to_vec()).expect("honest archive");
        assert_eq!(id, expected);
        assert_eq!(id.len(), 64, "a SHA-256 in hex");
    }

    /// The id is the package's identity: generated bindings embed it as
    /// `PACKAGE_ID` and cross-package references resolve through it. Taking a
    /// mismatched one on trust means addressing a package by a name its
    /// contents do not answer to.
    #[test]
    fn a_payload_that_does_not_match_its_declared_id_is_refused() {
        let mut archive = honest_archive();
        archive.hash = "0".repeat(64);
        let err = decode_package(&archive.encode_to_vec()).expect_err("must not decode");
        assert!(
            matches!(err, DecodeError::PackageIdMismatch { .. }),
            "{err}"
        );
        // Both sides belong in the message: without them a reader cannot tell a
        // corrupted download from a hand-assembled DAR.
        let message = err.to_string();
        assert!(message.contains(&"0".repeat(64)), "{message}");

        // The same when the payload is what changed, which is the realistic
        // shape of the failure.
        let mut tampered = honest_archive();
        tampered.payload.extend_from_slice(b"extra");
        assert!(matches!(
            decode_package(&tampered.encode_to_vec()),
            Err(DecodeError::PackageIdMismatch { .. })
        ));
    }

    #[test]
    fn an_unknown_hash_function_is_refused_rather_than_assumed_to_be_sha256() {
        let mut archive = honest_archive();
        archive.hash_function = 7;
        let err = decode_package(&archive.encode_to_vec()).expect_err("must not decode");
        assert!(
            matches!(err, DecodeError::UnsupportedHashFunction(7)),
            "{err}"
        );
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod minor_tests {
    use super::*;

    /// Build a **well-formed** `Archive` declaring `minor`: the id is the real
    /// SHA-256 of the payload, so these tests exercise the minor gate rather
    /// than tripping the hash gate that runs before it.
    fn archive_with_minor(minor: &str) -> Vec<u8> {
        let payload = ArchivePayload {
            minor: minor.to_string(),
            sum: Some(archive_payload::Sum::DamlLf2(Package::default())),
        }
        .encode_to_vec();
        let hash = hex(&Sha256::digest(&payload));
        Archive {
            hash_function: HashFunction::Sha256 as i32,
            payload,
            hash,
        }
        .encode_to_vec()
    }

    #[test]
    fn the_minor_this_build_targets_decodes() {
        for minor in SUPPORTED_LF2_MINORS {
            let bytes = archive_with_minor(minor);
            assert!(
                decode_package(&bytes).is_ok(),
                "minor {minor} should decode"
            );
        }
    }

    /// A newer minor must fail loudly. prost silently drops fields from a
    /// schema it does not know, so decoding one would hand the user bindings
    /// that compile and are missing template fields — the failure would surface
    /// on the wire, far from its cause.
    #[test]
    fn an_unknown_minor_is_refused_rather_than_silently_decoded() {
        for minor in ["3", "17", "dev", ""] {
            let bytes = archive_with_minor(minor);
            let err = decode_package(&bytes).expect_err("minor {minor} must be refused");
            let message = err.to_string();
            assert!(
                matches!(err, DecodeError::UnsupportedMinor { .. }),
                "minor {minor}: {message}"
            );
            // The message has to name what was found and what is accepted,
            // or the reader cannot tell whether to rebuild the DAR or the SDK.
            assert!(message.contains(&format!("2.{minor}")), "{message}");
            assert!(
                message.contains("2.1"),
                "should name what we support: {message}"
            );
        }
    }
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
