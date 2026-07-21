//! End-to-end proof: generate a module covering every data-type shape, then
//! compile and round-trip it as a standalone crate depending on `canton-daml`.
//! Gated on `CODEGEN_COMPILE_TEST` because it spawns `cargo` (slow); the
//! in-crate tests cover the fast path. This is the test that catches
//! *compilability*, not just `syn::parse` validity.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::too_many_lines)]

use std::fs;
use std::process::Command;

use canton_codegen::generate_module;
use canton_codegen::ir::{
    DamlType, DataType, Enum, Field, Module, Record, TypeRef, Variant, VariantConstructor,
};

fn field(label: &str, ty: DamlType) -> Field {
    Field {
        label: label.to_string(),
        ty,
    }
}

fn reference(name: &str, args: Vec<DamlType>) -> DamlType {
    DamlType::Ref(TypeRef {
        name: name.to_string(),
        args,
    })
}

#[test]
fn generated_module_compiles_and_round_trips_against_runtime() {
    if std::env::var("CODEGEN_COMPILE_TEST").is_err() {
        eprintln!("skipping: set CODEGEN_COMPILE_TEST=1 to run (spawns cargo)");
        return;
    }

    // A module exercising every shape: an enum, a variant (payload + nullary), a
    // generic record, and a record whose fields reference all of them plus a
    // GenMap and a nested Optional.
    let module = Module {
        data_types: vec![
            DataType::Enum(Enum {
                name: "DayOfWeek".to_string(),
                constructors: vec!["Monday".to_string(), "Tuesday".to_string()],
            }),
            DataType::Variant(Variant {
                name: "Shape".to_string(),
                type_params: Vec::new(),
                constructors: vec![
                    VariantConstructor {
                        name: "Circle".to_string(),
                        payload: Some(DamlType::Numeric(10)),
                    },
                    VariantConstructor {
                        name: "Point".to_string(),
                        payload: None,
                    },
                ],
            }),
            DataType::Record(Record {
                name: "Pair".to_string(),
                type_params: vec!["a".to_string(), "b".to_string()],
                fields: vec![
                    field("fst", DamlType::Var("a".to_string())),
                    field("snd", DamlType::Var("b".to_string())),
                ],
            }),
            DataType::Record(Record {
                name: "Payload".to_string(),
                type_params: Vec::new(),
                fields: vec![
                    field("owner", DamlType::Party),
                    field("day", reference("DayOfWeek", Vec::new())),
                    field("shape", reference("Shape", Vec::new())),
                    field(
                        "pair",
                        reference("Pair", vec![DamlType::Int64, DamlType::Text]),
                    ),
                    field(
                        "attrs",
                        DamlType::GenMap(Box::new(DamlType::Text), Box::new(DamlType::Int64)),
                    ),
                    field(
                        "maybe",
                        DamlType::Optional(Box::new(DamlType::Optional(Box::new(DamlType::Text)))),
                    ),
                ],
            }),
        ],
        templates: Vec::new(),
        interfaces: Vec::new(),
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
             [dependencies]\ncanton-daml = {{ path = {daml:?} }}\nserde_json = \"1\"\n\n\
             [workspace]\n"
        ),
    )
    .unwrap();

    // The generated module + a round-trip test exercising both codecs on every
    // shape (enum, variant, generic instantiation, GenMap, nested Optional).
    let test = r#"
#[test]
fn round_trip() {
    let payload = Payload {
        owner: rt::Party::new("alice::1"),
        day: DayOfWeek::Monday,
        shape: Shape::Circle(rt::Numeric("1.5".to_string())),
        pair: Pair { fst: 7i64, snd: "x".to_string() },
        attrs: rt::GenMap(vec![("k".to_string(), 1i64)]),
        maybe: Some(rt::NestedOpt(None)),
    };
    let back = <Payload as rt::FromValue>::from_value(&rt::ToValue::to_value(&payload)).unwrap();
    assert_eq!(back, payload);
    let json = serde_json::to_string(&payload).unwrap();
    let from_json: Payload = serde_json::from_str(&json).unwrap();
    assert_eq!(from_json, payload);
    assert!(json.contains("\"Monday\""), "enum is a bare string: {json}");
    assert!(json.contains("\"tag\":\"Circle\""), "variant is adjacently tagged: {json}");
}
"#;
    fs::write(dir.join("src/lib.rs"), format!("{generated}\n{test}")).unwrap();

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
