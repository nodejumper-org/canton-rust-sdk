//! Conversion between typed Rust values and the Ledger API [`Value`](pb::Value)
//! (the gRPC wire form). Generated records/variants/enums implement these
//! traits too; here they are provided for the primitive and container types.

use std::collections::BTreeMap;

use canton_proto::com::daml::ledger::api::v2 as pb;

use crate::primitives::{ContractId, Date, GenMap, NestedOpt, Numeric, Party, Timestamp};

/// An error converting a Ledger API [`Value`](pb::Value) into a typed value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValueError {
    /// A human-readable description of the mismatch.
    pub message: String,
}

impl ValueError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl std::fmt::Display for ValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "value conversion error: {}", self.message)
    }
}

impl std::error::Error for ValueError {}

/// Encode a typed Rust value as a Ledger API [`Value`](pb::Value).
pub trait ToValue {
    /// Encode `self` as a wire `Value`.
    fn to_value(&self) -> pb::Value;
}

/// Decode a typed Rust value from a Ledger API [`Value`](pb::Value).
pub trait FromValue: Sized {
    /// Decode a wire `Value` into `Self`.
    ///
    /// # Errors
    /// Returns [`ValueError`] if the value's shape does not match `Self`.
    fn from_value(value: &pb::Value) -> Result<Self, ValueError>;
}

fn wrap(sum: pb::value::Sum) -> pb::Value {
    pb::Value { sum: Some(sum) }
}

fn sum(value: &pb::Value) -> Result<&pb::value::Sum, ValueError> {
    value
        .sum
        .as_ref()
        .ok_or_else(|| ValueError::new("empty Value (no sum set)"))
}

fn mismatch(expected: &str, got: &pb::value::Sum) -> ValueError {
    ValueError::new(format!("expected {expected}, got {got:?}"))
}

// ---- record helpers (used by generated ToValue/FromValue) ------------------

/// Build a `Record` [`Value`](pb::Value) from labelled fields. Generated
/// `ToValue` impls call this.
#[must_use]
pub fn record(fields: Vec<(&str, pb::Value)>) -> pb::Value {
    wrap(pb::value::Sum::Record(pb::Record {
        record_id: None,
        fields: fields
            .into_iter()
            .map(|(label, value)| pb::RecordField {
                label: label.to_string(),
                value: Some(value),
            })
            .collect(),
    }))
}

/// Extract a record field by label. Generated `FromValue` impls call this.
///
/// # Errors
/// Returns [`ValueError`] if `value` is not a record or has no such field.
pub fn record_field<'a>(value: &'a pb::Value, label: &str) -> Result<&'a pb::Value, ValueError> {
    match sum(value)? {
        pb::value::Sum::Record(record) => record
            .fields
            .iter()
            .find(|field| field.label == label)
            .and_then(|field| field.value.as_ref())
            .ok_or_else(|| ValueError::new(format!("record has no field `{label}`"))),
        other => Err(mismatch("Record", other)),
    }
}

// ---- enum / variant helpers (used by generated ToValue/FromValue) ----------

/// A `Unit` value — a nullary variant constructor's payload.
#[must_use]
pub fn unit_value() -> pb::Value {
    wrap(pb::value::Sum::Unit(()))
}

/// Build an `Enum` value from its constructor name.
#[must_use]
pub fn enum_value(constructor: &str) -> pb::Value {
    wrap(pb::value::Sum::Enum(pb::Enum {
        enum_id: None,
        constructor: constructor.to_string(),
    }))
}

/// The constructor name of an `Enum` value.
///
/// # Errors
/// Returns [`ValueError`] if `value` is not an enum.
pub fn enum_constructor(value: &pb::Value) -> Result<&str, ValueError> {
    match sum(value)? {
        pb::value::Sum::Enum(enumeration) => Ok(enumeration.constructor.as_str()),
        other => Err(mismatch("Enum", other)),
    }
}

/// Build a `Variant` value from a constructor name and its payload.
#[must_use]
pub fn variant_value(constructor: &str, value: pb::Value) -> pb::Value {
    wrap(pb::value::Sum::Variant(Box::new(pb::Variant {
        variant_id: None,
        constructor: constructor.to_string(),
        value: Some(Box::new(value)),
    })))
}

/// The constructor name and payload of a `Variant` value.
///
/// # Errors
/// Returns [`ValueError`] if `value` is not a variant or carries no payload.
pub fn variant_parts(value: &pb::Value) -> Result<(&str, &pb::Value), ValueError> {
    match sum(value)? {
        pb::value::Sum::Variant(variant) => {
            let payload = variant
                .value
                .as_deref()
                .ok_or_else(|| ValueError::new("variant carries no value"))?;
            Ok((variant.constructor.as_str(), payload))
        }
        other => Err(mismatch("Variant", other)),
    }
}

/// A "no such constructor `got` for `type_name`" error, for the fall-through arm
/// of a generated enum/variant `FromValue`.
#[must_use]
pub fn unexpected_constructor(type_name: &str, got: &str) -> ValueError {
    ValueError::new(format!("`{type_name}` has no constructor `{got}`"))
}

// ---- primitives -----------------------------------------------------------

impl ToValue for bool {
    fn to_value(&self) -> pb::Value {
        wrap(pb::value::Sum::Bool(*self))
    }
}
impl FromValue for bool {
    fn from_value(value: &pb::Value) -> Result<Self, ValueError> {
        match sum(value)? {
            pb::value::Sum::Bool(b) => Ok(*b),
            other => Err(mismatch("Bool", other)),
        }
    }
}

impl ToValue for i64 {
    fn to_value(&self) -> pb::Value {
        wrap(pb::value::Sum::Int64(*self))
    }
}
impl FromValue for i64 {
    fn from_value(value: &pb::Value) -> Result<Self, ValueError> {
        match sum(value)? {
            pb::value::Sum::Int64(n) => Ok(*n),
            other => Err(mismatch("Int64", other)),
        }
    }
}

impl ToValue for String {
    fn to_value(&self) -> pb::Value {
        wrap(pb::value::Sum::Text(self.clone()))
    }
}
impl FromValue for String {
    fn from_value(value: &pb::Value) -> Result<Self, ValueError> {
        match sum(value)? {
            pb::value::Sum::Text(s) => Ok(s.clone()),
            other => Err(mismatch("Text", other)),
        }
    }
}

impl ToValue for () {
    fn to_value(&self) -> pb::Value {
        wrap(pb::value::Sum::Unit(()))
    }
}
impl FromValue for () {
    fn from_value(value: &pb::Value) -> Result<Self, ValueError> {
        match sum(value)? {
            pb::value::Sum::Unit(()) => Ok(()),
            other => Err(mismatch("Unit", other)),
        }
    }
}

impl ToValue for Party {
    fn to_value(&self) -> pb::Value {
        wrap(pb::value::Sum::Party(self.0.clone()))
    }
}
impl FromValue for Party {
    fn from_value(value: &pb::Value) -> Result<Self, ValueError> {
        match sum(value)? {
            pb::value::Sum::Party(p) => Ok(Party(p.clone())),
            other => Err(mismatch("Party", other)),
        }
    }
}

impl ToValue for Numeric {
    fn to_value(&self) -> pb::Value {
        wrap(pb::value::Sum::Numeric(self.0.clone()))
    }
}
impl FromValue for Numeric {
    fn from_value(value: &pb::Value) -> Result<Self, ValueError> {
        match sum(value)? {
            pb::value::Sum::Numeric(n) => Ok(Numeric(n.clone())),
            other => Err(mismatch("Numeric", other)),
        }
    }
}

impl ToValue for Timestamp {
    fn to_value(&self) -> pb::Value {
        wrap(pb::value::Sum::Timestamp(self.0))
    }
}
impl FromValue for Timestamp {
    fn from_value(value: &pb::Value) -> Result<Self, ValueError> {
        match sum(value)? {
            pb::value::Sum::Timestamp(t) => Ok(Timestamp(*t)),
            other => Err(mismatch("Timestamp", other)),
        }
    }
}

impl ToValue for Date {
    fn to_value(&self) -> pb::Value {
        wrap(pb::value::Sum::Date(self.0))
    }
}
impl FromValue for Date {
    fn from_value(value: &pb::Value) -> Result<Self, ValueError> {
        match sum(value)? {
            pb::value::Sum::Date(d) => Ok(Date(*d)),
            other => Err(mismatch("Date", other)),
        }
    }
}

impl<T> ToValue for ContractId<T> {
    fn to_value(&self) -> pb::Value {
        wrap(pb::value::Sum::ContractId(self.as_str().to_string()))
    }
}
impl<T> FromValue for ContractId<T> {
    fn from_value(value: &pb::Value) -> Result<Self, ValueError> {
        match sum(value)? {
            pb::value::Sum::ContractId(id) => Ok(ContractId::new(id.clone())),
            other => Err(mismatch("ContractId", other)),
        }
    }
}

// ---- containers -----------------------------------------------------------

// `Box` is transparent to the codec: recursive generated types are boxed for
// Rust's sake, but a `Box<T>` encodes exactly as its `T`.
impl<T: ToValue> ToValue for Box<T> {
    fn to_value(&self) -> pb::Value {
        T::to_value(self)
    }
}
impl<T: FromValue> FromValue for Box<T> {
    fn from_value(value: &pb::Value) -> Result<Self, ValueError> {
        T::from_value(value).map(Box::new)
    }
}

impl<T: ToValue> ToValue for Option<T> {
    fn to_value(&self) -> pb::Value {
        wrap(pb::value::Sum::Optional(Box::new(pb::Optional {
            value: self.as_ref().map(|inner| Box::new(inner.to_value())),
        })))
    }
}
impl<T: FromValue> FromValue for Option<T> {
    fn from_value(value: &pb::Value) -> Result<Self, ValueError> {
        match sum(value)? {
            pb::value::Sum::Optional(opt) => match &opt.value {
                Some(inner) => Ok(Some(T::from_value(inner)?)),
                None => Ok(None),
            },
            other => Err(mismatch("Optional", other)),
        }
    }
}

// `NestedOpt` is a Daml Optional at a nested position; on the gRPC wire it is a
// proto `Optional`, exactly like `Option` (only its JSON form differs).
impl<T: ToValue> ToValue for NestedOpt<T> {
    fn to_value(&self) -> pb::Value {
        wrap(pb::value::Sum::Optional(Box::new(pb::Optional {
            value: self.0.as_ref().map(|inner| Box::new(inner.to_value())),
        })))
    }
}
impl<T: FromValue> FromValue for NestedOpt<T> {
    fn from_value(value: &pb::Value) -> Result<Self, ValueError> {
        match sum(value)? {
            pb::value::Sum::Optional(opt) => match &opt.value {
                Some(inner) => Ok(NestedOpt(Some(T::from_value(inner)?))),
                None => Ok(NestedOpt(None)),
            },
            other => Err(mismatch("Optional", other)),
        }
    }
}

impl<T: ToValue> ToValue for Vec<T> {
    fn to_value(&self) -> pb::Value {
        wrap(pb::value::Sum::List(pb::List {
            elements: self.iter().map(ToValue::to_value).collect(),
        }))
    }
}
impl<T: FromValue> FromValue for Vec<T> {
    fn from_value(value: &pb::Value) -> Result<Self, ValueError> {
        match sum(value)? {
            pb::value::Sum::List(list) => list.elements.iter().map(T::from_value).collect(),
            other => Err(mismatch("List", other)),
        }
    }
}

impl<V: ToValue> ToValue for BTreeMap<String, V> {
    fn to_value(&self) -> pb::Value {
        wrap(pb::value::Sum::TextMap(pb::TextMap {
            entries: self
                .iter()
                .map(|(key, value)| pb::text_map::Entry {
                    key: key.clone(),
                    value: Some(value.to_value()),
                })
                .collect(),
        }))
    }
}
impl<V: FromValue> FromValue for BTreeMap<String, V> {
    fn from_value(value: &pb::Value) -> Result<Self, ValueError> {
        match sum(value)? {
            pb::value::Sum::TextMap(map) => map
                .entries
                .iter()
                .map(|entry| {
                    let inner = entry
                        .value
                        .as_ref()
                        .ok_or_else(|| ValueError::new("TextMap entry has no value"))?;
                    Ok((entry.key.clone(), V::from_value(inner)?))
                })
                .collect(),
            other => Err(mismatch("TextMap", other)),
        }
    }
}

impl<K: ToValue, V: ToValue> ToValue for GenMap<K, V> {
    fn to_value(&self) -> pb::Value {
        wrap(pb::value::Sum::GenMap(pb::GenMap {
            entries: self
                .0
                .iter()
                .map(|(key, value)| pb::gen_map::Entry {
                    key: Some(key.to_value()),
                    value: Some(value.to_value()),
                })
                .collect(),
        }))
    }
}
impl<K: FromValue, V: FromValue> FromValue for GenMap<K, V> {
    fn from_value(value: &pb::Value) -> Result<Self, ValueError> {
        match sum(value)? {
            pb::value::Sum::GenMap(map) => {
                let entries = map
                    .entries
                    .iter()
                    .map(|entry| {
                        let key = entry
                            .key
                            .as_ref()
                            .ok_or_else(|| ValueError::new("GenMap entry has no key"))?;
                        let value = entry
                            .value
                            .as_ref()
                            .ok_or_else(|| ValueError::new("GenMap entry has no value"))?;
                        Ok((K::from_value(key)?, V::from_value(value)?))
                    })
                    .collect::<Result<Vec<_>, ValueError>>()?;
                Ok(GenMap(entries))
            }
            other => Err(mismatch("GenMap", other)),
        }
    }
}
