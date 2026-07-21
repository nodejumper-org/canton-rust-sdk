//! End-to-end proof: generate a module, then compile and round-trip it as a
//! standalone crate depending on `canton-daml`. Gated on `CODEGEN_COMPILE_TEST`
//! because it spawns `cargo` (slow); the in-crate tests cover the fast path.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use std::fs;
use std::process::Command;

use canton_codegen::generate_module;
use canton_codegen::ir::{DamlType, DataType, Field, Module, Record};

#[test]
fn generated_module_compiles_and_round_trips_against_runtime() {
    if std::env::var("CODEGEN_COMPILE_TEST").is_err() {
        eprintln!("skipping: set CODEGEN_COMPILE_TEST=1 to run (spawns cargo)");
        return;
    }

    let module = Module {
        data_types: vec![DataType::Record(Record {
            name: "Payload".to_string(),
            type_params: Vec::new(),
            fields: vec![
                Field {
                    label: "owner".to_string(),
                    ty: DamlType::Party,
                },
                Field {
                    label: "count".to_string(),
                    ty: DamlType::Int64,
                },
                Field {
                    label: "tags".to_string(),
                    ty: DamlType::List(Box::new(DamlType::Text)),
                },
            ],
        })],
        templates: Vec::new(),
    };
    let generated = generate_module(&module).unwrap();

    let daml = concat!(env!("CARGO_MANIFEST_DIR"), "/../canton-daml");
    let dir = std::env::temp_dir().join("canton-codegen-compile-check");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.join("src")).unwrap();

    fs::write(
        dir.join("Cargo.toml"),
        format!(
            "[package]\nname = \"gen-check\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n\
             [dependencies]\ncanton-daml = {{ path = {daml:?} }}\n\n\
             [workspace]\n"
        ),
    )
    .unwrap();

    // The generated module plus a round-trip test appended.
    let src = format!(
        "{generated}\n\
         #[test]\n\
         fn round_trip() {{\n\
         let payload = Payload {{\n\
         owner: rt::Party::new(\"alice::1\"),\n\
         count: 7,\n\
         tags: vec![\"defi\".to_string()],\n\
         }};\n\
         let back = <Payload as rt::FromValue>::from_value(&rt::ToValue::to_value(&payload)).unwrap();\n\
         assert_eq!(back, payload);\n\
         }}\n"
    );
    fs::write(dir.join("src/lib.rs"), src).unwrap();

    let output = Command::new(env!("CARGO"))
        .args(["test", "--quiet"])
        .current_dir(&dir)
        .output()
        .expect("failed to run cargo");

    assert!(
        output.status.success(),
        "generated crate failed:\n--- stdout ---\n{}\n--- stderr ---\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}
