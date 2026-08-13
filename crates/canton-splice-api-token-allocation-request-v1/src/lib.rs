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
//! - [`splice_api_token_allocation_request_v1`] — 1 Daml module

pub mod splice_api_token_allocation_request_v1 {
    pub mod Splice_Api_Token_AllocationRequestV1 {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationRequestView {
            #[serde(rename = "settlement")]
            pub settlement: ::canton_splice_api_token_allocation_v1::splice_api_token_allocation_v1::Splice_Api_Token_AllocationV1::SettlementInfo,
            ///Daml field `transferLegs`.
            #[serde(rename = "transferLegs")]
            pub transfer_legs: rt::TextMap<
                ::canton_splice_api_token_allocation_v1::splice_api_token_allocation_v1::Splice_Api_Token_AllocationV1::TransferLeg,
            >,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for AllocationRequestView {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("settlement", rt::ToValue::to_value(&self.settlement)),
                    ("transferLegs", rt::ToValue::to_value(&self.transfer_legs)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for AllocationRequestView {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    settlement: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "settlement",
                    )?)
                    .map_err(|e| e.at("settlement"))?,
                    transfer_legs: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "transferLegs",
                    )?)
                    .map_err(|e| e.at("transferLegs"))?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 2usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationRequest_Withdraw {
            ///Daml field `extraArgs`.
            #[serde(rename = "extraArgs")]
            pub extra_args: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for AllocationRequest_Withdraw {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "extraArgs",
                    rt::ToValue::to_value(&self.extra_args)
                ),])
            }
        }
        impl rt::FromValue for AllocationRequest_Withdraw {
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
        pub struct AllocationRequest_Reject {
            #[serde(rename = "actor")]
            pub actor: rt::Party,
            ///Daml field `extraArgs`.
            #[serde(rename = "extraArgs")]
            pub extra_args: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for AllocationRequest_Reject {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("actor", rt::ToValue::to_value(&self.actor)),
                    ("extraArgs", rt::ToValue::to_value(&self.extra_args)),
                ])
            }
        }
        impl rt::FromValue for AllocationRequest_Reject {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    actor: rt::FromValue::from_value(rt::required_field(value, 0usize, "actor")?)
                        .map_err(|e| e.at("actor"))?,
                    extra_args: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "extraArgs",
                    )?)
                    .map_err(|e| e.at("extraArgs"))?,
                })
            }
        }
        ///Marker for the Daml interface `AllocationRequest` (held via `ContractId`).
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct AllocationRequest;
        impl rt::Contract for AllocationRequest {
            const PACKAGE_ID: &'static str =
                "6fe848530b2404017c4a12874c956ad7d5c8a419ee9b040f96b5c13172d2e193";
            const PACKAGE_NAME: &'static str = "splice-api-token-allocation-request-v1";
            const MODULE_NAME: &'static str = "Splice.Api.Token.AllocationRequestV1";
            const ENTITY_NAME: &'static str = "AllocationRequest";
        }
        impl rt::Interface for AllocationRequest {
            type View = crate::splice_api_token_allocation_request_v1::Splice_Api_Token_AllocationRequestV1::AllocationRequestView;
        }
        ///The `Archive` choice on [`AllocationRequest`] (consuming).
        impl rt::Choice<AllocationRequest>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `AllocationRequest_Reject` choice on [`AllocationRequest`] (consuming).
        impl rt::Choice<AllocationRequest>
        for crate::splice_api_token_allocation_request_v1::Splice_Api_Token_AllocationRequestV1::AllocationRequest_Reject {
            type Return = ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ChoiceExecutionMetadata;
            const NAME: &'static str = "AllocationRequest_Reject";
            const CONSUMING: bool = true;
        }
        ///The `AllocationRequest_Withdraw` choice on [`AllocationRequest`] (consuming).
        impl rt::Choice<AllocationRequest>
        for crate::splice_api_token_allocation_request_v1::Splice_Api_Token_AllocationRequestV1::AllocationRequest_Withdraw {
            type Return = ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ChoiceExecutionMetadata;
            const NAME: &'static str = "AllocationRequest_Withdraw";
            const CONSUMING: bool = true;
        }
    }
}
