//! The DAR **container** layer: a `.dar` is a JAR-style zip of `.dalf`
//! (Daml-LF-encoded) packages plus a `META-INF/MANIFEST.MF` that names the main
//! package and lists the dependency packages.
//!
//! This module reads the container and hands out the raw package bytes; turning
//! those bytes into a typed AST (the Daml-LF protobuf decode) is a later step.

use std::collections::BTreeMap;
use std::io::{Cursor, Read};
use std::path::Path;

/// The most a single archive entry may decompress to (256 MiB). Real `.dalf`
/// payloads are a few MB; the cap stops a hostile "zip bomb" DAR from
/// ballooning a small file into gigabytes of memory. (The official JVM reader
/// applies the same kind of per-entry threshold.)
const MAX_ENTRY_BYTES: u64 = 256 * 1024 * 1024;

/// The most an **entire** DAR may decompress to (2 GiB).
///
/// A per-entry cap alone bounds nothing: every `.dalf` is read into memory, so
/// an archive of a thousand entries each just under [`MAX_ENTRY_BYTES`] passes
/// every individual check and still asks for hundreds of gigabytes — and
/// because zeros DEFLATE at roughly 1000:1, the file on disk stays small enough
/// to arrive over a network without notice.
///
/// The ceiling is far above anything real: the largest available corpus is
/// 41 MiB of package payload across 18 DARs, so this is ~50× the whole of it in
/// a single archive.
const MAX_TOTAL_BYTES: u64 = 2 * 1024 * 1024 * 1024;

/// An error opening or reading a DAR.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum DarError {
    /// The file could not be read.
    #[error("reading DAR failed: {0}")]
    Io(#[from] std::io::Error),
    /// The zip container is malformed.
    #[error("malformed DAR archive: {0}")]
    Zip(#[from] zip::result::ZipError),
    /// The bytes are not a DAR at all (not a zip archive).
    #[error("not a DAR ({0}) — expected a .dar built with `daml build`")]
    NotADar(String),
    /// The manifest was missing or did not name the main package.
    #[error("invalid DAR manifest: {0}")]
    Manifest(String),
    /// An archive entry exceeds the decompression cap.
    #[error("archive entry `{name}` exceeds the {MAX_ENTRY_BYTES}-byte limit")]
    EntryTooLarge {
        /// The offending entry's path within the archive.
        name: String,
    },
    /// The archive's entries decompress to more, in total, than the
    /// archive-wide limit — no single one of them need be oversized.
    #[error(
        "DAR decompresses to more than the {MAX_TOTAL_BYTES}-byte limit in total \
         (reached at entry `{name}`)"
    )]
    ArchiveTooLarge {
        /// The entry the budget ran out on. Not the culprit on its own — the
        /// ones before it spent the budget — but the place to start looking.
        name: String,
    },
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
        Self::read(bytes, MAX_TOTAL_BYTES)
    }

    /// The reader, with the archive-wide budget as a parameter so a test can
    /// exercise the exhaustion path on kilobytes instead of gigabytes. Every
    /// caller outside the tests passes [`MAX_TOTAL_BYTES`].
    fn read(bytes: &[u8], total_budget: u64) -> Result<Self, DarError> {
        let mut archive = zip::ZipArchive::new(Cursor::new(bytes))
            .map_err(|error| DarError::NotADar(error.to_string()))?;

        let mut manifest = BTreeMap::new();
        let mut saw_manifest = false;
        let mut dalfs = BTreeMap::new();
        // What is left of the archive-wide budget. Every entry read is capped
        // by whichever of this and the per-entry ceiling is smaller, so a read
        // can never allocate past the total even once.
        let mut remaining = total_budget;

        for index in 0..archive.len() {
            let entry = archive.by_index(index)?;
            if !entry.is_file() {
                continue;
            }
            let name = entry.name().to_string();
            let is_manifest = name == "META-INF/MANIFEST.MF";
            let is_dalf = Path::new(&name)
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("dalf"));
            if !is_manifest && !is_dalf {
                continue;
            }

            let cap = MAX_ENTRY_BYTES.min(remaining);
            let data = read_capped(entry, cap).map_err(|why| match why {
                TooLarge::Io(error) => DarError::Io(error),
                // Which limit bit is decided here, where both are known: a cap
                // the budget shrank means the archive as a whole is too big,
                // and blaming this entry for it would be a lie.
                TooLarge::Refused if cap < MAX_ENTRY_BYTES => {
                    DarError::ArchiveTooLarge { name: name.clone() }
                }
                TooLarge::Refused => DarError::EntryTooLarge { name: name.clone() },
            })?;
            remaining -= data.len() as u64;

            if is_manifest {
                saw_manifest = true;
                let text = String::from_utf8(data).map_err(|_| {
                    DarError::Manifest("META-INF/MANIFEST.MF is not valid UTF-8".to_string())
                })?;
                manifest = parse_manifest(&text);
            } else {
                dalfs.insert(name, data);
            }
        }

        if manifest.is_empty() {
            return Err(DarError::Manifest(if saw_manifest {
                "META-INF/MANIFEST.MF carries no `Key: value` entries".to_string()
            } else {
                "no META-INF/MANIFEST.MF".to_string()
            }));
        }
        if dalfs.is_empty() {
            return Err(DarError::Manifest(
                "archive contains no .dalf packages".to_string(),
            ));
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

    /// Like [`Self::package_bytes`], each package paired with its archive path
    /// — so a decode failure can name the offending `.dalf`.
    pub fn package_entries(&self) -> impl Iterator<Item = (&str, &[u8])> {
        self.dalfs
            .iter()
            .map(|(name, bytes)| (name.as_str(), bytes.as_slice()))
    }

    /// The number of packages (`.dalf` entries) in the DAR.
    #[must_use]
    pub fn package_count(&self) -> usize {
        self.dalfs.len()
    }
}

/// Read a zip entry fully, refusing to decompress more than `cap` bytes.
///
/// `cap` is checked against both the declared and the actual size: a hostile
/// header can under-declare, so the declared size being small proves nothing.
/// The caller decides what `cap` is — the per-entry ceiling, or whatever is
/// left of the archive's budget when that is smaller — which is what keeps a
/// single read from over-allocating past the total.
///
/// `Err(TooLarge)` carries nothing: only the caller knows which of the two
/// limits was the binding one, and saying the wrong one sends the reader to
/// look at an entry that is not the problem.
fn read_capped(mut entry: zip::read::ZipFile<'_>, cap: u64) -> Result<Vec<u8>, TooLarge> {
    if entry.size() > cap {
        return Err(TooLarge::Refused);
    }
    let mut data = Vec::new();
    // `take` one byte past the cap: hitting it means the entry lied about its
    // size and is over the limit.
    Read::take(&mut entry, cap + 1)
        .read_to_end(&mut data)
        .map_err(TooLarge::Io)?;
    if data.len() as u64 > cap {
        return Err(TooLarge::Refused);
    }
    Ok(data)
}

/// Why [`read_capped`] stopped: over the cap, or the read itself failed.
enum TooLarge {
    Refused,
    Io(std::io::Error),
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

    /// Build an in-memory zip from `(name, bytes)` entries.
    fn zip_of(entries: &[(&str, &[u8])]) -> Vec<u8> {
        use std::io::Write as _;
        let mut cursor = Cursor::new(Vec::new());
        let mut writer = zip::ZipWriter::new(&mut cursor);
        for (name, bytes) in entries {
            writer
                .start_file(*name, zip::write::SimpleFileOptions::default())
                .unwrap();
            writer.write_all(bytes).unwrap();
        }
        writer.finish().unwrap();
        cursor.into_inner()
    }

    #[test]
    fn hostile_inputs_yield_typed_errors_not_panics() {
        // Not a zip at all.
        assert!(matches!(
            Dar::from_bytes(b"just some text"),
            Err(DarError::NotADar(_))
        ));
        // Empty input.
        assert!(matches!(Dar::from_bytes(b""), Err(DarError::NotADar(_))));
        // A zip with no manifest.
        let no_manifest = zip_of(&[("pkg.dalf", b"\x01\x02")]);
        assert!(
            matches!(Dar::from_bytes(&no_manifest), Err(DarError::Manifest(m)) if m.contains("no META-INF"))
        );
        // A manifest that parses to nothing gets a distinct message.
        let empty_manifest = zip_of(&[
            ("META-INF/MANIFEST.MF", b"garbage without colon-space\n"),
            ("pkg.dalf", b"\x01"),
        ]);
        assert!(
            matches!(Dar::from_bytes(&empty_manifest), Err(DarError::Manifest(m)) if m.contains("no `Key: value`"))
        );
        // A non-UTF8 manifest is a manifest error, not a raw IO error.
        let bad_utf8 = zip_of(&[("META-INF/MANIFEST.MF", &[0xFF, 0xFE, 0x00][..])]);
        assert!(
            matches!(Dar::from_bytes(&bad_utf8), Err(DarError::Manifest(m)) if m.contains("UTF-8"))
        );
        // A manifest-bearing zip with zero .dalf packages (e.g. a plain JAR).
        let jarlike = zip_of(&[(
            "META-INF/MANIFEST.MF",
            b"Manifest-Version: 1.0\nMain-Class: com.example.App\n".as_slice(),
        )]);
        assert!(
            matches!(Dar::from_bytes(&jarlike), Err(DarError::Manifest(m)) if m.contains("no .dalf"))
        );
    }

    #[test]
    fn oversized_entries_are_rejected_by_the_decompression_cap() {
        // A stored (uncompressed) entry cannot cheaply exceed the cap in a unit
        // test, so exercise the declared-size branch via a deflated entry that
        // inflates far beyond its compressed size — 512 MiB of zeros compresses
        // to ~half a MB but must still be rejected.
        use std::io::Write as _;
        let mut cursor = Cursor::new(Vec::new());
        let mut writer = zip::ZipWriter::new(&mut cursor);
        writer
            .start_file(
                "META-INF/MANIFEST.MF",
                zip::write::SimpleFileOptions::default(),
            )
            .unwrap();
        writer.write_all(b"Main-Dalf: bomb.dalf\n").unwrap();
        writer
            .start_file("bomb.dalf", zip::write::SimpleFileOptions::default())
            .unwrap();
        let zeros = vec![0_u8; 8 * 1024 * 1024];
        for _ in 0..33 {
            writer.write_all(&zeros).unwrap(); // 264 MiB declared — over the cap
        }
        writer.finish().unwrap();
        let bytes = cursor.into_inner();

        assert!(matches!(
            Dar::from_bytes(&bytes),
            Err(DarError::EntryTooLarge { name }) if name == "bomb.dalf"
        ));
    }

    /// The shape a per-entry cap cannot see: every entry is legal on its own.
    ///
    /// Each `.dalf` here is well under any per-entry ceiling, and there is
    /// nothing to object to until they are added up — which is the whole point,
    /// because every one of them is read into memory. Zeros DEFLATE at roughly
    /// 1000:1, so the archive that asks for all of it arrives as a small file.
    #[test]
    fn many_legal_entries_that_together_exceed_the_budget_are_refused() {
        use std::io::Write as _;

        const ENTRY: usize = 512 * 1024;
        const BUDGET: u64 = 1024 * 1024; // room for two entries, not four

        let mut cursor = Cursor::new(Vec::new());
        let mut writer = zip::ZipWriter::new(&mut cursor);
        let options = zip::write::SimpleFileOptions::default();
        writer.start_file("META-INF/MANIFEST.MF", options).unwrap();
        writer.write_all(b"Main-Dalf: a.dalf\n").unwrap();
        let zeros = vec![0_u8; ENTRY];
        for name in ["a.dalf", "b.dalf", "c.dalf", "d.dalf"] {
            writer.start_file(name, options).unwrap();
            writer.write_all(&zeros).unwrap();
        }
        writer.finish().unwrap();
        let bytes = cursor.into_inner();

        // Small enough to hand around unnoticed — that is what makes it worth
        // guarding against rather than assuming nobody would send it.
        assert!(bytes.len() < 64 * 1024, "archive is {} bytes", bytes.len());

        let error = Dar::read(&bytes, BUDGET).expect_err("four entries exceed the budget");
        assert!(
            matches!(&error, DarError::ArchiveTooLarge { .. }),
            "the total is what was exceeded, not any one entry: {error}"
        );
        // And it says so: blaming a single 512 KiB entry would send the reader
        // to look at something that is not the problem.
        assert!(error.to_string().contains("in total"), "{error}");

        // The same archive is fine when the budget covers it, so the guard is
        // not simply refusing everything.
        assert!(Dar::read(&bytes, BUDGET * 8).is_ok());
    }

    /// The real corpus has to keep working: a guard that fires on a legitimate
    /// DAR is worse than no guard, because it is the one people meet.
    #[test]
    fn a_real_dar_is_nowhere_near_the_budget() {
        let Ok(path) = std::env::var("CANTON_TEST_DAR") else {
            return;
        };
        let Ok(bytes) = std::fs::read(&path) else {
            return;
        };
        let dar = Dar::from_bytes(&bytes).expect("a real DAR");
        let total: usize = dar.package_bytes().map(<[u8]>::len).sum();
        assert!(
            (total as u64) < MAX_TOTAL_BYTES / 10,
            "{path} decompresses to {total} bytes, within 10x of the budget"
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
