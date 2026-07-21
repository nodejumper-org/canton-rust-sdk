//! Conversion between typed Rust values and the Ledger API [`Value`](pb::Value)
//! (the gRPC wire form). Generated records/variants/enums implement these
//! traits too; here they are provided for the primitive and container types.

use std::collections::BTreeMap;

use canton_proto::com::daml::ledger::api::v2 as pb;

use crate::primitives::{ContractId, Date, Numeric, Party, Timestamp};

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
