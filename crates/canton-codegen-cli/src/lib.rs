//! Core of the `dpm-codegen-rust` CLI: decode a DAR and its dependency closure,
//! lower it, and write a self-contained Rust crate of typed bindings.
//!
//! Kept as a library (with a thin `main.rs` over it) so the generation is
//! testable without spawning the binary.

use std::fs;
use std::path::PathBuf;

use canton_codegen::{generate_crate, lower_dar};
use canton_lf::Dar;

/// How the generated crate depends on the `canton-daml` runtime.
#[derive(Debug, Clone)]
pub enum Runtime {
    /// A published version requirement (e.g. `0.1`).
    Version(String),
    /// A local path — useful in a monorepo or for testing before publish.
    Path(PathBuf),
}

/// What to generate and where.
#[derive(Debug, Clone)]
pub struct Options {
    /// The input `.dar`.
    pub dar: PathBuf,
    /// The output crate directory (created if absent).
    pub out: PathBuf,
    /// The generated crate's package name.
    pub crate_name: String,
    /// The runtime dependency to write into the generated `Cargo.toml`.
    pub runtime: Runtime,
}

/// A summary of a successful generation.
#[derive(Debug, Clone)]
pub struct Stats {
    /// Packages in the DAR closure that produced modules.
    pub packages: usize,
    /// Generated Rust submodules.
    pub modules: usize,
    /// Generated named items (data types + templates + interfaces).
    pub items: usize,
    /// Size of the generated `lib.rs`, in bytes.
    pub bytes: usize,
    /// Types that could not be lowered (best-effort; surfaced as warnings).
    pub skipped: Vec<String>,
}

/// Decode `opts.dar`, lower its whole closure, and write a crate
/// (`Cargo.toml` + `src/lib.rs`) into `opts.out`.
///
/// # Errors
/// Returns an error if the DAR cannot be read/decoded, the generated tokens are
/// not valid Rust (a generator bug), or the output cannot be written.
pub fn generate(opts: &Options) -> Result<Stats, Box<dyn std::error::Error>> {
    let dar = Dar::open(&opts.dar)?;
    let (krate, errors) = lower_dar(&dar)?;
    let source = generate_crate(&krate)?;

    let modules = krate.packages.iter().map(|p| p.modules.len()).sum();
    let items = krate
        .packages
        .iter()
        .flat_map(|p| &p.modules)
        .map(|m| m.module.data_types.len() + m.module.templates.len() + m.module.interfaces.len())
        .sum();

    let src_dir = opts.out.join("src");
    fs::create_dir_all(&src_dir)?;
    fs::write(opts.out.join("Cargo.toml"), cargo_toml(opts))?;
    fs::write(src_dir.join("lib.rs"), &source)?;

    Ok(Stats {
        packages: krate.packages.len(),
        modules,
        items,
        bytes: source.len(),
        skipped: errors.into_iter().map(|error| error.0).collect(),
    })
}

/// The `Cargo.toml` for the generated crate: a standalone (`[workspace]`) crate
/// depending on the `canton-daml` runtime.
fn cargo_toml(opts: &Options) -> String {
    let dependency = match &opts.runtime {
        Runtime::Version(version) => format!("canton-daml = \"{version}\""),
        Runtime::Path(path) => {
            format!("canton-daml = {{ path = \"{}\" }}", path.display())
        }
    };
    format!(
        "[package]\nname = \"{name}\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n\
         [dependencies]\n{dependency}\n\n\
         # Generated bindings are their own crate; depend on it by path.\n\
         [workspace]\n",
        name = opts.crate_name,
    )
}

/// Derive a crate name from a DAR path: its file stem, sanitised to a valid
/// Cargo package name (`splice-amulet-0.1.14.dar` → `splice-amulet-0-1-14`).
#[must_use]
pub fn default_crate_name(dar: &std::path::Path) -> String {
    let stem = dar
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("bindings");
    let name: String = stem
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' {
                c
            } else {
                '-'
            }
        })
        .collect();
    if name.is_empty() {
        "bindings".to_string()
    } else {
        name
    }
}
