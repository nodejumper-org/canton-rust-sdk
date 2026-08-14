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
//! - [`splice_api_token_allocation_request_v2`] — 1 Daml module

pub mod splice_api_token_allocation_request_v2 {
    pub mod Splice_Api_Token_AllocationRequestV2 {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationRequest_WithdrawResult {
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for AllocationRequest_WithdrawResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("meta", rt::ToValue::to_value(&self.meta)),])
            }
        }
        impl rt::FromValue for AllocationRequest_WithdrawResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    meta: rt::FromValue::from_value(rt::required_field(value, 0usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationRequest_RejectResult {
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for AllocationRequest_RejectResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("meta", rt::ToValue::to_value(&self.meta)),])
            }
        }
        impl rt::FromValue for AllocationRequest_RejectResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    meta: rt::FromValue::from_value(rt::required_field(value, 0usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationRequest_AcceptResult {
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for AllocationRequest_AcceptResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("meta", rt::ToValue::to_value(&self.meta)),])
            }
        }
        impl rt::FromValue for AllocationRequest_AcceptResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    meta: rt::FromValue::from_value(rt::required_field(value, 0usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationRequestView {
            ///Daml field `originalRequestCid`.
            #[serde(rename = "originalRequestCid")]
            pub original_request_cid: ::core::option::Option<
                rt::ContractId<
                    crate::splice_api_token_allocation_request_v2::Splice_Api_Token_AllocationRequestV2::AllocationRequest,
                >,
            >,
            #[serde(rename = "settlement")]
            pub settlement: ::canton_splice_api_token_allocation_v2::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::SettlementInfo,
            #[serde(rename = "allocations")]
            pub allocations: ::std::vec::Vec<
                ::canton_splice_api_token_allocation_v2::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::AllocationSpecification,
            >,
            ///Daml field `requestedAt`.
            #[serde(rename = "requestedAt")]
            pub requested_at: rt::Timestamp,
            ///Daml field `settleAt`.
            #[serde(rename = "settleAt")]
            pub settle_at: ::core::option::Option<rt::Timestamp>,
            ///Daml field `availableActions`.
            #[serde(rename = "availableActions")]
            pub available_actions: rt::GenMap<
                crate::splice_api_token_allocation_request_v2::Splice_Api_Token_AllocationRequestV2::AllocationRequestAction,
                ::std::vec::Vec<::std::vec::Vec<rt::Party>>,
            >,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for AllocationRequestView {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "originalRequestCid",
                        rt::ToValue::to_value(&self.original_request_cid)
                    ),
                    ("settlement", rt::ToValue::to_value(&self.settlement)),
                    ("allocations", rt::ToValue::to_value(&self.allocations)),
                    ("requestedAt", rt::ToValue::to_value(&self.requested_at)),
                    ("settleAt", rt::ToValue::to_value(&self.settle_at)),
                    (
                        "availableActions",
                        rt::ToValue::to_value(&self.available_actions)
                    ),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for AllocationRequestView {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    original_request_cid: rt::optional_field(value, 0usize, "originalRequestCid")
                        .map_err(|e| e.at("originalRequestCid"))?,
                    settlement: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "settlement",
                    )?)
                    .map_err(|e| e.at("settlement"))?,
                    allocations: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "allocations",
                    )?)
                    .map_err(|e| e.at("allocations"))?,
                    requested_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "requestedAt",
                    )?)
                    .map_err(|e| e.at("requestedAt"))?,
                    settle_at: rt::optional_field(value, 4usize, "settleAt")
                        .map_err(|e| e.at("settleAt"))?,
                    available_actions: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "availableActions",
                    )?)
                    .map_err(|e| e.at("availableActions"))?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 6usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum AllocationRequestAction {
            #[serde(rename = "ARA_Accept")]
            ARA_Accept(rt::Unit),
            #[serde(rename = "ARA_Reject")]
            ARA_Reject(rt::Unit),
            #[serde(rename = "ARA_Custom")]
            ARA_Custom(
                crate::splice_api_token_allocation_request_v2::Splice_Api_Token_AllocationRequestV2::AllocationRequestAction_ARA_Custom,
            ),
        }
        impl rt::ToValue for AllocationRequestAction {
            fn to_value(&self) -> rt::Value {
                match self {
                    AllocationRequestAction::ARA_Accept(inner) => {
                        rt::variant_value("ARA_Accept", rt::ToValue::to_value(inner))
                    }
                    AllocationRequestAction::ARA_Reject(inner) => {
                        rt::variant_value("ARA_Reject", rt::ToValue::to_value(inner))
                    }
                    AllocationRequestAction::ARA_Custom(inner) => {
                        rt::variant_value("ARA_Custom", rt::ToValue::to_value(inner))
                    }
                }
            }
        }
        impl rt::FromValue for AllocationRequestAction {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                let (constructor, payload) = rt::variant_parts(value)?;
                match constructor {
                    "ARA_Accept" => {
                        ::core::result::Result::Ok(AllocationRequestAction::ARA_Accept(
                            rt::FromValue::from_value(payload).map_err(|e| e.at("ARA_Accept"))?,
                        ))
                    }
                    "ARA_Reject" => {
                        ::core::result::Result::Ok(AllocationRequestAction::ARA_Reject(
                            rt::FromValue::from_value(payload).map_err(|e| e.at("ARA_Reject"))?,
                        ))
                    }
                    "ARA_Custom" => {
                        ::core::result::Result::Ok(AllocationRequestAction::ARA_Custom(
                            rt::FromValue::from_value(payload).map_err(|e| e.at("ARA_Custom"))?,
                        ))
                    }
                    other => ::core::result::Result::Err(rt::unexpected_constructor(
                        "AllocationRequestAction",
                        other,
                    )),
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationRequestAction_ARA_Custom {
            #[serde(rename = "id")]
            pub id: ::std::string::String,
        }
        impl rt::ToValue for AllocationRequestAction_ARA_Custom {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("id", rt::ToValue::to_value(&self.id)),])
            }
        }
        impl rt::FromValue for AllocationRequestAction_ARA_Custom {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    id: rt::FromValue::from_value(rt::required_field(value, 0usize, "id")?)
                        .map_err(|e| e.at("id"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationRequest_Withdraw {
            #[serde(rename = "actors")]
            pub actors: ::std::vec::Vec<rt::Party>,
            ///Daml field `extraArgs`.
            #[serde(rename = "extraArgs")]
            pub extra_args: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for AllocationRequest_Withdraw {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("actors", rt::ToValue::to_value(&self.actors)),
                    ("extraArgs", rt::ToValue::to_value(&self.extra_args)),
                ])
            }
        }
        impl rt::FromValue for AllocationRequest_Withdraw {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    actors: rt::FromValue::from_value(rt::required_field(value, 0usize, "actors")?)
                        .map_err(|e| e.at("actors"))?,
                    extra_args: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "extraArgs",
                    )?)
                    .map_err(|e| e.at("extraArgs"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationRequest_Reject {
            #[serde(rename = "actors")]
            pub actors: ::std::vec::Vec<rt::Party>,
            ///Daml field `extraArgs`.
            #[serde(rename = "extraArgs")]
            pub extra_args: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for AllocationRequest_Reject {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("actors", rt::ToValue::to_value(&self.actors)),
                    ("extraArgs", rt::ToValue::to_value(&self.extra_args)),
                ])
            }
        }
        impl rt::FromValue for AllocationRequest_Reject {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    actors: rt::FromValue::from_value(rt::required_field(value, 0usize, "actors")?)
                        .map_err(|e| e.at("actors"))?,
                    extra_args: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "extraArgs",
                    )?)
                    .map_err(|e| e.at("extraArgs"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationRequest_Accept {
            #[serde(rename = "actors")]
            pub actors: ::std::vec::Vec<rt::Party>,
            ///Daml field `extraArgs`.
            #[serde(rename = "extraArgs")]
            pub extra_args: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for AllocationRequest_Accept {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("actors", rt::ToValue::to_value(&self.actors)),
                    ("extraArgs", rt::ToValue::to_value(&self.extra_args)),
                ])
            }
        }
        impl rt::FromValue for AllocationRequest_Accept {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    actors: rt::FromValue::from_value(rt::required_field(value, 0usize, "actors")?)
                        .map_err(|e| e.at("actors"))?,
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
                "adc16315a8943a8433886694720a2a000ae84c2315c4414bd6d0db4d1660de9c";
            const PACKAGE_NAME: &'static str = "splice-api-token-allocation-request-v2";
            const MODULE_NAME: &'static str = "Splice.Api.Token.AllocationRequestV2";
            const ENTITY_NAME: &'static str = "AllocationRequest";
        }
        impl rt::Interface for AllocationRequest {
            type View = crate::splice_api_token_allocation_request_v2::Splice_Api_Token_AllocationRequestV2::AllocationRequestView;
        }
        ///The `Archive` choice on [`AllocationRequest`] (consuming).
        impl rt::Choice<AllocationRequest>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `AllocationRequest_Accept` choice on [`AllocationRequest`] (non-consuming).
        impl rt::Choice<AllocationRequest>
        for crate::splice_api_token_allocation_request_v2::Splice_Api_Token_AllocationRequestV2::AllocationRequest_Accept {
            type Return = crate::splice_api_token_allocation_request_v2::Splice_Api_Token_AllocationRequestV2::AllocationRequest_AcceptResult;
            const NAME: &'static str = "AllocationRequest_Accept";
            const CONSUMING: bool = false;
        }
        ///The `AllocationRequest_Reject` choice on [`AllocationRequest`] (non-consuming).
        impl rt::Choice<AllocationRequest>
        for crate::splice_api_token_allocation_request_v2::Splice_Api_Token_AllocationRequestV2::AllocationRequest_Reject {
            type Return = crate::splice_api_token_allocation_request_v2::Splice_Api_Token_AllocationRequestV2::AllocationRequest_RejectResult;
            const NAME: &'static str = "AllocationRequest_Reject";
            const CONSUMING: bool = false;
        }
        ///The `AllocationRequest_Withdraw` choice on [`AllocationRequest`] (non-consuming).
        impl rt::Choice<AllocationRequest>
        for crate::splice_api_token_allocation_request_v2::Splice_Api_Token_AllocationRequestV2::AllocationRequest_Withdraw {
            type Return = crate::splice_api_token_allocation_request_v2::Splice_Api_Token_AllocationRequestV2::AllocationRequest_WithdrawResult;
            const NAME: &'static str = "AllocationRequest_Withdraw";
            const CONSUMING: bool = false;
        }
    }
}
