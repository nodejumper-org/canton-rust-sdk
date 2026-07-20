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
//! The generator ([`gen`]) and the type mapping ([`map`]) consume the IR and
//! never touch Daml-LF, so the LF-decoder choice (JVM `daml-lf-archive` vs a
//! native Rust decoder) is isolated to the decoder module (Phase B).
//!
//! Current status: Phase A — IR, the Daml-LF → Rust type mapping, and record
//! emission, with a test that the generated source is valid Rust.

pub mod emit;
pub mod ir;
pub mod map;

use crate::ir::Record;

/// Generate formatted Rust source for a single record data type.
///
/// # Errors
/// Returns a [`syn::Error`] if the generated tokens are not valid Rust — that
/// would be a generator bug, so callers can treat it as such.
pub fn generate_record(record: &Record) -> Result<String, syn::Error> {
    let tokens = emit::record_struct(record);
    let file: syn::File = syn::parse2(tokens)?;
    Ok(prettyplease::unparse(&file))
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
                    DamlType::ContractId(Box::new(DamlType::Ref(TypeRef {
                        name: "Foo".to_string(),
                        args: Vec::new(),
                    }))),
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
