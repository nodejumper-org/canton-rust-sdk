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
//! - [`splice_api_token_allocation_instruction_v1`] — 1 Daml module

pub mod splice_api_token_allocation_instruction_v1 {
    pub mod Splice_Api_Token_AllocationInstructionV1 {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum AllocationInstructionResult_Output {
            #[serde(rename = "AllocationInstructionResult_Pending")]
            AllocationInstructionResult_Pending(
                crate::splice_api_token_allocation_instruction_v1::Splice_Api_Token_AllocationInstructionV1::AllocationInstructionResult_Output_AllocationInstructionResult_Pending,
            ),
            #[serde(rename = "AllocationInstructionResult_Completed")]
            AllocationInstructionResult_Completed(
                crate::splice_api_token_allocation_instruction_v1::Splice_Api_Token_AllocationInstructionV1::AllocationInstructionResult_Output_AllocationInstructionResult_Completed,
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
                crate::splice_api_token_allocation_instruction_v1::Splice_Api_Token_AllocationInstructionV1::AllocationInstruction,
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
                ::canton_splice_api_token_allocation_v1::splice_api_token_allocation_v1::Splice_Api_Token_AllocationV1::Allocation,
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
            pub output: crate::splice_api_token_allocation_instruction_v1::Splice_Api_Token_AllocationInstructionV1::AllocationInstructionResult_Output,
            ///Daml field `senderChangeCids`.
            #[serde(rename = "senderChangeCids")]
            pub sender_change_cids: ::std::vec::Vec<
                rt::ContractId<
                    ::canton_splice_api_token_holding_v1::splice_api_token_holding_v1::Splice_Api_Token_HoldingV1::Holding,
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
                        "senderChangeCids",
                        rt::ToValue::to_value(&self.sender_change_cids)
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
                    sender_change_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "senderChangeCids",
                    )?)
                    .map_err(|e| e.at("senderChangeCids"))?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 2usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationFactory_PublicFetch {
            ///Daml field `expectedAdmin`.
            #[serde(rename = "expectedAdmin")]
            pub expected_admin: rt::Party,
            #[serde(rename = "actor")]
            pub actor: rt::Party,
        }
        impl rt::ToValue for AllocationFactory_PublicFetch {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("expectedAdmin", rt::ToValue::to_value(&self.expected_admin)),
                    ("actor", rt::ToValue::to_value(&self.actor)),
                ])
            }
        }
        impl rt::FromValue for AllocationFactory_PublicFetch {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    expected_admin: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "expectedAdmin",
                    )?)
                    .map_err(|e| e.at("expectedAdmin"))?,
                    actor: rt::FromValue::from_value(rt::required_field(value, 1usize, "actor")?)
                        .map_err(|e| e.at("actor"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationFactory_Allocate {
            ///Daml field `expectedAdmin`.
            #[serde(rename = "expectedAdmin")]
            pub expected_admin: rt::Party,
            #[serde(rename = "allocation")]
            pub allocation: ::canton_splice_api_token_allocation_v1::splice_api_token_allocation_v1::Splice_Api_Token_AllocationV1::AllocationSpecification,
            ///Daml field `requestedAt`.
            #[serde(rename = "requestedAt")]
            pub requested_at: rt::Timestamp,
            ///Daml field `inputHoldingCids`.
            #[serde(rename = "inputHoldingCids")]
            pub input_holding_cids: ::std::vec::Vec<
                rt::ContractId<
                    ::canton_splice_api_token_holding_v1::splice_api_token_holding_v1::Splice_Api_Token_HoldingV1::Holding,
                >,
            >,
            ///Daml field `extraArgs`.
            #[serde(rename = "extraArgs")]
            pub extra_args: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for AllocationFactory_Allocate {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("expectedAdmin", rt::ToValue::to_value(&self.expected_admin)),
                    ("allocation", rt::ToValue::to_value(&self.allocation)),
                    ("requestedAt", rt::ToValue::to_value(&self.requested_at)),
                    (
                        "inputHoldingCids",
                        rt::ToValue::to_value(&self.input_holding_cids)
                    ),
                    ("extraArgs", rt::ToValue::to_value(&self.extra_args)),
                ])
            }
        }
        impl rt::FromValue for AllocationFactory_Allocate {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    expected_admin: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "expectedAdmin",
                    )?)
                    .map_err(|e| e.at("expectedAdmin"))?,
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
        pub struct AllocationInstruction_Update {
            ///Daml field `extraActors`.
            #[serde(rename = "extraActors")]
            pub extra_actors: ::std::vec::Vec<rt::Party>,
            ///Daml field `extraArgs`.
            #[serde(rename = "extraArgs")]
            pub extra_args: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for AllocationInstruction_Update {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("extraActors", rt::ToValue::to_value(&self.extra_actors)),
                    ("extraArgs", rt::ToValue::to_value(&self.extra_args)),
                ])
            }
        }
        impl rt::FromValue for AllocationInstruction_Update {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    extra_actors: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "extraActors",
                    )?)
                    .map_err(|e| e.at("extraActors"))?,
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
        pub struct AllocationInstruction_Withdraw {
            ///Daml field `extraArgs`.
            #[serde(rename = "extraArgs")]
            pub extra_args: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for AllocationInstruction_Withdraw {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "extraArgs",
                    rt::ToValue::to_value(&self.extra_args)
                ),])
            }
        }
        impl rt::FromValue for AllocationInstruction_Withdraw {
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
        pub struct AllocationInstructionView {
            ///Daml field `originalInstructionCid`.
            #[serde(rename = "originalInstructionCid")]
            pub original_instruction_cid: ::core::option::Option<
                rt::ContractId<
                    crate::splice_api_token_allocation_instruction_v1::Splice_Api_Token_AllocationInstructionV1::AllocationInstruction,
                >,
            >,
            #[serde(rename = "allocation")]
            pub allocation: ::canton_splice_api_token_allocation_v1::splice_api_token_allocation_v1::Splice_Api_Token_AllocationV1::AllocationSpecification,
            ///Daml field `pendingActions`.
            #[serde(rename = "pendingActions")]
            pub pending_actions: rt::GenMap<rt::Party, ::std::string::String>,
            ///Daml field `requestedAt`.
            #[serde(rename = "requestedAt")]
            pub requested_at: rt::Timestamp,
            ///Daml field `inputHoldingCids`.
            #[serde(rename = "inputHoldingCids")]
            pub input_holding_cids: ::std::vec::Vec<
                rt::ContractId<
                    ::canton_splice_api_token_holding_v1::splice_api_token_holding_v1::Splice_Api_Token_HoldingV1::Holding,
                >,
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
                    ("allocation", rt::ToValue::to_value(&self.allocation)),
                    (
                        "pendingActions",
                        rt::ToValue::to_value(&self.pending_actions)
                    ),
                    ("requestedAt", rt::ToValue::to_value(&self.requested_at)),
                    (
                        "inputHoldingCids",
                        rt::ToValue::to_value(&self.input_holding_cids)
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
                    allocation: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "allocation",
                    )?)
                    .map_err(|e| e.at("allocation"))?,
                    pending_actions: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "pendingActions",
                    )?)
                    .map_err(|e| e.at("pendingActions"))?,
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
                    meta: rt::FromValue::from_value(rt::required_field(value, 5usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
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
                "275064aacfe99cea72ee0c80563936129563776f67415ef9f13e4297eecbc520";
            const PACKAGE_NAME: &'static str = "splice-api-token-allocation-instruction-v1";
            const MODULE_NAME: &'static str = "Splice.Api.Token.AllocationInstructionV1";
            const ENTITY_NAME: &'static str = "AllocationFactory";
        }
        impl rt::Interface for AllocationFactory {
            type View = crate::splice_api_token_allocation_instruction_v1::Splice_Api_Token_AllocationInstructionV1::AllocationFactoryView;
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
        for crate::splice_api_token_allocation_instruction_v1::Splice_Api_Token_AllocationInstructionV1::AllocationFactory_Allocate {
            type Return = crate::splice_api_token_allocation_instruction_v1::Splice_Api_Token_AllocationInstructionV1::AllocationInstructionResult;
            const NAME: &'static str = "AllocationFactory_Allocate";
            const CONSUMING: bool = false;
        }
        ///The `AllocationFactory_PublicFetch` choice on [`AllocationFactory`] (non-consuming).
        impl rt::Choice<AllocationFactory>
        for crate::splice_api_token_allocation_instruction_v1::Splice_Api_Token_AllocationInstructionV1::AllocationFactory_PublicFetch {
            type Return = crate::splice_api_token_allocation_instruction_v1::Splice_Api_Token_AllocationInstructionV1::AllocationFactoryView;
            const NAME: &'static str = "AllocationFactory_PublicFetch";
            const CONSUMING: bool = false;
        }
        impl rt::Contract for AllocationInstruction {
            const PACKAGE_ID: &'static str =
                "275064aacfe99cea72ee0c80563936129563776f67415ef9f13e4297eecbc520";
            const PACKAGE_NAME: &'static str = "splice-api-token-allocation-instruction-v1";
            const MODULE_NAME: &'static str = "Splice.Api.Token.AllocationInstructionV1";
            const ENTITY_NAME: &'static str = "AllocationInstruction";
        }
        impl rt::Interface for AllocationInstruction {
            type View = crate::splice_api_token_allocation_instruction_v1::Splice_Api_Token_AllocationInstructionV1::AllocationInstructionView;
        }
        ///The `Archive` choice on [`AllocationInstruction`] (consuming).
        impl rt::Choice<AllocationInstruction>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `AllocationInstruction_Withdraw` choice on [`AllocationInstruction`] (consuming).
        impl rt::Choice<AllocationInstruction>
        for crate::splice_api_token_allocation_instruction_v1::Splice_Api_Token_AllocationInstructionV1::AllocationInstruction_Withdraw {
            type Return = crate::splice_api_token_allocation_instruction_v1::Splice_Api_Token_AllocationInstructionV1::AllocationInstructionResult;
            const NAME: &'static str = "AllocationInstruction_Withdraw";
            const CONSUMING: bool = true;
        }
        ///The `AllocationInstruction_Update` choice on [`AllocationInstruction`] (consuming).
        impl rt::Choice<AllocationInstruction>
        for crate::splice_api_token_allocation_instruction_v1::Splice_Api_Token_AllocationInstructionV1::AllocationInstruction_Update {
            type Return = crate::splice_api_token_allocation_instruction_v1::Splice_Api_Token_AllocationInstructionV1::AllocationInstructionResult;
            const NAME: &'static str = "AllocationInstruction_Update";
            const CONSUMING: bool = true;
        }
    }
}
