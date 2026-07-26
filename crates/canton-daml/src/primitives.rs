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
/// is the id string), so `ContractId` is `Clone`/`Eq`/`Hash`/`Ord` regardless
/// of `T` (all manual impls: a derive would wrongly bound `T`).
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

    /// Re-tag the id as a handle to a different type `U` — the id string is
    /// unchanged. The intended use is exercising an **interface** choice on a
    /// contract read back as its concrete template:
    /// `cid.retag::<Holding>()` turns a `ContractId<Amulet>` into the
    /// `ContractId<Holding>` that `exercise_command` needs. The cast is not
    /// checked against the template's `implements` list, so exercising a
    /// mis-tagged id fails at the ledger, not at compile time.
    #[must_use]
    pub fn retag<U>(self) -> ContractId<U> {
        ContractId::new(self.id)
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

impl<T> std::hash::Hash for ContractId<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> PartialOrd for ContractId<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for ContractId<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

/// A fixed-scale decimal (`Numeric`), stored as its decimal string — the wire
/// representation (Daml numerics are transmitted as text to avoid binary-float
/// rounding).
///
/// **Comparison is numeric, not textual**: the ledger echoes values at the
/// type's full scale (submit `"1.5"` on a `Numeric 10`, read back
/// `"1.5000000000"`), so `==`/`Ord`/`Hash` compare the canonical decimal value
/// — `Numeric("1.5") == Numeric("1.5000000000")`. A string that does not parse
/// as a decimal falls back to plain string comparison (and never equals a
/// valid decimal).
///
/// LF-JSON: **emitted as a string** (what the Ledger API produces); on input
/// **also accepts a JSON number** (the spec allows it — high-precision values
/// should still use the string form, since a JSON number literal is already
/// `f64`-lossy on the wire).
#[derive(Clone, Debug)]
pub struct Numeric(pub String);

impl Numeric {
    /// Parse a decimal string (`-?digits[.digits]`; a leading `+` is accepted),
    /// rejecting anything the ledger would reject — use this over raw
    /// construction so a typo fails here instead of at command submission.
    ///
    /// # Errors
    /// Returns the offending input when it is not a plain decimal literal.
    pub fn parse(text: impl Into<String>) -> Result<Self, String> {
        let text = text.into();
        if canonical_decimal(&text).is_some() {
            Ok(Self(text))
        } else {
            Err(format!(
                "`{text}` is not a decimal literal (expected -?digits[.digits])"
            ))
        }
    }

    /// The canonical form used for comparison (sign-normalised, no leading or
    /// trailing zeros), or `None` when the content is not a decimal literal.
    fn canonical(&self) -> Option<String> {
        canonical_decimal(&self.0)
    }
}

/// Canonicalise a decimal literal: strip an explicit `+`, leading integer
/// zeros, trailing fraction zeros, and normalise `-0` to `0`. `None` if the
/// input is not `[+-]?digits[.digits]`.
fn canonical_decimal(raw: &str) -> Option<String> {
    let (negative, digits) = match raw.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, raw.strip_prefix('+').unwrap_or(raw)),
    };
    let has_dot = digits.contains('.');
    let (int, frac) = digits.split_once('.').unwrap_or((digits, ""));
    let valid = !int.is_empty()
        && int.bytes().all(|b| b.is_ascii_digit())
        // With a dot, the fraction must be present and all digits ("1." is not
        // a decimal literal; a second dot lands in `frac` and fails here).
        && (!has_dot || (!frac.is_empty() && frac.bytes().all(|b| b.is_ascii_digit())));
    if !valid {
        return None;
    }
    let int = int.trim_start_matches('0');
    let int = if int.is_empty() { "0" } else { int };
    let frac = frac.trim_end_matches('0');
    let mut canonical = String::with_capacity(raw.len() + 1);
    if negative && !(int == "0" && frac.is_empty()) {
        canonical.push('-');
    }
    canonical.push_str(int);
    if !frac.is_empty() {
        canonical.push('.');
        canonical.push_str(frac);
    }
    Some(canonical)
}

impl PartialEq for Numeric {
    fn eq(&self, other: &Self) -> bool {
        match (self.canonical(), other.canonical()) {
            (Some(a), Some(b)) => a == b,
            (None, None) => self.0 == other.0,
            _ => false,
        }
    }
}

impl Eq for Numeric {}

impl std::hash::Hash for Numeric {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Hash the same key equality compares, so Eq and Hash agree.
        match self.canonical() {
            Some(canonical) => canonical.hash(state),
            None => self.0.hash(state),
        }
    }
}

impl PartialOrd for Numeric {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Numeric {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        use std::cmp::Ordering;
        let (Some(a), Some(b)) = (self.canonical(), other.canonical()) else {
            // Non-decimal content: fall back to string order (valid decimals
            // sort before invalid strings, deterministically).
            return match (self.canonical(), other.canonical()) {
                (Some(_), None) => Ordering::Less,
                (None, Some(_)) => Ordering::Greater,
                _ => self.0.cmp(&other.0),
            };
        };
        let (neg_a, mag_a) = a
            .strip_prefix('-')
            .map_or((false, a.as_str()), |m| (true, m));
        let (neg_b, mag_b) = b
            .strip_prefix('-')
            .map_or((false, b.as_str()), |m| (true, m));
        match (neg_a, neg_b) {
            (false, true) => Ordering::Greater,
            (true, false) => Ordering::Less,
            (negative, _) => {
                let ordering = magnitude_cmp(mag_a, mag_b);
                if negative {
                    ordering.reverse()
                } else {
                    ordering
                }
            }
        }
    }
}

/// Compare two canonical non-negative decimals numerically: integer parts by
/// length then lexicographically, then fraction parts lexicographically
/// (canonical fractions carry no trailing zeros, so shorter-is-prefix means
/// smaller).
fn magnitude_cmp(a: &str, b: &str) -> std::cmp::Ordering {
    let (int_a, frac_a) = a.split_once('.').unwrap_or((a, ""));
    let (int_b, frac_b) = b.split_once('.').unwrap_or((b, ""));
    int_a
        .len()
        .cmp(&int_b.len())
        .then_with(|| int_a.cmp(int_b))
        .then_with(|| frac_a.cmp(frac_b))
}

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
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp(pub i64);

/// A Daml `Date` — days since the Unix epoch.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
        // Euclidean division floors toward negative infinity, so sub-microsecond
        // instants before the epoch truncate consistently (plain `/` would round
        // them toward zero, i.e. *forward* in time).
        let micros = datetime.unix_timestamp_nanos().div_euclid(1_000);
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
