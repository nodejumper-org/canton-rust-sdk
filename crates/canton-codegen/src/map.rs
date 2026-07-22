//! The Daml-LF → Rust **type mapping** (the documented M2 deliverable, as code).
//!
//! Container and primitive types map to `std` / runtime types; references map to
//! the generated type's name. The runtime types (`Party`, `ContractId`,
//! `Numeric`, `Timestamp`, `Date`, `TextMap`, `GenMap`) are provided by a small
//! runtime crate the generated code depends on, reached here through the `rt`
//! path — a placeholder until Phase C stands that crate up. The mapping itself
//! is decoder-independent, so it is stable regardless of the LF-decoder choice.

use proc_macro2::TokenStream;
use quote::quote;

use crate::emit::{type_path, type_var_ident};
use crate::ir::{DamlType, TypeRef};

/// Map a [`DamlType`] to the Rust type that represents it in generated code.
#[must_use]
pub fn rust_type(ty: &DamlType) -> TokenStream {
    match ty {
        DamlType::Unit => quote!(rt::Unit),
        DamlType::Bool => quote!(bool),
        DamlType::Int64 => quote!(i64),
        DamlType::Numeric(_) => quote!(rt::Numeric),
        DamlType::Text => quote!(String),
        DamlType::Timestamp => quote!(rt::Timestamp),
        DamlType::Date => quote!(rt::Date),
        DamlType::Party => quote!(rt::Party),
        DamlType::ContractId(inner) => {
            let inner = rust_type(inner);
            quote!(rt::ContractId<#inner>)
        }
        DamlType::List(inner) => {
            let inner = rust_type(inner);
            quote!(Vec<#inner>)
        }
        DamlType::Optional(inner) => {
            // The top-level Optional maps to `Option`; any Optional directly
            // nested inside it maps to `rt::NestedOpt`, so the JSON codec uses
            // the LF-JSON nested-optional list form (see `rt::NestedOpt`).
            let inner = nested_optional_inner(inner);
            quote!(Option<#inner>)
        }
        DamlType::TextMap(inner) => {
            let inner = rust_type(inner);
            quote!(rt::TextMap<#inner>)
        }
        DamlType::GenMap(key, value) => {
            let key = rust_type(key);
            let value = rust_type(value);
            quote!(rt::GenMap<#key, #value>)
        }
        DamlType::Ref(reference) => rust_ref(reference),
        DamlType::Var(name) => {
            let ident = type_var_ident(name);
            quote!(#ident)
        }
        DamlType::Boxed(inner) => {
            let inner = rust_type(inner);
            quote!(Box<#inner>)
        }
    }
}

/// Map the inner type of an `Optional`, wrapping any further nested `Optional`
/// layers in `rt::NestedOpt` (their JSON encoding is the list form). A
/// non-optional inner type maps normally.
fn nested_optional_inner(ty: &DamlType) -> TokenStream {
    match ty {
        DamlType::Optional(inner) => {
            let inner = nested_optional_inner(inner);
            quote!(rt::NestedOpt<#inner>)
        }
        other => rust_type(other),
    }
}

/// A reference to a named data type, applying any type arguments.
fn rust_ref(reference: &TypeRef) -> TokenStream {
    let path = type_path(&reference.path);
    if reference.args.is_empty() {
        path
    } else {
        let args = reference.args.iter().map(rust_type);
        quote!(#path<#(#args),*>)
    }
}
