//! Produce a version-bumped copy of a DAR, without the Daml toolchain.
//!
//! The proposal's M2 verification asks that "an SCU version bump regenerates
//! compatible code". Demonstrating that needs a second version of a package,
//! and `daml build` is not available here — so the bump is applied where the
//! version actually lives: `PackageMetadata.version_interned_str` inside the
//! main package. Changing it changes the payload, so the package id (its
//! SHA-256) changes with it, exactly as a real rebuild's would.
//!
//! This is not a recompile: the Daml source is untouched and only the declared
//! version moves. That is the point — it isolates the version change from every
//! other difference a rebuild would bring, which is what the property under
//! test is about.
use std::io::Write as _;

use canton_lf::pb::daml_lf_dev::{Archive, ArchivePayload, HashFunction, archive_payload};
use prost::Message as _;
use sha2::{Digest, Sha256};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let input = args
        .next()
        .ok_or("usage: bump_version <in.dar> <out.dar> <new-version>")?;
    let output = args.next().ok_or("missing output path")?;
    let new_version = args.next().ok_or("missing version")?;

    let bytes = std::fs::read(&input)?;
    let dar = canton_lf::Dar::open(&input)?;
    let main_path = {
        // The manifest's Main-Dalf, recovered through the public API.
        let main = dar.main_package_bytes()?;
        dar.package_entries()
            .find(|(_, b)| *b == main)
            .map(|(name, _)| name.to_string())
            .ok_or("main dalf not found among entries")?
    };

    let mut archive = Archive::decode(dar.main_package_bytes()?)?;
    let mut payload = ArchivePayload::decode(archive.payload.as_slice())?;
    let Some(archive_payload::Sum::DamlLf2(package)) = payload.sum.as_mut() else {
        return Err("not an LF 2 package".into());
    };
    let version_index = package
        .metadata
        .as_ref()
        .ok_or("package has no metadata")?
        .version_interned_str;
    let old_version = canton_lf::interned_str(package, version_index)
        .ok_or("version not interned")?
        .to_string();
    // A fresh interned string for the new version; the old one stays put so no
    // other index shifts.
    package.interned_strings.push(new_version.clone());
    let index = i32::try_from(package.interned_strings.len() - 1)?;
    let metadata = package
        .metadata
        .as_mut()
        .ok_or("package metadata vanished between reads")?;
    metadata.version_interned_str = index;

    let repacked = payload.encode_to_vec();
    let new_id = Sha256::digest(&repacked)
        .iter()
        .fold(String::new(), |mut id, byte| {
            use std::fmt::Write as _;
            let _ = write!(id, "{byte:02x}");
            id
        });
    archive.hash.clone_from(&new_id);
    archive.hash_function = HashFunction::Sha256 as i32;
    archive.payload = repacked;
    let new_dalf = archive.encode_to_vec();

    // Rebuild the zip: the main dalf under its new id, the manifest pointing at
    // it, everything else carried across untouched.
    let old_id = main_path
        .rsplit('-')
        .next()
        .and_then(|s| s.strip_suffix(".dalf"))
        .ok_or("cannot read the id out of the main dalf path")?
        .to_string();
    let rename = |name: &str| {
        name.replace(&old_id, &new_id)
            .replace(&format!("-{old_version}-"), &format!("-{new_version}-"))
    };

    let mut reader = zip::ZipArchive::new(std::io::Cursor::new(&bytes))?;
    let out = std::fs::File::create(&output)?;
    let mut writer = zip::ZipWriter::new(out);
    let options = zip::write::SimpleFileOptions::default();
    for i in 0..reader.len() {
        let mut entry = reader.by_index(i)?;
        let name = entry.name().to_string();
        if !entry.is_file() {
            continue;
        }
        let mut data = Vec::new();
        std::io::Read::read_to_end(&mut entry, &mut data)?;
        if name == "META-INF/MANIFEST.MF" {
            let text = String::from_utf8(data)?;
            // The manifest wraps at 72 bytes; unwrap, substitute, leave it
            // unwrapped (the reader un-wraps continuations either way).
            let unwrapped = text.replace("\r\n ", "").replace("\n ", "");
            data = rename(&unwrapped).into_bytes();
        } else if name == main_path {
            data.clone_from(&new_dalf);
        }
        writer.start_file(rename(&name), options)?;
        writer.write_all(&data)?;
    }
    writer.finish()?;

    println!("{input}");
    println!("  version {old_version} -> {new_version}");
    println!("  package id {old_id}");
    println!("          -> {new_id}");
    println!("  wrote {output}");
    Ok(())
}
