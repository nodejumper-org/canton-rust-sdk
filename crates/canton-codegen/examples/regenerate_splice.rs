//! Regenerate every checked-in bindings crate.
//!
//! The Splice DARs share their dependency closure, so each crate references the
//! packages that are published as crates of their own rather than copying them.
//! Copying is what made two of these crates unusable together: each declared
//! its own `Holding`, and Rust treats those as unrelated types.
//!
//! Which crate is generated from which DAR, and what each one references, is in
//! `examples/shared/splice_crates.rs` — shared with the drift guard so the two
//! cannot disagree.
//!
//! Usage:
//!
//! ```text
//! SPLICE_DARS=<dir> \
//! CANTON_LICENSING_DAR=<dir>/quickstart-licensing-0.0.1.dar \
//!   cargo run -p canton-codegen --example regenerate_splice
//! ```
//!
//! `CANTON_LICENSING_DAR` is optional; that crate is skipped without it.
//! Regenerate all of them together — a change to the emitter or to the table
//! affects every crate, and the guard checks them as a set.

include!("shared/splice_crates.rs");

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if std::env::var_os("SPLICE_DARS").is_none() {
        return Err("set SPLICE_DARS to the directory holding the Splice DARs".into());
    }
    let root = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");

    for (crate_name, source, externals, emits) in CRATES {
        let Some(path) = dar_path(*source) else {
            println!("{crate_name:<52} skipped — its DAR is not configured");
            continue;
        };
        // Only `src/lib.rs` is written. Every crate here keeps a curated
        // manifest — workspace inheritance, licence, keywords, `include`, a
        // README — and `Options::generate`'s manifest is a bootstrap for a
        // *user's* crate, not for one of ours: it would drop all of that and
        // take the version from the DAR instead of the workspace.
        let dar = canton_lf::Dar::open(&path)?;
        let (krate, skipped) = canton_codegen::lower_dar_selecting(
            &dar,
            &selection_for(*emits),
            &externals_for(externals, *emits),
        )?;
        let source = canton_codegen::generate_crate(&krate)?;
        let out = format!("{root}/crates/{crate_name}/src");
        std::fs::create_dir_all(&out)?;
        std::fs::write(format!("{out}/lib.rs"), &source)?;
        println!(
            "{crate_name:<52} {:>2} packages  {:>4} KB  {} skipped",
            krate.packages.len(),
            source.len() / 1024,
            skipped.len()
        );
        if std::env::var_os("SHOW_SKIPPED").is_some() {
            for s in &skipped {
                println!("      skipped: {s}");
            }
        }
    }
    Ok(())
}
