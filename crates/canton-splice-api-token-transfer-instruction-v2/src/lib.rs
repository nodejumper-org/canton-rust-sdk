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
//! - [`splice_api_token_transfer_instruction_v2`] — 1 Daml module

pub mod splice_api_token_transfer_instruction_v2 {
    pub mod Splice_Api_Token_TransferInstructionV2 {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferFactoryView {
            #[serde(rename = "admin")]
            pub admin: rt::Party,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for TransferFactoryView {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("admin", rt::ToValue::to_value(&self.admin)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for TransferFactoryView {
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
        pub struct TransferFactory_PublicFetch {
            #[serde(rename = "actors")]
            pub actors: ::std::vec::Vec<rt::Party>,
        }
        impl rt::ToValue for TransferFactory_PublicFetch {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("actors", rt::ToValue::to_value(&self.actors)),])
            }
        }
        impl rt::FromValue for TransferFactory_PublicFetch {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    actors: rt::FromValue::from_value(rt::required_field(value, 0usize, "actors")?)
                        .map_err(|e| e.at("actors"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferFactory_Transfer {
            #[serde(rename = "transfer")]
            pub transfer: crate::splice_api_token_transfer_instruction_v2::Splice_Api_Token_TransferInstructionV2::Transfer,
            #[serde(rename = "actors")]
            pub actors: ::std::vec::Vec<rt::Party>,
            ///Daml field `extraArgs`.
            #[serde(rename = "extraArgs")]
            pub extra_args: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for TransferFactory_Transfer {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("transfer", rt::ToValue::to_value(&self.transfer)),
                    ("actors", rt::ToValue::to_value(&self.actors)),
                    ("extraArgs", rt::ToValue::to_value(&self.extra_args)),
                ])
            }
        }
        impl rt::FromValue for TransferFactory_Transfer {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    transfer: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "transfer",
                    )?)
                    .map_err(|e| e.at("transfer"))?,
                    actors: rt::FromValue::from_value(rt::required_field(value, 1usize, "actors")?)
                        .map_err(|e| e.at("actors"))?,
                    extra_args: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "extraArgs",
                    )?)
                    .map_err(|e| e.at("extraArgs"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferInstruction_Withdraw {
            #[serde(rename = "actors")]
            pub actors: ::std::vec::Vec<rt::Party>,
            ///Daml field `extraArgs`.
            #[serde(rename = "extraArgs")]
            pub extra_args: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for TransferInstruction_Withdraw {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("actors", rt::ToValue::to_value(&self.actors)),
                    ("extraArgs", rt::ToValue::to_value(&self.extra_args)),
                ])
            }
        }
        impl rt::FromValue for TransferInstruction_Withdraw {
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
        pub struct TransferInstruction_Reject {
            #[serde(rename = "actors")]
            pub actors: ::std::vec::Vec<rt::Party>,
            ///Daml field `extraArgs`.
            #[serde(rename = "extraArgs")]
            pub extra_args: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for TransferInstruction_Reject {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("actors", rt::ToValue::to_value(&self.actors)),
                    ("extraArgs", rt::ToValue::to_value(&self.extra_args)),
                ])
            }
        }
        impl rt::FromValue for TransferInstruction_Reject {
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
        pub struct TransferInstruction_Accept {
            #[serde(rename = "actors")]
            pub actors: ::std::vec::Vec<rt::Party>,
            ///Daml field `extraArgs`.
            #[serde(rename = "extraArgs")]
            pub extra_args: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for TransferInstruction_Accept {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("actors", rt::ToValue::to_value(&self.actors)),
                    ("extraArgs", rt::ToValue::to_value(&self.extra_args)),
                ])
            }
        }
        impl rt::FromValue for TransferInstruction_Accept {
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
        pub struct TransferInstructionView {
            ///Daml field `originalInstructionCid`.
            #[serde(rename = "originalInstructionCid")]
            pub original_instruction_cid: ::core::option::Option<
                rt::ContractId<
                    crate::splice_api_token_transfer_instruction_v2::Splice_Api_Token_TransferInstructionV2::TransferInstruction,
                >,
            >,
            #[serde(rename = "transfer")]
            pub transfer: crate::splice_api_token_transfer_instruction_v2::Splice_Api_Token_TransferInstructionV2::Transfer,
            ///Daml field `expiresAt`.
            #[serde(rename = "expiresAt")]
            pub expires_at: ::core::option::Option<rt::Timestamp>,
            ///Daml field `availableActions`.
            #[serde(rename = "availableActions")]
            pub available_actions: rt::GenMap<
                crate::splice_api_token_transfer_instruction_v2::Splice_Api_Token_TransferInstructionV2::TransferInstructionAction,
                ::std::vec::Vec<::std::vec::Vec<rt::Party>>,
            >,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for TransferInstructionView {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "originalInstructionCid",
                        rt::ToValue::to_value(&self.original_instruction_cid)
                    ),
                    ("transfer", rt::ToValue::to_value(&self.transfer)),
                    ("expiresAt", rt::ToValue::to_value(&self.expires_at)),
                    (
                        "availableActions",
                        rt::ToValue::to_value(&self.available_actions)
                    ),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for TransferInstructionView {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    original_instruction_cid: rt::optional_field(
                        value,
                        0usize,
                        "originalInstructionCid",
                    )
                    .map_err(|e| e.at("originalInstructionCid"))?,
                    transfer: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "transfer",
                    )?)
                    .map_err(|e| e.at("transfer"))?,
                    expires_at: rt::optional_field(value, 2usize, "expiresAt")
                        .map_err(|e| e.at("expiresAt"))?,
                    available_actions: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "availableActions",
                    )?)
                    .map_err(|e| e.at("availableActions"))?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 4usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum TransferInstructionAction {
            #[serde(rename = "TIA_Accept")]
            TIA_Accept(rt::Unit),
            #[serde(rename = "TIA_Reject")]
            TIA_Reject(rt::Unit),
            #[serde(rename = "TIA_Withdraw")]
            TIA_Withdraw(rt::Unit),
            #[serde(rename = "TIA_Custom")]
            TIA_Custom(
                crate::splice_api_token_transfer_instruction_v2::Splice_Api_Token_TransferInstructionV2::TransferInstructionAction_TIA_Custom,
            ),
        }
        impl rt::ToValue for TransferInstructionAction {
            fn to_value(&self) -> rt::Value {
                match self {
                    TransferInstructionAction::TIA_Accept(inner) => {
                        rt::variant_value("TIA_Accept", rt::ToValue::to_value(inner))
                    }
                    TransferInstructionAction::TIA_Reject(inner) => {
                        rt::variant_value("TIA_Reject", rt::ToValue::to_value(inner))
                    }
                    TransferInstructionAction::TIA_Withdraw(inner) => {
                        rt::variant_value("TIA_Withdraw", rt::ToValue::to_value(inner))
                    }
                    TransferInstructionAction::TIA_Custom(inner) => {
                        rt::variant_value("TIA_Custom", rt::ToValue::to_value(inner))
                    }
                }
            }
        }
        impl rt::FromValue for TransferInstructionAction {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                let (constructor, payload) = rt::variant_parts(value)?;
                match constructor {
                    "TIA_Accept" => {
                        ::core::result::Result::Ok(TransferInstructionAction::TIA_Accept(
                            rt::FromValue::from_value(payload).map_err(|e| e.at("TIA_Accept"))?,
                        ))
                    }
                    "TIA_Reject" => {
                        ::core::result::Result::Ok(TransferInstructionAction::TIA_Reject(
                            rt::FromValue::from_value(payload).map_err(|e| e.at("TIA_Reject"))?,
                        ))
                    }
                    "TIA_Withdraw" => {
                        ::core::result::Result::Ok(TransferInstructionAction::TIA_Withdraw(
                            rt::FromValue::from_value(payload).map_err(|e| e.at("TIA_Withdraw"))?,
                        ))
                    }
                    "TIA_Custom" => {
                        ::core::result::Result::Ok(TransferInstructionAction::TIA_Custom(
                            rt::FromValue::from_value(payload).map_err(|e| e.at("TIA_Custom"))?,
                        ))
                    }
                    other => ::core::result::Result::Err(rt::unexpected_constructor(
                        "TransferInstructionAction",
                        other,
                    )),
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferInstructionAction_TIA_Custom {
            #[serde(rename = "id")]
            pub id: ::std::string::String,
        }
        impl rt::ToValue for TransferInstructionAction_TIA_Custom {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("id", rt::ToValue::to_value(&self.id)),])
            }
        }
        impl rt::FromValue for TransferInstructionAction_TIA_Custom {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    id: rt::FromValue::from_value(rt::required_field(value, 0usize, "id")?)
                        .map_err(|e| e.at("id"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum TransferInstructionResult_Output {
            #[serde(rename = "TransferInstructionResult_Pending")]
            TransferInstructionResult_Pending(
                crate::splice_api_token_transfer_instruction_v2::Splice_Api_Token_TransferInstructionV2::TransferInstructionResult_Output_TransferInstructionResult_Pending,
            ),
            #[serde(rename = "TransferInstructionResult_Completed")]
            TransferInstructionResult_Completed(
                crate::splice_api_token_transfer_instruction_v2::Splice_Api_Token_TransferInstructionV2::TransferInstructionResult_Output_TransferInstructionResult_Completed,
            ),
            #[serde(rename = "TransferInstructionResult_Failed")]
            TransferInstructionResult_Failed(rt::Unit),
        }
        impl rt::ToValue for TransferInstructionResult_Output {
            fn to_value(&self) -> rt::Value {
                match self {
                    TransferInstructionResult_Output::TransferInstructionResult_Pending(inner) => {
                        rt::variant_value(
                            "TransferInstructionResult_Pending",
                            rt::ToValue::to_value(inner),
                        )
                    }
                    TransferInstructionResult_Output::TransferInstructionResult_Completed(
                        inner,
                    ) => rt::variant_value(
                        "TransferInstructionResult_Completed",
                        rt::ToValue::to_value(inner),
                    ),
                    TransferInstructionResult_Output::TransferInstructionResult_Failed(inner) => {
                        rt::variant_value(
                            "TransferInstructionResult_Failed",
                            rt::ToValue::to_value(inner),
                        )
                    }
                }
            }
        }
        impl rt::FromValue for TransferInstructionResult_Output {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                let (constructor, payload) = rt::variant_parts(value)?;
                match constructor {
                    "TransferInstructionResult_Pending" => ::core::result::Result::Ok(
                        TransferInstructionResult_Output::TransferInstructionResult_Pending(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("TransferInstructionResult_Pending"))?,
                        ),
                    ),
                    "TransferInstructionResult_Completed" => ::core::result::Result::Ok(
                        TransferInstructionResult_Output::TransferInstructionResult_Completed(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("TransferInstructionResult_Completed"))?,
                        ),
                    ),
                    "TransferInstructionResult_Failed" => ::core::result::Result::Ok(
                        TransferInstructionResult_Output::TransferInstructionResult_Failed(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("TransferInstructionResult_Failed"))?,
                        ),
                    ),
                    other => ::core::result::Result::Err(rt::unexpected_constructor(
                        "TransferInstructionResult_Output",
                        other,
                    )),
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferInstructionResult_Output_TransferInstructionResult_Pending {
            ///Daml field `transferInstructionCid`.
            #[serde(rename = "transferInstructionCid")]
            pub transfer_instruction_cid: rt::ContractId<
                crate::splice_api_token_transfer_instruction_v2::Splice_Api_Token_TransferInstructionV2::TransferInstruction,
            >,
        }
        impl rt::ToValue for TransferInstructionResult_Output_TransferInstructionResult_Pending {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "transferInstructionCid",
                    rt::ToValue::to_value(&self.transfer_instruction_cid)
                ),])
            }
        }
        impl rt::FromValue for TransferInstructionResult_Output_TransferInstructionResult_Pending {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    transfer_instruction_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "transferInstructionCid",
                    )?)
                    .map_err(|e| e.at("transferInstructionCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferInstructionResult_Output_TransferInstructionResult_Completed {
            ///Daml field `receiverHoldingCids`.
            #[serde(rename = "receiverHoldingCids")]
            pub receiver_holding_cids: ::std::vec::Vec<
                rt::ContractId<
                    ::canton_splice_api_token_holding_v2::splice_api_token_holding_v2::Splice_Api_Token_HoldingV2::Holding,
                >,
            >,
        }
        impl rt::ToValue for TransferInstructionResult_Output_TransferInstructionResult_Completed {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "receiverHoldingCids",
                    rt::ToValue::to_value(&self.receiver_holding_cids)
                ),])
            }
        }
        impl rt::FromValue for TransferInstructionResult_Output_TransferInstructionResult_Completed {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    receiver_holding_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "receiverHoldingCids",
                    )?)
                    .map_err(|e| e.at("receiverHoldingCids"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferInstructionResult {
            #[serde(rename = "output")]
            pub output: crate::splice_api_token_transfer_instruction_v2::Splice_Api_Token_TransferInstructionV2::TransferInstructionResult_Output,
            ///Daml field `senderChangeCids`.
            #[serde(rename = "senderChangeCids")]
            pub sender_change_cids: ::std::vec::Vec<
                rt::ContractId<
                    ::canton_splice_api_token_holding_v2::splice_api_token_holding_v2::Splice_Api_Token_HoldingV2::Holding,
                >,
            >,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for TransferInstructionResult {
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
        impl rt::FromValue for TransferInstructionResult {
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
        pub struct Transfer {
            #[serde(rename = "sender")]
            pub sender: ::canton_splice_api_token_holding_v2::splice_api_token_holding_v2::Splice_Api_Token_HoldingV2::Account,
            #[serde(rename = "receiver")]
            pub receiver: ::canton_splice_api_token_holding_v2::splice_api_token_holding_v2::Splice_Api_Token_HoldingV2::Account,
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            ///Daml field `instrumentId`.
            #[serde(rename = "instrumentId")]
            pub instrument_id: ::canton_splice_api_token_holding_v2::splice_api_token_holding_v2::Splice_Api_Token_HoldingV2::InstrumentId,
            ///Daml field `requestedAt`.
            #[serde(rename = "requestedAt")]
            pub requested_at: rt::Timestamp,
            ///Daml field `executeBefore`.
            #[serde(rename = "executeBefore")]
            pub execute_before: rt::Timestamp,
            ///Daml field `inputHoldingCids`.
            #[serde(rename = "inputHoldingCids")]
            pub input_holding_cids: ::std::vec::Vec<
                rt::ContractId<
                    ::canton_splice_api_token_holding_v2::splice_api_token_holding_v2::Splice_Api_Token_HoldingV2::Holding,
                >,
            >,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for Transfer {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("sender", rt::ToValue::to_value(&self.sender)),
                    ("receiver", rt::ToValue::to_value(&self.receiver)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                    ("instrumentId", rt::ToValue::to_value(&self.instrument_id)),
                    ("requestedAt", rt::ToValue::to_value(&self.requested_at)),
                    ("executeBefore", rt::ToValue::to_value(&self.execute_before)),
                    (
                        "inputHoldingCids",
                        rt::ToValue::to_value(&self.input_holding_cids)
                    ),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for Transfer {
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
                    requested_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "requestedAt",
                    )?)
                    .map_err(|e| e.at("requestedAt"))?,
                    execute_before: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "executeBefore",
                    )?)
                    .map_err(|e| e.at("executeBefore"))?,
                    input_holding_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        6usize,
                        "inputHoldingCids",
                    )?)
                    .map_err(|e| e.at("inputHoldingCids"))?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 7usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        ///Marker for the Daml interface `TransferFactory` (held via `ContractId`).
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct TransferFactory;
        ///Marker for the Daml interface `TransferInstruction` (held via `ContractId`).
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct TransferInstruction;
        impl rt::Contract for TransferFactory {
            const PACKAGE_ID: &'static str =
                "29317e3b7b165d2bbf16721bcca0ec4869e53eddb2738bddf790d61af28e0099";
            const PACKAGE_NAME: &'static str = "splice-api-token-transfer-instruction-v2";
            const MODULE_NAME: &'static str = "Splice.Api.Token.TransferInstructionV2";
            const ENTITY_NAME: &'static str = "TransferFactory";
        }
        impl rt::Interface for TransferFactory {
            type View = crate::splice_api_token_transfer_instruction_v2::Splice_Api_Token_TransferInstructionV2::TransferFactoryView;
        }
        ///The `Archive` choice on [`TransferFactory`] (consuming).
        impl rt::Choice<TransferFactory>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `TransferFactory_Transfer` choice on [`TransferFactory`] (non-consuming).
        impl rt::Choice<TransferFactory>
        for crate::splice_api_token_transfer_instruction_v2::Splice_Api_Token_TransferInstructionV2::TransferFactory_Transfer {
            type Return = crate::splice_api_token_transfer_instruction_v2::Splice_Api_Token_TransferInstructionV2::TransferInstructionResult;
            const NAME: &'static str = "TransferFactory_Transfer";
            const CONSUMING: bool = false;
        }
        ///The `TransferFactory_PublicFetch` choice on [`TransferFactory`] (non-consuming).
        impl rt::Choice<TransferFactory>
        for crate::splice_api_token_transfer_instruction_v2::Splice_Api_Token_TransferInstructionV2::TransferFactory_PublicFetch {
            type Return = crate::splice_api_token_transfer_instruction_v2::Splice_Api_Token_TransferInstructionV2::TransferFactoryView;
            const NAME: &'static str = "TransferFactory_PublicFetch";
            const CONSUMING: bool = false;
        }
        impl rt::Contract for TransferInstruction {
            const PACKAGE_ID: &'static str =
                "29317e3b7b165d2bbf16721bcca0ec4869e53eddb2738bddf790d61af28e0099";
            const PACKAGE_NAME: &'static str = "splice-api-token-transfer-instruction-v2";
            const MODULE_NAME: &'static str = "Splice.Api.Token.TransferInstructionV2";
            const ENTITY_NAME: &'static str = "TransferInstruction";
        }
        impl rt::Interface for TransferInstruction {
            type View = crate::splice_api_token_transfer_instruction_v2::Splice_Api_Token_TransferInstructionV2::TransferInstructionView;
        }
        ///The `Archive` choice on [`TransferInstruction`] (consuming).
        impl rt::Choice<TransferInstruction>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `TransferInstruction_Accept` choice on [`TransferInstruction`] (non-consuming).
        impl rt::Choice<TransferInstruction>
        for crate::splice_api_token_transfer_instruction_v2::Splice_Api_Token_TransferInstructionV2::TransferInstruction_Accept {
            type Return = crate::splice_api_token_transfer_instruction_v2::Splice_Api_Token_TransferInstructionV2::TransferInstructionResult;
            const NAME: &'static str = "TransferInstruction_Accept";
            const CONSUMING: bool = false;
        }
        ///The `TransferInstruction_Reject` choice on [`TransferInstruction`] (non-consuming).
        impl rt::Choice<TransferInstruction>
        for crate::splice_api_token_transfer_instruction_v2::Splice_Api_Token_TransferInstructionV2::TransferInstruction_Reject {
            type Return = crate::splice_api_token_transfer_instruction_v2::Splice_Api_Token_TransferInstructionV2::TransferInstructionResult;
            const NAME: &'static str = "TransferInstruction_Reject";
            const CONSUMING: bool = false;
        }
        ///The `TransferInstruction_Withdraw` choice on [`TransferInstruction`] (non-consuming).
        impl rt::Choice<TransferInstruction>
        for crate::splice_api_token_transfer_instruction_v2::Splice_Api_Token_TransferInstructionV2::TransferInstruction_Withdraw {
            type Return = crate::splice_api_token_transfer_instruction_v2::Splice_Api_Token_TransferInstructionV2::TransferInstructionResult;
            const NAME: &'static str = "TransferInstruction_Withdraw";
            const CONSUMING: bool = false;
        }
    }
}
