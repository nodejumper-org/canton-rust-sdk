//! `dpm-codegen-rust` — generate a typed Rust crate from a DAR.
//!
//! Invoked as `dpm codegen-rust` (dpm resolves subcommands to `dpm-<name>`
//! binaries, like git/cargo) or standalone. Designed to be called from a build
//! script the way `prost-build` is, so a change to a DAR regenerates bindings.

use std::path::PathBuf;
use std::process::ExitCode;

use canton_codegen_cli::{Options, Runtime, default_crate_name, generate};

const USAGE: &str = "\
dpm-codegen-rust — generate a typed Rust crate from a DAR

USAGE:
    dpm-codegen-rust --dar <PATH> --out <DIR> [OPTIONS]

OPTIONS:
    --dar <PATH>              Input .dar (with its dependency closure)
    --out <DIR>               Output crate directory (created if absent)
    --name <NAME>             Generated crate name [default: derived from the DAR]
    --runtime-path <PATH>     Depend on canton-daml by path (default: by version)
    --runtime-version <VER>   canton-daml version requirement [default: 0.1]
    -h, --help                Print this help
";

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}

/// The next argument after a flag, or an error naming the flag.
fn next_value<I: Iterator<Item = String>>(
    args: &mut I,
    flag: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    args.next()
        .ok_or_else(|| format!("{flag} needs a value").into())
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut dar: Option<PathBuf> = None;
    let mut out: Option<PathBuf> = None;
    let mut name: Option<String> = None;
    let mut runtime_path: Option<PathBuf> = None;
    let mut runtime_version: Option<String> = None;

    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                print!("{USAGE}");
                return Ok(());
            }
            "--dar" => dar = Some(PathBuf::from(next_value(&mut args, "--dar")?)),
            "--out" => out = Some(PathBuf::from(next_value(&mut args, "--out")?)),
            "--name" => name = Some(next_value(&mut args, "--name")?),
            "--runtime-path" => {
                runtime_path = Some(PathBuf::from(next_value(&mut args, "--runtime-path")?));
            }
            "--runtime-version" => {
                runtime_version = Some(next_value(&mut args, "--runtime-version")?);
            }
            other => return Err(format!("unexpected argument `{other}`\n\n{USAGE}").into()),
        }
    }

    let dar = dar.ok_or("missing required --dar")?;
    let out = out.ok_or("missing required --out")?;
    let crate_name = name.unwrap_or_else(|| default_crate_name(&dar));
    let runtime = match runtime_path {
        Some(path) => Runtime::Path(path),
        None => Runtime::Version(runtime_version.unwrap_or_else(|| "0.1".to_string())),
    };

    let stats = generate(&Options {
        dar,
        out: out.clone(),
        crate_name: crate_name.clone(),
        runtime,
    })?;

    eprintln!(
        "generated `{crate_name}` in {}: {} packages / {} modules / {} items / {} KB",
        out.display(),
        stats.packages,
        stats.modules,
        stats.items,
        stats.bytes / 1024,
    );
    if !stats.skipped.is_empty() {
        eprintln!(
            "warning: {} type(s) could not be lowered and were skipped:",
            stats.skipped.len()
        );
        for message in stats.skipped.iter().take(20) {
            eprintln!("  - {message}");
        }
    }
    Ok(())
}
