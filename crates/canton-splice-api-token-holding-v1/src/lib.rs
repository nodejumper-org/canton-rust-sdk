#![allow(non_camel_case_types, non_snake_case, unused_imports, clippy::all)]
//! Typed Rust bindings generated from a Daml archive (DAR).
//!
//! **Generated file — do not edit by hand.** Regenerate with
//! `dpm-codegen-rust --dar <the DAR> --out <this crate>`; edits are lost.
//!
//! Each Daml package in the DAR's dependency closure is one top-level
//! module, and each Daml module a submodule under it, so cross-package
//! references resolve and names never collide. Templates carry typed
//! choices; the runtime traits they implement (`Template`, `Choice`,
//! `Contract`) and the command builders live in `canton-daml`, which the
//! generated code imports as `rt`.
//!
//! # Packages in this crate
//!
//! - [`splice_api_token_holding_v1`] — 1 Daml module

pub mod splice_api_token_holding_v1 {
    pub mod Splice_Api_Token_HoldingV1 {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct HoldingView {
            #[serde(rename = "owner")]
            pub owner: rt::Party,
            ///Daml field `instrumentId`.
            #[serde(rename = "instrumentId")]
            pub instrument_id: crate::splice_api_token_holding_v1::Splice_Api_Token_HoldingV1::InstrumentId,
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            #[serde(rename = "lock")]
            pub lock: ::core::option::Option<
                crate::splice_api_token_holding_v1::Splice_Api_Token_HoldingV1::Lock,
            >,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for HoldingView {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("owner", rt::ToValue::to_value(&self.owner)),
                    ("instrumentId", rt::ToValue::to_value(&self.instrument_id)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                    ("lock", rt::ToValue::to_value(&self.lock)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for HoldingView {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    owner: rt::FromValue::from_value(rt::required_field(value, 0usize, "owner")?)
                        .map_err(|e| e.at("owner"))?,
                    instrument_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "instrumentId",
                    )?)
                    .map_err(|e| e.at("instrumentId"))?,
                    amount: rt::FromValue::from_value(rt::required_field(value, 2usize, "amount")?)
                        .map_err(|e| e.at("amount"))?,
                    lock: rt::optional_field(value, 3usize, "lock").map_err(|e| e.at("lock"))?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 4usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Lock {
            #[serde(rename = "holders")]
            pub holders: ::std::vec::Vec<rt::Party>,
            ///Daml field `expiresAt`.
            #[serde(rename = "expiresAt")]
            pub expires_at: ::core::option::Option<rt::Timestamp>,
            ///Daml field `expiresAfter`.
            #[serde(rename = "expiresAfter")]
            pub expires_after: ::core::option::Option<
                ::canton_daml_stdlib::daml_stdlib_DA_Time_Types::DA_Time_Types::RelTime,
            >,
            #[serde(rename = "context")]
            pub context: ::core::option::Option<::std::string::String>,
        }
        impl rt::ToValue for Lock {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("holders", rt::ToValue::to_value(&self.holders)),
                    ("expiresAt", rt::ToValue::to_value(&self.expires_at)),
                    ("expiresAfter", rt::ToValue::to_value(&self.expires_after)),
                    ("context", rt::ToValue::to_value(&self.context)),
                ])
            }
        }
        impl rt::FromValue for Lock {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    holders: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "holders",
                    )?)
                    .map_err(|e| e.at("holders"))?,
                    expires_at: rt::optional_field(value, 1usize, "expiresAt")
                        .map_err(|e| e.at("expiresAt"))?,
                    expires_after: rt::optional_field(value, 2usize, "expiresAfter")
                        .map_err(|e| e.at("expiresAfter"))?,
                    context: rt::optional_field(value, 3usize, "context")
                        .map_err(|e| e.at("context"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct InstrumentId {
            #[serde(rename = "admin")]
            pub admin: rt::Party,
            #[serde(rename = "id")]
            pub id: ::std::string::String,
        }
        impl rt::ToValue for InstrumentId {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("admin", rt::ToValue::to_value(&self.admin)),
                    ("id", rt::ToValue::to_value(&self.id)),
                ])
            }
        }
        impl rt::FromValue for InstrumentId {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    admin: rt::FromValue::from_value(rt::required_field(value, 0usize, "admin")?)
                        .map_err(|e| e.at("admin"))?,
                    id: rt::FromValue::from_value(rt::required_field(value, 1usize, "id")?)
                        .map_err(|e| e.at("id"))?,
                })
            }
        }
        ///Marker for the Daml interface `Holding` (held via `ContractId`).
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct Holding;
        impl rt::Contract for Holding {
            const PACKAGE_ID: &'static str =
                "718a0f77e505a8de22f188bd4c87fe74101274e9d4cb1bfac7d09aec7158d35b";
            const PACKAGE_NAME: &'static str = "splice-api-token-holding-v1";
            const MODULE_NAME: &'static str = "Splice.Api.Token.HoldingV1";
            const ENTITY_NAME: &'static str = "Holding";
        }
        impl rt::Interface for Holding {
            type View = crate::splice_api_token_holding_v1::Splice_Api_Token_HoldingV1::HoldingView;
        }
        ///The `Archive` choice on [`Holding`] (consuming).
        impl rt::Choice<Holding>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
    }
}
