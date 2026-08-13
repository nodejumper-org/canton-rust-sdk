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
//! - [`splice_api_token_allocation_v1`] — 1 Daml module

pub mod splice_api_token_allocation_v1 {
    pub mod Splice_Api_Token_AllocationV1 {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Allocation_WithdrawResult {
            ///Daml field `senderHoldingCids`.
            #[serde(rename = "senderHoldingCids")]
            pub sender_holding_cids: ::std::vec::Vec<
                rt::ContractId<
                    ::canton_splice_api_token_holding_v1::splice_api_token_holding_v1::Splice_Api_Token_HoldingV1::Holding,
                >,
            >,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for Allocation_WithdrawResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "senderHoldingCids",
                        rt::ToValue::to_value(&self.sender_holding_cids)
                    ),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for Allocation_WithdrawResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    sender_holding_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "senderHoldingCids",
                    )?)
                    .map_err(|e| e.at("senderHoldingCids"))?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 1usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Allocation_CancelResult {
            ///Daml field `senderHoldingCids`.
            #[serde(rename = "senderHoldingCids")]
            pub sender_holding_cids: ::std::vec::Vec<
                rt::ContractId<
                    ::canton_splice_api_token_holding_v1::splice_api_token_holding_v1::Splice_Api_Token_HoldingV1::Holding,
                >,
            >,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for Allocation_CancelResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "senderHoldingCids",
                        rt::ToValue::to_value(&self.sender_holding_cids)
                    ),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for Allocation_CancelResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    sender_holding_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "senderHoldingCids",
                    )?)
                    .map_err(|e| e.at("senderHoldingCids"))?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 1usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Allocation_ExecuteTransferResult {
            ///Daml field `senderHoldingCids`.
            #[serde(rename = "senderHoldingCids")]
            pub sender_holding_cids: ::std::vec::Vec<
                rt::ContractId<
                    ::canton_splice_api_token_holding_v1::splice_api_token_holding_v1::Splice_Api_Token_HoldingV1::Holding,
                >,
            >,
            ///Daml field `receiverHoldingCids`.
            #[serde(rename = "receiverHoldingCids")]
            pub receiver_holding_cids: ::std::vec::Vec<
                rt::ContractId<
                    ::canton_splice_api_token_holding_v1::splice_api_token_holding_v1::Splice_Api_Token_HoldingV1::Holding,
                >,
            >,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for Allocation_ExecuteTransferResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "senderHoldingCids",
                        rt::ToValue::to_value(&self.sender_holding_cids)
                    ),
                    (
                        "receiverHoldingCids",
                        rt::ToValue::to_value(&self.receiver_holding_cids)
                    ),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for Allocation_ExecuteTransferResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    sender_holding_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "senderHoldingCids",
                    )?)
                    .map_err(|e| e.at("senderHoldingCids"))?,
                    receiver_holding_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "receiverHoldingCids",
                    )?)
                    .map_err(|e| e.at("receiverHoldingCids"))?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 2usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Allocation_Withdraw {
            ///Daml field `extraArgs`.
            #[serde(rename = "extraArgs")]
            pub extra_args: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for Allocation_Withdraw {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "extraArgs",
                    rt::ToValue::to_value(&self.extra_args)
                ),])
            }
        }
        impl rt::FromValue for Allocation_Withdraw {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    extra_args: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "extraArgs",
                    )?)
                    .map_err(|e| e.at("extraArgs"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Allocation_Cancel {
            ///Daml field `extraArgs`.
            #[serde(rename = "extraArgs")]
            pub extra_args: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for Allocation_Cancel {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "extraArgs",
                    rt::ToValue::to_value(&self.extra_args)
                ),])
            }
        }
        impl rt::FromValue for Allocation_Cancel {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    extra_args: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "extraArgs",
                    )?)
                    .map_err(|e| e.at("extraArgs"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Allocation_ExecuteTransfer {
            ///Daml field `extraArgs`.
            #[serde(rename = "extraArgs")]
            pub extra_args: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for Allocation_ExecuteTransfer {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "extraArgs",
                    rt::ToValue::to_value(&self.extra_args)
                ),])
            }
        }
        impl rt::FromValue for Allocation_ExecuteTransfer {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    extra_args: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "extraArgs",
                    )?)
                    .map_err(|e| e.at("extraArgs"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationView {
            #[serde(rename = "allocation")]
            pub allocation: crate::splice_api_token_allocation_v1::Splice_Api_Token_AllocationV1::AllocationSpecification,
            ///Daml field `holdingCids`.
            #[serde(rename = "holdingCids")]
            pub holding_cids: ::std::vec::Vec<
                rt::ContractId<
                    ::canton_splice_api_token_holding_v1::splice_api_token_holding_v1::Splice_Api_Token_HoldingV1::Holding,
                >,
            >,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for AllocationView {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("allocation", rt::ToValue::to_value(&self.allocation)),
                    ("holdingCids", rt::ToValue::to_value(&self.holding_cids)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for AllocationView {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    allocation: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "allocation",
                    )?)
                    .map_err(|e| e.at("allocation"))?,
                    holding_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "holdingCids",
                    )?)
                    .map_err(|e| e.at("holdingCids"))?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 2usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationSpecification {
            #[serde(rename = "settlement")]
            pub settlement: crate::splice_api_token_allocation_v1::Splice_Api_Token_AllocationV1::SettlementInfo,
            ///Daml field `transferLegId`.
            #[serde(rename = "transferLegId")]
            pub transfer_leg_id: ::std::string::String,
            ///Daml field `transferLeg`.
            #[serde(rename = "transferLeg")]
            pub transfer_leg: crate::splice_api_token_allocation_v1::Splice_Api_Token_AllocationV1::TransferLeg,
        }
        impl rt::ToValue for AllocationSpecification {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("settlement", rt::ToValue::to_value(&self.settlement)),
                    (
                        "transferLegId",
                        rt::ToValue::to_value(&self.transfer_leg_id)
                    ),
                    ("transferLeg", rt::ToValue::to_value(&self.transfer_leg)),
                ])
            }
        }
        impl rt::FromValue for AllocationSpecification {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    settlement: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "settlement",
                    )?)
                    .map_err(|e| e.at("settlement"))?,
                    transfer_leg_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "transferLegId",
                    )?)
                    .map_err(|e| e.at("transferLegId"))?,
                    transfer_leg: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "transferLeg",
                    )?)
                    .map_err(|e| e.at("transferLeg"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferLeg {
            #[serde(rename = "sender")]
            pub sender: rt::Party,
            #[serde(rename = "receiver")]
            pub receiver: rt::Party,
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            ///Daml field `instrumentId`.
            #[serde(rename = "instrumentId")]
            pub instrument_id: ::canton_splice_api_token_holding_v1::splice_api_token_holding_v1::Splice_Api_Token_HoldingV1::InstrumentId,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for TransferLeg {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("sender", rt::ToValue::to_value(&self.sender)),
                    ("receiver", rt::ToValue::to_value(&self.receiver)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                    ("instrumentId", rt::ToValue::to_value(&self.instrument_id)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for TransferLeg {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    sender: rt::FromValue::from_value(rt::required_field(value, 0usize, "sender")?)
                        .map_err(|e| e.at("sender"))?,
                    receiver: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "receiver",
                    )?)
                    .map_err(|e| e.at("receiver"))?,
                    amount: rt::FromValue::from_value(rt::required_field(value, 2usize, "amount")?)
                        .map_err(|e| e.at("amount"))?,
                    instrument_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "instrumentId",
                    )?)
                    .map_err(|e| e.at("instrumentId"))?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 4usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SettlementInfo {
            #[serde(rename = "executor")]
            pub executor: rt::Party,
            ///Daml field `settlementRef`.
            #[serde(rename = "settlementRef")]
            pub settlement_ref: crate::splice_api_token_allocation_v1::Splice_Api_Token_AllocationV1::Reference,
            ///Daml field `requestedAt`.
            #[serde(rename = "requestedAt")]
            pub requested_at: rt::Timestamp,
            ///Daml field `allocateBefore`.
            #[serde(rename = "allocateBefore")]
            pub allocate_before: rt::Timestamp,
            ///Daml field `settleBefore`.
            #[serde(rename = "settleBefore")]
            pub settle_before: rt::Timestamp,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for SettlementInfo {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("executor", rt::ToValue::to_value(&self.executor)),
                    ("settlementRef", rt::ToValue::to_value(&self.settlement_ref)),
                    ("requestedAt", rt::ToValue::to_value(&self.requested_at)),
                    (
                        "allocateBefore",
                        rt::ToValue::to_value(&self.allocate_before)
                    ),
                    ("settleBefore", rt::ToValue::to_value(&self.settle_before)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for SettlementInfo {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    executor: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "executor",
                    )?)
                    .map_err(|e| e.at("executor"))?,
                    settlement_ref: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "settlementRef",
                    )?)
                    .map_err(|e| e.at("settlementRef"))?,
                    requested_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "requestedAt",
                    )?)
                    .map_err(|e| e.at("requestedAt"))?,
                    allocate_before: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "allocateBefore",
                    )?)
                    .map_err(|e| e.at("allocateBefore"))?,
                    settle_before: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "settleBefore",
                    )?)
                    .map_err(|e| e.at("settleBefore"))?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 5usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Reference {
            #[serde(rename = "id")]
            pub id: ::std::string::String,
            #[serde(rename = "cid")]
            pub cid: ::core::option::Option<
                rt::ContractId<
                    ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::AnyContract,
                >,
            >,
        }
        impl rt::ToValue for Reference {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("id", rt::ToValue::to_value(&self.id)),
                    ("cid", rt::ToValue::to_value(&self.cid)),
                ])
            }
        }
        impl rt::FromValue for Reference {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    id: rt::FromValue::from_value(rt::required_field(value, 0usize, "id")?)
                        .map_err(|e| e.at("id"))?,
                    cid: rt::optional_field(value, 1usize, "cid").map_err(|e| e.at("cid"))?,
                })
            }
        }
        ///Marker for the Daml interface `Allocation` (held via `ContractId`).
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct Allocation;
        impl rt::Contract for Allocation {
            const PACKAGE_ID: &'static str =
                "93c942ae2b4c2ba674fb152fe38473c507bda4e82b4e4c5da55a552a9d8cce1d";
            const PACKAGE_NAME: &'static str = "splice-api-token-allocation-v1";
            const MODULE_NAME: &'static str = "Splice.Api.Token.AllocationV1";
            const ENTITY_NAME: &'static str = "Allocation";
        }
        impl rt::Interface for Allocation {
            type View = crate::splice_api_token_allocation_v1::Splice_Api_Token_AllocationV1::AllocationView;
        }
        ///The `Archive` choice on [`Allocation`] (consuming).
        impl rt::Choice<Allocation>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `Allocation_Withdraw` choice on [`Allocation`] (consuming).
        impl rt::Choice<Allocation>
        for crate::splice_api_token_allocation_v1::Splice_Api_Token_AllocationV1::Allocation_Withdraw {
            type Return = crate::splice_api_token_allocation_v1::Splice_Api_Token_AllocationV1::Allocation_WithdrawResult;
            const NAME: &'static str = "Allocation_Withdraw";
            const CONSUMING: bool = true;
        }
        ///The `Allocation_Cancel` choice on [`Allocation`] (consuming).
        impl rt::Choice<Allocation>
        for crate::splice_api_token_allocation_v1::Splice_Api_Token_AllocationV1::Allocation_Cancel {
            type Return = crate::splice_api_token_allocation_v1::Splice_Api_Token_AllocationV1::Allocation_CancelResult;
            const NAME: &'static str = "Allocation_Cancel";
            const CONSUMING: bool = true;
        }
        ///The `Allocation_ExecuteTransfer` choice on [`Allocation`] (consuming).
        impl rt::Choice<Allocation>
        for crate::splice_api_token_allocation_v1::Splice_Api_Token_AllocationV1::Allocation_ExecuteTransfer {
            type Return = crate::splice_api_token_allocation_v1::Splice_Api_Token_AllocationV1::Allocation_ExecuteTransferResult;
            const NAME: &'static str = "Allocation_ExecuteTransfer";
            const CONSUMING: bool = true;
        }
    }
}
