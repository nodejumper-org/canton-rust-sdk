//! Runtime for `canton-codegen`-generated Daml bindings.
//!
//! Generated code depends on this crate (imported as `rt`) for the Daml
//! primitive types ([`Party`], [`ContractId`], [`Numeric`], [`Timestamp`],
//! [`Date`], [`TextMap`], [`GenMap`]), the [`Template`] / [`Interface`] /
//! [`Choice`] traits, the command builders ([`create_command`],
//! [`exercise_command`], [`exercise_by_key_command`]), and the [`ToValue`] /
//! [`FromValue`] codecs that move typed values to and from the Ledger API
//! `Value` on the wire. The typed **read** path is
//! [`Template::from_created_event`].
//!
//! It contains no ledger I/O — it is the thin type/codec layer beneath the
//! generated bindings, kept separate from the codegen tool (which is
//! build-time) and from the client crates (which do the actual RPCs).
//!
//! # Using it
//!
//! You rarely name this crate directly: `canton-codegen` emits bindings that
//! import it as `rt`. What you *do* touch are its types, and the three things
//! below — build a payload, turn it into a command, decode a result:
//!
//! ```rust,ignore
//! use canton_daml as rt;
//! use rt::Template as _; // brings `from_created_event` into scope
//! use my_bindings::my_app::My_Module::{Asset, Asset_Transfer};
//!
//! // A typed payload, built from the runtime's Daml primitives.
//! let asset = Asset {
//!     owner: rt::Party::new("alice::1220ab…"),
//!     price: "12.50".parse::<rt::Numeric>()?,
//!     // `None` only for a datetime outside the microsecond range of an `i64`.
//!     minted_at: rt::Timestamp::from_datetime(time::OffsetDateTime::now_utc())
//!         .expect("now is representable"),
//! };
//!
//! // Commands for the ledger client to submit.
//! let create = rt::create_command(&asset);
//! let transfer = rt::exercise_command(&contract_id, &Asset_Transfer {
//!     new_owner: rt::Party::new("bob::1220cd…"),
//! });
//!
//! // The typed read path: a CreatedEvent from a transaction or the ACS.
//! let read_back: Asset = Asset::from_created_event(&event)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! Codec failures are [`ValueError`], which converts into `canton_core::Error`,
//! so one function can call this runtime and the ledger client and use `?` on
//! both.

mod choice;
mod commands;
mod primitives;
mod template;
mod value;

pub use choice::Choice;
pub use commands::{create_command, exercise_by_key_command, exercise_command};
pub use primitives::{
    ContractId, Date, GenMap, Int64, NestedOpt, Numeric, NumericParseError, Party, TextMap,
    Timestamp, Unit,
};
pub use template::{Contract, Interface, Template, WithKey};
pub use value::{FromValue, ToValue, ValueError, from_record};

/// The pieces an emitted `ToValue`/`FromValue` body is written in terms of.
///
/// Public because generated code names them as `rt::…`, but hidden from the
/// rendered docs: nothing here is meant to be called by hand. To build a
/// `Value` yourself, use the builders in `canton_ledger`; to decode one, use
/// [`FromValue`] or [`from_record`].
#[doc(hidden)]
pub use value::{
    AbsentField, enum_constructor, enum_value, find_field, optional_field, record, record_field,
    record_fields, record_value, required_field, unexpected_constructor, unit_value, variant_parts,
    variant_value,
};

/// The Ledger API `Value` and `Record` — the gRPC wire forms generated
/// `ToValue`/`FromValue`/`Template::to_record` move to and from. Re-exported so
/// generated code can name them as `rt::Value` / `rt::Record`.
pub use canton_proto::com::daml::ledger::api::v2::{Record, Value};

/// Re-exported `serde` so generated code can derive the JSON codec through the
/// runtime (`#[derive(rt::serde::Serialize, ...)]` + `#[serde(crate = "rt::serde")]`)
/// without depending on `serde` directly.
pub use serde;

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::assertions_on_constants,
    non_camel_case_types
)]
mod tests {
    use super::*;

    // As-if-generated shapes: a payload using the runtime types, its
    // `ToValue`/`FromValue` written exactly as the generator emits them (via
    // `record` / `record_field`), and a choice argument with its `Choice` impl.
    #[derive(Clone, Debug, PartialEq)]
    struct AppInstall {
        provider: Party,
        amount: Numeric,
        tags: Vec<String>,
        note: Option<String>,
    }

    impl ToValue for AppInstall {
        fn to_value(&self) -> Value {
            record(vec![
                ("provider", ToValue::to_value(&self.provider)),
                ("amount", ToValue::to_value(&self.amount)),
                ("tags", ToValue::to_value(&self.tags)),
                ("note", ToValue::to_value(&self.note)),
            ])
        }
    }

    impl FromValue for AppInstall {
        fn from_value(value: &Value) -> Result<Self, ValueError> {
            Ok(Self {
                provider: FromValue::from_value(required_field(value, 0, "provider")?)?,
                amount: FromValue::from_value(required_field(value, 1, "amount")?)?,
                tags: FromValue::from_value(required_field(value, 2, "tags")?)?,
                note: optional_field(value, 3, "note")?,
            })
        }
    }

    struct AppInstall_Accept;
    struct AppInstalled;

    impl Choice<AppInstall> for AppInstall_Accept {
        type Return = ContractId<AppInstalled>;
        const NAME: &'static str = "Accept";
        const CONSUMING: bool = true;
    }

    // A nullary choice argument is an empty record on the wire.
    impl ToValue for AppInstall_Accept {
        fn to_value(&self) -> Value {
            record(vec![])
        }
    }

    impl Contract for AppInstall {
        const PACKAGE_ID: &'static str = "deadbeef";
        const PACKAGE_NAME: &'static str = "app-install";
        const MODULE_NAME: &'static str = "Licensing.AppInstall";
        const ENTITY_NAME: &'static str = "AppInstall";
    }

    impl Template for AppInstall {
        fn to_record(&self) -> Record {
            record_fields(vec![
                ("provider", ToValue::to_value(&self.provider)),
                ("amount", ToValue::to_value(&self.amount)),
                ("tags", ToValue::to_value(&self.tags)),
                ("note", ToValue::to_value(&self.note)),
            ])
        }
    }

    impl WithKey for AppInstall {
        type Key = Party;
    }

    // An interface marker + its view and one choice, as generated code emits them.
    struct Holding;
    struct HoldingView;
    struct Holding_Transfer;

    impl Contract for Holding {
        const PACKAGE_ID: &'static str = "cafef00d";
        const PACKAGE_NAME: &'static str = "splice-api-token-holding";
        const MODULE_NAME: &'static str = "Splice.Api.Token.HoldingV1";
        const ENTITY_NAME: &'static str = "Holding";
    }
    impl Interface for Holding {
        type View = HoldingView;
    }
    impl FromValue for HoldingView {
        fn from_value(_value: &Value) -> Result<Self, ValueError> {
            Ok(HoldingView)
        }
    }
    impl Choice<Holding> for Holding_Transfer {
        type Return = ();
        const NAME: &'static str = "Transfer";
        const CONSUMING: bool = true;
    }
    impl ToValue for Holding_Transfer {
        fn to_value(&self) -> Value {
            record(vec![])
        }
    }

    #[test]
    fn record_round_trips_through_value() {
        let app = AppInstall {
            provider: Party::new("alice::1"),
            amount: Numeric("1.50".to_string()),
            tags: vec!["defi".to_string(), "wallet".to_string()],
            note: Some("hi".to_string()),
        };
        let back = AppInstall::from_value(&app.to_value()).unwrap();
        assert_eq!(back, app);

        // A missing field is a typed error, not a panic.
        let empty = record(vec![]);
        assert!(AppInstall::from_value(&empty).is_err());
    }

    #[test]
    fn primitives_round_trip_through_value() {
        let party = Party::new("alice::1");
        assert_eq!(Party::from_value(&party.to_value()).unwrap(), party);

        let numeric = Numeric("1.50".to_string());
        assert_eq!(Numeric::from_value(&numeric.to_value()).unwrap(), numeric);

        assert_eq!(i64::from_value(&42_i64.to_value()).unwrap(), 42);
        assert!(bool::from_value(&true.to_value()).unwrap());
    }

    #[test]
    fn containers_round_trip_through_value() {
        let list = vec!["a".to_string(), "b".to_string()];
        assert_eq!(Vec::<String>::from_value(&list.to_value()).unwrap(), list);

        let some: Option<String> = Some("x".to_string());
        assert_eq!(
            Option::<String>::from_value(&some.to_value()).unwrap(),
            some
        );
        let none: Option<String> = None;
        assert_eq!(
            Option::<String>::from_value(&none.to_value()).unwrap(),
            none
        );

        let mut map = TextMap::new();
        map.insert("k".to_string(), 7_i64);
        assert_eq!(TextMap::<i64>::from_value(&map.to_value()).unwrap(), map);
    }

    #[test]
    fn contract_id_is_typed_and_round_trips() {
        let cid: ContractId<AppInstalled> = ContractId::new("00abc");
        let back: ContractId<AppInstalled> = ContractId::from_value(&cid.to_value()).unwrap();
        assert_eq!(back, cid);
        assert_eq!(back.as_str(), "00abc");
    }

    #[test]
    fn choice_metadata_is_available() {
        assert_eq!(<AppInstall_Accept as Choice<AppInstall>>::NAME, "Accept");
        assert!(<AppInstall_Accept as Choice<AppInstall>>::CONSUMING);
    }

    #[test]
    fn wrong_shape_is_an_error() {
        // A Text value cannot be decoded as a Party.
        let text = "not a party".to_string().to_value();
        assert!(Party::from_value(&text).is_err());
    }

    #[test]
    fn newtypes_use_lf_json_encoding() {
        use serde_json::json;

        // String-shaped newtypes encode as JSON strings.
        assert_eq!(
            serde_json::to_value(Party::new("alice::1")).unwrap(),
            json!("alice::1")
        );
        assert_eq!(
            serde_json::to_value(Numeric("1.50".to_string())).unwrap(),
            json!("1.50")
        );
        let cid: ContractId<()> = ContractId::new("00abc");
        assert_eq!(serde_json::to_value(cid).unwrap(), json!("00abc"));

        // Date encodes as an ISO calendar string.
        assert_eq!(serde_json::to_value(Date(0)).unwrap(), json!("1970-01-01"));
        assert_eq!(serde_json::to_value(Date(1)).unwrap(), json!("1970-01-02"));

        // Timestamp and Date round-trip through JSON.
        for ts in [Timestamp(0), Timestamp(1_700_000_000_000_000)] {
            let text = serde_json::to_string(&ts).unwrap();
            assert_eq!(serde_json::from_str::<Timestamp>(&text).unwrap(), ts);
        }
        for date in [Date(0), Date(20_000), Date(-100)] {
            let text = serde_json::to_string(&date).unwrap();
            assert_eq!(serde_json::from_str::<Date>(&text).unwrap(), date);
        }
    }

    #[test]
    fn int64_uses_the_lf_json_string_form() {
        use serde_json::json;

        // Emitted as a string (the Ledger API's encodeInt64AsString form)…
        assert_eq!(serde_json::to_value(Int64(5)).unwrap(), json!("5"));
        // …and accepted from either the string or the number form on input.
        assert_eq!(serde_json::from_str::<Int64>("\"5\"").unwrap(), Int64(5));
        assert_eq!(serde_json::from_str::<Int64>("5").unwrap(), Int64(5));
        // A value beyond JS's safe integers still round-trips (why it's a string).
        let big = Int64(9_007_199_254_740_993);
        assert_eq!(
            serde_json::from_str::<Int64>(&serde_json::to_string(&big).unwrap()).unwrap(),
            big
        );
        // gRPC `Value` is a plain Int64.
        assert_eq!(Int64::from_value(&Int64(7).to_value()).unwrap(), Int64(7));
    }

    #[test]
    fn numeric_accepts_string_and_number_json() {
        use serde_json::json;

        // Emitted as a string…
        assert_eq!(
            serde_json::to_value(Numeric("1.50".to_string())).unwrap(),
            json!("1.50")
        );
        // …and accepted from a string or a JSON number on input.
        assert_eq!(
            serde_json::from_str::<Numeric>("\"1.5\"").unwrap(),
            Numeric("1.5".to_string())
        );
        assert_eq!(
            serde_json::from_str::<Numeric>("2").unwrap(),
            Numeric("2".to_string())
        );
    }

    #[test]
    fn nested_optional_uses_the_lf_json_list_form() {
        use serde_json::json;

        // Daml `Optional (Optional Text)` → `Option<NestedOpt<String>>`.
        // Top-level: None → null; nested: Some None → [], Some (Some x) → [x].
        let none: Option<NestedOpt<String>> = None;
        let some_none: Option<NestedOpt<String>> = Some(NestedOpt(None));
        let some_some: Option<NestedOpt<String>> = Some(NestedOpt(Some("x".to_string())));

        assert_eq!(serde_json::to_value(&none).unwrap(), json!(null));
        assert_eq!(serde_json::to_value(&some_none).unwrap(), json!([]));
        assert_eq!(serde_json::to_value(&some_some).unwrap(), json!(["x"]));

        // The whole point: Some(None) survives a JSON round-trip instead of
        // collapsing into None.
        for value in [none, some_none, some_some] {
            let text = serde_json::to_string(&value).unwrap();
            assert_eq!(
                serde_json::from_str::<Option<NestedOpt<String>>>(&text).unwrap(),
                value
            );
        }

        // Triple nesting: Some (Some None) → [[]].
        let triple: Option<NestedOpt<NestedOpt<String>>> = Some(NestedOpt(Some(NestedOpt(None))));
        assert_eq!(serde_json::to_value(&triple).unwrap(), json!([[]]));
    }

    #[test]
    fn nested_optional_grpc_matches_option() {
        // On the gRPC wire a nested Optional is still a proto Optional, so
        // Some(None) and None stay distinct and round-trip.
        let some_none: Option<NestedOpt<String>> = Some(NestedOpt(None));
        let back: Option<NestedOpt<String>> = FromValue::from_value(&some_none.to_value()).unwrap();
        assert_eq!(back, some_none);

        let none: Option<NestedOpt<String>> = None;
        assert_ne!(none.to_value(), some_none.to_value());
    }

    #[test]
    fn unit_is_the_empty_object_in_json() {
        use serde_json::json;

        // Daml `Unit` is LF-JSON `{}` (not `null`, which is what `()` would be).
        assert_eq!(serde_json::to_value(Unit).unwrap(), json!({}));
        assert_eq!(serde_json::from_str::<Unit>("{}").unwrap(), Unit);
        // Tolerates a null too, so either shape from the API is accepted.
        assert_eq!(serde_json::from_str::<Unit>("null").unwrap(), Unit);
        // gRPC `Value` round-trips as the proto Unit.
        assert_eq!(Unit::from_value(&Unit.to_value()).unwrap(), Unit);
    }

    #[test]
    fn command_builders_produce_ledger_api_commands() {
        use canton_proto::com::daml::ledger::api::v2::command::Command as Cmd;

        let app = AppInstall {
            provider: Party::new("alice::1"),
            amount: Numeric("1.00".to_string()),
            tags: vec![],
            note: None,
        };

        // create → CreateCommand with the SCU-friendly `#<package-name>` id and
        // the payload as its create arguments.
        let create = create_command(&app);
        match create.command {
            Some(Cmd::Create(c)) => {
                let id = c.template_id.unwrap();
                assert_eq!(id.package_id, "#app-install");
                assert_eq!(id.module_name, "Licensing.AppInstall");
                assert_eq!(id.entity_name, "AppInstall");
                assert!(c.create_arguments.is_some(), "payload becomes create args");
            }
            _ => panic!("expected a Create command"),
        }

        // exercise → ExerciseCommand carrying the contract id, choice name, arg.
        let cid: ContractId<AppInstall> = ContractId::new("00cid");
        let exercise = exercise_command(&cid, &AppInstall_Accept);
        match exercise.command {
            Some(Cmd::Exercise(e)) => {
                assert_eq!(e.contract_id, "00cid");
                assert_eq!(e.choice, "Accept");
                assert_eq!(e.template_id.unwrap().entity_name, "AppInstall");
                assert!(e.choice_argument.is_some());
            }
            _ => panic!("expected an Exercise command"),
        }

        // exercise-by-key → ExerciseByKeyCommand carrying the key as a Value.
        let key = Party::new("alice::1");
        let by_key = exercise_by_key_command::<AppInstall, _>(&key, &AppInstall_Accept);
        match by_key.command {
            Some(Cmd::ExerciseByKey(e)) => {
                assert_eq!(e.choice, "Accept");
                assert_eq!(e.template_id.unwrap().entity_name, "AppInstall");
                assert!(
                    e.contract_key.is_some(),
                    "key becomes the contract_key value"
                );
            }
            _ => panic!("expected an ExerciseByKey command"),
        }
    }

    #[test]
    fn exercise_on_interface_builds_a_command() {
        use canton_proto::com::daml::ledger::api::v2::command::Command as Cmd;

        // A choice exercised through a `ContractId<Interface>` uses the interface
        // id — no concrete template needed.
        let cid: ContractId<Holding> = ContractId::new("00hold");
        let cmd = exercise_command(&cid, &Holding_Transfer);
        match cmd.command {
            Some(Cmd::Exercise(e)) => {
                assert_eq!(e.contract_id, "00hold");
                assert_eq!(e.choice, "Transfer");
                let id = e.template_id.unwrap();
                assert_eq!(id.entity_name, "Holding");
                assert_eq!(id.package_id, "#splice-api-token-holding");
            }
            _ => panic!("expected an Exercise command on the interface"),
        }
    }

    // ---- record decode robustness (Canton wire shapes) ----------------------

    /// A record value as Canton emits it: `labelled` chooses verbose (labels
    /// present) vs non-verbose (labels empty), `take` truncates trailing fields
    /// the way record normalization drops empty optionals.
    fn wire_record(labelled: bool, take: usize) -> Value {
        use canton_proto::com::daml::ledger::api::v2 as pb;
        let sample = AppInstall {
            provider: Party::new("p::1"),
            amount: Numeric("1.5".to_string()),
            tags: vec!["a".to_string()],
            note: None,
        };
        let Some(pb::value::Sum::Record(mut record)) = sample.to_value().sum else {
            panic!("record expected");
        };
        record.fields.truncate(take);
        if !labelled {
            for field in &mut record.fields {
                field.label = String::new();
            }
        }
        Value {
            sum: Some(pb::value::Sum::Record(record)),
        }
    }

    #[test]
    fn decodes_verbose_records_by_label() {
        let decoded = AppInstall::from_value(&wire_record(true, 4)).unwrap();
        assert_eq!(decoded.provider, Party::new("p::1"));
        assert_eq!(decoded.note, None);
    }

    #[test]
    fn decodes_non_verbose_records_by_position() {
        // Non-verbose output omits every label; fields bind by declaration index.
        let decoded = AppInstall::from_value(&wire_record(false, 4)).unwrap();
        assert_eq!(decoded.provider, Party::new("p::1"));
        assert_eq!(decoded.tags, vec!["a".to_string()]);
    }

    #[test]
    fn decodes_normalized_records_with_trailing_optionals_dropped() {
        // Record normalization (Smart Contract Upgrade) drops trailing fields
        // whose value is an empty Optional — in both verbose and non-verbose
        // shapes the absent `note` must decode as `None`, not error.
        for labelled in [true, false] {
            let decoded = AppInstall::from_value(&wire_record(labelled, 3)).unwrap();
            assert_eq!(decoded.note, None, "labelled={labelled}");
            assert_eq!(
                decoded.amount,
                Numeric("1.5".to_string()),
                "labelled={labelled}"
            );
        }
    }

    #[test]
    fn genmap_is_constructible_for_real_daml_key_types() {
        // Regression: a derived `Default` bounded `K: Default, V: Default`, so
        // `GenMap::<Party, Numeric>::new()` did not compile — no Daml primitive
        // implements Default.
        let mut map: GenMap<Party, Numeric> = GenMap::new();
        assert!(map.is_empty());
        map.insert(Party::new("alice::1"), Numeric::from_wire("1.5"));
        assert_eq!(map.len(), 1);
        assert_eq!(GenMap::<Party, Numeric>::default().len(), 0);

        // The collection surface behaves like a collection.
        let collected: GenMap<Party, Int64> =
            [(Party::new("a::1"), Int64(1))].into_iter().collect();
        assert_eq!(collected.iter().count(), 1);
        assert_eq!((&collected).into_iter().count(), 1);
        assert_eq!(collected.into_iter().count(), 1);
    }

    #[test]
    fn genmap_equality_is_order_insensitive_and_symmetric() {
        let a = Party::new("a::1");
        let b = Party::new("b::1");

        // Order is a wire detail.
        let submitted: GenMap<Party, Int64> =
            vec![(a.clone(), Int64(1)), (b.clone(), Int64(2))].into();
        let echoed: GenMap<Party, Int64> =
            vec![(b.clone(), Int64(2)), (a.clone(), Int64(1))].into();
        assert_eq!(submitted, echoed);

        // Regression: equality must be a multiset comparison. With a repeated
        // entry, "every entry of self occurs in other" made `[a, a] == [a, b]`
        // true while the reverse was false — a broken PartialEq contract.
        let twice: GenMap<Party, Int64> = vec![(a.clone(), Int64(1)), (a.clone(), Int64(1))].into();
        let mixed: GenMap<Party, Int64> = vec![(a.clone(), Int64(1)), (b, Int64(2))].into();
        assert_ne!(twice, mixed);
        assert_ne!(mixed, twice, "equality must be symmetric");
        assert_eq!(twice, twice.clone());
    }

    #[test]
    fn timestamp_and_date_convert_through_time_without_saturating() {
        use time::macros::{date, datetime};

        let at = Timestamp::from_datetime(datetime!(2026-07-31 12:00:00 UTC)).unwrap();
        assert_eq!(
            at.to_datetime().unwrap(),
            datetime!(2026-07-31 12:00:00 UTC)
        );
        assert_eq!(Timestamp(at.micros()), at);

        // Sub-microsecond digits floor toward the past on both sides of the
        // epoch — they must never move an instant forward.
        let before_epoch = datetime!(1969-12-31 23:59:59.999_999_5 UTC);
        let floored = Timestamp::from_datetime(before_epoch).unwrap();
        assert!(floored.to_datetime().unwrap() <= before_epoch);

        // Regression: an out-of-range instant used to come back as i64::MAX —
        // a far-future timestamp silently standing in for an underflow. It is
        // `None` now. With `time`'s default features every representable
        // OffsetDateTime fits in i64 microseconds, so the out-of-range case is
        // only reachable when something in the dependency graph enables
        // `time/large-dates` (years to ±999_999) — which feature unification
        // can do without this crate asking. Hence the fallible signature; the
        // property asserted here is that conversion never fabricates a value.
        let extreme = time::OffsetDateTime::new_utc(date!(9999 - 12 - 31), time::Time::MIDNIGHT);
        let converted = Timestamp::from_datetime(extreme).expect("in range for default `time`");
        assert_eq!(converted.to_datetime().unwrap(), extreme);

        let day = Date::from_date(date!(2026 - 07 - 31)).unwrap();
        assert_eq!(day.to_date().unwrap(), date!(2026 - 07 - 31));
        assert_eq!(Date::from_date(date!(1970 - 01 - 01)).unwrap().days(), 0);
    }

    #[test]
    fn numeric_parsing_is_strict_for_callers_and_lenient_on_the_wire() {
        use std::str::FromStr as _;

        // Caller input is validated…
        assert_eq!(Numeric::from_str("1.5").unwrap().as_str(), "1.5");
        assert_eq!(Numeric::try_from("-0.25").unwrap().to_string(), "-0.25");
        let error = Numeric::from_str("twelve").unwrap_err();
        assert_eq!(error.input(), "twelve");

        // …while a value the ledger vouched for is taken as-is.
        let wire = Numeric::from_wire("1.5000000000");
        assert_eq!(wire, Numeric::from_str("1.5").unwrap());

        // `42.` is a legal spelling of `42` and must stay inside canonical
        // comparison rather than degrading to string equality.
        assert_eq!(Numeric::from_wire("42."), Numeric::from_wire("42"));
        assert_eq!(Numeric::from_wire("42."), Numeric::from_wire("42.000"));
    }

    #[test]
    fn value_errors_carry_a_path_and_flow_into_the_sdk_error() {
        // A decode failure deep in a payload names where it happened…
        let error = ValueError::new("expected Text")
            .at("city")
            .at("address")
            .at("owner");
        assert_eq!(error.path(), ["owner", "address", "city"]);
        assert!(
            error.to_string().contains("`owner.address.city`"),
            "{error}"
        );
        assert_eq!(error.message(), "expected Text");

        // …and `?` carries it into the SDK-wide error, so one function can call
        // the typed runtime and the ledger client and return canton::Result.
        let sdk: canton_core::Error = error.clone().into();
        assert!(!sdk.is_retriable(), "a shape mismatch will not fix itself");
        assert!(
            std::error::Error::source(&sdk)
                .map(ToString::to_string)
                .unwrap_or_default()
                .contains("owner.address.city"),
            "the cause chain keeps the path"
        );
    }

    #[test]
    fn numeric_compares_by_value_not_by_text() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        // The ledger echoes numerics at the type's full scale: a submitted
        // "1.5" comes back "1.5000000000" (Numeric 10) and must compare equal.
        let submitted = Numeric("1.5".to_string());
        let echoed = Numeric("1.5000000000".to_string());
        assert_eq!(submitted, echoed);

        let hash = |n: &Numeric| {
            let mut hasher = DefaultHasher::new();
            n.hash(&mut hasher);
            hasher.finish()
        };
        assert_eq!(hash(&submitted), hash(&echoed), "Eq and Hash must agree");

        // Sign/zero normalization and numeric (not lexicographic) order.
        assert_eq!(Numeric("-0".to_string()), Numeric("0.00".to_string()));
        assert_eq!(Numeric("+07.10".to_string()), Numeric("7.1".to_string()));
        assert!(Numeric("2".to_string()) < Numeric("10".to_string()));
        assert!(Numeric("1.45".to_string()) < Numeric("1.5".to_string()));
        assert!(Numeric("-1.5".to_string()) < Numeric("-1.45".to_string()));
        assert!(Numeric("-1".to_string()) < Numeric("0.1".to_string()));

        // Garbage never equals a valid decimal, and parse rejects it early.
        assert_ne!(Numeric("abc".to_string()), Numeric("1".to_string()));
        assert!(Numeric::parse("1.5").is_ok());
        assert!(Numeric::parse("-12.500").is_ok());
        // `1.` is a legal spelling of `1` (see canonical_decimal); the rest
        // are not decimal literals.
        assert!(Numeric::parse("1.").is_ok());
        for bad in ["", ".5", "1.2.3", "1e5", "abc", "1,5"] {
            assert!(Numeric::parse(bad).is_err(), "{bad:?} should be rejected");
        }
    }

    #[test]
    fn contract_id_retags_for_interface_exercise() {
        use canton_proto::com::daml::ledger::api::v2::command::Command as Cmd;

        // Read back as the concrete template, exercised through the interface.
        let concrete: ContractId<AppInstall> = ContractId::new("00abc");
        let as_interface: ContractId<Holding> = concrete.retag();
        assert_eq!(as_interface.as_str(), "00abc");
        let cmd = exercise_command(&as_interface, &Holding_Transfer);
        match cmd.command {
            Some(Cmd::Exercise(e)) => {
                assert_eq!(
                    e.template_id.unwrap().package_id,
                    "#splice-api-token-holding"
                );
            }
            _ => panic!("expected an interface exercise"),
        }

        // Typed ids work in hash/ordered collections.
        let mut set = std::collections::HashSet::new();
        set.insert(ContractId::<AppInstall>::new("00abc"));
        assert!(set.contains(&ContractId::<AppInstall>::new("00abc")));
    }

    #[test]
    fn from_created_event_decodes_the_typed_read_path() {
        use canton_proto::com::daml::ledger::api::v2 as pb;

        let payload = AppInstall {
            provider: Party::new("p::1"),
            amount: Numeric("2".to_string()),
            tags: vec![],
            note: None,
        };
        let Some(pb::value::Sum::Record(record)) = payload.to_value().sum else {
            panic!("record expected");
        };
        let event = pb::CreatedEvent {
            template_id: Some(pb::Identifier {
                // Any package id: SCU may report any vetted version — not compared.
                package_id: "someotherhash".to_string(),
                module_name: AppInstall::MODULE_NAME.to_string(),
                entity_name: AppInstall::ENTITY_NAME.to_string(),
            }),
            create_arguments: Some(record),
            ..Default::default()
        };

        let decoded = AppInstall::from_created_event(&event).unwrap();
        assert_eq!(decoded, payload);

        // The wrong template is rejected with a clear identity error, not a
        // confusing field-shape mismatch.
        let mut wrong = event.clone();
        wrong.template_id.as_mut().unwrap().entity_name = "SomethingElse".to_string();
        let error = AppInstall::from_created_event(&wrong).unwrap_err();
        assert!(error.message().contains("SomethingElse"), "{error}");
        assert!(error.message().contains("AppInstall"), "{error}");

        // No payload is a typed error.
        let mut empty = event.clone();
        empty.create_arguments = None;
        assert!(AppInstall::from_created_event(&empty).is_err());
    }

    #[test]
    fn missing_required_field_is_an_error_naming_the_field() {
        // Truncating past the optional tail removes the required `tags` field.
        let error = AppInstall::from_value(&wire_record(true, 2)).unwrap_err();
        assert!(
            error.message().contains("tags"),
            "error should name the missing field: {error}"
        );
    }
}
