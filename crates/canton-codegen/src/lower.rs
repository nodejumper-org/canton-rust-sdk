//! The AST→IR bridge: lower decoded Daml-LF [`Package`](lf::Package)s into the
//! codegen [`ir`](crate::ir).
//!
//! This is the one module that touches the LF AST; everything else stays
//! decoder-agnostic. Serializable records, variants, and enums lower fully, as
//! do **templates** (payload record, choices, contract key, on-ledger id) and
//! **interfaces** (view type, choices, and the identity choices are exercised
//! through — on the marker struct emitted from the interface's data type). Field
//! types cover the LF builtins, references to named types, and type variables;
//! unsupported LF type shapes yield a [`LowerError`] rather than silently-wrong
//! output.
//!
//! # Qualified references (PackageMap)
//!
//! A DAR bundles a package and its whole dependency closure. A type reference
//! (`Con`) carries the target module *and* package (self, or an imported
//! package identified by its id-hash). Lowering a **whole DAR**
//! ([`lower_crate`]) resolves every reference to a fully-qualified Rust path —
//! `crate::<package>::<module>::<Type>` — so cross-module and cross-package
//! references resolve and names from different modules can never collide.
//! Lowering a **single package** ([`lower_package`]) keeps references local
//! (bare type name), which is enough for the flat single-module output.

use std::collections::HashMap;

use canton_lf::pb::daml_lf_2 as lf;
use canton_lf::{
    Dar, DecodeError, decode_all, imported_package_id, interned_dotted_name, interned_str,
    interned_type, package_name, package_version,
};

use crate::ir::{
    Choice, Crate, DamlType, DataType, Enum, Field, Interface, Module, NamedModule, PackageModule,
    Record, Template, TypeRef, Variant, VariantConstructor,
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

/// Lower a whole DAR (its main package and its full dependency closure) into a
/// [`Crate`] of qualified modules, plus the list of types that could not be
/// lowered (best-effort — see [`lower_crate`]).
///
/// # Errors
/// Returns [`DecodeError`] if any package's bytes are malformed or not LF 2.x.
pub fn lower_dar(dar: &Dar) -> Result<(Crate, Vec<LowerError>), DecodeError> {
    Ok(lower_crate(&decode_all(dar)?))
}

/// Lower a set of decoded packages (a DAR's closure, each with its package id)
/// into a [`Crate`]: one Rust module per package, one submodule per Daml module,
/// with every type reference resolved to a fully-qualified path.
///
/// Best-effort: a type that cannot be lowered is skipped and its error recorded,
/// so a package with a few unsupported types still produces output.
#[must_use]
pub fn lower_crate(packages: &[(String, lf::Package)]) -> (Crate, Vec<LowerError>) {
    // package id → the Rust module name that package's types live under.
    let module_names: HashMap<&str, String> = packages
        .iter()
        .map(|(id, package)| (id.as_str(), package_module_name(package, id)))
        .collect();

    let mut krate = Crate::default();
    let mut errors = Vec::new();

    for (id, package) in packages {
        let lowering = Lowering {
            package,
            qualify: Some(Qualify {
                module_names: &module_names,
                current_id: id,
            }),
        };
        let mut package_module = PackageModule {
            name: module_names[id.as_str()].clone(),
            modules: Vec::new(),
        };
        for lf_module in &package.modules {
            let Some(dotted) = interned_dotted_name(package, lf_module.name_interned_dname) else {
                errors.push(LowerError::new("unresolved module name"));
                continue;
            };
            let mut ir_module = lowering.module(lf_module, &dotted, &mut errors);
            if !ir_module.data_types.is_empty()
                || !ir_module.templates.is_empty()
                || !ir_module.interfaces.is_empty()
            {
                let module_name = dotted.replace('.', "_");
                box_self_recursion(&mut ir_module, &module_names[id.as_str()], &module_name);
                package_module.modules.push(NamedModule {
                    name: module_name,
                    module: ir_module,
                });
            }
        }
        if !package_module.modules.is_empty() {
            krate.packages.push(package_module);
        }
    }

    (krate, errors)
}

/// Lower a single package's **data types** into a flat IR [`Module`] with
/// **local** references. Templates are not lowered here — they need the
/// qualified, whole-DAR form; use [`lower_crate`] for a crate with templates and
/// choices. This helper backs the single-module syntax path only.
#[must_use]
pub fn lower_package(package: &lf::Package) -> (Module, Vec<LowerError>) {
    let lowering = Lowering {
        package,
        qualify: None,
    };
    let mut module = Module::default();
    let mut errors = Vec::new();
    for lf_module in &package.modules {
        for data_type in &lf_module.data_types {
            match lowering.data_type(data_type) {
                Ok(Some(lowered)) => module.data_types.push(lowered),
                Ok(None) => {}
                Err(error) => errors.push(error),
            }
        }
    }
    (module, errors)
}

/// The Rust module name a package's types live under: its `name_version` (both
/// sanitised to identifier-safe characters), or `p_<hash8>` when metadata is
/// missing. Version is included so a DAR bundling two versions of one package
/// (Smart Contract Upgrade) gets two distinct modules.
fn package_module_name(package: &lf::Package, package_id: &str) -> String {
    match (package_name(package), package_version(package)) {
        (Some(name), Some(version)) => format!("{}_{}", sanitize(name), sanitize(version)),
        (Some(name), None) => sanitize(name),
        _ => format!("p_{}", &package_id[..8.min(package_id.len())]),
    }
}

/// Map each non-alphanumeric character to `_` so the result is a valid Rust
/// identifier fragment (`splice-amulet` → `splice_amulet`, `0.1.14` → `0_1_14`).
fn sanitize(input: &str) -> String {
    input
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .collect()
}

/// How a lowering resolves references, given the current package.
struct Lowering<'a> {
    package: &'a lf::Package,
    /// `None` → local references (bare type name). `Some` → fully-qualified
    /// paths resolved through the DAR's package map.
    qualify: Option<Qualify<'a>>,
}

/// The context for qualifying a reference to a fully-qualified Rust path.
struct Qualify<'a> {
    /// package id (archive hash) → the package's Rust module name. Resolves all
    /// three reference forms, since each ultimately names a package by its id.
    module_names: &'a HashMap<&'a str, String>,
    /// The id of the package currently being lowered (resolves `SelfPackageId`).
    current_id: &'a str,
}

impl Lowering<'_> {
    /// Lower a whole Daml module: its data types and its templates. A template's
    /// payload is a same-named record; it is folded into the [`Template`] (as its
    /// `fields`) and *not* emitted as a standalone record, so the payload struct
    /// is generated exactly once.
    fn module(
        &self,
        lf_module: &lf::Module,
        module_dotted: &str,
        errors: &mut Vec<LowerError>,
    ) -> Module {
        // The names that are templates — their payload record is folded in below.
        let template_names: std::collections::HashSet<String> = lf_module
            .templates
            .iter()
            .filter_map(|def| rust_name(self.package, def.tycon_interned_dname).ok())
            .collect();

        let mut module = Module::default();
        // Payload records held aside for their template (name → fields).
        let mut payloads: HashMap<String, Vec<Field>> = HashMap::new();
        for data_type in &lf_module.data_types {
            match self.data_type(data_type) {
                Ok(Some(DataType::Record(record))) if template_names.contains(&record.name) => {
                    payloads.insert(record.name, record.fields);
                }
                Ok(Some(lowered)) => module.data_types.push(lowered),
                Ok(None) => {}
                Err(error) => errors.push(error),
            }
        }

        for def in &lf_module.templates {
            match self.template(def, module_dotted, &mut payloads) {
                Ok(template) => module.templates.push(template),
                Err(error) => errors.push(error),
            }
        }

        for def in &lf_module.interfaces {
            match self.interface(def, module_dotted) {
                Ok(interface) => module.interfaces.push(interface),
                Err(error) => errors.push(error),
            }
        }
        module
    }

    /// Lower one `DefInterface`: its view type and choices, plus the on-ledger
    /// identity choices are exercised through. The marker struct is emitted from
    /// the interface's data type; this carries the impls on it.
    fn interface(
        &self,
        def: &lf::DefInterface,
        module_dotted: &str,
    ) -> Result<Interface, LowerError> {
        let name = rust_name(self.package, def.tycon_interned_dname)?;
        let choices = def
            .choices
            .iter()
            .map(|choice| self.choice(choice))
            .collect::<Result<Vec<_>, _>>()?;
        let view = def
            .view
            .as_ref()
            .map(|ty| self.type_(ty))
            .transpose()?
            // An interface always has a view; `Unit` means an empty view record.
            .filter(|view| !matches!(view, DamlType::Unit));
        Ok(Interface {
            name,
            module_name: module_dotted.to_string(),
            package_id: self
                .qualify
                .as_ref()
                .map_or_else(String::new, |q| q.current_id.to_string()),
            package_name: package_name(self.package).unwrap_or_default().to_string(),
            view,
            choices,
        })
    }

    /// Lower one `DefTemplate`: its payload fields (taken from the folded-in
    /// record), choices, and optional contract key.
    fn template(
        &self,
        def: &lf::DefTemplate,
        module_dotted: &str,
        payloads: &mut HashMap<String, Vec<Field>>,
    ) -> Result<Template, LowerError> {
        let name = rust_name(self.package, def.tycon_interned_dname)?;
        let fields = payloads
            .remove(&name)
            .ok_or_else(|| LowerError::new(format!("template {name}: payload record not found")))?;
        let choices = def
            .choices
            .iter()
            .map(|choice| self.choice(choice))
            .collect::<Result<Vec<_>, _>>()?;
        let key = def
            .key
            .as_ref()
            .and_then(|key| key.r#type.as_ref())
            .map(|ty| self.type_(ty))
            .transpose()?;
        Ok(Template {
            name,
            module_name: module_dotted.to_string(),
            package_id: self
                .qualify
                .as_ref()
                .map_or_else(String::new, |q| q.current_id.to_string()),
            package_name: package_name(self.package).unwrap_or_default().to_string(),
            fields,
            choices,
            key,
        })
    }

    /// Lower one `TemplateChoice`: its name, consuming flag, argument type, and
    /// return type. The controller/observer/body expressions are term-level and
    /// intentionally not modelled (see the decoder note on `Expr`).
    fn choice(&self, choice: &lf::TemplateChoice) -> Result<Choice, LowerError> {
        let name = interned_str(self.package, choice.name_interned_str)
            .ok_or_else(|| LowerError::new("unresolved choice name"))?
            .to_string();
        let argument = choice
            .arg_binder
            .as_ref()
            .and_then(|binder| binder.r#type.as_ref())
            .ok_or_else(|| LowerError::new(format!("choice {name}: no argument type")))?;
        let returns = choice
            .ret_type
            .as_ref()
            .ok_or_else(|| LowerError::new(format!("choice {name}: no return type")))?;
        Ok(Choice {
            name,
            consuming: choice.consuming,
            argument: self.type_(argument)?,
            returns: self.type_(returns)?,
        })
    }

    /// Lower one `DefDataType`. `Ok(None)` when intentionally skipped
    /// (non-serializable, or an interface view marker).
    fn data_type(&self, data_type: &lf::DefDataType) -> Result<Option<DataType>, LowerError> {
        let name = rust_name(self.package, data_type.name_interned_dname)?;

        // An interface is not serializable, but references to it (always
        // `ContractId<I>`) must resolve — emit a phantom marker. Handle it
        // before the serializable gate, which would otherwise skip it.
        if let Some(lf::def_data_type::DataCons::Interface(_)) = &data_type.data_cons {
            return Ok(Some(DataType::InterfaceMarker(name)));
        }

        // Codegen otherwise only wants serializable types (skip functions, and
        // internal non-serializable records like the interface `Any` wrappers).
        if !data_type.serializable {
            return Ok(None);
        }
        let type_params = data_type
            .params
            .iter()
            .map(|param| {
                interned_str(self.package, param.var_interned_str)
                    .map(str::to_string)
                    .ok_or_else(|| LowerError::new("unresolved type parameter"))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let Some(data_cons) = &data_type.data_cons else {
            return Err(LowerError::new(format!("{name}: no data constructor")));
        };

        match data_cons {
            lf::def_data_type::DataCons::Record(fields) => {
                let fields = self.fields(&fields.fields)?;
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
                    .map(|field| self.variant_constructor(field))
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
                        interned_str(self.package, index)
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

    fn fields(&self, fields: &[lf::FieldWithType]) -> Result<Vec<Field>, LowerError> {
        fields
            .iter()
            .map(|field| {
                let label = interned_str(self.package, field.field_interned_str)
                    .ok_or_else(|| LowerError::new("unresolved field label"))?
                    .to_string();
                let ty = self.type_(field_type(field)?)?;
                Ok(Field { label, ty })
            })
            .collect()
    }

    fn variant_constructor(
        &self,
        field: &lf::FieldWithType,
    ) -> Result<VariantConstructor, LowerError> {
        let name = interned_str(self.package, field.field_interned_str)
            .ok_or_else(|| LowerError::new("unresolved variant constructor"))?
            .to_string();
        // A `Unit` payload is a nullary constructor.
        let payload = match self.type_(field_type(field)?)? {
            DamlType::Unit => None,
            other => Some(other),
        };
        Ok(VariantConstructor { name, payload })
    }

    /// Lower an LF [`Type`](lf::Type) into a [`DamlType`], resolving interned
    /// types and (when qualifying) references to fully-qualified paths.
    fn type_(&self, ty: &lf::Type) -> Result<DamlType, LowerError> {
        self.apply(ty, &[])
    }

    /// Lower `ty` applied to `extra` type arguments. This unifies the flattened
    /// form (`Con`/`Builtin` with an `args` list) and the curried `TApp` form
    /// (LF 2.dev): `TApp(lhs, rhs)` applies `lhs` to `rhs` prepended to `extra`,
    /// so `((f a) b)` collapses to `f` applied to `[a, b]` regardless of shape.
    fn apply(&self, ty: &lf::Type, extra: &[&lf::Type]) -> Result<DamlType, LowerError> {
        let Some(sum) = &ty.sum else {
            return Err(LowerError::new("empty type"));
        };
        match sum {
            lf::r#type::Sum::Tapp(app) => {
                let lhs = app
                    .lhs
                    .as_deref()
                    .ok_or_else(|| LowerError::new("type application without a function"))?;
                let rhs = app
                    .rhs
                    .as_deref()
                    .ok_or_else(|| LowerError::new("type application without an argument"))?;
                let mut args = Vec::with_capacity(extra.len() + 1);
                args.push(rhs);
                args.extend_from_slice(extra);
                self.apply(lhs, &args)
            }
            lf::r#type::Sum::Var(var) => {
                if !extra.is_empty() {
                    return Err(LowerError::new("type-variable application is unsupported"));
                }
                let name = interned_str(self.package, var.var_interned_str)
                    .ok_or_else(|| LowerError::new("unresolved type variable"))?
                    .to_string();
                Ok(DamlType::Var(name))
            }
            lf::r#type::Sum::Con(con) => {
                let tycon = con
                    .tycon
                    .as_ref()
                    .ok_or_else(|| LowerError::new("type constructor without a name"))?;
                let path = self.con_path(tycon)?;
                let args = con
                    .args
                    .iter()
                    .chain(extra.iter().copied())
                    .map(|arg| self.type_(arg))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(DamlType::Ref(TypeRef { path, args }))
            }
            lf::r#type::Sum::Builtin(builtin) => {
                let args: Vec<&lf::Type> =
                    builtin.args.iter().chain(extra.iter().copied()).collect();
                self.builtin(builtin.builtin, &args)
            }
            lf::r#type::Sum::Interned(index) => {
                let resolved = interned_type(self.package, *index)
                    .ok_or_else(|| LowerError::new("unresolved interned type"))?;
                self.apply(resolved, extra)
            }
            lf::r#type::Sum::Nat(_) => Err(LowerError::new("unexpected bare type-level Nat")),
            lf::r#type::Sum::Forall(_) | lf::r#type::Sum::Struct(_) | lf::r#type::Sum::Syn(_) => {
                Err(LowerError::new(
                    "unsupported LF type (forall / struct / syn)",
                ))
            }
        }
    }

    /// The Rust path segments for a type constructor reference. Local when not
    /// qualifying; otherwise `["crate", <package>, <module>, <Type>]`. The target
    /// package is resolved from the reference's package identity, which comes in
    /// three forms: **self**, an **imported** package by its interned id-hash, or
    /// a Smart Contract Upgrade reference that names the package (resolved to the
    /// bundled version via the referenced module).
    fn con_path(&self, tycon: &lf::TypeConId) -> Result<Vec<String>, LowerError> {
        use lf::self_or_imported_package_id::Sum;

        let type_name = rust_name(self.package, tycon.name_interned_dname)?;
        let Some(qualify) = &self.qualify else {
            return Ok(vec![type_name]);
        };

        let module_id = tycon
            .module
            .as_ref()
            .ok_or_else(|| LowerError::new("type constructor without a module"))?;
        let module_dotted =
            interned_dotted_name(self.package, module_id.module_name_interned_dname)
                .ok_or_else(|| LowerError::new("unresolved referenced module name"))?;

        // Every reference form ultimately names a package by its id hash;
        // resolve to that hash, then to the package's Rust module name.
        let target_id = match module_id.package_id.as_ref().and_then(|p| p.sum.as_ref()) {
            // A self reference, or an absent package id, targets this package.
            None | Some(Sum::SelfPackageId(_)) => qualify.current_id.to_string(),
            // An imported package identified by its id-hash, interned here.
            Some(Sum::ImportedPackageIdInternedStr(index)) => interned_str(self.package, *index)
                .ok_or_else(|| LowerError::new("unresolved imported package id"))?
                .to_string(),
            // Newer LF: an index into the package's explicit import table, whose
            // entry is the target package's id hash.
            Some(Sum::PackageImportId(index)) => imported_package_id(self.package, *index)
                .ok_or_else(|| LowerError::new("unresolved package import id"))?
                .to_string(),
        };
        let package_module = qualify
            .module_names
            .get(target_id.as_str())
            .ok_or_else(|| {
                LowerError::new(format!("reference to package {target_id} not in the DAR"))
            })?;

        Ok(vec![
            "crate".to_string(),
            package_module.clone(),
            module_dotted.replace('.', "_"),
            type_name,
        ])
    }

    fn builtin(&self, builtin: i32, args: &[&lf::Type]) -> Result<DamlType, LowerError> {
        use lf::BuiltinType;

        let kind = BuiltinType::try_from(builtin)
            .map_err(|_| LowerError::new(format!("unknown builtin type {builtin}")))?;
        let arg = |index: usize| -> Result<DamlType, LowerError> {
            let ty = args.get(index).copied().ok_or_else(|| {
                LowerError::new(format!("{kind:?} missing type argument {index}"))
            })?;
            self.type_(ty)
        };

        let ty = match kind {
            BuiltinType::Unit => DamlType::Unit,
            BuiltinType::Bool => DamlType::Bool,
            BuiltinType::Int64 => DamlType::Int64,
            BuiltinType::Date => DamlType::Date,
            BuiltinType::Timestamp => DamlType::Timestamp,
            BuiltinType::Party => DamlType::Party,
            BuiltinType::Text => DamlType::Text,
            BuiltinType::Numeric => DamlType::Numeric(self.numeric_scale(args)?),
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
    fn numeric_scale(&self, args: &[&lf::Type]) -> Result<u8, LowerError> {
        let mut ty = *args
            .first()
            .ok_or_else(|| LowerError::new("Numeric without a scale argument"))?;
        // Follow one level of interning if the scale is stored in the type table.
        if let Some(lf::r#type::Sum::Interned(index)) = &ty.sum {
            ty = interned_type(self.package, *index)
                .ok_or_else(|| LowerError::new("unresolved Numeric scale"))?;
        }
        match &ty.sum {
            Some(lf::r#type::Sum::Nat(scale)) => u8::try_from(*scale)
                .map_err(|_| LowerError::new(format!("Numeric scale {scale} out of range"))),
            _ => Err(LowerError::new("Numeric scale is not a type-level Nat")),
        }
    }
}

/// Box the top-level references a data type makes to *itself*, giving directly
/// recursive types the indirection Rust requires (Daml permits a type to contain
/// itself directly; Rust needs a `Box`). Self-references already behind a
/// `List` / `Optional` / map have indirection and are left alone.
fn box_self_recursion(module: &mut Module, package_module: &str, module_name: &str) {
    for data_type in &mut module.data_types {
        match data_type {
            DataType::Record(record) => {
                let self_path = self_path(package_module, module_name, &record.name);
                for field in &mut record.fields {
                    box_if_self(&mut field.ty, &self_path);
                }
            }
            DataType::Variant(variant) => {
                let self_path = self_path(package_module, module_name, &variant.name);
                for constructor in &mut variant.constructors {
                    if let Some(payload) = &mut constructor.payload {
                        box_if_self(payload, &self_path);
                    }
                }
            }
            DataType::Enum(_) | DataType::InterfaceMarker(_) => {}
        }
    }
}

/// The fully-qualified path a data type refers to itself by.
fn self_path(package_module: &str, module_name: &str, type_name: &str) -> Vec<String> {
    vec![
        "crate".to_string(),
        package_module.to_string(),
        module_name.to_string(),
        type_name.to_string(),
    ]
}

/// Wrap `ty` in a `Box` if it is a direct reference to `self_path`.
fn box_if_self(ty: &mut DamlType, self_path: &[String]) {
    if let DamlType::Ref(reference) = ty
        && reference.path == self_path
    {
        let inner = std::mem::replace(ty, DamlType::Unit);
        *ty = DamlType::Boxed(Box::new(inner));
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
    use canton_lf::Dar;

    #[test]
    fn lowers_a_real_dar_to_a_qualified_crate() {
        // Env-gated: point at a real .dar (e.g. one from cn-quickstart).
        let Ok(path) = std::env::var("CANTON_TEST_DAR") else {
            eprintln!("skipping lower test: set CANTON_TEST_DAR=/path/to/x.dar");
            return;
        };

        let dar = Dar::open(&path).expect("open DAR");
        let (krate, errors) = lower_dar(&dar).expect("decode + lower DAR");

        assert!(
            !krate.packages.is_empty(),
            "a DAR should lower at least one package"
        );

        // Every reference must resolve to a package present in the DAR closure —
        // if the imported-package-id interning were wrong, this would surface as
        // a "reference to package … not in the DAR" error.
        let unresolved: Vec<&LowerError> = errors
            .iter()
            .filter(|error| error.0.contains("not in the DAR"))
            .collect();
        assert!(
            unresolved.is_empty(),
            "every cross-package reference should resolve: {unresolved:?}"
        );

        // The lowered IR generates syntactically valid Rust end to end.
        let src = crate::generate_crate(&krate).expect("generate crate");
        syn::parse_file(&src).expect("generated source must be valid Rust");

        let modules: usize = krate.packages.iter().map(|p| p.modules.len()).sum();
        let types: usize = krate
            .packages
            .iter()
            .flat_map(|p| &p.modules)
            .map(|m| m.module.data_types.len())
            .sum();
        println!(
            "lowered {} packages / {modules} modules / {types} data types → {} bytes of Rust ({} skipped)",
            krate.packages.len(),
            src.len(),
            errors.len(),
        );
    }
}
