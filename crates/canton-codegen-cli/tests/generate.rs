//! End-to-end test of the CLI's core: generate a crate from a real DAR and,
//! when `CODEGEN_COMPILE_TEST=1`, compile it against the local `canton-daml`.
//! Env-gated on `CANTON_TEST_DAR` (needs a real DAR).
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::path::PathBuf;
use std::process::Command;

use canton_codegen_cli::{Options, Runtime, default_crate_name, generate};

#[test]
fn generates_a_crate_from_a_real_dar() {
    let Ok(dar) = std::env::var("CANTON_TEST_DAR") else {
        eprintln!("skipping: set CANTON_TEST_DAR=/path/to/x.dar");
        return;
    };
    let dar = PathBuf::from(dar);

    let out = std::env::temp_dir().join("dpm-codegen-rust-check");
    let _ = std::fs::remove_dir_all(&out);
    // Depend on canton-daml by path so the generated crate builds in-tree.
    let runtime = PathBuf::from(concat!(env!("CARGO_MANIFEST_DIR"), "/../canton-daml"));

    let stats = generate(&Options {
        dar: dar.clone(),
        out: out.clone(),
        crate_name: default_crate_name(&dar),
        runtime: Runtime::Path(runtime),
    })
    .expect("generation succeeds");

    assert!(stats.packages > 0, "a DAR should yield packages");
    assert!(stats.items > 0, "a DAR should yield items");
    assert!(out.join("Cargo.toml").is_file(), "wrote Cargo.toml");
    assert!(out.join("src/lib.rs").is_file(), "wrote src/lib.rs");
    println!(
        "CLI generated {} packages / {} modules / {} items / {} KB ({} skipped)",
        stats.packages,
        stats.modules,
        stats.items,
        stats.bytes / 1024,
        stats.skipped.len(),
    );

    if std::env::var("CODEGEN_COMPILE_TEST").is_err() {
        eprintln!("skipping compile: set CODEGEN_COMPILE_TEST=1");
        return;
    }
    let output = Command::new(env!("CARGO"))
        .args(["build", "--quiet"])
        .current_dir(&out)
        .output()
        .expect("run cargo");
    assert!(
        output.status.success(),
        "generated crate must compile:\n{}",
        String::from_utf8_lossy(&output.stderr)
            .lines()
            .take(40)
            .collect::<Vec<_>>()
            .join("\n"),
    );
}
