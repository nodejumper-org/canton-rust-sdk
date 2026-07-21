//! Runtime for `canton-codegen`-generated Daml bindings.
//!
//! **Milestone 2, work in progress.** Generated code depends on this crate
//! (imported as `rt`) for the Daml primitive types ([`Party`], [`ContractId`],
//! [`Numeric`], [`Timestamp`], [`Date`], [`TextMap`], [`GenMap`]), the
//! [`Choice`] trait, and the [`ToValue`] / [`FromValue`] codecs that move typed
//! values to and from the Ledger API `Value` on the wire.
//!
//! It contains no ledger I/O — it is the thin type/codec layer beneath the
//! generated bindings, kept separate from the codegen tool (which is
//! build-time) and from the client crates (which do the actual RPCs).

mod choice;
mod primitives;
mod value;

pub use choice::Choice;
pub use primitives::{ContractId, Date, GenMap, NestedOpt, Numeric, Party, TextMap, Timestamp};
pub use value::{
    FromValue, ToValue, ValueError, enum_constructor, enum_value, record, record_field,
    unexpected_constructor, unit_value, variant_parts, variant_value,
};

/// The Ledger API `Value` — the gRPC wire form generated `ToValue`/`FromValue`
/// move to and from. Re-exported so generated code can name it as `rt::Value`.
pub use canton_proto::com::daml::ledger::api::v2::Value;

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
                provider: FromValue::from_value(record_field(value, "provider")?)?,
                amount: FromValue::from_value(record_field(value, "amount")?)?,
                tags: FromValue::from_value(record_field(value, "tags")?)?,
                note: FromValue::from_value(record_field(value, "note")?)?,
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
}
