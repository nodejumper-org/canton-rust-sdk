//! Emit Rust source items from the [`crate::ir`] types.

use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::ir::{DataType, Enum, Module, Record, Template, Variant};
use crate::map::rust_type;

/// Emit the Rust item(s) for a named data type (record, variant, or enum).
#[must_use]
pub fn data_type(data_type: &DataType) -> TokenStream {
    match data_type {
        DataType::Record(record) => record_items(record),
        DataType::Variant(variant) => variant_enum(variant),
        DataType::Enum(enumeration) => enum_type(enumeration),
    }
}

/// Emit every item of a module — its data types, then its templates.
#[must_use]
pub fn module_items(module: &Module) -> TokenStream {
    let data_types = module.data_types.iter().map(data_type);
    let templates = module.templates.iter().map(template);
    quote! {
        #(#data_types)*
        #(#templates)*
    }
}

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
        let label = &field.label;
        let doc = format!("The Daml `{}` field.", field.label);
        quote! {
            #[doc = #doc]
            #[serde(rename = #label)]
            pub #field_name: #ty,
        }
    });

    quote! {
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct #name #generics {
            #(#fields)*
        }
    }
}

/// Emit a record's `struct` together with its `ToValue`/`FromValue` codecs.
#[must_use]
pub fn record_items(record: &Record) -> TokenStream {
    let structure = record_struct(record);
    let codecs = record_codecs(record);
    quote! {
        #structure
        #codecs
    }
}

/// Emit the `ToValue`/`FromValue` impls mapping a record to a Ledger API
/// `Record` value (each field keyed by its Daml label). Generic records are
/// skipped for now — their codecs need per-parameter bounds (a later increment).
fn record_codecs(record: &Record) -> TokenStream {
    if !record.type_params.is_empty() {
        return TokenStream::new();
    }
    let name = type_ident(&record.name);
    let to_fields = record.fields.iter().map(|field| {
        let label = &field.label;
        let ident = field_ident(&field.label);
        quote! { (#label, rt::ToValue::to_value(&self.#ident)), }
    });
    let from_fields = record.fields.iter().map(|field| {
        let label = &field.label;
        let ident = field_ident(&field.label);
        quote! { #ident: rt::FromValue::from_value(rt::record_field(value, #label)?)?, }
    });

    quote! {
        impl rt::ToValue for #name {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![#(#to_fields)*])
            }
        }
        impl rt::FromValue for #name {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self { #(#from_fields)* })
            }
        }
    }
}

/// Emit a variant (sum) type as a Rust `enum` — one variant per constructor,
/// carrying the constructor's payload type (or nothing for a nullary one).
#[must_use]
pub fn variant_enum(variant: &Variant) -> TokenStream {
    let name = type_ident(&variant.name);
    let generics = generics(&variant.type_params);
    let constructors = variant.constructors.iter().map(|ctor| {
        let ctor_name = type_ident(&ctor.name);
        let doc = format!("The Daml `{}` constructor.", ctor.name);
        if let Some(payload) = &ctor.payload {
            let payload = rust_type(payload);
            quote! {
                #[doc = #doc]
                #ctor_name(#payload),
            }
        } else {
            quote! {
                #[doc = #doc]
                #ctor_name,
            }
        }
    });

    quote! {
        #[derive(Clone, Debug, PartialEq)]
        pub enum #name #generics {
            #(#constructors)*
        }
    }
}

/// Emit an enumeration as a C-like Rust `enum` (constructors carry no data).
#[must_use]
pub fn enum_type(enumeration: &Enum) -> TokenStream {
    let name = type_ident(&enumeration.name);
    let constructors = enumeration.constructors.iter().map(|ctor| {
        let ctor_name = type_ident(ctor);
        let doc = format!("The Daml `{ctor}` value.");
        quote! {
            #[doc = #doc]
            #ctor_name,
        }
    });

    quote! {
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum #name {
            #(#constructors)*
        }
    }
}

/// Emit a template: its payload `struct` plus a typed `rt::Choice` impl for each
/// choice, linking the choice-argument type to the template and its return type.
/// (The template identifier — package/module/entity — and the create/exercise
/// command builders arrive with the runtime crate in Phase C.)
#[must_use]
pub fn template(template: &Template) -> TokenStream {
    let payload = record_items(&Record {
        name: template.name.clone(),
        type_params: Vec::new(),
        fields: template.fields.clone(),
    });
    let self_ty = type_ident(&template.name);

    let choices = template.choices.iter().map(|choice| {
        let argument = rust_type(&choice.argument);
        let returns = rust_type(&choice.returns);
        let choice_name = &choice.name;
        let consuming = choice.consuming;
        let doc = format!(
            "The `{}` choice on [`{}`] ({}).",
            choice.name,
            template.name,
            if choice.consuming {
                "consuming"
            } else {
                "non-consuming"
            }
        );
        quote! {
            #[doc = #doc]
            impl rt::Choice<#self_ty> for #argument {
                type Return = #returns;
                const NAME: &'static str = #choice_name;
                const CONSUMING: bool = #consuming;
            }
        }
    });

    quote! {
        #payload
        #(#choices)*
    }
}

/// The generic parameter list `<A, B>` for a type's parameters, or empty tokens
/// when the type is not generic.
fn generics(type_params: &[String]) -> TokenStream {
    if type_params.is_empty() {
        return TokenStream::new();
    }
    let params = type_params.iter().map(|param| type_var_ident(param));
    quote!(<#(#params),*>)
}

/// A Rust identifier for a Daml type or constructor **name**. Daml type names
/// are already valid Rust identifiers, so they are used as-is (keywords
/// escaped) — not case-converted, which would mangle names containing `_`.
#[must_use]
pub fn type_ident(name: &str) -> Ident {
    ident(name)
}

/// A Rust identifier for a Daml **type variable** (`a` → `A`), upper-camel-cased
/// to follow Rust's generic-parameter naming convention.
#[must_use]
pub fn type_var_ident(name: &str) -> Ident {
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
