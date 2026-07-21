//! The DAR **container** layer: a `.dar` is a JAR-style zip of `.dalf`
//! (Daml-LF-encoded) packages plus a `META-INF/MANIFEST.MF` that names the main
//! package and lists the dependency packages.
//!
//! This module reads the container and hands out the raw package bytes; turning
//! those bytes into a typed AST (the Daml-LF protobuf decode) is a later step.

use std::collections::BTreeMap;
use std::io::{Cursor, Read};
use std::path::Path;

/// An error opening or reading a DAR.
#[derive(Debug, thiserror::Error)]
pub enum DarError {
    /// The file could not be read.
    #[error("reading DAR failed: {0}")]
    Io(#[from] std::io::Error),
    /// The zip container is malformed.
    #[error("malformed DAR archive: {0}")]
    Zip(#[from] zip::result::ZipError),
    /// The manifest was missing or did not name the main package.
    #[error("invalid DAR manifest: {0}")]
    Manifest(String),
}

/// A parsed DAR: its manifest fields plus the raw bytes of each `.dalf`.
#[derive(Debug)]
pub struct Dar {
    manifest: BTreeMap<String, String>,
    dalfs: BTreeMap<String, Vec<u8>>,
}

impl Dar {
    /// Read a DAR from a file on disk.
    ///
    /// # Errors
    /// Returns [`DarError`] if the file cannot be read or is not a valid DAR.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, DarError> {
        let bytes = std::fs::read(path)?;
        Self::from_bytes(&bytes)
    }

    /// Read a DAR from an in-memory byte buffer.
    ///
    /// # Errors
    /// Returns [`DarError`] if the bytes are not a valid DAR.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, DarError> {
        let mut archive = zip::ZipArchive::new(Cursor::new(bytes))?;

        let mut manifest = BTreeMap::new();
        let mut dalfs = BTreeMap::new();
        for index in 0..archive.len() {
            let mut entry = archive.by_index(index)?;
            if !entry.is_file() {
                continue;
            }
            let name = entry.name().to_string();
            if name == "META-INF/MANIFEST.MF" {
                let mut text = String::new();
                entry.read_to_string(&mut text)?;
                manifest = parse_manifest(&text);
            } else if Path::new(&name)
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("dalf"))
            {
                let mut data = Vec::new();
                entry.read_to_end(&mut data)?;
                dalfs.insert(name, data);
            }
        }

        if manifest.is_empty() {
            return Err(DarError::Manifest("no META-INF/MANIFEST.MF".to_string()));
        }
        Ok(Self { manifest, dalfs })
    }

    /// The package name (manifest `Name`), if present.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.manifest.get("Name").map(String::as_str)
    }

    /// The Daml SDK version the DAR was built with (manifest `Sdk-Version`).
    #[must_use]
    pub fn sdk_version(&self) -> Option<&str> {
        self.manifest.get("Sdk-Version").map(String::as_str)
    }

    /// The path of the main package's `.dalf` within the archive.
    fn main_dalf_path(&self) -> Result<&str, DarError> {
        self.manifest
            .get("Main-Dalf")
            .map(String::as_str)
            .ok_or_else(|| DarError::Manifest("manifest has no Main-Dalf".to_string()))
    }

    /// The raw Daml-LF bytes of the **main** package (the DAR's own package, as
    /// opposed to its `daml-prim` / `daml-stdlib` / transitive dependencies).
    ///
    /// # Errors
    /// Returns [`DarError`] if the manifest does not name the main package or
    /// the named entry is absent.
    pub fn main_package_bytes(&self) -> Result<&[u8], DarError> {
        let path = self.main_dalf_path()?;
        self.dalfs
            .get(path)
            .map(Vec::as_slice)
            .ok_or_else(|| DarError::Manifest(format!("main dalf `{path}` not in archive")))
    }

    /// The archive paths of every package (`.dalf`) in the DAR, main and
    /// dependencies, in sorted order.
    pub fn package_paths(&self) -> impl Iterator<Item = &str> {
        self.dalfs.keys().map(String::as_str)
    }

    /// The raw Daml-LF bytes of **every** package in the DAR — the main package
    /// and its full dependency closure (`daml-prim`, `daml-stdlib`, and any
    /// other packages the DAR bundles). Decoding all of them gives the
    /// PackageMap needed to resolve cross-package references.
    pub fn package_bytes(&self) -> impl Iterator<Item = &[u8]> {
        self.dalfs.values().map(Vec::as_slice)
    }

    /// The number of packages (`.dalf` entries) in the DAR.
    #[must_use]
    pub fn package_count(&self) -> usize {
        self.dalfs.len()
    }
}

/// Parse a JAR-style manifest into key → value pairs, un-wrapping continuation
/// lines (a line beginning with a single space continues the previous one — the
/// manifest wraps values at 72 bytes).
fn parse_manifest(text: &str) -> BTreeMap<String, String> {
    let mut logical: Vec<String> = Vec::new();
    for raw in text.split('\n') {
        let line = raw.strip_suffix('\r').unwrap_or(raw);
        if let Some(continuation) = line.strip_prefix(' ')
            && let Some(last) = logical.last_mut()
        {
            last.push_str(continuation);
            continue;
        }
        logical.push(line.to_string());
    }

    logical
        .iter()
        .filter_map(|line| {
            let (key, value) = line.split_once(": ")?;
            Some((key.to_string(), value.to_string()))
        })
        .collect()
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn manifest_unwraps_continuation_lines() {
        // A `Main-Dalf` value wrapped across two lines — the continuation line
        // begins with a single space (kept as separate literals so the space is
        // not swallowed by Rust's `\`-continuation).
        let text = concat!(
            "Manifest-Version: 1.0\r\n",
            "Name: splice-wallet-payments-0.1.14\r\n",
            "Main-Dalf: pkg/main-part1\r\n",
            " part2.dalf\r\n",
            "Sdk-Version: 3.3.0\r\n",
        );
        let manifest = parse_manifest(text);
        assert_eq!(
            manifest.get("Name").unwrap(),
            "splice-wallet-payments-0.1.14"
        );
        assert_eq!(manifest.get("Sdk-Version").unwrap(), "3.3.0");
        // The wrapped Main-Dalf is stitched back together.
        assert_eq!(
            manifest.get("Main-Dalf").unwrap(),
            "pkg/main-part1part2.dalf"
        );
    }

    #[test]
    fn opening_a_real_dar_reads_the_manifest_and_main_package() {
        // Env-gated: point at a real .dar (e.g. one from cn-quickstart).
        let Ok(path) = std::env::var("CANTON_TEST_DAR") else {
            eprintln!("skipping real-DAR test: set CANTON_TEST_DAR=/path/to/x.dar");
            return;
        };

        let dar = Dar::open(&path).expect("open DAR");
        assert!(dar.name().is_some(), "DAR should have a Name");
        assert!(
            dar.sdk_version().is_some(),
            "DAR should have an Sdk-Version"
        );
        assert!(
            dar.package_count() > 1,
            "a real DAR bundles its deps (daml-prim/stdlib)"
        );
        let main = dar.main_package_bytes().expect("main package bytes");
        assert!(!main.is_empty(), "main package should have LF bytes");
        println!(
            "DAR {} (sdk {}) — {} packages, main = {} bytes",
            dar.name().unwrap(),
            dar.sdk_version().unwrap(),
            dar.package_count(),
            main.len()
        );
    }
}
