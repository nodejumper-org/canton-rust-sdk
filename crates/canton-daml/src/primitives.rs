//! The Daml primitive runtime types that generated bindings use.

use std::collections::BTreeMap;
use std::fmt;
use std::marker::PhantomData;

/// A Daml `Party` identifier.
#[derive(
    Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, serde::Serialize, serde::Deserialize,
)]
#[serde(transparent)]
pub struct Party(pub(crate) String);

impl Party {
    /// Wrap a party-id string.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// The party id.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consume the wrapper, yielding the party id.
    #[must_use]
    pub fn into_string(self) -> String {
        self.0
    }
}

impl fmt::Display for Party {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for Party {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl std::str::FromStr for Party {
    type Err = std::convert::Infallible;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(id))
    }
}

impl From<String> for Party {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for Party {
    fn from(id: &str) -> Self {
        Self::new(id)
    }
}

impl From<Party> for String {
    fn from(party: Party) -> Self {
        party.0
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
///
/// **Validation is asymmetric on purpose.** [`Numeric::parse`] / [`str::parse`]
/// reject anything that is not a decimal literal, so a caller's typo fails at
/// construction. Values arriving from the ledger — deserialization and the gRPC
/// codec — are taken as-is via [`Numeric::from_wire`]: the participant is the
/// authority on the format, and rejecting an unfamiliar-but-harmless spelling
/// would break decoding of a payload the ledger considers valid. A value that
/// is not a decimal literal simply falls out of canonical comparison and never
/// equals a valid one.
#[derive(Clone, Debug)]
pub struct Numeric(pub(crate) String);

impl Numeric {
    /// Parse a decimal string (`-?digits[.digits]`; a leading `+` is accepted),
    /// rejecting anything the ledger would reject — use this over raw
    /// construction so a typo fails here instead of at command submission.
    ///
    /// # Errors
    /// Returns [`NumericParseError`] when the input is not a decimal literal.
    pub fn parse(text: impl Into<String>) -> Result<Self, NumericParseError> {
        let text = text.into();
        if canonical_decimal(&text).is_some() {
            Ok(Self(text))
        } else {
            Err(NumericParseError { input: text })
        }
    }

    /// Wrap a decimal string **without validating it** — for values that came
    /// off the wire, where the ledger has already vouched for the format.
    /// Prefer [`Numeric::parse`] for values a caller typed.
    #[must_use]
    pub fn from_wire(text: impl Into<String>) -> Self {
        Self(text.into())
    }

    /// The decimal string, as stored (the wire form, not canonicalised).
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The canonical form used for comparison (sign-normalised, no leading or
    /// trailing zeros), or `None` when the content is not a decimal literal.
    fn canonical(&self) -> Option<String> {
        canonical_decimal(&self.0)
    }
}

/// A string that is not a Daml `Numeric` (a decimal literal).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NumericParseError {
    input: String,
}

impl NumericParseError {
    /// The rejected input.
    #[must_use]
    pub fn input(&self) -> &str {
        &self.input
    }
}

impl fmt::Display for NumericParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "`{}` is not a decimal literal (expected -?digits[.digits])",
            self.input
        )
    }
}

impl std::error::Error for NumericParseError {}

impl std::str::FromStr for Numeric {
    type Err = NumericParseError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::parse(text)
    }
}

impl fmt::Display for Numeric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl TryFrom<&str> for Numeric {
    type Error = NumericParseError;
    fn try_from(text: &str) -> Result<Self, Self::Error> {
        Self::parse(text)
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
    let (int, frac) = digits.split_once('.').unwrap_or((digits, ""));
    // A trailing dot with no digits (`42.`) is a legal spelling of `42`; a
    // second dot lands in `frac` and fails the digit check below.
    let valid = !int.is_empty()
        && int.bytes().all(|b| b.is_ascii_digit())
        && frac.bytes().all(|b| b.is_ascii_digit());
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

impl From<Int64> for i64 {
    fn from(value: Int64) -> Self {
        value.0
    }
}

impl fmt::Display for Int64 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for Int64 {
    type Err = std::num::ParseIntError;
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        text.parse().map(Self)
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
///
/// Convert to and from [`time::OffsetDateTime`] rather than doing epoch
/// arithmetic by hand:
///
/// ```
/// use canton_daml::Timestamp;
/// use time::macros::datetime;
///
/// let at = Timestamp::from_datetime(datetime!(2026-07-31 12:00 UTC)).unwrap();
/// assert_eq!(at.to_datetime().unwrap().year(), 2026);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Timestamp(pub i64);

impl Timestamp {
    /// Microseconds since the Unix epoch.
    #[must_use]
    pub fn micros(self) -> i64 {
        self.0
    }

    /// The instant as a UTC date-time, or `None` when the microsecond count is
    /// outside the range `time` can represent.
    #[must_use]
    pub fn to_datetime(self) -> Option<time::OffsetDateTime> {
        time::OffsetDateTime::from_unix_timestamp_nanos(i128::from(self.0) * 1_000).ok()
    }

    /// The timestamp for a date-time, truncated toward the past (so
    /// sub-microsecond digits never move an instant forward), or `None` when
    /// the instant is outside the microsecond range a Daml `Timestamp` can
    /// hold. Never saturates: a silent clamp would turn an out-of-range instant
    /// into a plausible-looking wrong one.
    #[must_use]
    pub fn from_datetime(at: time::OffsetDateTime) -> Option<Self> {
        let micros = at.unix_timestamp_nanos().div_euclid(1_000);
        i64::try_from(micros).ok().map(Self)
    }
}

impl From<Timestamp> for i64 {
    fn from(at: Timestamp) -> Self {
        at.0
    }
}

/// A Daml `Date` — days since the Unix epoch.
///
/// Convert to and from [`time::Date`] rather than counting days by hand.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Date(pub i32);

impl Date {
    /// Days since the Unix epoch.
    #[must_use]
    pub fn days(self) -> i32 {
        self.0
    }

    /// The calendar date, or `None` when the day count is outside the range
    /// `time` can represent.
    #[must_use]
    pub fn to_date(self) -> Option<time::Date> {
        UNIX_EPOCH_DATE.checked_add(time::Duration::days(i64::from(self.0)))
    }

    /// The Daml date for a calendar date, or `None` when the day count is
    /// outside the range a Daml `Date` can hold.
    ///
    /// No `time::Date` reaches that bound: the year range is ±9999 here (and
    /// ±999999 under `time`'s `large-dates`), against the ~5.9 million years an
    /// `i32` day count spans. The `Option` is kept because [`Date::to_date`]
    /// genuinely needs one — a `Date` decoded from the wire can name a day
    /// `time` cannot represent — and a pair of conversions that disagree on
    /// fallibility invites the caller to assume the wrong one is total.
    #[must_use]
    pub fn from_date(date: time::Date) -> Option<Self> {
        i32::try_from((date - UNIX_EPOCH_DATE).whole_days())
            .ok()
            .map(Self)
    }
}

impl From<Date> for i32 {
    fn from(date: Date) -> Self {
        date.0
    }
}

/// A Daml `TextMap` — a map keyed by `Text`.
pub type TextMap<V> = BTreeMap<String, V>;

/// A Daml `GenMap` — a map with an arbitrary key type (key/value pairs on the
/// wire; the LF-JSON form is an array of `[key, value]` pairs).
///
/// Entry order is a wire detail, not part of the value: two maps with the same
/// entries compare equal regardless of the order the ledger returned them in.
#[derive(Clone, Debug, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct GenMap<K, V>(pub(crate) Vec<(K, V)>);

// Hand-written so the bound is on nothing: an empty map exists for every key
// and value type. `#[derive(Default)]` would require `K: Default, V: Default`,
// which no Daml primitive satisfies — `GenMap::<Party, Numeric>::new()` would
// not compile.
impl<K, V> Default for GenMap<K, V> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<K, V> GenMap<K, V> {
    /// An empty map.
    #[must_use]
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// The entries, in wire order.
    pub fn iter(&self) -> std::slice::Iter<'_, (K, V)> {
        self.0.iter()
    }

    /// The number of entries.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Whether the map has no entries.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Append an entry. Daml forbids duplicate keys; this does not check for
    /// them, so build maps from distinct keys.
    pub fn insert(&mut self, key: K, value: V) {
        self.0.push((key, value));
    }
}

impl<K, V> From<Vec<(K, V)>> for GenMap<K, V> {
    fn from(entries: Vec<(K, V)>) -> Self {
        Self(entries)
    }
}

impl<K, V> FromIterator<(K, V)> for GenMap<K, V> {
    fn from_iter<I: IntoIterator<Item = (K, V)>>(entries: I) -> Self {
        Self(entries.into_iter().collect())
    }
}

impl<K, V> IntoIterator for GenMap<K, V> {
    type Item = (K, V);
    type IntoIter = std::vec::IntoIter<(K, V)>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a, K, V> IntoIterator for &'a GenMap<K, V> {
    type Item = &'a (K, V);
    type IntoIter = std::slice::Iter<'a, (K, V)>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

/// Entry order is a wire detail: equality compares the entries as a *multiset*,
/// so a map read back from the ledger in a different order still equals the one
/// submitted — while a repeated entry still has to appear the same number of
/// times on both sides (a plain "every entry occurs in the other" test would be
/// asymmetric: `[a, a]` would equal `[a, b]` but not the reverse).
impl<K: PartialEq, V: PartialEq> PartialEq for GenMap<K, V> {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        // O(n²), but a GenMap is a handful of entries and `K` is only
        // `PartialEq` — there is no ordering or hashing to exploit.
        let mut matched = vec![false; other.0.len()];
        self.0.iter().all(|entry| {
            other
                .0
                .iter()
                .enumerate()
                .position(|(index, candidate)| !matched[index] && candidate == entry)
                .is_some_and(|index| {
                    matched[index] = true;
                    true
                })
        })
    }
}

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

    /// Unwrap to the [`Option`].
    #[must_use]
    pub fn into_inner(self) -> Option<T> {
        self.0
    }
}

impl<T> From<Option<T>> for NestedOpt<T> {
    fn from(value: Option<T>) -> Self {
        Self(value)
    }
}

impl<T> From<NestedOpt<T>> for Option<T> {
    fn from(value: NestedOpt<T>) -> Self {
        value.0
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
