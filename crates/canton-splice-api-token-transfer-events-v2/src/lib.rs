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
//! - [`splice_api_token_transfer_events_v2`] — 1 Daml module

pub mod splice_api_token_transfer_events_v2 {
    pub mod Splice_Api_Token_TransferEventsV2 {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct EventLog_HoldingsChangeResult {}
        impl rt::ToValue for EventLog_HoldingsChangeResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for EventLog_HoldingsChangeResult {
            fn from_value(_value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct EventLogView {
            #[serde(rename = "admin")]
            pub admin: rt::Party,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for EventLogView {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("admin", rt::ToValue::to_value(&self.admin)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for EventLogView {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    admin: rt::FromValue::from_value(rt::required_field(value, 0usize, "admin")?)
                        .map_err(|e| e.at("admin"))?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 1usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct EventLog_HoldingsChange {
            #[serde(rename = "admin")]
            pub admin: rt::Party,
            #[serde(rename = "account")]
            pub account: ::canton_splice_api_token_holding_v2::splice_api_token_holding_v2::Splice_Api_Token_HoldingV2::Account,
            ///Daml field `inputHoldingCids`.
            #[serde(rename = "inputHoldingCids")]
            pub input_holding_cids: ::std::vec::Vec<
                rt::ContractId<
                    ::canton_splice_api_token_holding_v2::splice_api_token_holding_v2::Splice_Api_Token_HoldingV2::Holding,
                >,
            >,
            ///Daml field `transferLegSides`.
            #[serde(rename = "transferLegSides")]
            pub transfer_leg_sides: ::std::vec::Vec<
                crate::splice_api_token_transfer_events_v2::Splice_Api_Token_TransferEventsV2::TransferLegSide,
            >,
            ///Daml field `outputHoldingCids`.
            #[serde(rename = "outputHoldingCids")]
            pub output_holding_cids: ::std::vec::Vec<
                rt::ContractId<
                    ::canton_splice_api_token_holding_v2::splice_api_token_holding_v2::Splice_Api_Token_HoldingV2::Holding,
                >,
            >,
            #[serde(rename = "observers")]
            pub observers: ::std::vec::Vec<rt::Party>,
            ///Daml field `extraArgs`.
            #[serde(rename = "extraArgs")]
            pub extra_args: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for EventLog_HoldingsChange {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("admin", rt::ToValue::to_value(&self.admin)),
                    ("account", rt::ToValue::to_value(&self.account)),
                    (
                        "inputHoldingCids",
                        rt::ToValue::to_value(&self.input_holding_cids)
                    ),
                    (
                        "transferLegSides",
                        rt::ToValue::to_value(&self.transfer_leg_sides)
                    ),
                    (
                        "outputHoldingCids",
                        rt::ToValue::to_value(&self.output_holding_cids)
                    ),
                    ("observers", rt::ToValue::to_value(&self.observers)),
                    ("extraArgs", rt::ToValue::to_value(&self.extra_args)),
                ])
            }
        }
        impl rt::FromValue for EventLog_HoldingsChange {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    admin: rt::FromValue::from_value(rt::required_field(value, 0usize, "admin")?)
                        .map_err(|e| e.at("admin"))?,
                    account: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "account",
                    )?)
                    .map_err(|e| e.at("account"))?,
                    input_holding_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "inputHoldingCids",
                    )?)
                    .map_err(|e| e.at("inputHoldingCids"))?,
                    transfer_leg_sides: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "transferLegSides",
                    )?)
                    .map_err(|e| e.at("transferLegSides"))?,
                    output_holding_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "outputHoldingCids",
                    )?)
                    .map_err(|e| e.at("outputHoldingCids"))?,
                    observers: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "observers",
                    )?)
                    .map_err(|e| e.at("observers"))?,
                    extra_args: rt::FromValue::from_value(rt::required_field(
                        value,
                        6usize,
                        "extraArgs",
                    )?)
                    .map_err(|e| e.at("extraArgs"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferLegSide {
            ///Daml field `transferLegId`.
            #[serde(rename = "transferLegId")]
            pub transfer_leg_id: ::std::string::String,
            #[serde(rename = "side")]
            pub side: crate::splice_api_token_transfer_events_v2::Splice_Api_Token_TransferEventsV2::TransferSide,
            #[serde(rename = "otherside")]
            pub otherside: ::canton_splice_api_token_holding_v2::splice_api_token_holding_v2::Splice_Api_Token_HoldingV2::Account,
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            ///Daml field `instrumentId`.
            #[serde(rename = "instrumentId")]
            pub instrument_id: ::std::string::String,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for TransferLegSide {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "transferLegId",
                        rt::ToValue::to_value(&self.transfer_leg_id)
                    ),
                    ("side", rt::ToValue::to_value(&self.side)),
                    ("otherside", rt::ToValue::to_value(&self.otherside)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                    ("instrumentId", rt::ToValue::to_value(&self.instrument_id)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for TransferLegSide {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    transfer_leg_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "transferLegId",
                    )?)
                    .map_err(|e| e.at("transferLegId"))?,
                    side: rt::FromValue::from_value(rt::required_field(value, 1usize, "side")?)
                        .map_err(|e| e.at("side"))?,
                    otherside: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "otherside",
                    )?)
                    .map_err(|e| e.at("otherside"))?,
                    amount: rt::FromValue::from_value(rt::required_field(value, 3usize, "amount")?)
                        .map_err(|e| e.at("amount"))?,
                    instrument_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "instrumentId",
                    )?)
                    .map_err(|e| e.at("instrumentId"))?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 5usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize,
        )]
        #[serde(crate = "rt::serde")]
        pub enum TransferSide {
            #[serde(rename = "SenderSide")]
            SenderSide,
            #[serde(rename = "ReceiverSide")]
            ReceiverSide,
        }
        impl rt::ToValue for TransferSide {
            fn to_value(&self) -> rt::Value {
                rt::enum_value(match self {
                    TransferSide::SenderSide => "SenderSide",
                    TransferSide::ReceiverSide => "ReceiverSide",
                })
            }
        }
        impl rt::FromValue for TransferSide {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                match rt::enum_constructor(value)? {
                    "SenderSide" => ::core::result::Result::Ok(TransferSide::SenderSide),
                    "ReceiverSide" => ::core::result::Result::Ok(TransferSide::ReceiverSide),
                    other => ::core::result::Result::Err(rt::unexpected_constructor(
                        "TransferSide",
                        other,
                    )),
                }
            }
        }
        ///Marker for the Daml interface `EventLog` (held via `ContractId`).
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct EventLog;
        impl rt::Contract for EventLog {
            const PACKAGE_ID: &'static str =
                "5c1097a9bad0af4bcfe6d3fb0fe55112d3d11f18eae57ddfb14c20836fee226c";
            const PACKAGE_NAME: &'static str = "splice-api-token-transfer-events-v2";
            const MODULE_NAME: &'static str = "Splice.Api.Token.TransferEventsV2";
            const ENTITY_NAME: &'static str = "EventLog";
        }
        impl rt::Interface for EventLog {
            type View = crate::splice_api_token_transfer_events_v2::Splice_Api_Token_TransferEventsV2::EventLogView;
        }
        ///The `Archive` choice on [`EventLog`] (consuming).
        impl rt::Choice<EventLog>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `EventLog_HoldingsChange` choice on [`EventLog`] (non-consuming).
        impl rt::Choice<EventLog>
        for crate::splice_api_token_transfer_events_v2::Splice_Api_Token_TransferEventsV2::EventLog_HoldingsChange {
            type Return = crate::splice_api_token_transfer_events_v2::Splice_Api_Token_TransferEventsV2::EventLog_HoldingsChangeResult;
            const NAME: &'static str = "EventLog_HoldingsChange";
            const CONSUMING: bool = false;
        }
    }
}
