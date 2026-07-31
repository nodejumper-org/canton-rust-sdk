//! The AST→IR bridge: lower decoded Daml-LF [`Package`](lf::Package)s into the
//! codegen [`ir`](crate::ir).
//!
//! This is the one module that touches the LF AST; everything else stays
//! decoder-agnostic. Serializable records, variants, and enums lower fully, as
//! do **templates** (payload record, choices, contract key, on-ledger id) and
//! **interfaces** (view type, choices, and the identity choices are exercised
//! through — on the marker struct emitted from the interface's data type). Field
//! types cover the LF builtins, references to named types, and type variables;
//! unsupported LF type shapes yield a [`SkippedType`] rather than silently-wrong
//! output.
//!
//! # Qualified references (PackageMap)
//!
//! A DAR bundles a package and its whole dependency closure. A type reference
//! (`Con`) carries the target module *and* package (self, or an imported
//! package identified by its id-hash). Lowering a **whole DAR**
//! resolves every reference to a fully-qualified Rust path —
//! `crate::<package>::<module>::<Type>` — so cross-module and cross-package
//! references resolve and names from different modules can never collide.

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

/// A declaration that could not be lowered into the IR, and why.
///
/// Lowering is best-effort: an LF shape the codegen cannot represent (a type
/// name that is not a Rust identifier, a non-serializable builtin, two fields
/// that would collide after snake-casing) is skipped and reported here rather
/// than failing the whole DAR or emitting something wrong. Callers surface
/// these as warnings — see [`crate::lower_dar`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SkippedType {
    module: String,
    reason: String,
}

impl SkippedType {
    fn new(reason: impl Into<String>) -> Self {
        Self {
            module: String::new(),
            reason: reason.into(),
        }
    }

    /// Attribute the skip to the Daml module it arose in, so the warning tells
    /// the user *where* to look.
    fn in_module(mut self, module: &str) -> Self {
        self.module = module.to_string();
        self
    }

    /// The dotted Daml module the skipped declaration lives in, or `""` when
    /// the skip could not be attributed to one.
    #[must_use]
    pub fn module(&self) -> &str {
        &self.module
    }

    /// Why the declaration was skipped.
    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

impl std::fmt::Display for SkippedType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.module.is_empty() {
            f.write_str(&self.reason)
        } else {
            write!(f, "{}: {}", self.module, self.reason)
        }
    }
}

impl std::error::Error for SkippedType {}

/// Lower a whole DAR (its main package and its full dependency closure) into a
/// [`Crate`] of qualified modules, plus the list of types that could not be
/// lowered (best-effort: see [`SkippedType`]).
///
/// # Errors
/// Returns [`DecodeError`] if any package's bytes are malformed or not LF 2.x.
pub fn lower_dar(dar: &Dar) -> Result<(Crate, Vec<SkippedType>), DecodeError> {
    Ok(lower_crate(&decode_all(dar)?))
}

/// Lower a set of decoded packages (a DAR's closure, each with its package id)
/// into a [`Crate`]: one Rust module per package, one submodule per Daml module,
/// with every type reference resolved to a fully-qualified path.
///
/// Best-effort: a type that cannot be lowered is skipped and its error recorded,
/// so a package with a few unsupported types still produces output.
#[must_use]
pub(crate) fn lower_crate(packages: &[(String, lf::Package)]) -> (Crate, Vec<SkippedType>) {
    // package id → the Rust module name that package's types live under.
    let mut module_names: HashMap<&str, String> = packages
        .iter()
        .map(|(id, package)| (id.as_str(), package_module_name(package, id)))
        .collect();

    // Two different packages can carry the same name+version (a rebuilt package
    // gets a new id but identical metadata); emitting two same-named `pub mod`s
    // would not compile. Disambiguate every colliding module name with the
    // package-id prefix — deterministic, and references resolve through the id.
    let mut by_name: HashMap<&str, usize> = HashMap::new();
    for name in module_names.values() {
        *by_name.entry(name.as_str()).or_default() += 1;
    }
    let colliding: std::collections::HashSet<String> = by_name
        .into_iter()
        .filter(|(_, count)| *count > 1)
        .map(|(name, _)| name.to_string())
        .collect();
    for (id, name) in &mut module_names {
        if colliding.contains(name.as_str()) {
            *name = format!("{name}_{}", ident_prefix(id));
        }
    }

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
                errors.push(SkippedType::new("unresolved module name"));
                continue;
            };
            let ir_module = lowering.module(lf_module, &dotted, &mut errors);
            if !ir_module.data_types.is_empty()
                || !ir_module.templates.is_empty()
                || !ir_module.interfaces.is_empty()
            {
                package_module.modules.push(NamedModule {
                    name: dotted.replace('.', "_"),
                    module: ir_module,
                });
            }
        }
        if !package_module.modules.is_empty() {
            krate.packages.push(package_module);
        }
    }

    // With every package lowered, break containment cycles crate-wide (direct,
    // Optional-wrapped, mutual, cross-module, and generic-instantiation ones).
    box_recursion(&mut krate);

    (krate, errors)
}

/// The Rust module name a package's types live under: its `name_version` (both
/// sanitised to identifier-safe characters), or `p_<hash8>` when metadata is
/// missing. Version is included so a DAR bundling two versions of one package
/// (Smart Contract Upgrade) gets two distinct modules.
fn package_module_name(package: &lf::Package, package_id: &str) -> String {
    match (package_name(package), package_version(package)) {
        (Some(name), Some(version)) => format!("{}_{}", sanitize(name), sanitize(version)),
        (Some(name), None) => sanitize(name),
        _ => format!("p_{}", ident_prefix(package_id)),
    }
}

/// The first 8 identifier-safe characters of a package id, for `p_<hash8>` /
/// collision-suffix module names. Character-based (a byte slice could panic on
/// a hostile non-ASCII id) and filtered to alphanumerics.
fn ident_prefix(package_id: &str) -> String {
    package_id
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .take(8)
        .collect()
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
        errors: &mut Vec<SkippedType>,
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
                Err(error) => errors.push(error.in_module(module_dotted)),
            }
        }

        for def in &lf_module.templates {
            match self.template(def, module_dotted, &mut payloads) {
                Ok(template) => module.templates.push(template),
                Err(error) => errors.push(error.in_module(module_dotted)),
            }
        }

        for def in &lf_module.interfaces {
            match self.interface(def, module_dotted) {
                Ok(interface) => module.interfaces.push(interface),
                Err(error) => errors.push(error.in_module(module_dotted)),
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
    ) -> Result<Interface, SkippedType> {
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
    ) -> Result<Template, SkippedType> {
        let name = rust_name(self.package, def.tycon_interned_dname)?;
        let fields = payloads.remove(&name).ok_or_else(|| {
            SkippedType::new(format!("template {name}: payload record not found"))
        })?;
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
    fn choice(&self, choice: &lf::TemplateChoice) -> Result<Choice, SkippedType> {
        let name = interned_str(self.package, choice.name_interned_str)
            .ok_or_else(|| SkippedType::new("unresolved choice name"))?
            .to_string();
        let argument = choice
            .arg_binder
            .as_ref()
            .and_then(|binder| binder.r#type.as_ref())
            .ok_or_else(|| SkippedType::new(format!("choice {name}: no argument type")))?;
        let returns = choice
            .ret_type
            .as_ref()
            .ok_or_else(|| SkippedType::new(format!("choice {name}: no return type")))?;
        Ok(Choice {
            name,
            consuming: choice.consuming,
            argument: self.type_(argument)?,
            returns: self.type_(returns)?,
        })
    }

    /// Lower one `DefDataType`. `Ok(None)` when intentionally skipped
    /// (non-serializable, or an interface view marker).
    fn data_type(&self, data_type: &lf::DefDataType) -> Result<Option<DataType>, SkippedType> {
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
                    .ok_or_else(|| SkippedType::new("unresolved type parameter"))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let Some(data_cons) = &data_type.data_cons else {
            return Err(SkippedType::new(format!("{name}: no data constructor")));
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
                            .ok_or_else(|| SkippedType::new("unresolved enum constructor"))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(Some(DataType::Enum(Enum { name, constructors })))
            }
            // An interface's view marker — reserved, lowered separately later.
            lf::def_data_type::DataCons::Interface(_) => Ok(None),
        }
    }

    fn fields(&self, fields: &[lf::FieldWithType]) -> Result<Vec<Field>, SkippedType> {
        let lowered = fields
            .iter()
            .map(|field| {
                let label = interned_str(self.package, field.field_interned_str)
                    .ok_or_else(|| SkippedType::new("unresolved field label"))?
                    .to_string();
                let ty = self.type_(field_type(field)?)?;
                Ok(Field { label, ty })
            })
            .collect::<Result<Vec<Field>, SkippedType>>()?;

        // Two Daml labels can collapse onto one Rust field name (`fooBar` and
        // `foo_bar` both snake_case to `foo_bar`); the emitted struct would not
        // compile. Fail the type with a clear message instead.
        let mut by_rust_name: HashMap<String, &str> = HashMap::new();
        for field in &lowered {
            let rust = heck::ToSnakeCase::to_snake_case(field.label.as_str());
            if let Some(previous) = by_rust_name.insert(rust.clone(), &field.label) {
                return Err(SkippedType::new(format!(
                    "fields `{previous}` and `{}` both map to the Rust field `{rust}`",
                    field.label
                )));
            }
        }
        Ok(lowered)
    }

    fn variant_constructor(
        &self,
        field: &lf::FieldWithType,
    ) -> Result<VariantConstructor, SkippedType> {
        let name = interned_str(self.package, field.field_interned_str)
            .ok_or_else(|| SkippedType::new("unresolved variant constructor"))?
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
    fn type_(&self, ty: &lf::Type) -> Result<DamlType, SkippedType> {
        self.apply(ty, &[])
    }

    /// Lower `ty` applied to `extra` type arguments. This unifies the flattened
    /// form (`Con`/`Builtin` with an `args` list) and the curried `TApp` form
    /// (LF 2.dev): `TApp(lhs, rhs)` applies `lhs` to `rhs` prepended to `extra`,
    /// so `((f a) b)` collapses to `f` applied to `[a, b]` regardless of shape.
    fn apply(&self, ty: &lf::Type, extra: &[&lf::Type]) -> Result<DamlType, SkippedType> {
        let Some(sum) = &ty.sum else {
            return Err(SkippedType::new("empty type"));
        };
        match sum {
            lf::r#type::Sum::Tapp(app) => {
                let lhs = app
                    .lhs
                    .as_deref()
                    .ok_or_else(|| SkippedType::new("type application without a function"))?;
                let rhs = app
                    .rhs
                    .as_deref()
                    .ok_or_else(|| SkippedType::new("type application without an argument"))?;
                let mut args = Vec::with_capacity(extra.len() + 1);
                args.push(rhs);
                args.extend_from_slice(extra);
                self.apply(lhs, &args)
            }
            lf::r#type::Sum::Var(var) => {
                if !extra.is_empty() {
                    return Err(SkippedType::new("type-variable application is unsupported"));
                }
                let name = interned_str(self.package, var.var_interned_str)
                    .ok_or_else(|| SkippedType::new("unresolved type variable"))?
                    .to_string();
                Ok(DamlType::Var(name))
            }
            lf::r#type::Sum::Con(con) => {
                let tycon = con
                    .tycon
                    .as_ref()
                    .ok_or_else(|| SkippedType::new("type constructor without a name"))?;
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
                    .ok_or_else(|| SkippedType::new("unresolved interned type"))?;
                self.apply(resolved, extra)
            }
            lf::r#type::Sum::Nat(_) => Err(SkippedType::new("unexpected bare type-level Nat")),
            lf::r#type::Sum::Forall(_) | lf::r#type::Sum::Struct(_) | lf::r#type::Sum::Syn(_) => {
                Err(SkippedType::new(
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
    fn con_path(&self, tycon: &lf::TypeConId) -> Result<Vec<String>, SkippedType> {
        use lf::self_or_imported_package_id::Sum;

        let type_name = rust_name(self.package, tycon.name_interned_dname)?;
        let Some(qualify) = &self.qualify else {
            return Ok(vec![type_name]);
        };

        let module_id = tycon
            .module
            .as_ref()
            .ok_or_else(|| SkippedType::new("type constructor without a module"))?;
        let module_dotted =
            interned_dotted_name(self.package, module_id.module_name_interned_dname)
                .ok_or_else(|| SkippedType::new("unresolved referenced module name"))?;

        // Every reference form ultimately names a package by its id hash;
        // resolve to that hash, then to the package's Rust module name.
        let target_id = match module_id.package_id.as_ref().and_then(|p| p.sum.as_ref()) {
            // A self reference, or an absent package id, targets this package.
            None | Some(Sum::SelfPackageId(_)) => qualify.current_id.to_string(),
            // An imported package identified by its id-hash, interned here.
            Some(Sum::ImportedPackageIdInternedStr(index)) => interned_str(self.package, *index)
                .ok_or_else(|| SkippedType::new("unresolved imported package id"))?
                .to_string(),
            // Newer LF: an index into the package's explicit import table, whose
            // entry is the target package's id hash.
            Some(Sum::PackageImportId(index)) => imported_package_id(self.package, *index)
                .ok_or_else(|| SkippedType::new("unresolved package import id"))?
                .to_string(),
        };
        let package_module = qualify
            .module_names
            .get(target_id.as_str())
            .ok_or_else(|| {
                SkippedType::new(format!("reference to package {target_id} not in the DAR"))
            })?;

        Ok(vec![
            "crate".to_string(),
            package_module.clone(),
            module_dotted.replace('.', "_"),
            type_name,
        ])
    }

    fn builtin(&self, builtin: i32, args: &[&lf::Type]) -> Result<DamlType, SkippedType> {
        use lf::BuiltinType;

        let kind = BuiltinType::try_from(builtin)
            .map_err(|_| SkippedType::new(format!("unknown builtin type {builtin}")))?;
        let arg = |index: usize| -> Result<DamlType, SkippedType> {
            let ty = args.get(index).copied().ok_or_else(|| {
                SkippedType::new(format!("{kind:?} missing type argument {index}"))
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
                return Err(SkippedType::new(format!(
                    "builtin type {other:?} is not representable in codegen"
                )));
            }
        };
        Ok(ty)
    }

    /// The scale of a `Numeric n` from its type-level `Nat` argument.
    fn numeric_scale(&self, args: &[&lf::Type]) -> Result<u8, SkippedType> {
        let mut ty = *args
            .first()
            .ok_or_else(|| SkippedType::new("Numeric without a scale argument"))?;
        // Follow one level of interning if the scale is stored in the type table.
        if let Some(lf::r#type::Sum::Interned(index)) = &ty.sum {
            ty = interned_type(self.package, *index)
                .ok_or_else(|| SkippedType::new("unresolved Numeric scale"))?;
        }
        match &ty.sum {
            Some(lf::r#type::Sum::Nat(scale)) => u8::try_from(*scale)
                .map_err(|_| SkippedType::new(format!("Numeric scale {scale} out of range"))),
            _ => Err(SkippedType::new("Numeric scale is not a type-level Nat")),
        }
    }
}

// ---- recursion breaking -----------------------------------------------------
//
// Daml permits arbitrarily recursive data types; Rust requires indirection on
// every cycle. `Vec` / `BTreeMap` / `GenMap` heap-allocate and so break cycles,
// but `Option<T>` stores `T` **inline** — `Optional` recursion (`data Tree =
// Node { left : Optional Tree }`), mutual recursion (`A` ↔ `B`, including
// across modules), and recursion through a generic instantiation (`Wrap T`
// where `Wrap a` stores `a` inline) all need a `Box`, or the generated crate
// fails to compile (E0072). `Box` is transparent to both codecs, so boxing is
// always safe; the pass below is therefore deliberately conservative — it may
// box an occurrence that a finer analysis could leave bare, but it can never
// produce an infinitely-sized type.

/// Break every containment cycle in the crate by boxing the reference
/// occurrences that close one.
///
/// A type "inline-contains" the types its fields reach without crossing heap
/// indirection: through `Optional`, and through the *arguments* of a named-type
/// reference (a generic target may store its parameter inline — assumed
/// conservatively). `List` / `TextMap` / `GenMap` / `Boxed` stop containment
/// (heap), as does `ContractId` (a phantom-typed id, it does not contain its
/// payload). Every reference occurrence whose target can inline-reach back to
/// the type that holds the field is wrapped in [`DamlType::Boxed`].
fn box_recursion(krate: &mut Crate) {
    // Pass 1: the inline-containment graph, node = fully-qualified path key.
    let mut edges: HashMap<String, Vec<String>> = HashMap::new();
    for package in &krate.packages {
        for module in &package.modules {
            let at = |name: &str| path_key_for(&package.name, &module.name, name);
            module_containment_edges(&module.module, &at, &mut edges);
        }
    }

    // Pass 2: box every reference occurrence that closes a cycle back to the
    // type holding it.
    for package in &mut krate.packages {
        let package_name = package.name.clone();
        for module in &mut package.modules {
            let module_name = module.name.clone();
            let at = |name: &str| path_key_for(&package_name, &module_name, name);
            module_box_cycles(&mut module.module, &at, &edges);
        }
    }
}

/// Pass 1 for one module: record which reference keys each declared type
/// inline-contains. `at` maps a declared type name to its graph key.
fn module_containment_edges(
    module: &Module,
    at: &impl Fn(&str) -> String,
    edges: &mut HashMap<String, Vec<String>>,
) {
    for data_type in &module.data_types {
        match data_type {
            DataType::Record(record) => {
                let node = edges.entry(at(&record.name)).or_default();
                for field in &record.fields {
                    collect_inline_refs(&field.ty, node);
                }
            }
            DataType::Variant(variant) => {
                let node = edges.entry(at(&variant.name)).or_default();
                for constructor in &variant.constructors {
                    if let Some(payload) = &constructor.payload {
                        collect_inline_refs(payload, node);
                    }
                }
            }
            DataType::Enum(_) | DataType::InterfaceMarker(_) => {}
        }
    }
    // Template payloads are types too (other types may reference them).
    for template in &module.templates {
        let node = edges.entry(at(&template.name)).or_default();
        for field in &template.fields {
            collect_inline_refs(&field.ty, node);
        }
    }
}

/// Pass 2 for one module: box every cycle-closing reference occurrence.
fn module_box_cycles(
    module: &mut Module,
    at: &impl Fn(&str) -> String,
    edges: &HashMap<String, Vec<String>>,
) {
    for data_type in &mut module.data_types {
        match data_type {
            DataType::Record(record) => {
                let holder = at(&record.name);
                for field in &mut record.fields {
                    box_cycle_closers(&mut field.ty, &holder, edges);
                }
            }
            DataType::Variant(variant) => {
                let holder = at(&variant.name);
                for constructor in &mut variant.constructors {
                    if let Some(payload) = &mut constructor.payload {
                        box_cycle_closers(payload, &holder, edges);
                    }
                }
            }
            DataType::Enum(_) | DataType::InterfaceMarker(_) => {}
        }
    }
    for template in &mut module.templates {
        let holder = at(&template.name);
        for field in &mut template.fields {
            box_cycle_closers(&mut field.ty, &holder, edges);
        }
    }
}

/// The graph key of a type declared in (`package`, `module`) — the same shape
/// [`Lowering::con_path`] resolves references to, joined.
fn path_key_for(package_module: &str, module_name: &str, type_name: &str) -> String {
    format!("crate::{package_module}::{module_name}::{type_name}")
}

/// The graph key of a reference occurrence. Local (unqualified) references have
/// a single segment; qualified ones are `crate::pkg::module::Type`.
fn path_key_of(reference: &TypeRef) -> String {
    reference.path.join("::")
}

/// Collect into `out` the reference keys `ty` reaches inline (without crossing
/// heap indirection).
fn collect_inline_refs(ty: &DamlType, out: &mut Vec<String>) {
    match ty {
        DamlType::Optional(inner) => collect_inline_refs(inner, out),
        DamlType::Ref(reference) => {
            out.push(path_key_of(reference));
            // A generic target may store its arguments inline (conservative).
            for arg in &reference.args {
                collect_inline_refs(arg, out);
            }
        }
        // Heap containers / phantom ids stop inline containment; the rest of
        // the leaf types contain no references.
        _ => {}
    }
}

/// True if `from` can reach `to` through the inline-containment graph.
fn inline_reaches(edges: &HashMap<String, Vec<String>>, from: &str, to: &str) -> bool {
    let mut stack = vec![from];
    let mut seen = std::collections::HashSet::new();
    while let Some(node) = stack.pop() {
        if node == to {
            return true;
        }
        if seen.insert(node)
            && let Some(next) = edges.get(node)
        {
            stack.extend(next.iter().map(String::as_str));
        }
    }
    false
}

/// Box, in place, every reference occurrence inside `ty` (at an inline
/// position) whose target inline-reaches `holder` — i.e. every occurrence that
/// closes a containment cycle.
fn box_cycle_closers(ty: &mut DamlType, holder: &str, edges: &HashMap<String, Vec<String>>) {
    match ty {
        DamlType::Optional(inner) => box_cycle_closers(inner, holder, edges),
        DamlType::Ref(reference) => {
            for arg in &mut reference.args {
                box_cycle_closers(arg, holder, edges);
            }
            let key = path_key_of(reference);
            if key == holder || inline_reaches(edges, &key, holder) {
                let inner = std::mem::replace(ty, DamlType::Unit);
                *ty = DamlType::Boxed(Box::new(inner));
            }
        }
        _ => {}
    }
}

/// The `Type` of a field, or an error if absent.
fn field_type(field: &lf::FieldWithType) -> Result<&lf::Type, SkippedType> {
    field
        .r#type
        .as_ref()
        .ok_or_else(|| SkippedType::new("field without a type"))
}

/// Resolve an interned dotted type name into a Rust-usable identifier: the Daml
/// qualified name with `.` replaced by `_` (Daml type names are otherwise valid
/// Rust identifiers). Definitions and references use this same mapping, so they
/// agree. A name that is not a valid Rust identifier (LF permits characters
/// like `$` in compiler-internal names) is a [`SkippedType`], not a panic in
/// the emitter.
fn rust_name(package: &lf::Package, name_interned_dname: i32) -> Result<String, SkippedType> {
    let dotted = interned_dotted_name(package, name_interned_dname)
        .ok_or_else(|| SkippedType::new("unresolved dotted name"))?;
    let name = dotted.replace('.', "_");
    let valid_ident = !name.is_empty()
        && !name.starts_with(|c: char| c.is_ascii_digit())
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
    if valid_ident {
        Ok(name)
    } else {
        Err(SkippedType::new(format!(
            "`{dotted}` is not representable as a Rust identifier"
        )))
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use canton_lf::Dar;

    // ---- recursion breaking (always-on, pure IR) ---------------------------

    fn qualified(name: &str) -> DamlType {
        DamlType::Ref(TypeRef {
            path: vec![
                "crate".to_string(),
                "pkg_1_0_0".to_string(),
                "Mod".to_string(),
                name.to_string(),
            ],
            args: vec![],
        })
    }

    fn record(name: &str, fields: Vec<(&str, DamlType)>) -> DataType {
        DataType::Record(Record {
            name: name.to_string(),
            type_params: vec![],
            fields: fields
                .into_iter()
                .map(|(label, ty)| Field {
                    label: label.to_string(),
                    ty,
                })
                .collect(),
        })
    }

    fn crate_of(data_types: Vec<DataType>) -> Crate {
        Crate {
            packages: vec![PackageModule {
                name: "pkg_1_0_0".to_string(),
                modules: vec![NamedModule {
                    name: "Mod".to_string(),
                    module: Module {
                        data_types,
                        ..Module::default()
                    },
                }],
            }],
        }
    }

    fn field_ty(krate: &Crate, type_index: usize, field_index: usize) -> &DamlType {
        match &krate.packages[0].modules[0].module.data_types[type_index] {
            DataType::Record(record) => &record.fields[field_index].ty,
            other => panic!("expected a record, got {other:?}"),
        }
    }

    #[test]
    fn optional_self_recursion_is_boxed() {
        // data Tree = Node { left : Optional Tree } — `Option<Tree>` stores the
        // payload inline, so without a Box the generated struct is E0072.
        let mut krate = crate_of(vec![record(
            "Tree",
            vec![
                ("left", DamlType::Optional(Box::new(qualified("Tree")))),
                ("size", DamlType::Int64),
            ],
        )]);
        box_recursion(&mut krate);
        assert_eq!(
            field_ty(&krate, 0, 0),
            &DamlType::Optional(Box::new(DamlType::Boxed(Box::new(qualified("Tree"))))),
        );
        // Non-recursive fields are untouched.
        assert_eq!(field_ty(&krate, 0, 1), &DamlType::Int64);
    }

    #[test]
    fn mutual_recursion_is_boxed() {
        // data A = A { b : B }; data B = B { a : Optional A } — the cycle is
        // A → B → A; every occurrence that closes it must be boxed.
        let mut krate = crate_of(vec![
            record("A", vec![("b", qualified("B"))]),
            record(
                "B",
                vec![("a", DamlType::Optional(Box::new(qualified("A"))))],
            ),
        ]);
        box_recursion(&mut krate);
        assert_eq!(
            field_ty(&krate, 0, 0),
            &DamlType::Boxed(Box::new(qualified("B")))
        );
        assert_eq!(
            field_ty(&krate, 1, 0),
            &DamlType::Optional(Box::new(DamlType::Boxed(Box::new(qualified("A"))))),
        );
    }

    #[test]
    fn generic_instantiation_recursion_is_boxed() {
        // data T = T { w : Wrap T } — Wrap may store its parameter inline, so
        // the argument occurrence is (conservatively) boxed: Wrap<Box<T>>.
        let wrap_of_t = DamlType::Ref(TypeRef {
            path: vec![
                "crate".to_string(),
                "pkg_1_0_0".to_string(),
                "Mod".to_string(),
                "Wrap".to_string(),
            ],
            args: vec![qualified("T")],
        });
        let mut krate = crate_of(vec![
            DataType::Record(Record {
                name: "Wrap".to_string(),
                type_params: vec!["a".to_string()],
                fields: vec![Field {
                    label: "w".to_string(),
                    ty: DamlType::Var("a".to_string()),
                }],
            }),
            record("T", vec![("w", wrap_of_t)]),
        ]);
        box_recursion(&mut krate);
        let DamlType::Ref(reference) = field_ty(&krate, 1, 0) else {
            panic!("outer Wrap reference must stay a Ref");
        };
        assert_eq!(
            reference.args[0],
            DamlType::Boxed(Box::new(qualified("T"))),
            "the recursive argument occurrence is boxed"
        );
    }

    #[test]
    fn heap_containers_already_break_cycles() {
        // Vec/TextMap/GenMap heap-allocate; recursion through them needs no Box.
        let mut krate = crate_of(vec![record(
            "Tree",
            vec![
                ("children", DamlType::List(Box::new(qualified("Tree")))),
                ("index", DamlType::TextMap(Box::new(qualified("Tree")))),
            ],
        )]);
        let before = krate.clone();
        box_recursion(&mut krate);
        assert_eq!(krate, before, "heap-indirected recursion is left alone");
    }

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
        let unresolved: Vec<&SkippedType> = errors
            .iter()
            .filter(|error| error.reason().contains("not in the DAR"))
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
