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
}

/// A reference to a named Daml data type, with any applied type arguments.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TypeRef {
    /// The data type's name (PascalCase, as in Daml).
    pub name: String,
    /// Applied type arguments, if the referenced type is generic.
    pub args: Vec<DamlType>,
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
