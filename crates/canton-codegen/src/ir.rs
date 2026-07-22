//! A decoder-agnostic intermediate representation (IR) of Daml types.
//!
//! The IR is the seam between "decode Daml-LF" (Phase B — JVM `daml-lf-archive`
//! or a native decoder) and "emit Rust" (this crate's generator). Neither side
//! knows about the other: a decoder produces this IR, the generator consumes it.
//! That keeps the pivotal LF-decoder decision isolated to one module.

/// A Daml type — a primitive, a container, or a reference to a named data type.
///
/// This is the type a record field, choice argument, or contract key can take.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DamlType {
    /// The `Unit` type `()`.
    Unit,
    /// `Bool`.
    Bool,
    /// `Int64`.
    Int64,
    /// `Numeric n` — a fixed-scale decimal; the value is the scale (decimals).
    Numeric(u8),
    /// `Text`.
    Text,
    /// `Timestamp` (microseconds since the Unix epoch, UTC).
    Timestamp,
    /// `Date` (days since the Unix epoch).
    Date,
    /// `Party`.
    Party,
    /// `ContractId t` — a handle to a contract of the referenced payload type.
    ContractId(Box<DamlType>),
    /// `List t` / `[t]`.
    List(Box<DamlType>),
    /// `Optional t`.
    Optional(Box<DamlType>),
    /// `TextMap t` — a map keyed by `Text`.
    TextMap(Box<DamlType>),
    /// `GenMap k v` — a map with arbitrary key type.
    GenMap(Box<DamlType>, Box<DamlType>),
    /// A reference to a named data type (record / variant / enum).
    Ref(TypeRef),
    /// A type parameter (`a`, `b`, …) inside a generic data type.
    Var(String),
    /// A type behind a `Box`, used to give recursive types the indirection Rust
    /// requires (Daml allows a type to contain itself directly; Rust does not).
    Boxed(Box<DamlType>),
}

/// A reference to a named Daml data type, with any applied type arguments.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeRef {
    /// The Rust path to the referenced type, as segments. A local reference is a
    /// single segment (`["Foo"]`); a qualified one carries its full path
    /// (`["crate", "splice_amulet", "Amulet"]`), which is how cross-module and
    /// cross-package references are disambiguated.
    pub path: Vec<String>,
    /// Applied type arguments, if the referenced type is generic.
    pub args: Vec<DamlType>,
}

impl TypeRef {
    /// A local (single-segment) reference to `name`, no path qualification.
    #[must_use]
    pub fn local(name: impl Into<String>, args: Vec<DamlType>) -> Self {
        Self {
            path: vec![name.into()],
            args,
        }
    }
}

/// One field of a record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Field {
    /// The Daml field label, in its source casing (usually camelCase).
    pub label: String,
    /// The field's type.
    pub ty: DamlType,
}

/// A record data type. Template payloads are records too, so this is reused for
/// both a plain `data … = … with` record and a `template … with` payload.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Record {
    /// The type name (PascalCase, as in Daml).
    pub name: String,
    /// Type parameters, in order, if the record is generic.
    pub type_params: Vec<String>,
    /// The fields, in declaration order.
    pub fields: Vec<Field>,
}

/// A named data type declared in a module: a record, a variant, or an enum.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DataType {
    /// A record (product) type.
    Record(Record),
    /// A variant (sum) type.
    Variant(Variant),
    /// An enumeration (constructors carrying no payload).
    Enum(Enum),
    /// An interface **marker**: a phantom tag emitted so references to the
    /// interface (always `ContractId<I>`) resolve. The interface itself is not
    /// serializable and carries no data of its own; full interface codegen (its
    /// view and choices) is reserved for a later step. The `String` is the name.
    InterfaceMarker(String),
}

/// A variant (sum) type: named constructors, each optionally carrying a payload.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Variant {
    /// The type name (PascalCase).
    pub name: String,
    /// Type parameters, in order, if generic.
    pub type_params: Vec<String>,
    /// The constructors, in declaration order.
    pub constructors: Vec<VariantConstructor>,
}

/// One constructor of a [`Variant`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VariantConstructor {
    /// The constructor name (PascalCase).
    pub name: String,
    /// The payload type, or `None` for a constructor that carries no data.
    pub payload: Option<DamlType>,
}

/// An enumeration: named constructors that carry no payload.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Enum {
    /// The type name (PascalCase).
    pub name: String,
    /// The constructor names, in declaration order.
    pub constructors: Vec<String>,
}

/// A template: its payload fields, its choices, and an optional contract key.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Template {
    /// The template name (PascalCase) — also the Rust payload struct name.
    pub name: String,
    /// The Daml module the template is defined in, dotted (e.g. `Splice.Amulet`).
    /// Part of the on-ledger template id.
    pub module_name: String,
    /// The id (hash) of the package the template is defined in. Pins the exact
    /// template version in an on-ledger template id.
    pub package_id: String,
    /// The Daml package **name** (e.g. `splice-amulet`), used for the
    /// upgrade-friendly `#<package-name>` template-id form so the participant
    /// resolves the vetted version under Smart Contract Upgrade.
    pub package_name: String,
    /// The payload fields, in declaration order.
    pub fields: Vec<Field>,
    /// The choices exercisable on a contract of this template.
    pub choices: Vec<Choice>,
    /// The contract key type, if the template declares a key.
    pub key: Option<DamlType>,
}

/// A choice on a [`Template`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Choice {
    /// The choice name (PascalCase).
    pub name: String,
    /// Whether exercising the choice archives the contract.
    pub consuming: bool,
    /// The choice argument type (usually a reference to a record).
    pub argument: DamlType,
    /// The type the choice returns.
    pub returns: DamlType,
}

/// A module's worth of generated declarations: its data types, templates, and
/// interfaces.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Module {
    /// The named data types (records, variants, enums).
    pub data_types: Vec<DataType>,
    /// The templates.
    pub templates: Vec<Template>,
    /// The interfaces. The IR reserves their shape (view + choices) so the
    /// AST→IR bridge fixes it now; full codegen of interfaces lands later.
    pub interfaces: Vec<Interface>,
}

/// A Daml interface. Its shape is reserved in the IR — a PR reviewer named
/// DAR-interfaces → typed Rust as a hard requirement, so the bridge pins the
/// form now (view type + choices) even before the generator emits them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Interface {
    /// The interface name (PascalCase).
    pub name: String,
    /// The interface's view type (its `viewtype`), if known.
    pub view: Option<DamlType>,
    /// The choices the interface declares.
    pub choices: Vec<Choice>,
}

/// A whole generated crate: every package in a DAR, each as its own Rust module
/// (so cross-module and cross-package references resolve, and names from
/// different modules cannot collide).
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Crate {
    /// The packages, each a top-level module in the generated crate.
    pub packages: Vec<PackageModule>,
}

/// One package rendered as a Rust module (`pub mod <name> { … }`), containing a
/// submodule per Daml module.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PackageModule {
    /// The Rust module name for this package (derived from its name + version).
    pub name: String,
    /// The Daml modules of this package.
    pub modules: Vec<NamedModule>,
}

/// A Daml module rendered as a Rust submodule.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NamedModule {
    /// The Rust module name for this Daml module (dotted name, `.` → `_`).
    pub name: String,
    /// The module's declarations.
    pub module: Module,
}
