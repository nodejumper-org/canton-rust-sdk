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
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Numeric(pub String);

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
