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
        DataType::Variant(variant) => variant_items(variant),
        DataType::Enum(enumeration) => enum_items(enumeration),
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
/// `Record` value (each field keyed by its Daml label). Generic records get the
/// impls too, bounded by the trait on every type parameter.
fn record_codecs(record: &Record) -> TokenStream {
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

    let (impl_generics, ty, to_where) =
        codec_header(&name, &record.type_params, &quote!(rt::ToValue));
    let (_, _, from_where) = codec_header(&name, &record.type_params, &quote!(rt::FromValue));
    quote! {
        impl #impl_generics rt::ToValue for #ty #to_where {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![#(#to_fields)*])
            }
        }
        impl #impl_generics rt::FromValue for #ty #from_where {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self { #(#from_fields)* })
            }
        }
    }
}

/// The `(impl-generics, Self-type, where-clause)` for a codec `impl` on a type
/// that may be generic: `impl <A, B> Trait for Name<A, B> where A: Trait, ...`.
/// A non-generic type yields empty generics and no `where`.
fn codec_header(
    name: &Ident,
    type_params: &[String],
    trait_bound: &TokenStream,
) -> (TokenStream, TokenStream, TokenStream) {
    if type_params.is_empty() {
        return (TokenStream::new(), quote!(#name), TokenStream::new());
    }
    let params = type_params
        .iter()
        .map(|param| type_var_ident(param))
        .collect::<Vec<_>>();
    let impl_generics = quote!(<#(#params),*>);
    let ty = quote!(#name<#(#params),*>);
    let where_clause = quote!(where #(#params: #trait_bound),*);
    (impl_generics, ty, where_clause)
}

/// Emit a variant (sum) type as a Rust `enum` — one variant per constructor,
/// carrying the constructor's payload type (or nothing for a nullary one).
#[must_use]
pub fn variant_enum(variant: &Variant) -> TokenStream {
    let name = type_ident(&variant.name);
    let generics = generics(&variant.type_params);
    let constructors = variant.constructors.iter().map(|ctor| {
        let ctor_name = type_ident(&ctor.name);
        let label = &ctor.name;
        let doc = format!("The Daml `{}` constructor.", ctor.name);
        if let Some(payload) = &ctor.payload {
            let payload = rust_type(payload);
            quote! {
                #[doc = #doc]
                #[serde(rename = #label)]
                #ctor_name(#payload),
            }
        } else {
            quote! {
                #[doc = #doc]
                #[serde(rename = #label)]
                #ctor_name,
            }
        }
    });

    // The LF-JSON variant form is `{"tag": <ctor>, "value": <payload>}` —
    // serde's adjacently-tagged representation.
    quote! {
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum #name #generics {
            #(#constructors)*
        }
    }
}

/// Emit a variant type together with its `ToValue`/`FromValue` codecs.
#[must_use]
pub fn variant_items(variant: &Variant) -> TokenStream {
    let structure = variant_enum(variant);
    let codecs = variant_codecs(variant);
    quote! {
        #structure
        #codecs
    }
}

/// The gRPC `Value` codecs for a variant (a proto `Variant` — constructor name
/// plus the payload value; a nullary constructor carries `Unit`).
fn variant_codecs(variant: &Variant) -> TokenStream {
    let name = type_ident(&variant.name);
    let type_name = &variant.name;
    let to_arms = variant.constructors.iter().map(|ctor| {
        let ctor_name = type_ident(&ctor.name);
        let label = &ctor.name;
        if ctor.payload.is_some() {
            quote! { #name::#ctor_name(inner) => rt::variant_value(#label, rt::ToValue::to_value(inner)), }
        } else {
            quote! { #name::#ctor_name => rt::variant_value(#label, rt::unit_value()), }
        }
    });
    let from_arms = variant.constructors.iter().map(|ctor| {
        let ctor_name = type_ident(&ctor.name);
        let label = &ctor.name;
        if ctor.payload.is_some() {
            quote! { #label => ::core::result::Result::Ok(#name::#ctor_name(rt::FromValue::from_value(payload)?)), }
        } else {
            quote! { #label => ::core::result::Result::Ok(#name::#ctor_name), }
        }
    });

    let (impl_generics, ty, to_where) =
        codec_header(&name, &variant.type_params, &quote!(rt::ToValue));
    let (_, _, from_where) = codec_header(&name, &variant.type_params, &quote!(rt::FromValue));
    quote! {
        impl #impl_generics rt::ToValue for #ty #to_where {
            fn to_value(&self) -> rt::Value {
                match self {
                    #(#to_arms)*
                }
            }
        }
        impl #impl_generics rt::FromValue for #ty #from_where {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                let (constructor, payload) = rt::variant_parts(value)?;
                match constructor {
                    #(#from_arms)*
                    other => ::core::result::Result::Err(rt::unexpected_constructor(#type_name, other)),
                }
            }
        }
    }
}

/// Emit an enumeration as a C-like Rust `enum` (constructors carry no data).
#[must_use]
pub fn enum_type(enumeration: &Enum) -> TokenStream {
    let name = type_ident(&enumeration.name);
    let constructors = enumeration.constructors.iter().map(|ctor| {
        let ctor_name = type_ident(ctor);
        let label = ctor;
        let doc = format!("The Daml `{ctor}` value.");
        quote! {
            #[doc = #doc]
            #[serde(rename = #label)]
            #ctor_name,
        }
    });

    // An enum's LF-JSON form is just its constructor name (a string), which is
    // serde's default for a fieldless enum.
    quote! {
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize,
        )]
        #[serde(crate = "rt::serde")]
        pub enum #name {
            #(#constructors)*
        }
    }
}

/// Emit an enum type together with its `ToValue`/`FromValue` codecs.
#[must_use]
pub fn enum_items(enumeration: &Enum) -> TokenStream {
    let structure = enum_type(enumeration);
    let codecs = enum_codecs(enumeration);
    quote! {
        #structure
        #codecs
    }
}

/// The gRPC `Value` codecs for an enum (a proto `Enum` — the constructor name).
fn enum_codecs(enumeration: &Enum) -> TokenStream {
    let name = type_ident(&enumeration.name);
    let type_name = &enumeration.name;
    let to_arms = enumeration.constructors.iter().map(|ctor| {
        let ctor_name = type_ident(ctor);
        let label = ctor;
        quote! { #name::#ctor_name => #label, }
    });
    let from_arms = enumeration.constructors.iter().map(|ctor| {
        let ctor_name = type_ident(ctor);
        let label = ctor;
        quote! { #label => ::core::result::Result::Ok(#name::#ctor_name), }
    });

    quote! {
        impl rt::ToValue for #name {
            fn to_value(&self) -> rt::Value {
                rt::enum_value(match self { #(#to_arms)* })
            }
        }
        impl rt::FromValue for #name {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                match rt::enum_constructor(value)? {
                    #(#from_arms)*
                    other => ::core::result::Result::Err(rt::unexpected_constructor(#type_name, other)),
                }
            }
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
