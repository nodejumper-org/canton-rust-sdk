//! Emit Rust source items from the [`crate::ir`] types.

use heck::{ToSnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::ir::{
    Choice, Crate, DamlType, DataType, Enum, Interface, Module, NamedModule, PackageModule, Record,
    Template, Variant,
};
use crate::map::rust_type;

/// Emit the Rust item(s) for a named data type (record, variant, or enum).
#[must_use]
pub(crate) fn data_type(data_type: &DataType) -> TokenStream {
    match data_type {
        DataType::Record(record) => record_items(record),
        DataType::Variant(variant) => variant_items(variant),
        DataType::Enum(enumeration) => enum_items(enumeration),
        DataType::InterfaceMarker(name) => interface_marker(name),
    }
}

/// Emit an interface **marker**: a phantom tag `struct` that only ever appears
/// as the type argument of a `ContractId` (which is unconditional in its tag),
/// so it needs no derives or codecs of its own.
#[must_use]
fn interface_marker(name: &str) -> TokenStream {
    let name = type_ident(name);
    let doc = format!("Marker for the Daml interface `{name}` (held via `ContractId`).");
    quote! {
        #[doc = #doc]
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct #name;
    }
}

/// Emit every item of a module — its data types, then its templates.
#[must_use]
pub(crate) fn module_items(module: &Module) -> TokenStream {
    let data_types = module.data_types.iter().map(data_type);
    let templates = module.templates.iter().map(template);
    let interfaces = module.interfaces.iter().map(interface);
    quote! {
        #(#data_types)*
        #(#templates)*
        #(#interfaces)*
    }
}

/// Emit the module tree for a whole generated crate: one `pub mod` per package,
/// one submodule per Daml module. Cross-module and cross-package references
/// resolve through the `crate::<package>::<module>::<Type>` paths the lowering
/// produced, and names from different modules cannot collide.
#[must_use]
pub(crate) fn crate_items(krate: &Crate) -> TokenStream {
    let packages = krate.packages.iter().map(package_module);
    quote! {
        #(#packages)*
    }
}

/// Emit one package as a `pub mod`, wrapping its Daml modules.
fn package_module(package: &PackageModule) -> TokenStream {
    let name = ident(&package.name);
    let modules = package.modules.iter().map(named_module);
    quote! {
        pub mod #name {
            #(#modules)*
        }
    }
}

/// Emit one Daml module as a `pub mod`, aliasing the runtime as `rt` so the
/// module's `rt::…` references resolve locally.
fn named_module(module: &NamedModule) -> TokenStream {
    let name = ident(&module.name);
    let items = module_items(&module.module);
    quote! {
        pub mod #name {
            use canton_daml as rt;

            #items
        }
    }
}

/// A doc line for a generated item, but only where the Rust identifier had to
/// be spelled differently from the Daml one.
///
/// Where the two agree the doc would only restate the identifier next to it —
/// noise on every field of every record on docs.rs. The `#[serde(rename)]`
/// beside it records the wire label either way.
fn renamed_doc(rust_name: &Ident, daml_name: &str, kind: &str) -> TokenStream {
    if rust_name.to_string().trim_start_matches("r#") == daml_name {
        return TokenStream::new();
    }
    let doc = format!("Daml {kind} `{daml_name}`.");
    quote!(#[doc = #doc])
}

/// Generate the `struct` for a record data type (also used for template
/// payloads). Field names are snake-cased for Rust; the original Daml label is
/// pinned on the wire by the emitted codecs (the gRPC `Record` labels, and
/// `serde(rename)` for JSON) and appears in a doc only where the two differ.
#[must_use]
pub(crate) fn record_struct(record: &Record) -> TokenStream {
    let name = type_ident(&record.name);
    let generics = generics(&record.type_params);
    let fields = record.fields.iter().map(|field| {
        let field_name = field_ident(&field.label);
        let ty = rust_type(&field.ty);
        let label = &field.label;
        let doc = renamed_doc(&field_name, label, "field");
        quote! {
            #doc
            #[serde(rename = #label)]
            pub #field_name: #ty,
        }
    });
    let phantom = phantom_field(&record.type_params, record.fields.iter().map(|f| &f.ty));

    quote! {
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct #name #generics {
            #(#fields)*
            #phantom
        }
    }
}

/// A hidden `PhantomData` field binding any type parameters that a generic
/// record declares but never uses in its fields (Daml permits such phantom
/// parameters; Rust rejects an unused type parameter). Empty when every
/// parameter is used. The field is `#[serde(skip)]` and ignored by the `Value`
/// codec, so it never touches the wire form.
fn phantom_field<'a>(
    type_params: &'a [String],
    field_types: impl Iterator<Item = &'a DamlType>,
) -> TokenStream {
    let unused = unused_params(type_params, field_types);
    if unused.is_empty() {
        return quote!();
    }
    let params = unused.iter().map(|param| type_var_ident(param));
    quote! {
        #[doc(hidden)]
        #[serde(skip)]
        pub _phantom: ::core::marker::PhantomData<(#(#params,)*)>,
    }
}

/// The `_phantom` field initializer for a record's `FromValue`/constructor form,
/// or empty when the record has no phantom parameters.
fn phantom_init<'a>(
    type_params: &'a [String],
    field_types: impl Iterator<Item = &'a DamlType>,
) -> TokenStream {
    if unused_params(type_params, field_types).is_empty() {
        quote!()
    } else {
        quote! { _phantom: ::core::marker::PhantomData, }
    }
}

/// The type parameters a generic type declares but never uses in the given field
/// types (Daml's phantom parameters).
fn unused_params<'a>(
    type_params: &'a [String],
    field_types: impl Iterator<Item = &'a DamlType>,
) -> Vec<&'a String> {
    let mut used = std::collections::BTreeSet::new();
    for ty in field_types {
        collect_type_vars(ty, &mut used);
    }
    type_params
        .iter()
        .filter(|param| !used.contains(*param))
        .collect()
}

/// Collect the names of every type variable referenced anywhere in `ty`.
fn collect_type_vars(ty: &DamlType, out: &mut std::collections::BTreeSet<String>) {
    match ty {
        DamlType::Var(name) => {
            out.insert(name.clone());
        }
        DamlType::ContractId(inner)
        | DamlType::List(inner)
        | DamlType::Optional(inner)
        | DamlType::TextMap(inner)
        | DamlType::Boxed(inner) => collect_type_vars(inner, out),
        DamlType::GenMap(key, value) => {
            collect_type_vars(key, out);
            collect_type_vars(value, out);
        }
        DamlType::Ref(reference) => {
            for arg in &reference.args {
                collect_type_vars(arg, out);
            }
        }
        _ => {}
    }
}

/// Emit a record's `struct` together with its `ToValue`/`FromValue` codecs.
#[must_use]
pub(crate) fn record_items(record: &Record) -> TokenStream {
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
///
/// Decoding is robust to the two shapes Canton legitimately produces: fields
/// are located by label *or* declaration index (non-verbose output omits
/// labels), and an absent `Optional` field decodes as `None` (normalized
/// records omit trailing empty optionals under Smart Contract Upgrade).
fn record_codecs(record: &Record) -> TokenStream {
    let name = type_ident(&record.name);
    let to_fields = record.fields.iter().map(|field| {
        let label = &field.label;
        let ident = field_ident(&field.label);
        quote! { (#label, rt::ToValue::to_value(&self.#ident)), }
    });
    let from_fields = record.fields.iter().enumerate().map(|(index, field)| {
        let label = &field.label;
        let ident = field_ident(&field.label);
        // `.at(label)` on the *decode* of the field, not on locating it:
        // `required_field` already names the field it could not find, while a
        // failure inside `from_value` — the wrong type three records down —
        // otherwise arrives as a bare "expected Text" with no way to tell which
        // field it came from. Each layer prepends its own label as the error
        // travels up, so the path reads `owner.address.city`.
        if matches!(field.ty, DamlType::Optional(_)) {
            // Absent (normalized-away) optional fields decode as `None`.
            quote! { #ident: rt::optional_field(value, #index, #label).map_err(|e| e.at(#label))?, }
        } else {
            quote! {
                #ident: rt::FromValue::from_value(rt::required_field(value, #index, #label)?)
                    .map_err(|e| e.at(#label))?,
            }
        }
    });

    let used = used_params(record.fields.iter().map(|f| &f.ty));
    let (impl_generics, ty, to_where) =
        codec_header(&name, &record.type_params, &used, &quote!(rt::ToValue));
    let (_, _, from_where) =
        codec_header(&name, &record.type_params, &used, &quote!(rt::FromValue));
    let phantom = phantom_init(&record.type_params, record.fields.iter().map(|f| &f.ty));
    // A record with no fields never reads the wire value.
    let value_binding = ident(if record.fields.is_empty() {
        "_value"
    } else {
        "value"
    });
    quote! {
        impl #impl_generics rt::ToValue for #ty #to_where {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![#(#to_fields)*])
            }
        }
        impl #impl_generics rt::FromValue for #ty #from_where {
            fn from_value(#value_binding: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self { #(#from_fields)* #phantom })
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
    used: &std::collections::BTreeSet<String>,
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
    // Bound only the parameters the codec actually touches. Daml permits a
    // phantom parameter — declared but used in no field — and bounding one
    // would demand a codec from a type this impl never encodes. Interface
    // markers are exactly such a type: they carry no codec by design, so
    // `Wrapper Holding` for a phantom `Wrapper a` would generate Rust that does
    // not compile.
    let bounded = type_params
        .iter()
        .filter(|param| used.contains(*param))
        .map(|param| type_var_ident(param))
        .collect::<Vec<_>>();
    let where_clause = if bounded.is_empty() {
        TokenStream::new()
    } else {
        quote!(where #(#bounded: #trait_bound),*)
    };
    (impl_generics, ty, where_clause)
}

/// The type variables a set of field/payload types actually mentions.
fn used_params<'a>(
    types: impl Iterator<Item = &'a DamlType>,
) -> std::collections::BTreeSet<String> {
    let mut used = std::collections::BTreeSet::new();
    for ty in types {
        collect_type_vars(ty, &mut used);
    }
    used
}

/// Emit a variant (sum) type as a Rust `enum` — one variant per constructor,
/// carrying the constructor's payload type (or nothing for a nullary one).
#[must_use]
pub(crate) fn variant_enum(variant: &Variant) -> TokenStream {
    let name = type_ident(&variant.name);
    let generics = generics(&variant.type_params);
    let constructors = variant.constructors.iter().map(|ctor| {
        let ctor_name = type_ident(&ctor.name);
        let label = &ctor.name;
        let doc = renamed_doc(&ctor_name, label, "constructor");
        // A nullary constructor carries `Unit`, not nothing: the LF-JSON variant
        // form always has a `value`, and a nullary one is `Unit` (`{}`). Emitting
        // it as a bare unit variant would serialize to `{"tag":<c>}`, which the
        // Ledger API's `{"tag":<c>,"value":{}}` neither matches nor parses.
        let payload = ctor
            .payload
            .as_ref()
            .map_or_else(|| quote!(rt::Unit), rust_type);
        quote! {
            #doc
            #[serde(rename = #label)]
            #ctor_name(#payload),
        }
    });

    // The LF-JSON variant form is `{"tag": <ctor>, "value": <payload>}` —
    // serde's adjacently-tagged representation.
    quote! {
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum #name #generics {
            #(#constructors)*
        }
    }
}

/// Emit a variant type together with its `ToValue`/`FromValue` codecs.
#[must_use]
pub(crate) fn variant_items(variant: &Variant) -> TokenStream {
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
    // Every constructor is a newtype variant (a nullary one carries `rt::Unit`),
    // so both codecs treat the payload uniformly.
    let to_arms = variant.constructors.iter().map(|ctor| {
        let ctor_name = type_ident(&ctor.name);
        let label = &ctor.name;
        quote! { #name::#ctor_name(inner) => rt::variant_value(#label, rt::ToValue::to_value(inner)), }
    });
    let from_arms = variant.constructors.iter().map(|ctor| {
        let ctor_name = type_ident(&ctor.name);
        let label = &ctor.name;
        quote! {
            #label => ::core::result::Result::Ok(#name::#ctor_name(
                rt::FromValue::from_value(payload).map_err(|e| e.at(#label))?,
            )),
        }
    });

    let used = used_params(
        variant
            .constructors
            .iter()
            .filter_map(|c| c.payload.as_ref()),
    );
    let (impl_generics, ty, to_where) =
        codec_header(&name, &variant.type_params, &used, &quote!(rt::ToValue));
    let (_, _, from_where) =
        codec_header(&name, &variant.type_params, &used, &quote!(rt::FromValue));
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
pub(crate) fn enum_type(enumeration: &Enum) -> TokenStream {
    let name = type_ident(&enumeration.name);
    let constructors = enumeration.constructors.iter().map(|ctor| {
        let ctor_name = type_ident(ctor);
        let label = ctor;
        let doc = renamed_doc(&ctor_name, label, "constructor");
        quote! {
            #doc
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
pub(crate) fn enum_items(enumeration: &Enum) -> TokenStream {
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

/// The doc on a template's payload struct: its on-ledger identity and the
/// choices exercisable on it.
///
/// rustdoc has no reverse index for `impl Choice<This> for That`, so without
/// this a reader on docs.rs sees a payload struct and no way to discover what
/// can be done with it.
///
/// One `#[doc]` attribute per line: a single multi-line string is rendered as a
/// `/** … */` block whose continuation lines are indented to match the item,
/// and rustdoc reads a four-space indent as a code block — which would turn the
/// prose into a failing doctest in the user's crate.
fn template_doc(template: &Template) -> TokenStream {
    let mut lines = vec![
        format!(
            "The Daml template `{}:{}`.",
            template.module_name, template.name
        ),
        String::new(),
        format!(
            "Submit with `rt::create_command`; its on-ledger id is `#{}:{}:{}`.",
            template.package_name, template.module_name, template.name,
        ),
    ];
    if !template.choices.is_empty() {
        lines.extend([
            String::new(),
            "# Choices".to_string(),
            String::new(),
            "Exercise with `rt::exercise_command`:".to_string(),
            String::new(),
        ]);
        for choice in &template.choices {
            let consuming = if choice.consuming {
                "consuming"
            } else {
                "non-consuming"
            };
            lines.push(format!("- `{}` — {consuming}", choice.name));
        }
    }
    if template.key.is_some() {
        lines.extend([
            String::new(),
            "Keyed: also exercisable with `rt::exercise_by_key_command`.".to_string(),
        ]);
    }
    quote!(#(#[doc = #lines])*)
}

/// Emit a template: its payload `struct`, its on-ledger identity
/// (`rt::Contract` + `rt::Template`), an `rt::WithKey` impl when it is keyed, and
/// a typed `rt::Choice` impl per choice.
#[must_use]
pub(crate) fn template(template: &Template) -> TokenStream {
    let payload = record_items(&Record {
        name: template.name.clone(),
        type_params: Vec::new(),
        fields: template.fields.clone(),
    });
    let self_ty = type_ident(&template.name);
    let doc = template_doc(template);

    let contract_impl = contract_impl(
        &self_ty,
        &template.package_id,
        &template.package_name,
        &template.module_name,
        &template.name,
    );

    // A keyed template exposes its key type so contracts can be exercised by key.
    let key_impl = template.key.as_ref().map_or_else(TokenStream::new, |key| {
        let key_ty = rust_type(key);
        quote! {
            impl rt::WithKey for #self_ty {
                type Key = #key_ty;
            }
        }
    });

    let choices = choice_impls(&self_ty, &template.name, &template.choices);

    // `to_record` mirrors the payload's `ToValue`, but returns the bare
    // `Record` a create command carries — so the runtime never has to unwrap a
    // `Value` it merely assumes is a record.
    let record_fields = template.fields.iter().map(|field| {
        let label = &field.label;
        let ident = field_ident(&field.label);
        quote! { (#label, rt::ToValue::to_value(&self.#ident)), }
    });

    quote! {
        #doc
        #payload
        #contract_impl
        impl rt::Template for #self_ty {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![#(#record_fields)*])
            }
        }
        #key_impl
        #choices
    }
}

/// Emit an interface's impls on its marker type: its on-ledger identity
/// (`rt::Contract` + `rt::Interface`, carrying the view type) and a typed
/// `rt::Choice` impl per interface choice, so a `ContractId<Interface>` can be
/// exercised without the concrete template. The marker `struct` itself is
/// emitted from the interface's data type (`interface_marker`).
#[must_use]
pub(crate) fn interface(interface: &Interface) -> TokenStream {
    let self_ty = type_ident(&interface.name);
    let contract_impl = contract_impl(
        &self_ty,
        &interface.package_id,
        &interface.package_name,
        &interface.module_name,
        &interface.name,
    );
    // The view is a record (or `Unit` for an empty view).
    let view_ty = interface
        .view
        .as_ref()
        .map_or_else(|| quote!(rt::Unit), rust_type);
    let choices = choice_impls(&self_ty, &interface.name, &interface.choices);

    quote! {
        #contract_impl
        impl rt::Interface for #self_ty {
            type View = #view_ty;
        }
        #choices
    }
}

/// Emit the `rt::Contract` impl carrying a template's/interface's on-ledger
/// identity (package id + name, module, entity).
fn contract_impl(
    self_ty: &Ident,
    package_id: &str,
    package_name: &str,
    module_name: &str,
    entity_name: &str,
) -> TokenStream {
    quote! {
        impl rt::Contract for #self_ty {
            const PACKAGE_ID: &'static str = #package_id;
            const PACKAGE_NAME: &'static str = #package_name;
            const MODULE_NAME: &'static str = #module_name;
            const ENTITY_NAME: &'static str = #entity_name;
        }
    }
}

/// Emit a typed `rt::Choice<Owner>` impl for each choice (shared by templates and
/// interfaces), linking the choice-argument type to its owner and return type.
fn choice_impls(self_ty: &Ident, owner: &str, choices: &[Choice]) -> TokenStream {
    let impls = choices.iter().map(|choice| {
        let argument = rust_type(&choice.argument);
        let returns = rust_type(&choice.returns);
        let choice_name = &choice.name;
        let consuming = choice.consuming;
        let doc = format!(
            "The `{}` choice on [`{}`] ({}).",
            choice.name,
            owner,
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
    quote! { #(#impls)* }
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
pub(crate) fn type_ident(name: &str) -> Ident {
    ident(name)
}

/// A Rust identifier for a Daml **type variable** (`a` → `A`), upper-camel-cased
/// to follow Rust's generic-parameter naming convention.
#[must_use]
pub(crate) fn type_var_ident(name: &str) -> Ident {
    ident(&name.to_upper_camel_case())
}

/// Render a type reference path (`["crate", "m", "Type"]` → `crate::m::Type`),
/// keyword-escaping each named segment. The path keywords `crate` / `self` /
/// `super` / `Self` pass through unescaped.
#[must_use]
pub(crate) fn type_path(segments: &[String]) -> TokenStream {
    let parts = segments.iter().map(|segment| match segment.as_str() {
        "crate" | "self" | "super" | "Self" => segment.parse::<TokenStream>().unwrap_or_default(),
        other => {
            let id = ident(other);
            quote!(#id)
        }
    });
    quote!(#(#parts)::*)
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
        // A name that is empty or starts with a digit is not a valid Rust
        // identifier — for example a tuple field `_1`, whose snake_case drops
        // the leading underscore to `1`. Prefix `_` to make it valid.
        _ if name.is_empty() || name.starts_with(|c: char| c.is_ascii_digit()) => {
            Ident::new(&format!("_{name}"), Span::call_site())
        }
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
            // Reserved by the 2024 edition. A generated crate declares 2021, so
            // a Daml field named `gen` compiles there today — but the same file
            // in a 2024 crate would not, and `r#gen` is valid in both. The
            // `serde(rename)` beside it keeps the wire label either way.
            | "gen"
    )
}
