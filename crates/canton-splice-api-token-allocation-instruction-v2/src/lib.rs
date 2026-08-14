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
//! - [`splice_api_token_allocation_instruction_v2`] — 1 Daml module

pub mod splice_api_token_allocation_instruction_v2 {
    pub mod Splice_Api_Token_AllocationInstructionV2 {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum AllocationInstructionResult_Output {
            #[serde(rename = "AllocationInstructionResult_Pending")]
            AllocationInstructionResult_Pending(
                crate::splice_api_token_allocation_instruction_v2::Splice_Api_Token_AllocationInstructionV2::AllocationInstructionResult_Output_AllocationInstructionResult_Pending,
            ),
            #[serde(rename = "AllocationInstructionResult_Completed")]
            AllocationInstructionResult_Completed(
                crate::splice_api_token_allocation_instruction_v2::Splice_Api_Token_AllocationInstructionV2::AllocationInstructionResult_Output_AllocationInstructionResult_Completed,
            ),
            #[serde(rename = "AllocationInstructionResult_Failed")]
            AllocationInstructionResult_Failed(rt::Unit),
        }
        impl rt::ToValue for AllocationInstructionResult_Output {
            fn to_value(&self) -> rt::Value {
                match self {
                    AllocationInstructionResult_Output::AllocationInstructionResult_Pending(
                        inner,
                    ) => rt::variant_value(
                        "AllocationInstructionResult_Pending",
                        rt::ToValue::to_value(inner),
                    ),
                    AllocationInstructionResult_Output::AllocationInstructionResult_Completed(
                        inner,
                    ) => rt::variant_value(
                        "AllocationInstructionResult_Completed",
                        rt::ToValue::to_value(inner),
                    ),
                    AllocationInstructionResult_Output::AllocationInstructionResult_Failed(
                        inner,
                    ) => rt::variant_value(
                        "AllocationInstructionResult_Failed",
                        rt::ToValue::to_value(inner),
                    ),
                }
            }
        }
        impl rt::FromValue for AllocationInstructionResult_Output {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                let (constructor, payload) = rt::variant_parts(value)?;
                match constructor {
                    "AllocationInstructionResult_Pending" => ::core::result::Result::Ok(
                        AllocationInstructionResult_Output::AllocationInstructionResult_Pending(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("AllocationInstructionResult_Pending"))?,
                        ),
                    ),
                    "AllocationInstructionResult_Completed" => ::core::result::Result::Ok(
                        AllocationInstructionResult_Output::AllocationInstructionResult_Completed(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("AllocationInstructionResult_Completed"))?,
                        ),
                    ),
                    "AllocationInstructionResult_Failed" => ::core::result::Result::Ok(
                        AllocationInstructionResult_Output::AllocationInstructionResult_Failed(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("AllocationInstructionResult_Failed"))?,
                        ),
                    ),
                    other => ::core::result::Result::Err(rt::unexpected_constructor(
                        "AllocationInstructionResult_Output",
                        other,
                    )),
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationInstructionResult_Output_AllocationInstructionResult_Pending {
            ///Daml field `allocationInstructionCid`.
            #[serde(rename = "allocationInstructionCid")]
            pub allocation_instruction_cid: rt::ContractId<
                crate::splice_api_token_allocation_instruction_v2::Splice_Api_Token_AllocationInstructionV2::AllocationInstruction,
            >,
        }
        impl rt::ToValue for AllocationInstructionResult_Output_AllocationInstructionResult_Pending {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "allocationInstructionCid",
                    rt::ToValue::to_value(&self.allocation_instruction_cid)
                ),])
            }
        }
        impl rt::FromValue for AllocationInstructionResult_Output_AllocationInstructionResult_Pending {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    allocation_instruction_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "allocationInstructionCid",
                    )?)
                    .map_err(|e| e.at("allocationInstructionCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationInstructionResult_Output_AllocationInstructionResult_Completed {
            ///Daml field `allocationCid`.
            #[serde(rename = "allocationCid")]
            pub allocation_cid: rt::ContractId<
                ::canton_splice_api_token_allocation_v2::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::Allocation,
            >,
        }
        impl rt::ToValue for AllocationInstructionResult_Output_AllocationInstructionResult_Completed {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "allocationCid",
                    rt::ToValue::to_value(&self.allocation_cid)
                ),])
            }
        }
        impl rt::FromValue for AllocationInstructionResult_Output_AllocationInstructionResult_Completed {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    allocation_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "allocationCid",
                    )?)
                    .map_err(|e| e.at("allocationCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationInstructionResult {
            #[serde(rename = "output")]
            pub output: crate::splice_api_token_allocation_instruction_v2::Splice_Api_Token_AllocationInstructionV2::AllocationInstructionResult_Output,
            ///Daml field `authorizerChangeCids`.
            #[serde(rename = "authorizerChangeCids")]
            pub authorizer_change_cids: rt::TextMap<
                ::std::vec::Vec<
                    rt::ContractId<
                        ::canton_splice_api_token_holding_v2::splice_api_token_holding_v2::Splice_Api_Token_HoldingV2::Holding,
                    >,
                >,
            >,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for AllocationInstructionResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("output", rt::ToValue::to_value(&self.output)),
                    (
                        "authorizerChangeCids",
                        rt::ToValue::to_value(&self.authorizer_change_cids)
                    ),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for AllocationInstructionResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    output: rt::FromValue::from_value(rt::required_field(value, 0usize, "output")?)
                        .map_err(|e| e.at("output"))?,
                    authorizer_change_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "authorizerChangeCids",
                    )?)
                    .map_err(|e| e.at("authorizerChangeCids"))?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 2usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationFactory_PublicFetch {
            #[serde(rename = "actors")]
            pub actors: ::std::vec::Vec<rt::Party>,
        }
        impl rt::ToValue for AllocationFactory_PublicFetch {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("actors", rt::ToValue::to_value(&self.actors)),])
            }
        }
        impl rt::FromValue for AllocationFactory_PublicFetch {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    actors: rt::FromValue::from_value(rt::required_field(value, 0usize, "actors")?)
                        .map_err(|e| e.at("actors"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationFactory_Allocate {
            #[serde(rename = "settlement")]
            pub settlement: ::canton_splice_api_token_allocation_v2::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::SettlementInfo,
            #[serde(rename = "allocation")]
            pub allocation: ::canton_splice_api_token_allocation_v2::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::AllocationSpecification,
            ///Daml field `requestedAt`.
            #[serde(rename = "requestedAt")]
            pub requested_at: rt::Timestamp,
            ///Daml field `inputHoldingCids`.
            #[serde(rename = "inputHoldingCids")]
            pub input_holding_cids: ::std::vec::Vec<
                rt::ContractId<
                    ::canton_splice_api_token_holding_v2::splice_api_token_holding_v2::Splice_Api_Token_HoldingV2::Holding,
                >,
            >,
            ///Daml field `extraArgs`.
            #[serde(rename = "extraArgs")]
            pub extra_args: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ExtraArgs,
            #[serde(rename = "actors")]
            pub actors: ::std::vec::Vec<rt::Party>,
        }
        impl rt::ToValue for AllocationFactory_Allocate {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("settlement", rt::ToValue::to_value(&self.settlement)),
                    ("allocation", rt::ToValue::to_value(&self.allocation)),
                    ("requestedAt", rt::ToValue::to_value(&self.requested_at)),
                    (
                        "inputHoldingCids",
                        rt::ToValue::to_value(&self.input_holding_cids)
                    ),
                    ("extraArgs", rt::ToValue::to_value(&self.extra_args)),
                    ("actors", rt::ToValue::to_value(&self.actors)),
                ])
            }
        }
        impl rt::FromValue for AllocationFactory_Allocate {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    settlement: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "settlement",
                    )?)
                    .map_err(|e| e.at("settlement"))?,
                    allocation: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "allocation",
                    )?)
                    .map_err(|e| e.at("allocation"))?,
                    requested_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "requestedAt",
                    )?)
                    .map_err(|e| e.at("requestedAt"))?,
                    input_holding_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "inputHoldingCids",
                    )?)
                    .map_err(|e| e.at("inputHoldingCids"))?,
                    extra_args: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "extraArgs",
                    )?)
                    .map_err(|e| e.at("extraArgs"))?,
                    actors: rt::FromValue::from_value(rt::required_field(value, 5usize, "actors")?)
                        .map_err(|e| e.at("actors"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationFactoryView {
            #[serde(rename = "admin")]
            pub admin: rt::Party,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for AllocationFactoryView {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("admin", rt::ToValue::to_value(&self.admin)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for AllocationFactoryView {
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
        pub struct AllocationInstruction_Withdraw {
            #[serde(rename = "actors")]
            pub actors: ::std::vec::Vec<rt::Party>,
            ///Daml field `extraArgs`.
            #[serde(rename = "extraArgs")]
            pub extra_args: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for AllocationInstruction_Withdraw {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("actors", rt::ToValue::to_value(&self.actors)),
                    ("extraArgs", rt::ToValue::to_value(&self.extra_args)),
                ])
            }
        }
        impl rt::FromValue for AllocationInstruction_Withdraw {
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
        pub struct AllocationInstruction_Accept {
            #[serde(rename = "actors")]
            pub actors: ::std::vec::Vec<rt::Party>,
            ///Daml field `extraArgs`.
            #[serde(rename = "extraArgs")]
            pub extra_args: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for AllocationInstruction_Accept {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("actors", rt::ToValue::to_value(&self.actors)),
                    ("extraArgs", rt::ToValue::to_value(&self.extra_args)),
                ])
            }
        }
        impl rt::FromValue for AllocationInstruction_Accept {
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
        pub struct AllocationInstructionView {
            ///Daml field `originalInstructionCid`.
            #[serde(rename = "originalInstructionCid")]
            pub original_instruction_cid: ::core::option::Option<
                rt::ContractId<
                    crate::splice_api_token_allocation_instruction_v2::Splice_Api_Token_AllocationInstructionV2::AllocationInstruction,
                >,
            >,
            #[serde(rename = "settlement")]
            pub settlement: ::canton_splice_api_token_allocation_v2::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::SettlementInfo,
            #[serde(rename = "allocation")]
            pub allocation: ::canton_splice_api_token_allocation_v2::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::AllocationSpecification,
            ///Daml field `requestedAt`.
            #[serde(rename = "requestedAt")]
            pub requested_at: rt::Timestamp,
            ///Daml field `inputHoldingCids`.
            #[serde(rename = "inputHoldingCids")]
            pub input_holding_cids: ::std::vec::Vec<
                rt::ContractId<
                    ::canton_splice_api_token_holding_v2::splice_api_token_holding_v2::Splice_Api_Token_HoldingV2::Holding,
                >,
            >,
            ///Daml field `expiresAt`.
            #[serde(rename = "expiresAt")]
            pub expires_at: ::core::option::Option<rt::Timestamp>,
            ///Daml field `availableActions`.
            #[serde(rename = "availableActions")]
            pub available_actions: rt::GenMap<
                crate::splice_api_token_allocation_instruction_v2::Splice_Api_Token_AllocationInstructionV2::AllocationInstructionAction,
                ::std::vec::Vec<::std::vec::Vec<rt::Party>>,
            >,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for AllocationInstructionView {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "originalInstructionCid",
                        rt::ToValue::to_value(&self.original_instruction_cid)
                    ),
                    ("settlement", rt::ToValue::to_value(&self.settlement)),
                    ("allocation", rt::ToValue::to_value(&self.allocation)),
                    ("requestedAt", rt::ToValue::to_value(&self.requested_at)),
                    (
                        "inputHoldingCids",
                        rt::ToValue::to_value(&self.input_holding_cids)
                    ),
                    ("expiresAt", rt::ToValue::to_value(&self.expires_at)),
                    (
                        "availableActions",
                        rt::ToValue::to_value(&self.available_actions)
                    ),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for AllocationInstructionView {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    original_instruction_cid: rt::optional_field(
                        value,
                        0usize,
                        "originalInstructionCid",
                    )
                    .map_err(|e| e.at("originalInstructionCid"))?,
                    settlement: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "settlement",
                    )?)
                    .map_err(|e| e.at("settlement"))?,
                    allocation: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "allocation",
                    )?)
                    .map_err(|e| e.at("allocation"))?,
                    requested_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "requestedAt",
                    )?)
                    .map_err(|e| e.at("requestedAt"))?,
                    input_holding_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "inputHoldingCids",
                    )?)
                    .map_err(|e| e.at("inputHoldingCids"))?,
                    expires_at: rt::optional_field(value, 5usize, "expiresAt")
                        .map_err(|e| e.at("expiresAt"))?,
                    available_actions: rt::FromValue::from_value(rt::required_field(
                        value,
                        6usize,
                        "availableActions",
                    )?)
                    .map_err(|e| e.at("availableActions"))?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 7usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum AllocationInstructionAction {
            #[serde(rename = "AIA_Withdraw")]
            AIA_Withdraw(rt::Unit),
            #[serde(rename = "AIA_Accept")]
            AIA_Accept(rt::Unit),
            #[serde(rename = "AIA_Custom")]
            AIA_Custom(
                crate::splice_api_token_allocation_instruction_v2::Splice_Api_Token_AllocationInstructionV2::AllocationInstructionAction_AIA_Custom,
            ),
        }
        impl rt::ToValue for AllocationInstructionAction {
            fn to_value(&self) -> rt::Value {
                match self {
                    AllocationInstructionAction::AIA_Withdraw(inner) => {
                        rt::variant_value("AIA_Withdraw", rt::ToValue::to_value(inner))
                    }
                    AllocationInstructionAction::AIA_Accept(inner) => {
                        rt::variant_value("AIA_Accept", rt::ToValue::to_value(inner))
                    }
                    AllocationInstructionAction::AIA_Custom(inner) => {
                        rt::variant_value("AIA_Custom", rt::ToValue::to_value(inner))
                    }
                }
            }
        }
        impl rt::FromValue for AllocationInstructionAction {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                let (constructor, payload) = rt::variant_parts(value)?;
                match constructor {
                    "AIA_Withdraw" => {
                        ::core::result::Result::Ok(AllocationInstructionAction::AIA_Withdraw(
                            rt::FromValue::from_value(payload).map_err(|e| e.at("AIA_Withdraw"))?,
                        ))
                    }
                    "AIA_Accept" => {
                        ::core::result::Result::Ok(AllocationInstructionAction::AIA_Accept(
                            rt::FromValue::from_value(payload).map_err(|e| e.at("AIA_Accept"))?,
                        ))
                    }
                    "AIA_Custom" => {
                        ::core::result::Result::Ok(AllocationInstructionAction::AIA_Custom(
                            rt::FromValue::from_value(payload).map_err(|e| e.at("AIA_Custom"))?,
                        ))
                    }
                    other => ::core::result::Result::Err(rt::unexpected_constructor(
                        "AllocationInstructionAction",
                        other,
                    )),
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationInstructionAction_AIA_Custom {
            #[serde(rename = "id")]
            pub id: ::std::string::String,
        }
        impl rt::ToValue for AllocationInstructionAction_AIA_Custom {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("id", rt::ToValue::to_value(&self.id)),])
            }
        }
        impl rt::FromValue for AllocationInstructionAction_AIA_Custom {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    id: rt::FromValue::from_value(rt::required_field(value, 0usize, "id")?)
                        .map_err(|e| e.at("id"))?,
                })
            }
        }
        ///Marker for the Daml interface `AllocationFactory` (held via `ContractId`).
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct AllocationFactory;
        ///Marker for the Daml interface `AllocationInstruction` (held via `ContractId`).
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct AllocationInstruction;
        impl rt::Contract for AllocationFactory {
            const PACKAGE_ID: &'static str =
                "9818a0b5b827109de03a04c8f6151cde9d1e7fe5123dbb2dfeb0e52d7271287c";
            const PACKAGE_NAME: &'static str = "splice-api-token-allocation-instruction-v2";
            const MODULE_NAME: &'static str = "Splice.Api.Token.AllocationInstructionV2";
            const ENTITY_NAME: &'static str = "AllocationFactory";
        }
        impl rt::Interface for AllocationFactory {
            type View = crate::splice_api_token_allocation_instruction_v2::Splice_Api_Token_AllocationInstructionV2::AllocationFactoryView;
        }
        ///The `Archive` choice on [`AllocationFactory`] (consuming).
        impl rt::Choice<AllocationFactory>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `AllocationFactory_Allocate` choice on [`AllocationFactory`] (non-consuming).
        impl rt::Choice<AllocationFactory>
        for crate::splice_api_token_allocation_instruction_v2::Splice_Api_Token_AllocationInstructionV2::AllocationFactory_Allocate {
            type Return = crate::splice_api_token_allocation_instruction_v2::Splice_Api_Token_AllocationInstructionV2::AllocationInstructionResult;
            const NAME: &'static str = "AllocationFactory_Allocate";
            const CONSUMING: bool = false;
        }
        ///The `AllocationFactory_PublicFetch` choice on [`AllocationFactory`] (non-consuming).
        impl rt::Choice<AllocationFactory>
        for crate::splice_api_token_allocation_instruction_v2::Splice_Api_Token_AllocationInstructionV2::AllocationFactory_PublicFetch {
            type Return = crate::splice_api_token_allocation_instruction_v2::Splice_Api_Token_AllocationInstructionV2::AllocationFactoryView;
            const NAME: &'static str = "AllocationFactory_PublicFetch";
            const CONSUMING: bool = false;
        }
        impl rt::Contract for AllocationInstruction {
            const PACKAGE_ID: &'static str =
                "9818a0b5b827109de03a04c8f6151cde9d1e7fe5123dbb2dfeb0e52d7271287c";
            const PACKAGE_NAME: &'static str = "splice-api-token-allocation-instruction-v2";
            const MODULE_NAME: &'static str = "Splice.Api.Token.AllocationInstructionV2";
            const ENTITY_NAME: &'static str = "AllocationInstruction";
        }
        impl rt::Interface for AllocationInstruction {
            type View = crate::splice_api_token_allocation_instruction_v2::Splice_Api_Token_AllocationInstructionV2::AllocationInstructionView;
        }
        ///The `Archive` choice on [`AllocationInstruction`] (consuming).
        impl rt::Choice<AllocationInstruction>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `AllocationInstruction_Withdraw` choice on [`AllocationInstruction`] (non-consuming).
        impl rt::Choice<AllocationInstruction>
        for crate::splice_api_token_allocation_instruction_v2::Splice_Api_Token_AllocationInstructionV2::AllocationInstruction_Withdraw {
            type Return = crate::splice_api_token_allocation_instruction_v2::Splice_Api_Token_AllocationInstructionV2::AllocationInstructionResult;
            const NAME: &'static str = "AllocationInstruction_Withdraw";
            const CONSUMING: bool = false;
        }
        ///The `AllocationInstruction_Accept` choice on [`AllocationInstruction`] (non-consuming).
        impl rt::Choice<AllocationInstruction>
        for crate::splice_api_token_allocation_instruction_v2::Splice_Api_Token_AllocationInstructionV2::AllocationInstruction_Accept {
            type Return = crate::splice_api_token_allocation_instruction_v2::Splice_Api_Token_AllocationInstructionV2::AllocationInstructionResult;
            const NAME: &'static str = "AllocationInstruction_Accept";
            const CONSUMING: bool = false;
        }
    }
}
