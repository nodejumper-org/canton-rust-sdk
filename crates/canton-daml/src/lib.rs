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
pub use primitives::{ContractId, Date, GenMap, Numeric, Party, TextMap, Timestamp};
pub use value::{FromValue, ToValue, ValueError};

#[cfg(test)]
#[allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::assertions_on_constants,
    non_camel_case_types
)]
mod tests {
    use super::*;

    // As-if-generated shapes: a payload using the runtime types, and a choice
    // argument with its `Choice` impl. Increment 2 will have the generator emit
    // the `ToValue`/`FromValue` for records; this proves the runtime API
    // supports the generated shape and compiles.
    #[derive(Clone, Debug, PartialEq)]
    struct AppInstall {
        provider: Party,
        amount: Numeric,
        tags: Vec<String>,
        note: Option<String>,
    }

    struct AppInstall_Accept;
    struct AppInstalled;

    impl Choice<AppInstall> for AppInstall_Accept {
        type Return = ContractId<AppInstalled>;
        const NAME: &'static str = "Accept";
        const CONSUMING: bool = true;
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
}
