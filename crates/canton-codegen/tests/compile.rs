//! End-to-end proof: generate a module covering every data-type shape, then
//! compile and round-trip it as a standalone crate depending on `canton-daml`.
//! Gated on `CODEGEN_COMPILE_TEST` because it spawns `cargo` (slow); the
//! in-crate tests cover the fast path. This is the test that catches
//! *compilability*, not just `syn::parse` validity.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::too_many_lines)]

use std::fs;
use std::process::Command;

use canton_codegen::ir::{
    DamlType, DataType, Enum, Field, Module, Record, TypeRef, Variant, VariantConstructor,
};
use canton_codegen::{generate_crate, generate_module};

fn field(label: &str, ty: DamlType) -> Field {
    Field {
        label: label.to_string(),
        ty,
    }
}

fn reference(name: &str, args: Vec<DamlType>) -> DamlType {
    DamlType::Ref(TypeRef::local(name, args))
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

#[test]
fn nullary_variant_matches_ledger_api_json() {
    // A nullary constructor's LF-JSON is `{"tag":<c>,"value":{}}` (Unit = `{}`),
    // which is exactly what the JSON Ledger API sends and accepts.
    let point = Shape::Point(rt::Unit);
    let json = serde_json::to_string(&point).unwrap();
    assert_eq!(json, "{\"tag\":\"Point\",\"value\":{}}", "nullary variant JSON");
    // Round-trips through JSON…
    let back: Shape = serde_json::from_str(&json).unwrap();
    assert_eq!(back, point);
    // …and parses the exact API form regardless of how we happened to emit it.
    let from_api: Shape = serde_json::from_str("{\"tag\":\"Point\",\"value\":{}}").unwrap();
    assert_eq!(from_api, point);
    // gRPC `Value` round-trips too.
    let grpc = <Shape as rt::FromValue>::from_value(&rt::ToValue::to_value(&point)).unwrap();
    assert_eq!(grpc, point);
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

/// A one-line summary of a successfully generated + compiled DAR crate.
struct DarReport {
    slug: String,
    packages: usize,
    modules: usize,
    types: usize,
    bytes: usize,
}

/// A filesystem-safe slug for a DAR (its file stem, non-alphanumerics → `_`),
/// used to give every DAR its own temp crate dir so many can build in parallel
/// without racing on sources or on the target dir.
fn dar_slug(dar_path: &str) -> String {
    std::path::Path::new(dar_path)
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("dar")
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

/// Decode a DAR and its full dependency closure, lower it to a qualified module
/// tree, generate the crate, and **compile** it against `canton-daml`. Runs in a
/// temp dir unique to the DAR. `target_dir` shares one build target across many
/// DARs (fast, low disk) when `Some`; `None` isolates the target per DAR (needed
/// when several DARs build concurrently). Returns a summary or a failure reason.
fn compile_dar(dar_path: &str, target_dir: Option<&std::path::Path>) -> Result<DarReport, String> {
    use canton_codegen::lower_dar;
    use canton_lf::Dar;

    let slug = dar_slug(dar_path);
    let dar = Dar::open(dar_path).map_err(|error| format!("open DAR: {error}"))?;
    let (krate, errors) = lower_dar(&dar).map_err(|error| format!("decode+lower: {error}"))?;
    let unresolved: Vec<&String> = errors
        .iter()
        .map(|error| &error.0)
        .filter(|message| message.contains("not in the DAR"))
        .collect();
    if !unresolved.is_empty() {
        return Err(format!(
            "{} unresolved cross-package refs, e.g. {}",
            unresolved.len(),
            unresolved[0]
        ));
    }
    let generated = generate_crate(&krate).map_err(|error| format!("generate: {error}"))?;

    let report = DarReport {
        slug: slug.clone(),
        packages: krate.packages.len(),
        modules: krate.packages.iter().map(|p| p.modules.len()).sum(),
        types: krate
            .packages
            .iter()
            .flat_map(|p| &p.modules)
            .map(|m| m.module.data_types.len())
            .sum(),
        bytes: generated.len(),
    };

    let daml = concat!(env!("CARGO_MANIFEST_DIR"), "/../canton-daml");
    let dir = std::env::temp_dir().join(format!("canton-codegen-dar-{slug}"));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(dir.join("src")).map_err(|error| error.to_string())?;
    fs::write(
        dir.join("Cargo.toml"),
        format!(
            "[package]\nname = \"gen_{slug}\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n\
             [dependencies]\ncanton-daml = {{ path = {daml:?} }}\n\n\
             [workspace]\n"
        ),
    )
    .map_err(|error| error.to_string())?;
    fs::write(dir.join("src/lib.rs"), &generated).map_err(|error| error.to_string())?;

    let mut command = Command::new(env!("CARGO"));
    command.args(["build", "--quiet"]).current_dir(&dir);
    if let Some(target) = target_dir {
        command.env("CARGO_TARGET_DIR", target);
    }
    let output = command
        .output()
        .map_err(|error| format!("spawn cargo: {error}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let head: Vec<&str> = stderr
            .lines()
            .filter(|line| line.contains("error"))
            .take(8)
            .collect();
        return Err(format!("compile failed: {}", head.join(" | ")));
    }
    Ok(report)
}

/// The whole-DAR proof: decode a real DAR and its full dependency closure,
/// lower it into a qualified `pub mod` tree, and **compile** the generated crate
/// against `canton-daml`. Syntactic validity (`syn::parse`) is not enough — this
/// is what proves cross-package references and module qualification actually
/// type-check at scale. Gated on both env vars (needs a DAR and spawns cargo).
#[test]
fn generated_dar_crate_compiles_against_runtime() {
    if std::env::var("CODEGEN_COMPILE_TEST").is_err() {
        eprintln!("skipping: set CODEGEN_COMPILE_TEST=1 to run (spawns cargo)");
        return;
    }
    let Ok(dar_path) = std::env::var("CANTON_TEST_DAR") else {
        eprintln!("skipping: set CANTON_TEST_DAR=/path/to/x.dar");
        return;
    };
    match compile_dar(&dar_path, None) {
        Ok(report) => println!(
            "compiled {}: {} pkgs / {} mods / {} types / {} KB",
            report.slug,
            report.packages,
            report.modules,
            report.types,
            report.bytes / 1024,
        ),
        Err(reason) => panic!("{dar_path}: {reason}"),
    }
}

/// The corpus proof: compile **every** `.dar` in `CANTON_TEST_DAR_DIR` (each with
/// its full dependency closure) against `canton-daml`, and fail if any does not
/// type-check. This is the "super tested" sweep — one run covers the whole DAR
/// zoo. Builds share one target dir (fast, bounded disk); runs sequentially.
#[test]
fn generated_dar_corpus_compiles_against_runtime() {
    if std::env::var("CODEGEN_COMPILE_TEST").is_err() {
        eprintln!("skipping: set CODEGEN_COMPILE_TEST=1 to run (spawns cargo)");
        return;
    }
    let Ok(corpus) = std::env::var("CANTON_TEST_DAR_DIR") else {
        eprintln!("skipping: set CANTON_TEST_DAR_DIR=/dir/of/dars");
        return;
    };

    let mut dars: Vec<std::path::PathBuf> = fs::read_dir(&corpus)
        .expect("read corpus dir")
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.extension().is_some_and(|ext| ext == "dar"))
        .collect();
    dars.sort();
    assert!(!dars.is_empty(), "no .dar files under {corpus}");

    let target = std::env::temp_dir().join("canton-codegen-corpus-target");
    let mut failures = Vec::new();
    for dar in &dars {
        let path = dar.to_string_lossy().to_string();
        match compile_dar(&path, Some(&target)) {
            Ok(report) => println!(
                "PASS {:<46} {:>2} pkgs / {:>3} mods / {:>4} types / {:>4} KB",
                report.slug,
                report.packages,
                report.modules,
                report.types,
                report.bytes / 1024,
            ),
            Err(reason) => {
                println!("FAIL {:<46} {reason}", dar_slug(&path));
                failures.push((dar_slug(&path), reason));
            }
        }
    }
    println!("corpus: {} DARs, {} failed", dars.len(), failures.len());
    assert!(
        failures.is_empty(),
        "{} of {} DARs failed to compile:\n{}",
        failures.len(),
        dars.len(),
        failures
            .iter()
            .map(|(name, reason)| format!("  {name}: {reason}"))
            .collect::<Vec<_>>()
            .join("\n"),
    );
}
