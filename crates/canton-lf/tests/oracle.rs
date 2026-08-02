//! Conformance oracle: our native Daml-LF decoder vs the official JVM
//! `daml-lf-archive` reader.
//!
//! Both sides read the same DAR and render its **type-signature surface** — the
//! part of a package codegen consumes: package id/name/version, serializable
//! data types (records, variants, enums), templates (key, implements, choices),
//! and interfaces (view, choices) — as one canonical JSON document. If the two
//! documents are equal, our decoder interprets the archive exactly as the
//! authoritative implementation does. Expression bodies (controllers, updates,
//! signatories) are deliberately out of scope: codegen never reads them.
//!
//! Env-gated twice over:
//! - `CANTON_LF_ORACLE_DAR=/path/to/x.dar` selects the DAR (skips otherwise);
//! - `scala-cli` must be on `PATH` (skips otherwise) — it fetches the official
//!   `daml-lf-archive-reader` from Maven and runs `tools/lf-oracle/LfOracle.scala`.
//!
//! ```sh
//! CANTON_LF_ORACLE_DAR=/path/to/quickstart-licensing-0.0.1.dar \
//!   cargo test -p canton-lf --test oracle -- --nocapture
//! ```

#![allow(clippy::unwrap_used, clippy::expect_used)]

use canton_lf::pb::daml_lf_2::r#type::Sum;
use canton_lf::pb::daml_lf_2::{
    self as lf, BuiltinType, DefDataType, DefInterface, DefTemplate, Package, TemplateChoice, Type,
};
use canton_lf::{Dar, decode_all, imported_package_id, interned_dotted_name, interned_str};
use serde_json::{Value, json};

// ── type rendering ──────────────────────────────────────────────────────────
//
// A type renders to a compact canonical string, identical on both sides:
//   bare builtin/var/ref:  `Party`, `a`, `<pkgid>:Mod.Ule:Name`
//   applied:               `(List Party)`, `(<pkgid>:Mod:Name a)`
//   numeric scale (Nat):   `10`  →  `(Numeric 10)`
//   synonym:               `(%syn <ref> args…)`
// The 2.dev curried `TApp` spelling is flattened, so `TApp(TApp(C,a),b)` and
// `Con(C,[a,b])` render identically.

/// Resolve a `TypeConId`/`TypeSynId`-style reference to `pkgid:Module:Name`.
fn render_ref(pkg: &Package, self_id: &str, module: &lf::ModuleId, name_dname: i32) -> String {
    let package_id = match module.package_id.as_ref().and_then(|p| p.sum.as_ref()) {
        Some(lf::self_or_imported_package_id::Sum::SelfPackageId(_)) => self_id.to_string(),
        Some(lf::self_or_imported_package_id::Sum::ImportedPackageIdInternedStr(i)) => {
            interned_str(pkg, *i)
                .expect("imported package id")
                .to_string()
        }
        Some(lf::self_or_imported_package_id::Sum::PackageImportId(i)) => {
            imported_package_id(pkg, *i)
                .expect("package import")
                .to_string()
        }
        None => panic!("type reference without a package id"),
    };
    let module_name = interned_dotted_name(pkg, module.module_name_interned_dname)
        .expect("module name of a type reference");
    let name = interned_dotted_name(pkg, name_dname).expect("name of a type reference");
    format!("{package_id}:{module_name}:{name}")
}

/// Render a type; `head` + already-rendered `args` (from flattened `TApp`s).
#[allow(clippy::too_many_lines)] // one arm per LF type constructor
fn render_type_applied(pkg: &Package, self_id: &str, ty: &Type, mut extra: Vec<String>) -> String {
    let Some(sum) = ty.sum.as_ref() else {
        panic!("empty Type")
    };
    let (head, args): (String, Vec<String>) = match sum {
        Sum::Interned(index) => {
            let resolved = canton_lf::interned_type(pkg, *index).expect("interned type");
            return render_type_applied(pkg, self_id, resolved, extra);
        }
        Sum::Tapp(app) => {
            // Curried application: flatten by pushing rhs onto the pending args.
            let rhs = render_type(pkg, self_id, app.rhs.as_ref().expect("tapp rhs"));
            let mut pending = vec![rhs];
            pending.append(&mut extra);
            return render_type_applied(pkg, self_id, app.lhs.as_ref().expect("tapp lhs"), pending);
        }
        Sum::Nat(n) => (n.to_string(), vec![]),
        Sum::Var(v) => (
            interned_str(pkg, v.var_interned_str)
                .expect("type var")
                .to_string(),
            v.args
                .iter()
                .map(|a| render_type(pkg, self_id, a))
                .collect(),
        ),
        Sum::Builtin(b) => {
            let name = match BuiltinType::try_from(b.builtin).expect("builtin type") {
                BuiltinType::Unit => "Unit",
                BuiltinType::Bool => "Bool",
                BuiltinType::Int64 => "Int64",
                BuiltinType::Date => "Date",
                BuiltinType::Timestamp => "Timestamp",
                BuiltinType::Numeric => "Numeric",
                BuiltinType::Party => "Party",
                BuiltinType::Text => "Text",
                BuiltinType::ContractId => "ContractId",
                BuiltinType::Optional => "Optional",
                BuiltinType::List => "List",
                BuiltinType::Genmap => "GenMap",
                BuiltinType::Textmap => "TextMap",
                BuiltinType::Any => "Any",
                BuiltinType::AnyException => "AnyException",
                BuiltinType::TypeRep => "TypeRep",
                BuiltinType::Arrow => "Arrow",
                BuiltinType::Update => "Update",
                BuiltinType::FailureCategory => "FailureCategory",
                BuiltinType::Bignumeric => "BigNumeric",
                BuiltinType::RoundingMode => "RoundingMode",
            };
            (
                name.to_string(),
                b.args
                    .iter()
                    .map(|a| render_type(pkg, self_id, a))
                    .collect(),
            )
        }
        Sum::Con(c) => {
            let tycon = c.tycon.as_ref().expect("tycon id");
            (
                render_ref(
                    pkg,
                    self_id,
                    tycon.module.as_ref().expect("tycon module"),
                    tycon.name_interned_dname,
                ),
                c.args
                    .iter()
                    .map(|a| render_type(pkg, self_id, a))
                    .collect(),
            )
        }
        Sum::Syn(s) => {
            let tysyn = s.tysyn.as_ref().expect("tysyn id");
            let reference = render_ref(
                pkg,
                self_id,
                tysyn.module.as_ref().expect("tysyn module"),
                tysyn.name_interned_dname,
            );
            let args: Vec<String> = s
                .args
                .iter()
                .map(|a| render_type(pkg, self_id, a))
                .collect();
            (format!("%syn {reference}"), args)
        }
        Sum::Forall(f) => {
            let vars: Vec<String> = f
                .vars
                .iter()
                .map(|v| {
                    interned_str(pkg, v.var_interned_str)
                        .expect("forall var")
                        .to_string()
                })
                .collect();
            let body = render_type(pkg, self_id, f.body.as_ref().expect("forall body"));
            (format!("%forall ({}) {body}", vars.join(" ")), vec![])
        }
        Sum::Struct(s) => {
            let fields: Vec<String> = s
                .fields
                .iter()
                .map(|f| {
                    let label = interned_str(pkg, f.field_interned_str).expect("struct field");
                    let ty = render_type(pkg, self_id, f.r#type.as_ref().expect("struct type"));
                    format!("({label} {ty})")
                })
                .collect();
            (format!("%struct {}", fields.join(" ")), vec![])
        }
    };
    let mut all = args;
    all.append(&mut extra);
    if all.is_empty() && !head.contains(' ') {
        head
    } else {
        format!("({} {})", head, all.join(" ")).replace(" )", ")")
    }
}

fn render_type(pkg: &Package, self_id: &str, ty: &Type) -> String {
    render_type_applied(pkg, self_id, ty, Vec::new())
}

// ── signature-surface JSON ──────────────────────────────────────────────────

fn fields_json(pkg: &Package, self_id: &str, fields: &[lf::FieldWithType]) -> Value {
    Value::Array(
        fields
            .iter()
            .map(|f| {
                let label = interned_str(pkg, f.field_interned_str).expect("field label");
                let ty = render_type(pkg, self_id, f.r#type.as_ref().expect("field type"));
                json!([label, ty])
            })
            .collect(),
    )
}

fn data_type_json(pkg: &Package, self_id: &str, dt: &DefDataType) -> Option<Value> {
    // Serializable types only: that is the surface codegen consumes, and the
    // filter drops interface markers + internal helpers on both sides equally.
    if !dt.serializable {
        return None;
    }
    let name = interned_dotted_name(pkg, dt.name_interned_dname).expect("data type name");
    let params: Vec<Value> = dt
        .params
        .iter()
        .map(|p| {
            Value::String(
                interned_str(pkg, p.var_interned_str)
                    .expect("type param")
                    .to_string(),
            )
        })
        .collect();
    let (kind, body) = match dt.data_cons.as_ref()? {
        lf::def_data_type::DataCons::Record(fields) => (
            "record",
            json!({"fields": fields_json(pkg, self_id, &fields.fields)}),
        ),
        lf::def_data_type::DataCons::Variant(constructors) => (
            "variant",
            json!({"constructors": fields_json(pkg, self_id, &constructors.fields)}),
        ),
        lf::def_data_type::DataCons::Enum(constructors) => (
            "enum",
            json!({
                "constructors": constructors
                    .constructors_interned_str
                    .iter()
                    .map(|&c| interned_str(pkg, c).expect("enum constructor").to_string())
                    .collect::<Vec<_>>()
            }),
        ),
        lf::def_data_type::DataCons::Interface(_) => return None, // marker; never serializable
    };
    let mut object = json!({"name": name, "kind": kind, "params": params});
    object
        .as_object_mut()
        .unwrap()
        .extend(body.as_object().unwrap().clone());
    Some(object)
}

fn choices_json(pkg: &Package, self_id: &str, choices: &[TemplateChoice]) -> Value {
    Value::Array(
        choices
            .iter()
            .map(|c| {
                let arg = c
                    .arg_binder
                    .as_ref()
                    .and_then(|b| b.r#type.as_ref())
                    .map(|t| render_type(pkg, self_id, t))
                    .expect("choice argument type");
                let ret = c
                    .ret_type
                    .as_ref()
                    .map(|t| render_type(pkg, self_id, t))
                    .expect("choice return type");
                json!({
                    "name": interned_str(pkg, c.name_interned_str).expect("choice name"),
                    "consuming": c.consuming,
                    "arg": arg,
                    "ret": ret,
                })
            })
            .collect(),
    )
}

fn template_json(pkg: &Package, self_id: &str, template: &DefTemplate) -> Value {
    let name = interned_dotted_name(pkg, template.tycon_interned_dname).expect("template name");
    let key = template
        .key
        .as_ref()
        .map(|k| render_type(pkg, self_id, k.r#type.as_ref().expect("key type")));
    let mut implements: Vec<String> = template
        .implements
        .iter()
        .map(|i| {
            let id = i.interface.as_ref().expect("implemented interface id");
            render_ref(
                pkg,
                self_id,
                id.module.as_ref().expect("interface module"),
                id.name_interned_dname,
            )
        })
        .collect();
    implements.sort();
    json!({
        "name": name,
        "key": key,
        "implements": implements,
        "choices": choices_json(pkg, self_id, &template.choices),
    })
}

fn interface_json(pkg: &Package, self_id: &str, interface: &DefInterface) -> Value {
    let name = interned_dotted_name(pkg, interface.tycon_interned_dname).expect("interface name");
    let view = interface
        .view
        .as_ref()
        .map(|v| render_type(pkg, self_id, v));
    json!({
        "name": name,
        "view": view,
        "choices": choices_json(pkg, self_id, &interface.choices),
    })
}

/// The whole DAR's signature surface: every package, every module that has any
/// serializable data type, template, or interface.
fn signature_json(packages: &[(String, Package)]) -> Value {
    let rendered: Vec<Value> = packages
        .iter()
        .map(|(id, pkg)| {
            let modules: Vec<Value> = pkg
                .modules
                .iter()
                .filter_map(|module| {
                    let name =
                        interned_dotted_name(pkg, module.name_interned_dname).expect("module name");
                    let data_types: Vec<Value> = module
                        .data_types
                        .iter()
                        .filter_map(|dt| data_type_json(pkg, id, dt))
                        .collect();
                    let templates: Vec<Value> = module
                        .templates
                        .iter()
                        .map(|t| template_json(pkg, id, t))
                        .collect();
                    let interfaces: Vec<Value> = module
                        .interfaces
                        .iter()
                        .map(|i| interface_json(pkg, id, i))
                        .collect();
                    if data_types.is_empty() && templates.is_empty() && interfaces.is_empty() {
                        return None; // value-only modules carry no signature surface
                    }
                    Some(json!({
                        "name": name,
                        "dataTypes": data_types,
                        "templates": templates,
                        "interfaces": interfaces,
                    }))
                })
                .collect();
            json!({
                "id": id,
                "name": canton_lf::package_name(pkg),
                "version": canton_lf::package_version(pkg),
                "modules": modules,
            })
        })
        .collect();
    json!({ "packages": rendered })
}

// ── canonicalization + diff ─────────────────────────────────────────────────

/// Sort every name-keyed array so both sides compare order-independently.
/// (Field/constructor order inside a type is declaration order — significant —
/// and is left untouched.)
fn canonicalize(value: &mut Value) {
    match value {
        Value::Object(map) => {
            for (key, child) in map.iter_mut() {
                canonicalize(child);
                if let (
                    "packages" | "modules" | "dataTypes" | "templates" | "interfaces" | "choices",
                    Value::Array(items),
                ) = (key.as_str(), &mut *child)
                {
                    items.sort_by_key(|item| {
                        item.get("name")
                            .or_else(|| item.get("id"))
                            .and_then(Value::as_str)
                            .unwrap_or_default()
                            .to_string()
                    });
                }
            }
        }
        Value::Array(items) => items.iter_mut().for_each(canonicalize),
        _ => {}
    }
}

/// The JSON-pointer path of the first difference between two values, if any.
fn first_diff(a: &Value, b: &Value, path: &str) -> Option<String> {
    match (a, b) {
        (Value::Object(ma), Value::Object(mb)) => {
            for key in ma.keys().chain(mb.keys()) {
                match (ma.get(key), mb.get(key)) {
                    (Some(va), Some(vb)) => {
                        if let Some(diff) = first_diff(va, vb, &format!("{path}/{key}")) {
                            return Some(diff);
                        }
                    }
                    _ => return Some(format!("{path}/{key} (present on one side only)")),
                }
            }
            None
        }
        (Value::Array(xs), Value::Array(ys)) => {
            if xs.len() != ys.len() {
                return Some(format!(
                    "{path} (array lengths {} vs {})",
                    xs.len(),
                    ys.len()
                ));
            }
            xs.iter()
                .zip(ys)
                .enumerate()
                .find_map(|(i, (x, y))| first_diff(x, y, &format!("{path}/{i}")))
        }
        _ => (a != b).then(|| format!("{path}: ours={a} theirs={b}")),
    }
}

// ── the test ────────────────────────────────────────────────────────────────

#[test]
fn decoder_matches_the_official_jvm_reader() {
    let Ok(dar_path) = std::env::var("CANTON_LF_ORACLE_DAR") else {
        eprintln!("skipping oracle: set CANTON_LF_ORACLE_DAR=/path/to/x.dar");
        return;
    };
    // Cargo runs an integration test with the *crate* directory as its working
    // directory, so a relative path typed at the repo root would not resolve.
    let dar_path = {
        let path = std::path::PathBuf::from(&dar_path);
        if path.is_absolute() {
            path
        } else {
            std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../..")).join(path)
        }
    };
    if std::process::Command::new("scala-cli")
        .arg("--version")
        .output()
        .is_err()
    {
        eprintln!("skipping oracle: scala-cli not on PATH (needed for the JVM reader)");
        return;
    }

    // Ours: decode natively and render the signature surface.
    let dar = Dar::open(&dar_path).expect("open DAR");
    let packages = decode_all(&dar).expect("decode all packages");
    let mut ours = signature_json(&packages);

    // Theirs: the official daml-lf-archive reader, via the scala-cli helper.
    let helper = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../tools/lf-oracle/LfOracle.scala"
    );
    let output = std::process::Command::new("scala-cli")
        .args(["run", "--quiet", helper, "--"])
        .arg(&dar_path)
        .output()
        .expect("run scala-cli");
    assert!(
        output.status.success(),
        "oracle helper failed:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let mut theirs: Value = serde_json::from_slice(&output.stdout).expect("oracle output is JSON");

    canonicalize(&mut ours);
    canonicalize(&mut theirs);

    if ours != theirs {
        let dir = std::env::temp_dir();
        std::fs::write(
            dir.join("lf_oracle_ours.json"),
            serde_json::to_string_pretty(&ours).unwrap(),
        )
        .ok();
        std::fs::write(
            dir.join("lf_oracle_theirs.json"),
            serde_json::to_string_pretty(&theirs).unwrap(),
        )
        .ok();
        panic!(
            "decoder disagrees with the official daml-lf-archive reader\n\
             first difference: {}\n\
             full documents: {}/lf_oracle_{{ours,theirs}}.json",
            first_diff(&ours, &theirs, "").unwrap_or_default(),
            dir.display(),
        );
    }

    let n_packages = packages.len();
    println!("oracle agreement: {n_packages} package(s) match the official reader");
}
