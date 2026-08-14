//! `canton-codegen` — generate typed Rust bindings from Daml packages.
//!
//! This crate turns a DAR into idiomatic Rust: templates and records into
//! structs, choices into typed exercise impls, interfaces into typed markers
//! with views, with JSON and gRPC codecs on every generated type. The
//! command-line front-end is `canton-codegen-cli` (`dpm codegen-rust`); most
//! users drive codegen through that.
//!
//! # Usage
//!
//! [`lower_dar`] turns a `.dar` into the [`ir`], and [`generate_crate`] turns
//! that into the source of a self-contained bindings crate:
//!
//! ```no_run
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let dar = canton_lf::Dar::open("my-app-0.1.0.dar")?;
//! let (krate, skipped) = canton_codegen::lower_dar(&dar)?;
//! let source = canton_codegen::generate_crate(&krate)?;
//! for skip in &skipped {
//!     eprintln!("warning: skipped {}", skip);
//! }
//! std::fs::write("src/lib.rs", source)?;
//! # Ok(()) }
//! ```
//!
//! # Architecture
//!
//! A decoder-agnostic [`ir`] (intermediate representation) sits between
//! decoding Daml-LF and emitting Rust:
//!
//! ```text
//! DAR ──(canton-lf: decode)──▶ lower ──▶ ir ──(emit/map)──▶ Rust source
//! ```
//!
//! The generator and the type mapping consume the IR and never touch Daml-LF;
//! lowering is the one step that reads the decoded LF AST (from `canton-lf`,
//! which is held to the official `daml-lf-archive` reader by a conformance
//! oracle). Every generator checks its output parses as valid Rust before
//! returning it. The IR is public so a caller can emit something other than
//! Rust from it, or post-process before emission; the emitter and type mapper
//! are private (their signatures are `proc-macro2` token streams, which would
//! otherwise become part of this crate's SemVer surface).
//!
//! The human-readable specification of the type mapping lives in
//! `docs/daml-lf-type-mapping.md`.

pub mod ir;

mod generate;

pub(crate) mod emit;
pub(crate) mod lower;
pub(crate) mod map;

/// The Daml-LF reader this crate is built on, re-exported because its types
/// (`Dar`, `DarError`, `DecodeError`) appear in the signatures below — so a
/// caller never has to add `canton-lf` separately and match its version.
pub use canton_lf;

pub use generate::{GenerateError, Options, Runtime, Stats, default_crate_name, generate};
pub use lower::{
    ExternalPackages, Selection, SkippedType, lower_dar, lower_dar_selecting, lower_dar_with,
    lower_packages_selecting,
};

use proc_macro2::TokenStream;

/// Emitting Rust from the [`ir`] failed.
///
/// This means the generator produced tokens that are not valid Rust — always a
/// bug in `canton-codegen` (or in a hand-built [`ir`]), never bad user input.
/// The parse error is the payload; it is deliberately opaque so the `syn`
/// dependency stays out of this crate's public API.
#[derive(Debug)]
pub struct CodegenError(syn::Error);

impl std::fmt::Display for CodegenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "generated source is not valid Rust: {}", self.0)
    }
}

impl std::error::Error for CodegenError {}

use crate::ir::{Crate, DataType, Module, Record, Template};

/// The header prepended to every generated module: lints appropriate for
/// generated code, plus the `use canton_daml as rt;` alias the emitted `rt::…`
/// references resolve through.
const MODULE_PREAMBLE: &str = concat!(
    "#![allow(non_camel_case_types, non_snake_case, unused_imports, dead_code, unused_variables, clippy::all)]\n",
    "//! Generated Daml bindings — do not edit by hand.\n\n",
    "use canton_daml as rt;\n\n",
);

/// The lint header for a whole generated crate. Daml names are not Rust
/// casing and the emitter deliberately emits some unused glue, so those two
/// lints are silenced; `clippy::all` is off because generated code is not
/// hand-maintained and its style is not the reader's business.
const CRATE_LINTS: &str =
    "#![allow(non_camel_case_types, non_snake_case, unused_imports, clippy::all)]\n";

/// The crate-level docs for a generated crate: what it is, how it is used, and
/// an index of the Daml packages inside it — otherwise a reader landing on
/// docs.rs sees twenty-odd opaque module names and no entry point.
fn crate_docs(krate: &Crate) -> String {
    use std::fmt::Write as _;

    let mut docs = String::new();
    docs.push_str("//! Typed Rust bindings generated from a Daml archive (DAR).\n//!\n");
    docs.push_str(
        "//! **Generated file — do not edit by hand.** Regenerate with\n\
         //! `dpm-codegen-rust --dar <the DAR> --out <this crate>`; edits are lost.\n//!\n\
         //! Each Daml package in the DAR's dependency closure is one top-level\n\
         //! module, and each Daml module a submodule under it, so cross-package\n\
         //! references resolve and names never collide. Templates carry typed\n\
         //! choices; the runtime traits they implement (`Template`, `Choice`,\n\
         //! `Contract`) and the command builders live in `canton-daml`, which the\n\
         //! generated code imports as `rt`.\n//!\n\
         //! # Packages in this crate\n//!\n",
    );
    for package in &krate.packages {
        let modules = package.modules.len();
        let plural = if modules == 1 { "module" } else { "modules" };
        let _ = writeln!(docs, "//! - [`{}`] — {modules} Daml {plural}", package.name);
    }
    docs.push('\n');
    docs
}

/// Format a stream of generated items as Rust source, first checking it is
/// syntactically valid Rust.
///
/// # Errors
/// Returns [`CodegenError`] if the tokens are not valid Rust — a generator bug.
fn format_items(tokens: TokenStream) -> Result<String, CodegenError> {
    let file: syn::File = syn::parse2(tokens).map_err(CodegenError)?;
    Ok(prettyplease::unparse(&file))
}

/// Generate formatted Rust source for a single record data type.
///
/// # Errors
/// Returns [`CodegenError`] if the generated tokens are not valid Rust.
pub fn generate_record(record: &Record) -> Result<String, CodegenError> {
    format_items(emit::record_items(record))
}

/// Generate formatted Rust source for a named data type (record / variant / enum).
///
/// # Errors
/// Returns [`CodegenError`] if the generated tokens are not valid Rust.
pub fn generate_data_type(data_type: &DataType) -> Result<String, CodegenError> {
    format_items(emit::data_type(data_type))
}

/// Generate formatted Rust source for a template: its payload struct plus the
/// typed choice impls.
///
/// # Errors
/// Returns [`CodegenError`] if the generated tokens are not valid Rust.
pub fn generate_template(template: &Template) -> Result<String, CodegenError> {
    format_items(emit::template(template))
}

/// Generate formatted Rust source for an interface's impls (its `Contract` /
/// `Interface` identity and typed choice impls) on its marker type.
///
/// # Errors
/// Returns [`CodegenError`] if the generated tokens are not valid Rust.
pub fn generate_interface(interface: &crate::ir::Interface) -> Result<String, CodegenError> {
    format_items(emit::interface(interface))
}

/// Generate a complete module file: the `use canton_daml as rt;` preamble plus
/// every data type and template, ready to write to a `.rs` file in a generated
/// crate.
///
/// # Errors
/// Returns [`CodegenError`] if the generated tokens are not valid Rust.
pub fn generate_module(module: &Module) -> Result<String, CodegenError> {
    let body = format_items(emit::module_items(module))?;
    Ok(format!("{MODULE_PREAMBLE}{body}"))
}

/// Generate a whole crate's `lib.rs` from a lowered DAR: a `pub mod` tree
/// (package → module → types) where every cross-module and cross-package
/// reference resolves through its qualified path.
///
/// # Errors
/// Returns [`CodegenError`] if the generated tokens are not valid Rust.
pub fn generate_crate(krate: &Crate) -> Result<String, CodegenError> {
    let body = format_items(emit::crate_items(krate))?;
    Ok(format!("{CRATE_LINTS}{}{body}", crate_docs(krate)))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use crate::ir::{DamlType, Field, Record, TypeRef, Variant, VariantConstructor};

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
        assert!(
            src.contains("pub install_id: ::std::string::String"),
            "{src}"
        );
        assert!(src.contains("pub amount: rt::Numeric"), "{src}");
        assert!(src.contains("rt::ContractId<Foo>"), "{src}");
        assert!(src.contains("Vec<::std::string::String>"), "{src}");
        assert!(src.contains("Option<::std::string::String>"), "{src}");
        assert!(src.contains("r#type: ::std::string::String"), "{src}");
    }

    #[test]
    fn only_a_renamed_field_carries_a_doc() {
        let record = Record {
            name: "Payload".to_string(),
            type_params: Vec::new(),
            fields: vec![
                field("owner", DamlType::Party),
                field("acceptArg", DamlType::Text),
                field("type", DamlType::Text),
            ],
        };

        let src = generate_record(&record).unwrap();
        syn::parse_file(&src).unwrap();
        // Camel-cased on the wire, snake_cased in Rust: the doc is the only
        // place a reader sees the label they have to send.
        assert!(src.contains("Daml field `acceptArg`"), "{src}");
        // Same spelling in both languages — a doc here would only restate the
        // identifier under it, on every field of every record.
        assert!(!src.contains("Daml field `owner`"), "{src}");
        // A raw identifier is the same name, so it is not a rename either.
        assert!(src.contains("r#type"), "{src}");
        assert!(!src.contains("Daml field `type`"), "{src}");
    }

    #[test]
    fn generated_decoders_attach_the_field_name_to_a_failure() {
        // Without this a mismatch three records down arrives as a bare
        // "expected Text" with nothing to locate it by. `ValueError::at`
        // prepends, so each layer contributes its own label on the way up.
        let record = Record {
            name: "Payload".to_string(),
            type_params: Vec::new(),
            fields: vec![
                field("owner", DamlType::Party),
                field("note", DamlType::Optional(Box::new(DamlType::Text))),
            ],
        };
        let src = generate_record(&record).unwrap();
        syn::parse_file(&src).unwrap();

        // Both the required and the optional path carry it.
        assert!(src.contains(r#".map_err(|e| e.at("owner"))"#), "{src}");
        assert!(src.contains(r#".map_err(|e| e.at("note"))"#), "{src}");
        // …and it wraps the *decode*, not the lookup: `required_field` already
        // names the field it could not find, so wrapping that too would say it
        // twice.
        assert!(
            src.contains(r#"rt::required_field(value, 0usize, "owner")?)"#),
            "{src}"
        );
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
        // Fields decode by label *and* declaration index, so non-verbose
        // (label-less) records bind positionally.
        assert!(
            src.contains("rt::required_field(value, 0usize, \"owner\")"),
            "{src}"
        );
    }

    #[test]
    fn optional_fields_decode_as_absent_tolerant() {
        // Canton normalizes records by dropping trailing empty-Optional fields;
        // an Optional field must therefore decode through `optional_field` (an
        // absent field is `None`), while required fields stay strict.
        let record = Record {
            name: "Payload".to_string(),
            type_params: Vec::new(),
            fields: vec![
                field("owner", DamlType::Party),
                field("note", DamlType::Optional(Box::new(DamlType::Text))),
            ],
        };

        let src = generate_record(&record).unwrap();
        syn::parse_file(&src).unwrap();
        assert!(
            src.contains("rt::optional_field(value, 1usize, \"note\")"),
            "{src}"
        );
        assert!(
            src.contains("rt::required_field(value, 0usize, \"owner\")"),
            "{src}"
        );
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
        assert!(
            src.contains("pub single: ::core::option::Option<::std::string::String>"),
            "{src}"
        );
        // A nested Optional wraps the inner layer in rt::NestedOpt (list JSON).
        assert!(
            src.contains(
                "pub nested: ::core::option::Option<rt::NestedOpt<::std::string::String>>"
            ),
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
        assert!(
            src.contains("pub install_id: ::std::string::String"),
            "{src}"
        );
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
            module_name: "Licensing.AppInstall".to_string(),
            package_id: "abc123".to_string(),
            package_name: "app-install".to_string(),
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
        // …and the on-ledger template identity.
        assert!(src.contains("impl rt::Template for AppInstall"), "{src}");
        assert!(
            src.contains("const MODULE_NAME: &'static str = \"Licensing.AppInstall\""),
            "{src}"
        );
        assert!(
            src.contains("const ENTITY_NAME: &'static str = \"AppInstall\""),
            "{src}"
        );
        assert!(
            src.contains("const PACKAGE_NAME: &'static str = \"app-install\""),
            "{src}"
        );
    }

    #[test]
    fn a_template_documents_its_identity_and_its_choices() {
        use crate::ir::{Choice, Template, TypeRef};

        let template = Template {
            name: "AppInstall".to_string(),
            module_name: "Licensing.AppInstall".to_string(),
            package_id: "abc123".to_string(),
            package_name: "app-install".to_string(),
            fields: vec![field("provider", DamlType::Party)],
            choices: vec![
                Choice {
                    name: "Accept".to_string(),
                    consuming: true,
                    argument: DamlType::Ref(TypeRef::local("AppInstall_Accept", Vec::new())),
                    returns: DamlType::Unit,
                },
                Choice {
                    name: "Peek".to_string(),
                    consuming: false,
                    argument: DamlType::Ref(TypeRef::local("AppInstall_Peek", Vec::new())),
                    returns: DamlType::Unit,
                },
            ],
            key: None,
        };

        let src = generate_template(&template).unwrap();
        syn::parse_file(&src).unwrap();
        // rustdoc has no reverse index for `impl Choice<This> for That`, so the
        // payload struct is the only place a reader can learn what it can do.
        assert!(
            src.contains("The Daml template `Licensing.AppInstall:AppInstall`"),
            "{src}"
        );
        assert!(
            src.contains("`#app-install:Licensing.AppInstall:AppInstall`"),
            "{src}"
        );
        assert!(src.contains("- `Accept` — consuming"), "{src}");
        assert!(src.contains("- `Peek` — non-consuming"), "{src}");
        // One `#[doc]` per line, never a block comment: prettyplease indents a
        // `/** */` continuation line to match the item, and rustdoc reads four
        // spaces as a code block — which makes the prose a failing doctest in
        // the user's crate.
        assert!(!src.contains("/**"), "{src}");
    }

    #[test]
    fn keyed_template_emits_with_key_impl() {
        use crate::ir::{Template, TypeRef};

        let template = Template {
            name: "Account".to_string(),
            module_name: "Bank.Account".to_string(),
            package_id: "abc123".to_string(),
            package_name: "bank".to_string(),
            fields: vec![field("owner", DamlType::Party)],
            choices: Vec::new(),
            // A record key (owner + number) referenced by name.
            key: Some(DamlType::Ref(TypeRef::local("AccountKey", Vec::new()))),
        };

        let src = generate_template(&template).unwrap();
        syn::parse_file(&src).unwrap();
        assert!(src.contains("impl rt::WithKey for Account"), "{src}");
        assert!(src.contains("type Key = AccountKey"), "{src}");
    }

    #[test]
    fn interface_emits_identity_view_and_choice_impls() {
        use crate::ir::{Choice, Interface, TypeRef};

        let interface = Interface {
            name: "Holding".to_string(),
            module_name: "Splice.Api.Token.HoldingV1".to_string(),
            package_id: "abc123".to_string(),
            package_name: "splice-api-token-holding".to_string(),
            view: Some(DamlType::Ref(TypeRef::local("HoldingView", Vec::new()))),
            choices: vec![Choice {
                name: "Transfer".to_string(),
                consuming: true,
                argument: DamlType::Ref(TypeRef::local("Holding_Transfer", Vec::new())),
                returns: DamlType::Unit,
            }],
        };

        let src = generate_interface(&interface).unwrap();
        syn::parse_file(&src).unwrap();
        // Identity (Contract) + the interface's view type…
        assert!(src.contains("impl rt::Contract for Holding"), "{src}");
        assert!(src.contains("impl rt::Interface for Holding"), "{src}");
        assert!(src.contains("type View = HoldingView"), "{src}");
        // …and the choice, exercisable through the interface marker.
        assert!(
            src.contains("impl rt::Choice<Holding> for Holding_Transfer"),
            "{src}"
        );
    }

    #[test]
    fn keyless_template_emits_no_with_key_impl() {
        use crate::ir::Template;

        let template = Template {
            name: "Note".to_string(),
            module_name: "M".to_string(),
            package_id: "p".to_string(),
            package_name: "m".to_string(),
            fields: vec![field("text", DamlType::Text)],
            choices: Vec::new(),
            key: None,
        };

        let src = generate_template(&template).unwrap();
        syn::parse_file(&src).unwrap();
        assert!(
            !src.contains("WithKey"),
            "keyless template has no key impl: {src}"
        );
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
                module_name: "Licensing.AppInstall".to_string(),
                package_id: "abc123".to_string(),
                package_name: "app-install".to_string(),
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
    /// Every keyword the escape list claims to cover, checked by generating a
    /// field named after it and looking for the escape.
    ///
    /// The list is only worth what it is complete for, and completeness is
    /// exactly what reading it cannot establish — `gen` was missing until this
    /// went looking. Generated crates declare edition 2021, where `gen` is an
    /// ordinary identifier, so nothing would have failed until the same file
    /// was compiled in a 2024 crate; `r#gen` is valid in both.
    #[test]
    fn every_keyword_in_the_list_is_escaped_in_a_field_name() {
        // Spot-check across the three shapes the escape takes: raw identifiers,
        // the four that cannot be raw and get a `_` suffix, and a 2024 addition.
        for (daml, expected) in [
            ("type", "r#type:"),
            ("match", "r#match:"),
            ("gen", "r#gen:"),
            ("try", "r#try:"),
            ("async", "r#async:"),
            ("box", "r#box:"),
            ("crate", "crate_:"),
            ("self", "self_:"),
        ] {
            let record = Record {
                name: "Keyworded".to_string(),
                type_params: Vec::new(),
                fields: vec![field(daml, DamlType::Text)],
            };
            let src = generate_record(&record).expect("generator emits valid Rust");
            assert!(
                src.contains(expected),
                "a Daml field named `{daml}` should emit `{expected}`:\n{src}"
            );
            // And the wire label survives the escaping, or the field would be
            // written to the ledger under the Rust spelling.
            assert!(
                src.contains(&format!("#[serde(rename = \"{daml}\")]")),
                "the wire label `{daml}` must survive:\n{src}"
            );
        }
    }
    /// A Daml type parameter that no field uses must not be forced to carry a
    /// codec.
    ///
    /// Daml permits a phantom parameter, and the natural thing to instantiate
    /// one with is an interface marker — which by design has no codec at all,
    /// because it exists only as the tag of a `ContractId`. Bounding every
    /// declared parameter therefore emitted Rust that is valid until somebody
    /// writes `Wrapper Holding`, and then does not compile, with the error
    /// landing in generated code the reader did not write.
    #[test]
    fn a_phantom_type_parameter_is_not_forced_to_carry_a_codec() {
        let mut record = Record::new("Wrapper");
        record.type_params = vec!["a".to_string(), "b".to_string()];
        // Only `b` is used by a field; `a` is phantom.
        record.fields = vec![Field::new("value", DamlType::Var("b".to_string()))];

        let src = generate_data_type(&DataType::Record(record)).unwrap();
        assert!(
            src.contains("B: rt::ToValue"),
            "the used parameter is bounded: {src}"
        );
        assert!(
            !src.contains("A: rt::ToValue"),
            "the phantom parameter must not be bounded: {src}"
        );
        // It still appears as a parameter of the type, and in the PhantomData.
        assert!(src.contains("pub struct Wrapper<A, B>"), "{src}");
        assert!(src.contains("PhantomData"), "{src}");
    }

    /// The same for a variant: only the parameters a constructor payload
    /// mentions get the bound.
    #[test]
    fn a_variants_phantom_parameter_is_not_bounded_either() {
        let mut variant = Variant::new("Choice");
        variant.type_params = vec!["a".to_string(), "b".to_string()];
        variant.constructors = vec![
            VariantConstructor::with_payload("Some", DamlType::Var("b".to_string())),
            VariantConstructor::new("None"),
        ];

        let src = generate_data_type(&DataType::Variant(variant)).unwrap();
        assert!(src.contains("B: rt::ToValue"), "{src}");
        assert!(!src.contains("A: rt::ToValue"), "{src}");
    }
}
