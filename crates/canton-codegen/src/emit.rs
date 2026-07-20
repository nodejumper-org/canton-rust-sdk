//! Emit Rust source items from the [`crate::ir`] types.

use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::ir::Record;
use crate::map::rust_type;

/// Generate the `struct` for a record data type (also used for template
/// payloads). Field names are snake-cased for Rust; the original Daml label is
/// kept in a doc comment (the serde/`Value` codec that pins the wire name lands
/// in Phase C).
#[must_use]
pub fn record_struct(record: &Record) -> TokenStream {
    let name = type_ident(&record.name);
    let generics = generics(&record.type_params);
    let fields = record.fields.iter().map(|field| {
        let field_name = field_ident(&field.label);
        let ty = rust_type(&field.ty);
        let doc = format!("The Daml `{}` field.", field.label);
        quote! {
            #[doc = #doc]
            pub #field_name: #ty,
        }
    });

    quote! {
        #[derive(Clone, Debug, PartialEq)]
        pub struct #name #generics {
            #(#fields)*
        }
    }
}

/// The generic parameter list `<A, B>` for a type's parameters, or empty tokens
/// when the type is not generic.
fn generics(type_params: &[String]) -> TokenStream {
    if type_params.is_empty() {
        return TokenStream::new();
    }
    let params = type_params.iter().map(|param| type_ident(param));
    quote!(<#(#params),*>)
}

/// A Rust identifier for a type name or type parameter (Daml `PascalCase`; a
/// lowercase type variable like `a` becomes `A` for Rust convention).
#[must_use]
pub fn type_ident(name: &str) -> Ident {
    ident(&name.to_upper_camel_case())
}

/// A Rust identifier for a record field: the Daml label, snake-cased, with Rust
/// keywords escaped so labels like `type` stay valid.
fn field_ident(label: &str) -> Ident {
    ident(&label.to_snake_case())
}

/// Build an [`Ident`], escaping Rust keywords. Most keywords become raw
/// identifiers (`r#type`); the four that cannot be raw are suffixed with `_`.
fn ident(name: &str) -> Ident {
    match name {
        "crate" | "self" | "Self" | "super" => Ident::new(&format!("{name}_"), Span::call_site()),
        _ if is_keyword(name) => Ident::new_raw(name, Span::call_site()),
        _ => Ident::new(name, Span::call_site()),
    }
}

/// Whether `name` is a Rust keyword (strict + reserved) that must be escaped.
fn is_keyword(name: &str) -> bool {
    matches!(
        name,
        "as" | "break"
            | "const"
            | "continue"
            | "crate"
            | "dyn"
            | "else"
            | "enum"
            | "extern"
            | "false"
            | "fn"
            | "for"
            | "if"
            | "impl"
            | "in"
            | "let"
            | "loop"
            | "match"
            | "mod"
            | "move"
            | "mut"
            | "pub"
            | "ref"
            | "return"
            | "self"
            | "Self"
            | "static"
            | "struct"
            | "super"
            | "trait"
            | "true"
            | "type"
            | "unsafe"
            | "use"
            | "where"
            | "while"
            | "async"
            | "await"
            | "abstract"
            | "become"
            | "box"
            | "do"
            | "final"
            | "macro"
            | "override"
            | "priv"
            | "typeof"
            | "unsized"
            | "virtual"
            | "yield"
            | "try"
    )
}
