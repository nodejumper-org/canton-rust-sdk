//! The AST→IR bridge: lower a decoded Daml-LF [`Package`](lf::Package) into the
//! codegen [`ir`](crate::ir).
//!
//! This is the one module that touches the LF AST; everything else stays
//! decoder-agnostic. **Records-first:** serializable records, variants, and
//! enums lower fully (this includes template payloads, which are records).
//! Templates-with-choices and interfaces have their IR shape reserved and lower
//! in a later step. Field types cover the LF builtins, references to named
//! types, and type variables; unsupported LF type shapes yield a [`LowerError`]
//! rather than silently-wrong output.

use canton_lf::pb::daml_lf_2 as lf;
use canton_lf::{interned_dotted_name, interned_str, interned_type};

use crate::ir::{
    DamlType, DataType, Enum, Field, Module, Record, TypeRef, Variant, VariantConstructor,
};

/// An error lowering the LF AST into the IR (an unsupported or malformed shape).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LowerError(pub String);

impl LowerError {
    fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl std::fmt::Display for LowerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "lowering error: {}", self.0)
    }
}

impl std::error::Error for LowerError {}

/// Lower a whole package into an IR [`Module`], best-effort: every serializable
/// record/variant/enum that lowers cleanly is included; a data type whose shape
/// is not yet supported is skipped (so partial DARs still produce output).
///
/// Templates and interfaces are not populated yet — their IR shape is reserved.
#[must_use]
pub fn lower_package(package: &lf::Package) -> Module {
    let mut module = Module::default();
    for lf_module in &package.modules {
        for data_type in &lf_module.data_types {
            if let Ok(Some(lowered)) = lower_data_type(package, data_type) {
                module.data_types.push(lowered);
            }
        }
    }
    module
}

/// Lower one `DefDataType`. Returns `Ok(None)` when it is intentionally skipped
/// (non-serializable, or an interface view marker).
///
/// # Errors
/// Returns [`LowerError`] if the data type's name or a field type cannot be
/// lowered.
pub fn lower_data_type(
    package: &lf::Package,
    data_type: &lf::DefDataType,
) -> Result<Option<DataType>, LowerError> {
    // Codegen only wants serializable types (skip functions, interface markers).
    if !data_type.serializable {
        return Ok(None);
    }
    let name = rust_name(package, data_type.name_interned_dname)?;
    let type_params = data_type
        .params
        .iter()
        .map(|param| {
            interned_str(package, param.var_interned_str)
                .map(str::to_string)
                .ok_or_else(|| LowerError::new("unresolved type parameter"))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let Some(data_cons) = &data_type.data_cons else {
        return Err(LowerError::new(format!("{name}: no data constructor")));
    };

    match data_cons {
        lf::def_data_type::DataCons::Record(fields) => {
            let fields = lower_fields(package, &fields.fields)?;
            Ok(Some(DataType::Record(Record {
                name,
                type_params,
                fields,
            })))
        }
        lf::def_data_type::DataCons::Variant(fields) => {
            let constructors = fields
                .fields
                .iter()
                .map(|field| lower_variant_constructor(package, field))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Some(DataType::Variant(Variant {
                name,
                type_params,
                constructors,
            })))
        }
        lf::def_data_type::DataCons::Enum(constructors) => {
            let constructors = constructors
                .constructors_interned_str
                .iter()
                .map(|&index| {
                    interned_str(package, index)
                        .map(str::to_string)
                        .ok_or_else(|| LowerError::new("unresolved enum constructor"))
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Some(DataType::Enum(Enum { name, constructors })))
        }
        // An interface's view marker — reserved, lowered separately later.
        lf::def_data_type::DataCons::Interface(_) => Ok(None),
    }
}

fn lower_fields(
    package: &lf::Package,
    fields: &[lf::FieldWithType],
) -> Result<Vec<Field>, LowerError> {
    fields
        .iter()
        .map(|field| {
            let label = interned_str(package, field.field_interned_str)
                .ok_or_else(|| LowerError::new("unresolved field label"))?
                .to_string();
            let ty = lower_type(package, field_type(field)?)?;
            Ok(Field { label, ty })
        })
        .collect()
}

fn lower_variant_constructor(
    package: &lf::Package,
    field: &lf::FieldWithType,
) -> Result<VariantConstructor, LowerError> {
    let name = interned_str(package, field.field_interned_str)
        .ok_or_else(|| LowerError::new("unresolved variant constructor"))?
        .to_string();
    // A `Unit` payload is a nullary constructor.
    let payload = match lower_type(package, field_type(field)?)? {
        DamlType::Unit => None,
        other => Some(other),
    };
    Ok(VariantConstructor { name, payload })
}

/// Lower an LF [`Type`](lf::Type) into a [`DamlType`], resolving interned types.
///
/// # Errors
/// Returns [`LowerError`] for LF type shapes codegen does not model (`Forall`,
/// `Struct`, `Syn`, unknown builtins, or a bare type-level `Nat`).
pub fn lower_type(package: &lf::Package, ty: &lf::Type) -> Result<DamlType, LowerError> {
    let Some(sum) = &ty.sum else {
        return Err(LowerError::new("empty type"));
    };
    match sum {
        lf::r#type::Sum::Var(var) => {
            let name = interned_str(package, var.var_interned_str)
                .ok_or_else(|| LowerError::new("unresolved type variable"))?
                .to_string();
            Ok(DamlType::Var(name))
        }
        lf::r#type::Sum::Con(con) => {
            let tycon = con
                .tycon
                .as_ref()
                .ok_or_else(|| LowerError::new("type constructor without a name"))?;
            let name = rust_name(package, tycon.name_interned_dname)?;
            let args = con
                .args
                .iter()
                .map(|arg| lower_type(package, arg))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(DamlType::Ref(TypeRef { name, args }))
        }
        lf::r#type::Sum::Builtin(builtin) => lower_builtin(package, builtin.builtin, &builtin.args),
        lf::r#type::Sum::Interned(index) => {
            let resolved = interned_type(package, *index)
                .ok_or_else(|| LowerError::new("unresolved interned type"))?;
            lower_type(package, resolved)
        }
        lf::r#type::Sum::Nat(_) => Err(LowerError::new("unexpected bare type-level Nat")),
        lf::r#type::Sum::Forall(_) | lf::r#type::Sum::Struct(_) | lf::r#type::Sum::Syn(_) => Err(
            LowerError::new("unsupported LF type (forall / struct / syn)"),
        ),
    }
}

fn lower_builtin(
    package: &lf::Package,
    builtin: i32,
    args: &[lf::Type],
) -> Result<DamlType, LowerError> {
    use lf::BuiltinType;

    let kind = BuiltinType::try_from(builtin)
        .map_err(|_| LowerError::new(format!("unknown builtin type {builtin}")))?;
    let arg = |index: usize| -> Result<DamlType, LowerError> {
        let ty = args
            .get(index)
            .ok_or_else(|| LowerError::new(format!("{kind:?} missing type argument {index}")))?;
        lower_type(package, ty)
    };

    let ty = match kind {
        BuiltinType::Unit => DamlType::Unit,
        BuiltinType::Bool => DamlType::Bool,
        BuiltinType::Int64 => DamlType::Int64,
        BuiltinType::Date => DamlType::Date,
        BuiltinType::Timestamp => DamlType::Timestamp,
        BuiltinType::Party => DamlType::Party,
        BuiltinType::Text => DamlType::Text,
        BuiltinType::Numeric => DamlType::Numeric(numeric_scale(package, args)?),
        BuiltinType::ContractId => DamlType::ContractId(Box::new(arg(0)?)),
        BuiltinType::Optional => DamlType::Optional(Box::new(arg(0)?)),
        BuiltinType::List => DamlType::List(Box::new(arg(0)?)),
        BuiltinType::Textmap => DamlType::TextMap(Box::new(arg(0)?)),
        BuiltinType::Genmap => DamlType::GenMap(Box::new(arg(0)?), Box::new(arg(1)?)),
        other => {
            return Err(LowerError::new(format!(
                "builtin type {other:?} is not representable in codegen"
            )));
        }
    };
    Ok(ty)
}

/// The scale of a `Numeric n` from its type-level `Nat` argument.
fn numeric_scale(package: &lf::Package, args: &[lf::Type]) -> Result<u8, LowerError> {
    let mut ty = args
        .first()
        .ok_or_else(|| LowerError::new("Numeric without a scale argument"))?;
    // Follow one level of interning if the scale is stored in the type table.
    if let Some(lf::r#type::Sum::Interned(index)) = &ty.sum {
        ty = interned_type(package, *index)
            .ok_or_else(|| LowerError::new("unresolved Numeric scale"))?;
    }
    match &ty.sum {
        Some(lf::r#type::Sum::Nat(scale)) => u8::try_from(*scale)
            .map_err(|_| LowerError::new(format!("Numeric scale {scale} out of range"))),
        _ => Err(LowerError::new("Numeric scale is not a type-level Nat")),
    }
}

/// The `Type` of a field, or an error if absent.
fn field_type(field: &lf::FieldWithType) -> Result<&lf::Type, LowerError> {
    field
        .r#type
        .as_ref()
        .ok_or_else(|| LowerError::new("field without a type"))
}

/// Resolve an interned dotted type name into a Rust-usable identifier: the Daml
/// qualified name with `.` replaced by `_` (Daml type names are otherwise valid
/// Rust identifiers). Definitions and references use this same mapping, so they
/// agree.
fn rust_name(package: &lf::Package, name_interned_dname: i32) -> Result<String, LowerError> {
    let dotted = interned_dotted_name(package, name_interned_dname)
        .ok_or_else(|| LowerError::new("unresolved dotted name"))?;
    Ok(dotted.replace('.', "_"))
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use canton_lf::{Dar, decode_main_package};

    #[test]
    fn lowers_a_real_dar_to_valid_rust() {
        // Env-gated: point at a real .dar (e.g. one from cn-quickstart).
        let Ok(path) = std::env::var("CANTON_TEST_DAR") else {
            eprintln!("skipping lower test: set CANTON_TEST_DAR=/path/to/x.dar");
            return;
        };

        let dar = Dar::open(&path).expect("open DAR");
        let package = decode_main_package(&dar).expect("decode LF package");
        let module = lower_package(&package);

        assert!(
            !module.data_types.is_empty(),
            "a real package should lower at least one data type"
        );

        // The lowered IR generates syntactically valid Rust end to end.
        let src = crate::generate_module(&module).expect("generate module");
        syn::parse_file(&src).expect("generated source must be valid Rust");

        // Show a few of the lowered type names for visibility.
        let names: Vec<&str> = module
            .data_types
            .iter()
            .take(5)
            .map(|dt| match dt {
                DataType::Record(record) => record.name.as_str(),
                DataType::Variant(variant) => variant.name.as_str(),
                DataType::Enum(enumeration) => enumeration.name.as_str(),
            })
            .collect();
        println!(
            "lowered {} data types from {} modules → {} bytes of Rust; e.g. {:?}",
            module.data_types.len(),
            package.modules.len(),
            src.len(),
            names,
        );
    }
}
