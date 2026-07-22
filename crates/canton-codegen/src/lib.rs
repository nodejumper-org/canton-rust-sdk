//! `canton-codegen` — generate typed Rust bindings from Daml packages.
//!
//! **Milestone 2, work in progress.** This crate turns a Daml package into
//! idiomatic Rust: templates and records into structs, choices into typed
//! exercise builders, with JSON and gRPC codecs on the generated types.
//!
//! # Architecture
//!
//! A decoder-agnostic [`ir`] (intermediate representation) sits between decoding
//! Daml-LF and emitting Rust:
//!
//! ```text
//! DAR ──(decoder)──▶ ir ──(this crate)──▶ Rust source ──▶ crate
//! ```
//!
//! The generator ([`emit`]) and the type mapping ([`map`]) consume the IR and
//! never touch Daml-LF, so the LF-decoder choice (JVM `daml-lf-archive` vs a
//! native Rust decoder) is isolated to the decoder module (Phase B).
//!
//! Current status: Phase A — the IR, the Daml-LF → Rust type mapping, and
//! emission of records, variants, enums, and templates (with typed choice
//! impls); every generator verifies its output is valid Rust.

pub mod emit;
pub mod ir;
pub mod lower;
pub mod map;

pub use lower::{LowerError, lower_crate, lower_dar, lower_package};

use proc_macro2::TokenStream;

use crate::ir::{Crate, DataType, Module, Record, Template};

/// The header prepended to every generated module: lints appropriate for
/// generated code, plus the `use canton_daml as rt;` alias the emitted `rt::…`
/// references resolve through.
const MODULE_PREAMBLE: &str = concat!(
    "#![allow(non_camel_case_types, non_snake_case, unused_imports, dead_code, unused_variables, clippy::all)]\n",
    "//! Generated Daml bindings — do not edit by hand.\n\n",
    "use canton_daml as rt;\n\n",
);

/// The header prepended to a whole generated crate (its `lib.rs`): the same
/// lints, but no top-level `rt` alias — each generated module aliases the
/// runtime itself, so references resolve within their own module.
const CRATE_PREAMBLE: &str = concat!(
    "#![allow(non_camel_case_types, non_snake_case, unused_imports, dead_code, unused_variables, clippy::all)]\n",
    "//! Generated Daml bindings — do not edit by hand.\n\n",
);

/// Format a stream of generated items as Rust source, first checking it is
/// syntactically valid Rust.
///
/// # Errors
/// Returns a [`syn::Error`] if the tokens are not valid Rust — a generator bug.
fn format_items(tokens: TokenStream) -> Result<String, syn::Error> {
    let file: syn::File = syn::parse2(tokens)?;
    Ok(prettyplease::unparse(&file))
}

/// Generate formatted Rust source for a single record data type.
///
/// # Errors
/// Returns a [`syn::Error`] if the generated tokens are not valid Rust.
pub fn generate_record(record: &Record) -> Result<String, syn::Error> {
    format_items(emit::record_items(record))
}

/// Generate formatted Rust source for a named data type (record / variant / enum).
///
/// # Errors
/// Returns a [`syn::Error`] if the generated tokens are not valid Rust.
pub fn generate_data_type(data_type: &DataType) -> Result<String, syn::Error> {
    format_items(emit::data_type(data_type))
}

/// Generate formatted Rust source for a template: its payload struct plus the
/// typed choice impls.
///
/// # Errors
/// Returns a [`syn::Error`] if the generated tokens are not valid Rust.
pub fn generate_template(template: &Template) -> Result<String, syn::Error> {
    format_items(emit::template(template))
}

/// Generate a complete module file: the `use canton_daml as rt;` preamble plus
/// every data type and template, ready to write to a `.rs` file in a generated
/// crate.
///
/// # Errors
/// Returns a [`syn::Error`] if the generated tokens are not valid Rust.
pub fn generate_module(module: &Module) -> Result<String, syn::Error> {
    let body = format_items(emit::module_items(module))?;
    Ok(format!("{MODULE_PREAMBLE}{body}"))
}

/// Generate a whole crate's `lib.rs` from a lowered DAR: a `pub mod` tree
/// (package → module → types) where every cross-module and cross-package
/// reference resolves through its qualified path.
///
/// # Errors
/// Returns a [`syn::Error`] if the generated tokens are not valid Rust.
pub fn generate_crate(krate: &Crate) -> Result<String, syn::Error> {
    let body = format_items(emit::crate_items(krate))?;
    Ok(format!("{CRATE_PREAMBLE}{body}"))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::ir::{DamlType, Field, Record, TypeRef};

    fn field(label: &str, ty: DamlType) -> Field {
        Field {
            label: label.to_string(),
            ty,
        }
    }

    #[test]
    fn generates_valid_rust_for_a_record() {
        let record = Record {
            name: "AppInstallRequest".to_string(),
            type_params: Vec::new(),
            fields: vec![
                field("provider", DamlType::Party),
                field("installId", DamlType::Text),
                field("amount", DamlType::Numeric(10)),
                field(
                    "cid",
                    DamlType::ContractId(Box::new(DamlType::Ref(TypeRef::local(
                        "Foo",
                        Vec::new(),
                    )))),
                ),
                field("tags", DamlType::List(Box::new(DamlType::Text))),
                field("note", DamlType::Optional(Box::new(DamlType::Text))),
                // A Daml field whose name collides with a Rust keyword.
                field("type", DamlType::Text),
            ],
        };

        let src = generate_record(&record).expect("generator emits valid Rust");
        // The strongest guarantee: the output parses as a Rust file.
        syn::parse_file(&src).expect("output must be valid Rust");

        // Spot-check the type mapping, snake_case, and keyword escaping.
        assert!(src.contains("pub struct AppInstallRequest"), "{src}");
        assert!(src.contains("pub provider: rt::Party"), "{src}");
        assert!(src.contains("pub install_id: String"), "{src}");
        assert!(src.contains("pub amount: rt::Numeric"), "{src}");
        assert!(src.contains("rt::ContractId<Foo>"), "{src}");
        assert!(src.contains("Vec<String>"), "{src}");
        assert!(src.contains("Option<String>"), "{src}");
        assert!(src.contains("r#type: String"), "{src}");
    }

    #[test]
    fn record_emits_value_codecs() {
        let record = Record {
            name: "Payload".to_string(),
            type_params: Vec::new(),
            fields: vec![
                field("owner", DamlType::Party),
                field("count", DamlType::Int64),
            ],
        };

        let src = generate_record(&record).unwrap();
        syn::parse_file(&src).unwrap();
        assert!(src.contains("impl rt::ToValue for Payload"), "{src}");
        assert!(src.contains("impl rt::FromValue for Payload"), "{src}");
        assert!(src.contains("rt::record("), "{src}");
        assert!(src.contains("rt::record_field(value, \"owner\")"), "{src}");
    }

    #[test]
    fn nested_optional_maps_to_nested_opt() {
        let record = Record {
            name: "R".to_string(),
            type_params: Vec::new(),
            fields: vec![
                field("single", DamlType::Optional(Box::new(DamlType::Text))),
                field(
                    "nested",
                    DamlType::Optional(Box::new(DamlType::Optional(Box::new(DamlType::Text)))),
                ),
            ],
        };

        let src = generate_record(&record).unwrap();
        syn::parse_file(&src).unwrap();
        // A single Optional stays a plain Option.
        assert!(src.contains("pub single: Option<String>"), "{src}");
        // A nested Optional wraps the inner layer in rt::NestedOpt (list JSON).
        assert!(
            src.contains("pub nested: Option<rt::NestedOpt<String>>"),
            "{src}"
        );
    }

    #[test]
    fn record_derives_the_json_codec() {
        let record = Record {
            name: "Payload".to_string(),
            type_params: Vec::new(),
            fields: vec![field("installId", DamlType::Text)],
        };

        let src = generate_record(&record).unwrap();
        syn::parse_file(&src).unwrap();
        assert!(src.contains("rt::serde::Serialize"), "{src}");
        assert!(src.contains("rt::serde::Deserialize"), "{src}");
        assert!(src.contains("serde(crate = \"rt::serde\")"), "{src}");
        // The JSON key is the Daml label, not the snake_cased Rust field.
        assert!(src.contains("serde(rename = \"installId\")"), "{src}");
        assert!(src.contains("pub install_id: String"), "{src}");
    }

    #[test]
    fn variant_generates_a_rust_enum() {
        use crate::ir::{Variant, VariantConstructor};

        let variant = Variant {
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
        };

        let src = generate_data_type(&DataType::Variant(variant)).unwrap();
        syn::parse_file(&src).unwrap();
        assert!(src.contains("pub enum Shape"), "{src}");
        assert!(src.contains("Circle(rt::Numeric)"), "{src}");
        // A nullary constructor carries `rt::Unit` (LF-JSON `{}`), not nothing —
        // so its wire form is `{"tag":"Point","value":{}}`, which the Ledger API
        // sends and a bare unit variant would fail to parse.
        assert!(src.contains("Point(rt::Unit)"), "{src}");
    }

    #[test]
    fn enum_generates_a_c_like_enum() {
        use crate::ir::Enum;

        let enumeration = Enum {
            name: "DayOfWeek".to_string(),
            constructors: vec!["Monday".to_string(), "Tuesday".to_string()],
        };

        let src = generate_data_type(&DataType::Enum(enumeration)).unwrap();
        syn::parse_file(&src).unwrap();
        assert!(src.contains("pub enum DayOfWeek"), "{src}");
        assert!(src.contains("Monday,"), "{src}");
        // C-like enums are `Copy`.
        assert!(src.contains("Copy"), "{src}");
    }

    #[test]
    fn template_generates_payload_and_typed_choices() {
        use crate::ir::{Choice, Template, TypeRef};

        let template = Template {
            name: "AppInstall".to_string(),
            fields: vec![field("provider", DamlType::Party)],
            choices: vec![Choice {
                name: "Accept".to_string(),
                consuming: true,
                argument: DamlType::Ref(TypeRef::local("AppInstall_Accept", Vec::new())),
                returns: DamlType::ContractId(Box::new(DamlType::Ref(TypeRef::local(
                    "AppInstalled",
                    Vec::new(),
                )))),
            }],
            key: None,
        };

        let src = generate_template(&template).unwrap();
        syn::parse_file(&src).unwrap();
        // The payload struct…
        assert!(src.contains("pub struct AppInstall"), "{src}");
        assert!(src.contains("pub provider: rt::Party"), "{src}");
        // …and the typed choice impl linking arg → template → return.
        assert!(
            src.contains("impl rt::Choice<AppInstall> for AppInstall_Accept"),
            "{src}"
        );
        assert!(
            src.contains("type Return = rt::ContractId<AppInstalled>"),
            "{src}"
        );
        assert!(
            src.contains("const NAME: &'static str = \"Accept\""),
            "{src}"
        );
        assert!(src.contains("const CONSUMING: bool = true"), "{src}");
    }

    #[test]
    fn module_wraps_items_with_the_runtime_preamble() {
        use crate::ir::{Choice, Enum, Module, Template};

        let module = Module {
            data_types: vec![DataType::Enum(Enum {
                name: "Color".to_string(),
                constructors: vec!["Red".to_string(), "Green".to_string()],
            })],
            templates: vec![Template {
                name: "AppInstall".to_string(),
                fields: vec![field("provider", DamlType::Party)],
                choices: vec![Choice {
                    name: "Accept".to_string(),
                    consuming: true,
                    argument: DamlType::Ref(TypeRef::local("AppInstall_Accept", Vec::new())),
                    returns: DamlType::Unit,
                }],
                key: None,
            }],
            interfaces: Vec::new(),
        };

        let src = generate_module(&module).unwrap();
        // The whole file (preamble + items) is valid Rust.
        syn::parse_file(&src).unwrap();
        assert!(src.contains("use canton_daml as rt;"), "{src}");
        assert!(src.contains("#![allow(non_camel_case_types"), "{src}");
        assert!(src.contains("pub enum Color"), "{src}");
        assert!(src.contains("pub struct AppInstall"), "{src}");
        assert!(
            src.contains("impl rt::Choice<AppInstall> for AppInstall_Accept"),
            "{src}"
        );
    }

    #[test]
    fn generic_record_carries_type_params() {
        let record = Record {
            name: "Pair".to_string(),
            type_params: vec!["a".to_string(), "b".to_string()],
            fields: vec![
                field("fst", DamlType::Var("a".to_string())),
                field("snd", DamlType::Var("b".to_string())),
            ],
        };

        let src = generate_record(&record).unwrap();
        syn::parse_file(&src).unwrap();
        // Lowercase Daml vars are upper-camel-cased to Rust generics.
        assert!(src.contains("pub struct Pair<A, B>"), "{src}");
        assert!(src.contains("pub fst: A"), "{src}");
    }
}
