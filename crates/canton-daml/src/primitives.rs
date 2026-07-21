//! The Daml primitive runtime types that generated bindings use.

use std::collections::BTreeMap;
use std::fmt;
use std::marker::PhantomData;

/// A Daml `Party` identifier.
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
#[derive(Clone, Debug, PartialEq, Eq)]
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
/// on the wire).
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct GenMap<K, V>(pub Vec<(K, V)>);
