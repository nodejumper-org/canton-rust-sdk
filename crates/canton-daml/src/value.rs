//! Conversion between typed Rust values and the Ledger API [`Value`](pb::Value)
//! (the gRPC wire form). Generated records/variants/enums implement these
//! traits too; here they are provided for the primitive and container types.

use std::collections::BTreeMap;

use canton_proto::com::daml::ledger::api::v2 as pb;

use crate::primitives::{
    ContractId, Date, GenMap, Int64, NestedOpt, Numeric, Party, Timestamp, Unit,
};

/// An error converting a Ledger API [`Value`](pb::Value) into a typed value.
///
/// Carries the **path** to the offending spot inside the payload — a decode
/// failure deep in a nested record reports `owner.address.city: expected Text`
/// rather than just `expected Text`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValueError {
    /// Field path from the root of the decoded value, outermost first.
    path: Vec<String>,
    message: String,
}

impl ValueError {
    /// A conversion error with no path context yet. Generated `FromValue`
    /// bodies wrap each field decode in [`ValueError::at`], so by the time one
    /// of these reaches a caller it names the field it came from.
    #[must_use]
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            path: Vec::new(),
            message: message.into(),
        }
    }

    /// Prepend a field (or index) to the path — called as the error propagates
    /// out of a nested value, so the path reads outermost-first.
    #[must_use]
    pub fn at(mut self, field: impl Into<String>) -> Self {
        self.path.insert(0, field.into());
        self
    }

    /// The failure description, without the path.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// The field path from the root of the value, outermost first; empty when
    /// the failure was at the root.
    #[must_use]
    pub fn path(&self) -> &[String] {
        &self.path
    }
}

impl std::fmt::Display for ValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.path.is_empty() {
            write!(f, "value conversion error: {}", self.message)
        } else {
            write!(
                f,
                "value conversion error at `{}`: {}",
                self.path.join("."),
                self.message
            )
        }
    }
}

impl std::error::Error for ValueError {}

/// Lets `?` carry a codec failure straight into the SDK-wide error type, so one
/// function can call the typed runtime and the ledger client and return
/// `canton::Result`.
impl From<ValueError> for canton_core::Error {
    fn from(error: ValueError) -> Self {
        canton_core::Error::Payload(Box::new(error))
    }
}

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

/// The name of a `Value`'s kind — `"Record"`, `"Party"`, … — and nothing else.
///
/// Deliberately not `{sum:?}`: prost's derived `Debug` prints a value in full,
/// so a type mismatch on a contract payload would copy party ids, amounts and
/// free text into the error message. That message travels through
/// `From<ValueError> for canton_core::Error` into whatever the application
/// logs, so the leak would land in traces and metrics of code that never asked
/// to see the payload.
///
/// The match is exhaustive on purpose: a `Value` kind added in a future Ledger
/// API is a build break here, not a silent `"unknown"`.
fn kind(sum: &pb::value::Sum) -> &'static str {
    use pb::value::Sum;
    match sum {
        Sum::Unit(()) => "Unit",
        Sum::Bool(_) => "Bool",
        Sum::Int64(_) => "Int64",
        Sum::Date(_) => "Date",
        Sum::Timestamp(_) => "Timestamp",
        Sum::Numeric(_) => "Numeric",
        Sum::Party(_) => "Party",
        Sum::Text(_) => "Text",
        Sum::ContractId(_) => "ContractId",
        Sum::Optional(_) => "Optional",
        Sum::List(_) => "List",
        Sum::TextMap(_) => "TextMap",
        Sum::GenMap(_) => "GenMap",
        Sum::Record(_) => "Record",
        Sum::Variant(_) => "Variant",
        Sum::Enum(_) => "Enum",
    }
}

fn mismatch(expected: &str, got: &pb::value::Sum) -> ValueError {
    ValueError::new(format!("expected {expected}, got {}", kind(got)))
}

// ---- record helpers (used by generated ToValue/FromValue) ------------------

/// Build a bare `Record` from labelled fields. Generated `Template::to_record`
/// impls call this; [`record`] wraps the result as a [`Value`](pb::Value).
#[must_use]
pub fn record_fields(fields: Vec<(&str, pb::Value)>) -> pb::Record {
    pb::Record {
        record_id: None,
        fields: fields
            .into_iter()
            .map(|(label, value)| pb::RecordField {
                label: label.to_string(),
                value: Some(value),
            })
            .collect(),
    }
}

/// Build a `Record` [`Value`](pb::Value) from labelled fields. Generated
/// `ToValue` impls call this.
#[must_use]
pub fn record(fields: Vec<(&str, pb::Value)>) -> pb::Value {
    wrap(pb::value::Sum::Record(record_fields(fields)))
}

/// Locate a record field by **label or declaration position**, tolerating the
/// two shapes Canton legitimately produces:
///
/// - **non-verbose** output omits labels entirely — the field is matched by its
///   declaration `index`;
/// - **normalized** records (Smart Contract Upgrade) omit trailing fields whose
///   value is an empty `Optional` — the field is *absent*, which is `Ok(None)`
///   here so an `Optional`-typed field can decode as `None`.
///
/// Labels win when present; position applies only to unlabelled fields, so a
/// verbose record whose trailing fields were normalized away never mis-binds.
///
/// # Errors
/// Returns [`ValueError`] if `value` is not a record.
pub fn find_field<'a>(
    value: &'a pb::Value,
    index: usize,
    label: &str,
) -> Result<Option<&'a pb::Value>, ValueError> {
    match sum(value)? {
        pb::value::Sum::Record(record) => {
            if let Some(field) = record
                .fields
                .iter()
                .find(|field| !field.label.is_empty() && field.label == label)
            {
                return Ok(field.value.as_ref());
            }
            if let Some(field) = record.fields.get(index)
                && field.label.is_empty()
            {
                return Ok(field.value.as_ref());
            }
            Ok(None)
        }
        other => Err(mismatch("Record", other)),
    }
}

/// [`find_field`] for a field the type requires: absence is an error naming the
/// field. Generated `FromValue` impls call this for non-`Optional` fields.
///
/// # Errors
/// Returns [`ValueError`] if `value` is not a record or the field is absent.
pub fn required_field<'a>(
    value: &'a pb::Value,
    index: usize,
    label: &str,
) -> Result<&'a pb::Value, ValueError> {
    find_field(value, index, label)?
        .ok_or_else(|| ValueError::new(format!("record has no field `{label}` (index {index})")))
}

/// Decode an `Optional`-typed record field, treating an **absent** field as
/// `None`: Canton normalizes records by dropping trailing empty-`Optional`
/// fields, so absence of an optional field is a legitimate wire shape, not an
/// error. Generated `FromValue` impls call this for `Optional` fields.
///
/// # Errors
/// Returns [`ValueError`] if `value` is not a record or the present field's
/// shape does not match `T`.
pub fn optional_field<T: FromValue + AbsentField>(
    value: &pb::Value,
    index: usize,
    label: &str,
) -> Result<T, ValueError> {
    match find_field(value, index, label)? {
        Some(present) => T::from_value(present),
        None => Ok(T::absent()),
    }
}

/// The decoded form of an **absent** optional record field (`None`).
/// Implemented by the two Rust spellings of a Daml `Optional` field.
pub trait AbsentField {
    /// The value an absent field decodes to.
    fn absent() -> Self;
}

impl<T> AbsentField for Option<T> {
    fn absent() -> Self {
        None
    }
}

impl<T> AbsentField for NestedOpt<T> {
    fn absent() -> Self {
        NestedOpt(None)
    }
}

/// Decode a bare wire `Record` — the shape `CreatedEvent.create_arguments`
/// carries — into a typed value. See also
/// [`Template::from_created_event`](crate::Template::from_created_event),
/// which wraps this with a template-identity check.
///
/// # Errors
/// Returns [`ValueError`] if the record's shape does not match `T`.
pub fn from_record<T: FromValue>(record: &pb::Record) -> Result<T, ValueError> {
    let value = pb::Value {
        sum: Some(pb::value::Sum::Record(record.clone())),
    };
    T::from_value(&value)
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

/// **gRPC only.** The wire form here is right, but `i64`'s serde half is std's:
/// it writes a JSON *number* and refuses the string the Ledger API sends. Use
/// [`Int64`](crate::Int64) for anything that also travels as JSON — which is
/// what generated code uses, and why that newtype exists.
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

impl ToValue for Int64 {
    fn to_value(&self) -> pb::Value {
        wrap(pb::value::Sum::Int64(self.0))
    }
}
impl FromValue for Int64 {
    fn from_value(value: &pb::Value) -> Result<Self, ValueError> {
        match sum(value)? {
            pb::value::Sum::Int64(n) => Ok(Int64(*n)),
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

/// **gRPC only.** As with `i64`: correct on the wire, but `()` serializes to
/// JSON `null` where Daml `Unit` is `{}`. Use [`Unit`](crate::Unit) for the
/// JSON lane.
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

impl ToValue for Unit {
    fn to_value(&self) -> pb::Value {
        unit_value()
    }
}
impl FromValue for Unit {
    fn from_value(value: &pb::Value) -> Result<Self, ValueError> {
        match sum(value)? {
            pb::value::Sum::Unit(()) => Ok(Unit),
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
            pb::value::Sum::Numeric(n) => Ok(Numeric::from_wire(n.clone())),
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
            // The index goes on the path for the same reason a field name does:
            // without it, one bad element in a list of five hundred reports
            // only that the list is bad.
            pb::value::Sum::List(list) => list
                .elements
                .iter()
                .enumerate()
                .map(|(index, element)| {
                    T::from_value(element).map_err(|error| error.at(index.to_string()))
                })
                .collect(),
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
                    let inner = entry.value.as_ref().ok_or_else(|| {
                        ValueError::new("TextMap entry has no value").at(entry.key.clone())
                    })?;
                    // The key locates the entry better than a position would.
                    let value =
                        V::from_value(inner).map_err(|error| error.at(entry.key.clone()))?;
                    Ok((entry.key.clone(), value))
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
                    .enumerate()
                    .map(|(index, entry)| {
                        // A GenMap key is an arbitrary value, so unlike a
                        // TextMap there is no name to point at — the position
                        // is what a reader can act on. `key`/`value` says which
                        // half of the entry failed.
                        let at = |part: &str| {
                            let part = part.to_string();
                            move |error: ValueError| error.at(part).at(index.to_string())
                        };
                        let key = entry
                            .key
                            .as_ref()
                            .ok_or_else(|| ValueError::new("GenMap entry has no key"))
                            .map_err(at("key"))?;
                        let value = entry
                            .value
                            .as_ref()
                            .ok_or_else(|| ValueError::new("GenMap entry has no value"))
                            .map_err(at("value"))?;
                        Ok((
                            K::from_value(key).map_err(at("key"))?,
                            V::from_value(value).map_err(at("value"))?,
                        ))
                    })
                    .collect::<Result<Vec<_>, ValueError>>()?;
                Ok(GenMap(entries))
            }
            other => Err(mismatch("GenMap", other)),
        }
    }
}
