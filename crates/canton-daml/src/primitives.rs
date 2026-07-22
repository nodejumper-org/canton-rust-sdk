//! The Daml primitive runtime types that generated bindings use.

use std::collections::BTreeMap;
use std::fmt;
use std::marker::PhantomData;

/// A Daml `Party` identifier.
#[derive(
    Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(transparent)]
pub struct Party(pub String);

impl Party {
    /// Wrap a party-id string.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
}

/// A typed contract id: a handle to a contract of template/type `T`.
///
/// `T` is a compile-time tag only — it carries no runtime data (the wire form
/// is the id string), so `ContractId` is `Clone`/`Eq` regardless of `T`.
pub struct ContractId<T> {
    id: String,
    _marker: PhantomData<fn() -> T>,
}

impl<T> ContractId<T> {
    /// Wrap a contract-id string.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            _marker: PhantomData,
        }
    }

    /// The underlying contract-id string.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.id
    }
}

impl<T> Clone for ContractId<T> {
    fn clone(&self) -> Self {
        Self::new(self.id.clone())
    }
}

impl<T> fmt::Debug for ContractId<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("ContractId").field(&self.id).finish()
    }
}

impl<T> PartialEq for ContractId<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for ContractId<T> {}

/// A fixed-scale decimal (`Numeric`), stored as its canonical decimal string —
/// the wire representation (Daml numerics are transmitted as text to avoid
/// binary-float rounding).
///
/// LF-JSON: **emitted as a string** (what the Ledger API produces); on input
/// **also accepts a JSON number** (the spec allows it — high-precision values
/// should still use the string form, since a JSON number literal is already
/// `f64`-lossy on the wire).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Numeric(pub String);

impl serde::Serialize for Numeric {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0)
    }
}

impl<'de> serde::Deserialize<'de> for Numeric {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct NumericVisitor;
        impl serde::de::Visitor<'_> for NumericVisitor {
            type Value = Numeric;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("a Daml Numeric as a decimal string or a JSON number")
            }
            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Numeric, E> {
                Ok(Numeric(value.to_string()))
            }
            fn visit_i64<E: serde::de::Error>(self, value: i64) -> Result<Numeric, E> {
                Ok(Numeric(value.to_string()))
            }
            fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Numeric, E> {
                Ok(Numeric(value.to_string()))
            }
            fn visit_f64<E: serde::de::Error>(self, value: f64) -> Result<Numeric, E> {
                Ok(Numeric(value.to_string()))
            }
        }
        deserializer.deserialize_any(NumericVisitor)
    }
}

/// A Daml `Int64`.
///
/// LF-JSON encodes `Int64` **as a string** (`encodeInt64AsString`, to survive
/// JavaScript's 53-bit number precision), and accepts a string *or* a number on
/// input. So this serializes as a string (matching the Ledger API's output) and
/// deserializes from either form. The gRPC (`Value`) form is a plain `Int64`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Int64(pub i64);

impl From<i64> for Int64 {
    fn from(value: i64) -> Self {
        Self(value)
    }
}

impl serde::Serialize for Int64 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for Int64 {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct Int64Visitor;
        impl serde::de::Visitor<'_> for Int64Visitor {
            type Value = Int64;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("an Int64 as a JSON number or a string")
            }
            fn visit_i64<E: serde::de::Error>(self, value: i64) -> Result<Int64, E> {
                Ok(Int64(value))
            }
            fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Int64, E> {
                i64::try_from(value)
                    .map(Int64)
                    .map_err(serde::de::Error::custom)
            }
            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Int64, E> {
                value
                    .parse::<i64>()
                    .map(Int64)
                    .map_err(serde::de::Error::custom)
            }
        }
        deserializer.deserialize_any(Int64Visitor)
    }
}

/// A Daml `Timestamp` — microseconds since the Unix epoch (UTC).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(pub i64);

/// A Daml `Date` — days since the Unix epoch.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date(pub i32);

/// A Daml `TextMap` — a map keyed by `Text`.
pub type TextMap<V> = BTreeMap<String, V>;

/// A Daml `GenMap` — a map with an arbitrary key type (ordered key/value pairs
/// on the wire; the LF-JSON form is an array of `[key, value]` pairs).
#[derive(Clone, Debug, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct GenMap<K, V>(pub Vec<(K, V)>);

/// 1970-01-01, the anchor for [`Date`]'s day count.
const UNIX_EPOCH_DATE: time::Date = time::macros::date!(1970 - 01 - 01);

/// The LF-JSON `Date` format: `YYYY-MM-DD`.
const DATE_FORMAT: &[time::format_description::FormatItem<'static>] =
    time::macros::format_description!("[year]-[month]-[day]");

// `ContractId`, `Timestamp`, and `Date` need manual serde: the first is generic
// over a phantom tag, and the latter two render as LF-JSON ISO strings, not as
// their raw integers.

impl<T> serde::Serialize for ContractId<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.id)
    }
}

impl<'de, T> serde::Deserialize<'de> for ContractId<T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::new(String::deserialize(deserializer)?))
    }
}

impl serde::Serialize for Timestamp {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let datetime = time::OffsetDateTime::from_unix_timestamp_nanos(i128::from(self.0) * 1_000)
            .map_err(serde::ser::Error::custom)?;
        let text = datetime
            .format(&time::format_description::well_known::Rfc3339)
            .map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&text)
    }
}

impl<'de> serde::Deserialize<'de> for Timestamp {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        let datetime =
            time::OffsetDateTime::parse(&text, &time::format_description::well_known::Rfc3339)
                .map_err(serde::de::Error::custom)?;
        let micros = datetime.unix_timestamp_nanos() / 1_000;
        Ok(Self(
            i64::try_from(micros).map_err(serde::de::Error::custom)?,
        ))
    }
}

impl serde::Serialize for Date {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let date = UNIX_EPOCH_DATE
            .checked_add(time::Duration::days(i64::from(self.0)))
            .ok_or_else(|| serde::ser::Error::custom("date out of range"))?;
        let text = date
            .format(DATE_FORMAT)
            .map_err(serde::ser::Error::custom)?;
        serializer.serialize_str(&text)
    }
}

impl<'de> serde::Deserialize<'de> for Date {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        let date = time::Date::parse(&text, DATE_FORMAT).map_err(serde::de::Error::custom)?;
        let days = (date - UNIX_EPOCH_DATE).whole_days();
        Ok(Self(i32::try_from(days).map_err(serde::de::Error::custom)?))
    }
}

/// A Daml `Optional` **nested inside another `Optional`**. Same runtime meaning
/// as [`Option`], but its LF-JSON encoding is the nested-optional *list* form
/// (`None → []`, `Some(x) → [x]`) instead of `null`/value.
///
/// The Daml-LF JSON spec keeps the top-level Optional as `null`/value, but every
/// Optional below it must use the list form so `Some None` (encoded `[]`) is
/// distinguishable from `None` (encoded `null`). Codegen wraps each nested
/// Optional layer in `NestedOpt`; the gRPC (`Value`) encoding is identical to
/// `Option` (a proto `Optional`), so only the JSON form differs.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct NestedOpt<T>(pub Option<T>);

impl<T> NestedOpt<T> {
    /// Wrap an [`Option`].
    pub fn new(value: Option<T>) -> Self {
        Self(value)
    }
}

impl<T: serde::Serialize> serde::Serialize for NestedOpt<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // Nested-optional rule: None → [], Some(x) → [x]. If `T` is itself a
        // `NestedOpt`, its own serde recurses, giving `[[]]` / `[[x]]`, etc.
        match &self.0 {
            None => serializer.collect_seq(core::iter::empty::<&T>()),
            Some(value) => serializer.collect_seq(core::iter::once(value)),
        }
    }
}

impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for NestedOpt<T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let mut items = Vec::<T>::deserialize(deserializer)?;
        match items.len() {
            0 => Ok(Self(None)),
            1 => Ok(Self(Some(items.remove(0)))),
            n => Err(serde::de::Error::custom(format!(
                "nested Optional expects a 0- or 1-element array, got {n}"
            ))),
        }
    }
}

/// The Daml `Unit` type. Its LF-JSON form is the **empty object** `{}` — not
/// JSON `null`, which is what Rust's `()` serializes to — so `Unit` needs its
/// own type with a manual serde impl. This is what a nullary variant
/// constructor and a `Unit`-typed field map to, so their JSON matches the
/// Ledger API's `{"tag": <ctor>, "value": {}}` form. The gRPC (`Value`) form is
/// the proto `Unit`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Unit;

impl serde::Serialize for Unit {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // An empty map serializes as `{}`.
        use serde::ser::SerializeMap;
        serializer.serialize_map(Some(0))?.end()
    }
}

impl<'de> serde::Deserialize<'de> for Unit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        /// Accepts the LF-JSON `{}` (and tolerates `null`), ignoring any content.
        struct UnitVisitor;
        impl<'de> serde::de::Visitor<'de> for UnitVisitor {
            type Value = Unit;
            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("the empty object `{}` (Daml Unit)")
            }
            fn visit_map<A: serde::de::MapAccess<'de>>(self, mut map: A) -> Result<Unit, A::Error> {
                while map
                    .next_entry::<serde::de::IgnoredAny, serde::de::IgnoredAny>()?
                    .is_some()
                {}
                Ok(Unit)
            }
            fn visit_unit<E: serde::de::Error>(self) -> Result<Unit, E> {
                Ok(Unit)
            }
        }
        deserializer.deserialize_any(UnitVisitor)
    }
}
