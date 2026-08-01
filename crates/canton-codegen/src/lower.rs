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
    let module_names = package_module_names(packages);

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
        // The same flattening applies to module names: `A.B` and `A_B` would
        // both become `pub mod A_B`. Detect it here rather than emitting a
        // crate that fails to compile.
        let colliding_modules = colliding_module_names(package);

        for lf_module in &package.modules {
            let Some(dotted) = interned_dotted_name(package, lf_module.name_interned_dname) else {
                errors.push(SkippedType::new("unresolved module name"));
                continue;
            };
            let module_ident = dotted.replace('.', "_");
            if !is_rust_ident(&module_ident) {
                errors.push(SkippedType::new(format!(
                    "module `{dotted}` is not representable as a Rust module name"
                )));
                continue;
            }
            if let Some(sources) = colliding_modules.get(&module_ident) {
                errors.push(SkippedType::new(format!(
                    "modules `{}` both map to the Rust module `{}`; \
                     rename one in Daml to generate either",
                    sources.join("` and `"),
                    dotted.replace('.', "_"),
                )));
                continue;
            }
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

    // An `Optional` handed to a generic that nests it under its own `Optional`
    // has no correct JSON form here — reject those before they ship.
    drop_ambiguous_nested_optionals(&mut krate, &mut errors);

    // A skipped declaration leaves its name unresolvable, so anything that
    // referenced it cannot be emitted either. Drop those too, transitively.
    drop_dangling_references(&mut krate, &mut errors);

    // With every package lowered, break containment cycles crate-wide (direct,
    // Optional-wrapped, mutual, cross-module, and generic-instantiation ones).
    box_recursion(&mut krate);

    (krate, errors)
}

/// Reject a generic instantiation whose `Optional` argument lands under the
/// target's own `Optional`.
///
/// LF-JSON encodes a top-level `Optional` as `null`/value but every `Optional`
/// below one as a list (`[]` / `[x]`), which is why the mapping turns
/// `Optional (Optional t)` into `Option<NestedOpt<T>>`. That rewrite is
/// structural, so it cannot see through a type parameter: for
/// `data Wrap a = Wrap { value : Optional a }` instantiated at
/// `Wrap (Optional Text)`, the nesting exists on the wire but not in the
/// spelled-out type, and the emitted `Option<Option<String>>` would serialize
/// the inner layer as `null` instead of `[]`.
///
/// Encoding that correctly needs the instantiation site to know how the target
/// uses its parameter, and a distinct IR node for "this `Optional` is nested".
/// Until then the case is refused rather than emitted wrong — it does not occur
/// anywhere in the Splice corpus, so nothing real is lost.
/// For each generic data type, which of its declared parameters appear under an
/// `Optional` in its own fields — the question that decides whether an
/// `Optional` type argument lands in a nested position.
fn parameters_used_under_optional(krate: &Crate) -> HashMap<String, Vec<bool>> {
    // path key -> for each declared parameter, whether it is used under an
    // `Optional` in that type's own fields.
    let mut nests_parameter: HashMap<String, Vec<bool>> = HashMap::new();
    for package in &krate.packages {
        for module in &package.modules {
            for data_type in &module.module.data_types {
                let (name, params, types) = match data_type {
                    DataType::Record(record) => (
                        &record.name,
                        &record.type_params,
                        record
                            .fields
                            .iter()
                            .map(|field| &field.ty)
                            .collect::<Vec<_>>(),
                    ),
                    DataType::Variant(variant) => (
                        &variant.name,
                        &variant.type_params,
                        variant
                            .constructors
                            .iter()
                            .filter_map(|constructor| constructor.payload.as_ref())
                            .collect(),
                    ),
                    DataType::Enum(_) | DataType::InterfaceMarker(_) => continue,
                };
                if params.is_empty() {
                    continue;
                }
                let mut under_optional = std::collections::BTreeSet::new();
                for ty in types {
                    collect_vars_under_optional(ty, false, &mut under_optional);
                }
                nests_parameter.insert(
                    path_key_for(&package.name, &module.name, name),
                    params
                        .iter()
                        .map(|param| under_optional.contains(param))
                        .collect(),
                );
            }
        }
    }
    nests_parameter
}

fn drop_ambiguous_nested_optionals(krate: &mut Crate, errors: &mut Vec<SkippedType>) {
    let nests_parameter = parameters_used_under_optional(krate);

    // Every declaration kind is an instantiation site: a template's payload is
    // folded into `Template.fields` and never appears in `data_types` at all,
    // so scanning only data types missed exactly the case that matters most.
    let mut ambiguous: Vec<(String, String, String, String)> = Vec::new();
    for package in &krate.packages {
        for module in &package.modules {
            let mut check = |name: &str, types: Vec<&TypeRef>| {
                if let Some(target) = types.iter().find_map(|reference| {
                    let nests = nests_parameter.get(&path_key_of(reference))?;
                    reference
                        .args
                        .iter()
                        .zip(nests)
                        .any(|(arg, &nested)| nested && matches!(arg, DamlType::Optional(_)))
                        .then(|| path_key_of(reference))
                }) {
                    ambiguous.push((
                        package.name.clone(),
                        module.name.clone(),
                        name.to_string(),
                        target,
                    ));
                }
            };

            for data_type in &module.module.data_types {
                let mut types = Vec::new();
                collect_type_refs_of_data_type(data_type, &mut types);
                check(data_type_name(data_type), types);
            }
            for template in &module.module.templates {
                let mut types = Vec::new();
                each_type_of_template(template, &mut |ty| collect_type_refs(ty, &mut types));
                check(&template.name, types);
            }
            for interface in &module.module.interfaces {
                let mut types = Vec::new();
                each_type_of_interface(interface, &mut |ty| collect_type_refs(ty, &mut types));
                check(&interface.name, types);
            }
        }
    }

    for (package_name, module_name, name, target) in ambiguous {
        let key = path_key_for(&package_name, &module_name, &name);
        for package in &mut krate.packages {
            for module in &mut package.modules {
                let at = |name: &str| path_key_for(&package.name, &module.name, name);
                module
                    .module
                    .data_types
                    .retain(|data_type| at(data_type_name(data_type)) != key);
                module
                    .module
                    .templates
                    .retain(|template| at(&template.name) != key);
                module
                    .module
                    .interfaces
                    .retain(|interface| at(&interface.name) != key);
            }
        }
        errors.push(
            SkippedType::new(format!(
                "`{name}` instantiates `{}` with an Optional that the target nests \
                 under its own Optional; the LF-JSON form of that is not yet representable",
                target.rsplit("::").next().unwrap_or(&target),
            ))
            .in_module(&module_name),
        );
    }
}

/// Type variables appearing under an `Optional` anywhere inside `ty`.
fn collect_vars_under_optional(
    ty: &DamlType,
    under: bool,
    out: &mut std::collections::BTreeSet<String>,
) {
    match ty {
        DamlType::Var(name) if under => {
            out.insert(name.clone());
        }
        DamlType::Optional(inner) => collect_vars_under_optional(inner, true, out),
        DamlType::ContractId(inner)
        | DamlType::List(inner)
        | DamlType::TextMap(inner)
        | DamlType::Boxed(inner) => collect_vars_under_optional(inner, under, out),
        DamlType::GenMap(key, value) => {
            collect_vars_under_optional(key, under, out);
            collect_vars_under_optional(value, under, out);
        }
        DamlType::Ref(reference) => reference
            .args
            .iter()
            .for_each(|arg| collect_vars_under_optional(arg, under, out)),
        _ => {}
    }
}

/// Every `TypeRef` reachable from a data type (the references themselves, so
/// their type arguments can be inspected).
fn collect_type_refs_of_data_type<'a>(data_type: &'a DataType, out: &mut Vec<&'a TypeRef>) {
    let mut walk = |ty: &'a DamlType| collect_type_refs(ty, out);
    match data_type {
        DataType::Record(record) => record.fields.iter().for_each(|field| walk(&field.ty)),
        DataType::Variant(variant) => variant
            .constructors
            .iter()
            .filter_map(|constructor| constructor.payload.as_ref())
            .for_each(walk),
        DataType::Enum(_) | DataType::InterfaceMarker(_) => {}
    }
}

fn collect_type_refs<'a>(ty: &'a DamlType, out: &mut Vec<&'a TypeRef>) {
    match ty {
        DamlType::Ref(reference) => {
            out.push(reference);
            reference
                .args
                .iter()
                .for_each(|arg| collect_type_refs(arg, out));
        }
        DamlType::ContractId(inner)
        | DamlType::List(inner)
        | DamlType::Optional(inner)
        | DamlType::TextMap(inner)
        | DamlType::Boxed(inner) => collect_type_refs(inner, out),
        DamlType::GenMap(key, value) => {
            collect_type_refs(key, out);
            collect_type_refs(value, out);
        }
        _ => {}
    }
}

/// Remove declarations that reference a type which was skipped, repeating until
/// nothing changes.
///
/// Lowering is best-effort, so an unrepresentable type is skipped with a
/// warning — but a *surviving* type that mentioned it still emits
/// `crate::pkg::Mod::Gone` in a field, and the crate handed to the user fails
/// to compile (E0425) with nothing tying the error back to the warning. A
/// binding that cannot compile is worth no more than the one that was skipped,
/// so its dependents go the same way, each reported with the name it was
/// waiting on.
fn drop_dangling_references(krate: &mut Crate, errors: &mut Vec<SkippedType>) {
    let mut declared: std::collections::HashSet<String> = krate
        .packages
        .iter()
        .flat_map(|package| {
            package.modules.iter().flat_map(move |module| {
                let at = |name: &str| path_key_for(&package.name, &module.name, name);
                module
                    .module
                    .data_types
                    .iter()
                    .map(move |data_type| at(data_type_name(data_type)))
                    .chain(
                        module
                            .module
                            .templates
                            .iter()
                            .map(move |template| at(&template.name)),
                    )
            })
        })
        .collect();

    loop {
        let mut dropped = false;
        for package in &mut krate.packages {
            let package_name = package.name.clone();
            for module in &mut package.modules {
                let module_name = module.name.clone();
                let at = |name: &str| path_key_for(&package_name, &module_name, name);

                let missing = |referenced: Vec<String>| -> Option<String> {
                    referenced
                        .into_iter()
                        .find(|key| key.starts_with("crate::") && !declared.contains(key))
                };

                let mut report = Vec::new();
                module.module.data_types.retain(|data_type| {
                    let mut referenced = Vec::new();
                    collect_all_refs_of_data_type(data_type, &mut referenced);
                    match missing(referenced) {
                        Some(gone) => {
                            report.push((
                                at(data_type_name(data_type)),
                                data_type_name(data_type).to_string(),
                                gone,
                            ));
                            false
                        }
                        None => true,
                    }
                });
                module.module.templates.retain(|template| {
                    let mut referenced = Vec::new();
                    collect_all_refs_of_template(template, &mut referenced);
                    match missing(referenced) {
                        Some(gone) => {
                            report.push((at(&template.name), template.name.clone(), gone));
                            false
                        }
                        None => true,
                    }
                });
                // An interface's impls name its view and choice types, so it
                // dangles the same way a record does — and its marker must
                // still exist.
                module.module.interfaces.retain(|interface| {
                    if !declared.contains(&at(&interface.name)) {
                        return false;
                    }
                    let mut referenced = Vec::new();
                    collect_all_refs_of_interface(interface, &mut referenced);
                    match missing(referenced) {
                        Some(gone) => {
                            report.push((at(&interface.name), interface.name.clone(), gone));
                            false
                        }
                        None => true,
                    }
                });

                for (key, name, gone) in report {
                    declared.remove(&key);
                    errors.push(
                        SkippedType::new(format!(
                            "`{name}` references `{}`, which was skipped",
                            gone.rsplit("::").next().unwrap_or(&gone),
                        ))
                        .in_module(&module_name),
                    );
                    dropped = true;
                }
            }
        }
        if !dropped {
            break;
        }
    }

    krate.packages.iter_mut().for_each(|package| {
        package.modules.retain(|module| {
            !module.module.data_types.is_empty()
                || !module.module.templates.is_empty()
                || !module.module.interfaces.is_empty()
        });
    });
    krate.packages.retain(|package| !package.modules.is_empty());
}

/// Every named-type reference reachable from a data type.
fn collect_all_refs_of_data_type(data_type: &DataType, out: &mut Vec<String>) {
    match data_type {
        DataType::Record(record) => record
            .fields
            .iter()
            .for_each(|field| collect_all_refs(&field.ty, out)),
        DataType::Variant(variant) => variant
            .constructors
            .iter()
            .filter_map(|constructor| constructor.payload.as_ref())
            .for_each(|payload| collect_all_refs(payload, out)),
        DataType::Enum(_) | DataType::InterfaceMarker(_) => {}
    }
}

/// Every named-type reference reachable from a template: its payload, its
/// choices' argument and return types, and its contract key.
fn collect_all_refs_of_template(template: &Template, out: &mut Vec<String>) {
    each_type_of_template(template, &mut |ty| collect_all_refs(ty, out));
}

/// Every named-type reference reachable from an interface: its view type and
/// its choices' argument and return types.
fn collect_all_refs_of_interface(interface: &Interface, out: &mut Vec<String>) {
    each_type_of_interface(interface, &mut |ty| collect_all_refs(ty, out));
}

/// Visit every type a template mentions. Both post-lowering passes go through
/// this rather than each walking the fields it happens to remember.
fn each_type_of_template<'a>(template: &'a Template, visit: &mut impl FnMut(&'a DamlType)) {
    for field in &template.fields {
        visit(&field.ty);
    }
    for choice in &template.choices {
        visit(&choice.argument);
        visit(&choice.returns);
    }
    if let Some(key) = &template.key {
        visit(key);
    }
}

/// Visit every type an interface mentions.
fn each_type_of_interface<'a>(interface: &'a Interface, visit: &mut impl FnMut(&'a DamlType)) {
    if let Some(view) = &interface.view {
        visit(view);
    }
    for choice in &interface.choices {
        visit(&choice.argument);
        visit(&choice.returns);
    }
}

/// Every named-type reference inside a type, at any depth (unlike
/// `collect_inline_refs`, which stops at heap indirection).
fn collect_all_refs(ty: &DamlType, out: &mut Vec<String>) {
    match ty {
        DamlType::Ref(reference) => {
            out.push(path_key_of(reference));
            reference
                .args
                .iter()
                .for_each(|arg| collect_all_refs(arg, out));
        }
        DamlType::ContractId(inner)
        | DamlType::List(inner)
        | DamlType::Optional(inner)
        | DamlType::TextMap(inner)
        | DamlType::Boxed(inner) => collect_all_refs(inner, out),
        DamlType::GenMap(key, value) => {
            collect_all_refs(key, out);
            collect_all_refs(value, out);
        }
        _ => {}
    }
}

/// The Rust module name each package's types live under, keyed by package id.
///
/// The name alone (`splice_amulet`), **not** name-and-version: under Smart
/// Contract Upgrade the participant resolves which vetted version a command
/// targets, so a routine DAR bump should not rename every path a caller
/// imports. The version is appended only where it is doing work — when one DAR
/// genuinely bundles two versions of the same package — and the package-id
/// prefix only if even that is ambiguous.
fn package_module_names(packages: &[(String, lf::Package)]) -> HashMap<&str, String> {
    let mut names: HashMap<&str, String> = packages
        .iter()
        .map(|(id, package)| {
            let base =
                package_name(package).map_or_else(|| format!("p_{}", ident_prefix(id)), sanitize);
            (id.as_str(), base)
        })
        .collect();

    // One pass per disambiguation step: add the version where the bare name
    // repeats, then the id prefix where even that repeats (a rebuilt package
    // keeps its metadata but gets a new id).
    // An empty or metadata-less version is no disambiguator — several packages
    // carry `version: ""`, and using it would collapse them all onto one name.
    let versioned = |id: &str| {
        packages
            .iter()
            .find(|(candidate, _)| candidate == id)
            .and_then(|(_, package)| package_version(package))
            .map(sanitize)
            .filter(|version| !version.is_empty())
    };
    for step in 0..2 {
        let mut counts: HashMap<String, usize> = HashMap::new();
        for name in names.values() {
            *counts.entry(name.clone()).or_default() += 1;
        }
        let ambiguous: std::collections::HashSet<String> = counts
            .into_iter()
            .filter(|(_, count)| *count > 1)
            .map(|(name, _)| name)
            .collect();
        if ambiguous.is_empty() {
            break;
        }
        for (id, name) in &mut names {
            if !ambiguous.contains(name.as_str()) {
                continue;
            }
            let suffix = if step == 0 {
                versioned(id).unwrap_or_else(|| ident_prefix(id))
            } else {
                ident_prefix(id)
            };
            *name = format!("{name}_{suffix}");
        }
    }
    names
}

/// The first 8 identifier-safe characters of a package id, for `p_<hash8>` /
/// collision-suffix module names. Character-based (a byte slice could panic on
/// a hostile non-ASCII id) and filtered to alphanumerics. An id with no
/// alphanumerics at all would yield an *empty* suffix, defeating the
/// de-collision this exists for, so it falls back to a hash of the id.
fn ident_prefix(package_id: &str) -> String {
    let prefix: String = package_id
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .take(8)
        .collect();
    if prefix.is_empty() {
        use std::hash::{Hash as _, Hasher as _};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        package_id.hash(&mut hasher);
        format!("h{:x}", hasher.finish())
    } else {
        prefix
    }
}

/// Rust module names claimed by more than one distinct Daml module name in a
/// package, mapped to the Daml names that claim them.
fn colliding_module_names(
    package: &lf::Package,
) -> std::collections::BTreeMap<String, Vec<String>> {
    let mut claims: std::collections::BTreeMap<String, Vec<String>> =
        std::collections::BTreeMap::new();
    for lf_module in &package.modules {
        let Some(dotted) = interned_dotted_name(package, lf_module.name_interned_dname) else {
            continue;
        };
        let sources = claims.entry(dotted.replace('.', "_")).or_default();
        if !sources.contains(&dotted) {
            sources.push(dotted);
        }
    }
    claims.retain(|_, sources| sources.len() > 1);
    claims
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
        // Daml names are dotted and Rust identifiers are not, so `A.B` and `A_B`
        // both flatten to `A_B`. Emitting both would either overwrite one
        // silently (a template taking another type's payload — wrong commands,
        // no compile error) or produce a duplicate definition. Detect the clash
        // up front and skip the whole colliding set, naming both Daml sources.
        let colliding = self.colliding_flattened_names(lf_module);

        // The names that are templates — their payload record is folded in below.
        let template_names: std::collections::HashSet<String> = lf_module
            .templates
            .iter()
            .filter_map(|def| rust_name(self.package, def.tycon_interned_dname).ok())
            .filter(|name| !colliding.contains_key(name))
            .collect();

        for (flattened, sources) in &colliding {
            errors.push(
                SkippedType::new(format!(
                    "`{}` both map to the Rust name `{flattened}`; \
                     rename one in Daml to generate either",
                    sources.join("` and `"),
                ))
                .in_module(module_dotted),
            );
        }
        let skip = |name: &str| colliding.contains_key(name);

        let mut module = Module::default();
        // Payload records held aside for their template (name → fields).
        let mut payloads: HashMap<String, Vec<Field>> = HashMap::new();
        for data_type in &lf_module.data_types {
            match self.data_type(data_type) {
                Ok(Some(lowered)) if skip(data_type_name(&lowered)) => {}
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
                Ok(template) if skip(&template.name) => {}
                Ok(template) => module.templates.push(template),
                Err(error) => errors.push(error.in_module(module_dotted)),
            }
        }

        for def in &lf_module.interfaces {
            match self.interface(def, module_dotted) {
                Ok(interface) if skip(&interface.name) => {}
                Ok(interface) => module.interfaces.push(interface),
                Err(error) => errors.push(error.in_module(module_dotted)),
            }
        }
        module
    }

    /// Rust names claimed by more than one *distinct* Daml name in this module,
    /// mapped to the Daml names that claim them. A template and its payload
    /// record share one Daml name, so they never collide with each other.
    fn colliding_flattened_names(
        &self,
        lf_module: &lf::Module,
    ) -> std::collections::BTreeMap<String, Vec<String>> {
        let declared = lf_module
            .data_types
            .iter()
            .map(|def| def.name_interned_dname)
            .chain(
                lf_module
                    .templates
                    .iter()
                    .map(|def| def.tycon_interned_dname),
            )
            .chain(
                lf_module
                    .interfaces
                    .iter()
                    .map(|def| def.tycon_interned_dname),
            );

        let mut claims: std::collections::BTreeMap<String, Vec<String>> =
            std::collections::BTreeMap::new();
        for index in declared {
            let Some(dotted) = interned_dotted_name(self.package, index) else {
                continue;
            };
            let flattened = dotted.replace('.', "_");
            let sources = claims.entry(flattened).or_default();
            if !sources.contains(&dotted) {
                sources.push(dotted);
            }
        }
        claims.retain(|_, sources| sources.len() > 1);
        claims
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
        let key = match def.key.as_ref() {
            // A declared key with no type is malformed LF: lowering it to a
            // keyless template would silently drop `exercise_by_key`.
            Some(key) => Some(self.type_(key.r#type.as_ref().ok_or_else(|| {
                SkippedType::new(format!("template {name}: contract key has no type"))
            })?)?),
            None => None,
        };
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
                        let name = interned_str(self.package, index)
                            .ok_or_else(|| SkippedType::new("unresolved enum constructor"))?;
                        if is_rust_ident(name) {
                            Ok(name.to_string())
                        } else {
                            Err(SkippedType::new(format!(
                                "enum constructor `{name}` is not representable as a Rust identifier"
                            )))
                        }
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
        if !is_rust_ident(&name) {
            return Err(SkippedType::new(format!(
                "variant constructor `{name}` is not representable as a Rust identifier"
            )));
        }
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
                // A higher-kinded application (`f a`) has no Rust equivalent.
                // `Var` carries its own `args` as well as any from a curried
                // `TApp`; dropping either would silently emit the bare
                // parameter in place of the applied type.
                if !extra.is_empty() || !var.args.is_empty() {
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

/// The Rust name a lowered declaration will be emitted under.
fn data_type_name(data_type: &DataType) -> &str {
    match data_type {
        DataType::Record(record) => &record.name,
        DataType::Variant(variant) => &variant.name,
        DataType::Enum(enumeration) => &enumeration.name,
        DataType::InterfaceMarker(name) => name,
    }
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
    if is_rust_ident(&name) {
        Ok(name)
    } else {
        Err(SkippedType::new(format!(
            "`{dotted}` is not representable as a Rust identifier"
        )))
    }
}

/// Whether `name` can be emitted as a Rust identifier.
///
/// Everything handed to the emitter must pass this: `Ident::new` **panics** on
/// anything else, and Daml permits names Rust does not — an apostrophe suffix
/// (`Red'`) is ordinary Daml and Haskell. A name that fails is skipped with a
/// reason, which is the contract this crate documents; aborting the process
/// with a `proc_macro2` backtrace is not.
fn is_rust_ident(name: &str) -> bool {
    !name.is_empty()
        && !name.starts_with(|c: char| c.is_ascii_digit())
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;
    use canton_lf::Dar;

    // ---- name flattening (always-on, synthetic LF) -------------------------
    //
    // Daml names are dotted, Rust identifiers are not. These build the minimal
    // LF packages where that difference used to produce silently-wrong output.

    /// An LF package with the given interned strings and dotted names, where
    /// dotted name `i` is built from the single string `i` unless it contains
    /// a `.`, in which case it is split into segments.
    fn package_with(names: &[&str]) -> lf::Package {
        let mut strings: Vec<String> = Vec::new();
        let mut dotted = Vec::new();
        for name in names {
            let segments = name
                .split('.')
                .map(|segment| {
                    strings.push(segment.to_string());
                    i32::try_from(strings.len() - 1).unwrap()
                })
                .collect();
            dotted.push(lf::InternedDottedName {
                segments_interned_str: segments,
            });
        }
        lf::Package {
            interned_strings: strings,
            interned_dotted_names: dotted,
            ..lf::Package::default()
        }
    }

    /// A serializable record data type named by dotted-name index `name`, with
    /// one `Party` field labelled by interned-string index `label`.
    fn record_def(name: i32, label: i32) -> lf::DefDataType {
        lf::DefDataType {
            name_interned_dname: name,
            serializable: true,
            data_cons: Some(lf::def_data_type::DataCons::Record(
                lf::def_data_type::Fields {
                    fields: vec![lf::FieldWithType {
                        field_interned_str: label,
                        r#type: Some(lf::Type {
                            sum: Some(lf::r#type::Sum::Builtin(lf::r#type::Builtin {
                                builtin: lf::BuiltinType::Party as i32,
                                args: vec![],
                            })),
                        }),
                    }],
                },
            )),
            ..lf::DefDataType::default()
        }
    }

    #[test]
    fn two_daml_names_that_flatten_alike_are_skipped_not_silently_merged() {
        // `Foo.Bar` (a data type) and `Foo_Bar` (a template) both flatten to the
        // Rust name `Foo_Bar`. The payload fold was keyed on the flattened name,
        // so the data type's fields silently replaced the template's — a create
        // command with the wrong shape, and no compile error anywhere.
        // strings: 0=Foo 1=Bar 2=Foo_Bar 3=owner 4=note ; dnames: 0=Foo.Bar 1=Foo_Bar
        let mut package = package_with(&["Foo.Bar", "Foo_Bar"]);
        package.interned_strings.push("owner".to_string());
        package.interned_strings.push("note".to_string());
        let (owner, note) = (3, 4);

        package.modules.push(lf::Module {
            name_interned_dname: 1, // reuse `Foo_Bar` as the module name
            data_types: vec![record_def(0, note), record_def(1, owner)],
            templates: vec![lf::DefTemplate {
                tycon_interned_dname: 1,
                ..lf::DefTemplate::default()
            }],
            ..lf::Module::default()
        });

        let (krate, skipped) = lower_crate(&[("aa".to_string(), package)]);
        let reasons: Vec<String> = skipped.iter().map(ToString::to_string).collect();
        assert!(
            reasons.iter().any(|r| r.contains("Foo.Bar")
                && r.contains("Foo_Bar")
                && r.contains("map to the Rust name")),
            "the clash must be reported naming both Daml sources: {reasons:?}"
        );
        // Nothing under that name is emitted — better no bindings than bindings
        // that submit the wrong payload.
        let emitted: Vec<&str> = krate
            .packages
            .iter()
            .flat_map(|p| &p.modules)
            .flat_map(|m| m.module.templates.iter().map(|t| t.name.as_str()))
            .collect();
        assert!(!emitted.contains(&"Foo_Bar"), "emitted: {emitted:?}");
    }

    /// A package carrying just the metadata `package_module_names` reads.
    fn named_package(name: &str, version: Option<&str>) -> lf::Package {
        lf::Package {
            interned_strings: vec![name.to_string(), version.unwrap_or_default().to_string()],
            metadata: Some(lf::PackageMetadata {
                name_interned_str: 0,
                version_interned_str: 1,
                upgraded_package_id: None,
            }),
            ..lf::Package::default()
        }
    }

    fn module_names(packages: &[(&str, lf::Package)]) -> Vec<String> {
        let owned: Vec<(String, lf::Package)> = packages
            .iter()
            .map(|(id, package)| ((*id).to_string(), package.clone()))
            .collect();
        let names = package_module_names(&owned);
        // Report in the caller's order, not the map's.
        owned
            .iter()
            .map(|(id, _)| names[id.as_str()].clone())
            .collect()
    }

    #[test]
    fn a_package_module_is_named_after_the_package_alone() {
        // The DAR's own version is not in the path: bumping the package must not
        // rename every type a consumer imports. See docs/scu-regeneration.md.
        assert_eq!(
            module_names(&[
                ("aaa", named_package("quickstart-licensing", Some("0.0.1"))),
                ("bbb", named_package("splice-amulet", Some("0.1.14"))),
            ]),
            ["quickstart_licensing", "splice_amulet"],
        );
    }

    #[test]
    fn two_versions_of_one_package_are_separated_by_version() {
        // The SCU shape: one DAR bundling v1 and v2 of the same package. Both
        // must be reachable, so here — and only here — the version is the
        // disambiguator.
        assert_eq!(
            module_names(&[
                ("aaa", named_package("my-app", Some("1.0.0"))),
                ("bbb", named_package("my-app", Some("2.0.0"))),
                ("ccc", named_package("other", Some("1.0.0"))),
            ]),
            ["my_app_1_0_0", "my_app_2_0_0", "other"],
        );
    }

    #[test]
    fn packages_alike_down_to_the_version_fall_back_to_the_id() {
        // A rebuilt package keeps its metadata but gets a new id; and several
        // real packages (daml-prim, …) carry `version: ""`, which would collapse
        // them all onto one name if it were used as a suffix.
        assert_eq!(
            module_names(&[
                ("aaaaaaaaaaaa", named_package("my-app", Some("1.0.0"))),
                ("bbbbbbbbbbbb", named_package("my-app", Some("1.0.0"))),
                ("cccccccccccc", named_package("no-version", Some(""))),
                ("dddddddddddd", named_package("no-version", None)),
                ("eeeeeeeeeeee", lf::Package::default()),
            ]),
            [
                "my_app_1_0_0_aaaaaaaa",
                "my_app_1_0_0_bbbbbbbb",
                "no_version_cccccccc",
                "no_version_dddddddd",
                "p_eeeeeeee",
            ],
        );
    }

    #[test]
    fn two_module_names_that_flatten_alike_are_skipped() {
        // `A.B` and `A_B` would both become `pub mod A_B` — E0428 in the crate
        // we hand the user, reported as a successful generation.
        let mut package = package_with(&["A.B", "A_B", "T"]);
        package.interned_strings.push("owner".to_string());
        let owner = i32::try_from(package.interned_strings.len() - 1).unwrap();
        for name in [0, 1] {
            package.modules.push(lf::Module {
                name_interned_dname: name,
                data_types: vec![record_def(2, owner)],
                ..lf::Module::default()
            });
        }

        let (krate, skipped) = lower_crate(&[("aa".to_string(), package)]);
        assert!(
            skipped
                .iter()
                .any(|s| s.to_string().contains("map to the Rust module")),
            "{skipped:?}"
        );
        let modules: Vec<&str> = krate
            .packages
            .iter()
            .flat_map(|p| p.modules.iter().map(|m| m.name.as_str()))
            .collect();
        assert!(modules.is_empty(), "no module may be emitted: {modules:?}");
    }

    #[test]
    fn an_optional_passed_to_a_generic_that_nests_it_is_refused_not_mis_encoded() {
        // `Wrap a = { value : Optional a }` puts its parameter under an
        // Optional, so `Wrap (Optional Text)` is a nested optional on the wire
        // and needs the LF-JSON list form — which the structural mapping cannot
        // see through a type parameter. Refuse it rather than emit
        // `Option<Option<String>>`, which serializes the inner layer as `null`.
        let reference = |name: &str, args: Vec<DamlType>| {
            DamlType::Ref(TypeRef {
                path: vec![
                    "crate".to_string(),
                    "pkg_1_0_0".to_string(),
                    "Mod".to_string(),
                    name.to_string(),
                ],
                args,
            })
        };
        let field = |label: &str, ty: DamlType| Field {
            label: label.to_string(),
            ty,
        };
        let module = |data_types: Vec<DataType>| Crate {
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
        };

        // The generic that nests its parameter, plus a user of it.
        let wrap_nesting = DataType::Record(Record {
            name: "Wrap".to_string(),
            type_params: vec!["a".to_string()],
            fields: vec![field(
                "value",
                DamlType::Optional(Box::new(DamlType::Var("a".to_string()))),
            )],
        });
        let user = |arg: DamlType| {
            DataType::Record(Record {
                name: "User".to_string(),
                type_params: vec![],
                fields: vec![field("w", reference("Wrap", vec![arg]))],
            })
        };

        let mut krate = module(vec![
            wrap_nesting.clone(),
            user(DamlType::Optional(Box::new(DamlType::Text))),
        ]);
        let mut errors = Vec::new();
        drop_ambiguous_nested_optionals(&mut krate, &mut errors);
        assert!(
            errors
                .iter()
                .any(|e| e.to_string().contains("not yet representable")),
            "{errors:?}"
        );
        let names: Vec<&str> = krate.packages[0].modules[0]
            .module
            .data_types
            .iter()
            .map(data_type_name)
            .collect();
        assert_eq!(names, ["Wrap"], "only the ambiguous user is dropped");

        // A non-Optional argument is unaffected…
        let mut fine = module(vec![wrap_nesting, user(DamlType::Text)]);
        let mut errors = Vec::new();
        drop_ambiguous_nested_optionals(&mut fine, &mut errors);
        assert!(errors.is_empty(), "{errors:?}");

        // …and so is an Optional argument to a generic that does *not* nest it
        // (there the top-level `null`/value form is the correct one).
        let wrap_plain = DataType::Record(Record {
            name: "Wrap".to_string(),
            type_params: vec!["a".to_string()],
            fields: vec![field("value", DamlType::Var("a".to_string()))],
        });
        let mut plain = module(vec![
            wrap_plain,
            user(DamlType::Optional(Box::new(DamlType::Text))),
        ]);
        let mut errors = Vec::new();
        drop_ambiguous_nested_optionals(&mut plain, &mut errors);
        assert!(errors.is_empty(), "{errors:?}");
        assert_eq!(plain.packages[0].modules[0].module.data_types.len(), 2);
    }

    /// The two post-lowering passes each walk every declaration kind. They were
    /// written one kind at a time and both missed one — the dangling-reference
    /// pass judged interfaces only by their marker, and the nested-Optional
    /// pass never looked at templates, whose payload is folded into
    /// `Template.fields` and so never appears in `data_types` at all.
    #[test]
    fn both_post_lowering_passes_cover_templates_and_interfaces() {
        let reference = |name: &str, args: Vec<DamlType>| {
            DamlType::Ref(TypeRef {
                path: vec![
                    "crate".to_string(),
                    "pkg_1_0_0".to_string(),
                    "Mod".to_string(),
                    name.to_string(),
                ],
                args,
            })
        };
        let module_of = |data_types: Vec<DataType>,
                         templates: Vec<Template>,
                         interfaces: Vec<Interface>| Crate {
            packages: vec![PackageModule {
                name: "pkg_1_0_0".to_string(),
                modules: vec![NamedModule {
                    name: "Mod".to_string(),
                    module: Module {
                        data_types,
                        templates,
                        interfaces,
                    },
                }],
            }],
        };
        let choice = |ty: DamlType| Choice {
            name: "Do".to_string(),
            consuming: true,
            argument: DamlType::Unit,
            returns: ty,
        };
        let template_of = |name: &str, field: DamlType| Template {
            name: name.to_string(),
            module_name: "Mod".to_string(),
            package_id: "aa".to_string(),
            package_name: "pkg".to_string(),
            fields: vec![Field {
                label: "x".to_string(),
                ty: field,
            }],
            choices: vec![],
            key: None,
        };

        // (1) An interface whose choice returns a skipped type used to survive,
        // emitting `type Return = crate::…::Bad` and failing the user's build
        // with E0425 — the very error the pass exists to prevent.
        let mut krate = module_of(
            vec![DataType::InterfaceMarker("Iface".to_string())],
            vec![],
            vec![Interface {
                name: "Iface".to_string(),
                module_name: "Mod".to_string(),
                package_id: "aa".to_string(),
                package_name: "pkg".to_string(),
                view: None,
                choices: vec![choice(reference("Bad", vec![]))], // `Bad` was skipped
            }],
        );
        let mut errors = Vec::new();
        drop_dangling_references(&mut krate, &mut errors);
        assert!(
            errors
                .iter()
                .any(|e| e.to_string().contains("`Iface` references `Bad`")),
            "an interface dangles like anything else: {errors:?}"
        );
        assert!(
            krate.packages.is_empty() || krate.packages[0].modules[0].module.interfaces.is_empty(),
            "the interface must not be emitted"
        );

        // (2) The same ambiguous instantiation inside a *template payload* used
        // to be emitted, serialising the inner Optional as `null` where LF-JSON
        // requires `[]`.
        let wrap = DataType::Record(Record {
            name: "Wrap".to_string(),
            type_params: vec!["a".to_string()],
            fields: vec![Field {
                label: "value".to_string(),
                ty: DamlType::Optional(Box::new(DamlType::Var("a".to_string()))),
            }],
        });
        let mut krate = module_of(
            vec![wrap],
            vec![template_of(
                "Tpl",
                reference("Wrap", vec![DamlType::Optional(Box::new(DamlType::Text))]),
            )],
            vec![],
        );
        let mut errors = Vec::new();
        drop_ambiguous_nested_optionals(&mut krate, &mut errors);
        assert!(
            errors
                .iter()
                .any(|e| e.to_string().contains("not yet representable")),
            "a template payload is an instantiation site too: {errors:?}"
        );
        assert!(
            krate.packages[0].modules[0].module.templates.is_empty(),
            "the template must not be emitted"
        );
    }

    #[test]
    fn a_type_referencing_a_skipped_type_is_dropped_transitively() {
        // `Bad` is unrepresentable and skipped; `Good` has a field of type
        // `Bad`, and `Worse` a field of type `Good`. Emitting either leaves
        // `crate::…::Bad` dangling and the user's crate fails with E0425, with
        // nothing tying that to the warning — so both must go too.
        let path = |name: &str| {
            DamlType::Ref(TypeRef {
                path: vec![
                    "crate".to_string(),
                    "pkg_1_0_0".to_string(),
                    "Mod".to_string(),
                    name.to_string(),
                ],
                args: vec![],
            })
        };
        let record = |name: &str, field: DamlType| {
            DataType::Record(Record {
                name: name.to_string(),
                type_params: vec![],
                fields: vec![Field {
                    label: "x".to_string(),
                    ty: field,
                }],
            })
        };
        let mut krate = Crate {
            packages: vec![PackageModule {
                name: "pkg_1_0_0".to_string(),
                modules: vec![NamedModule {
                    name: "Mod".to_string(),
                    module: Module {
                        // `Bad` is absent — as if lowering had skipped it.
                        data_types: vec![
                            record("Good", path("Bad")),
                            record("Worse", path("Good")),
                            record("Fine", DamlType::Text),
                        ],
                        ..Module::default()
                    },
                }],
            }],
        };

        let mut errors = Vec::new();
        drop_dangling_references(&mut krate, &mut errors);

        let surviving: Vec<&str> = krate.packages[0].modules[0]
            .module
            .data_types
            .iter()
            .map(data_type_name)
            .collect();
        assert_eq!(surviving, ["Fine"], "only the self-contained type survives");

        let reasons: Vec<String> = errors.iter().map(ToString::to_string).collect();
        assert!(
            reasons
                .iter()
                .any(|r| r.contains("`Good` references `Bad`")),
            "{reasons:?}"
        );
        assert!(
            reasons
                .iter()
                .any(|r| r.contains("`Worse` references `Good`")),
            "the drop must cascade: {reasons:?}"
        );
    }

    #[test]
    fn a_daml_name_rust_cannot_spell_is_skipped_not_panicked_on() {
        // `Red'` is ordinary Daml and Haskell. Only *type* names were checked,
        // so a module or constructor carrying an apostrophe reached
        // `Ident::new` and aborted the CLI with a proc_macro2 panic instead of
        // the skip-with-a-reason this crate promises.
        assert!(is_rust_ident("Red"));
        assert!(is_rust_ident("_x9"));
        for bad in ["Red'", "", "9lives", "naïve", "a-b", "A.B"] {
            assert!(!is_rust_ident(bad), "{bad:?} is not a Rust identifier");
        }

        // An enum constructor Rust cannot spell skips its declaration.
        // strings: 0=Colour 1=Red' ; dnames: 0=Colour
        let mut package = package_with(&["Colour"]);
        package.interned_strings.push("Red'".to_string());
        let constructor = i32::try_from(package.interned_strings.len() - 1).unwrap();
        package.modules.push(lf::Module {
            name_interned_dname: 0,
            data_types: vec![lf::DefDataType {
                name_interned_dname: 0,
                serializable: true,
                data_cons: Some(lf::def_data_type::DataCons::Enum(
                    lf::def_data_type::EnumConstructors {
                        constructors_interned_str: vec![constructor],
                    },
                )),
                ..lf::DefDataType::default()
            }],
            ..lf::Module::default()
        });

        let (krate, skipped) = lower_crate(&[("aa".to_string(), package)]);
        assert!(
            skipped
                .iter()
                .any(|s| s.to_string().contains("enum constructor `Red'`")),
            "{skipped:?}"
        );
        assert!(krate.packages.is_empty(), "nothing may be emitted");
    }

    #[test]
    fn the_collision_suffix_is_never_empty() {
        // Two metadata-less packages whose ids share no ASCII alphanumerics both
        // derived `p_`, and the de-collision suffix was empty — two `pub mod p__`.
        assert_ne!(ident_prefix("€€€€"), ident_prefix("£££££"));
        assert!(!ident_prefix("€€€€").is_empty());
        assert_eq!(ident_prefix("aabbccddee"), "aabbccdd");
    }

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
