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
//! - [`splice_api_token_metadata_v1`] — 1 Daml module

pub mod splice_api_token_metadata_v1 {
    pub mod Splice_Api_Token_MetadataV1 {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ChoiceExecutionMetadata {
            #[serde(rename = "meta")]
            pub meta: crate::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for ChoiceExecutionMetadata {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("meta", rt::ToValue::to_value(&self.meta)),])
            }
        }
        impl rt::FromValue for ChoiceExecutionMetadata {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    meta: rt::FromValue::from_value(rt::required_field(value, 0usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ExtraArgs {
            #[serde(rename = "context")]
            pub context:
                crate::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ChoiceContext,
            #[serde(rename = "meta")]
            pub meta: crate::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for ExtraArgs {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("context", rt::ToValue::to_value(&self.context)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for ExtraArgs {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    context: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "context",
                    )?)
                    .map_err(|e| e.at("context"))?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 1usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Metadata {
            #[serde(rename = "values")]
            pub values: rt::TextMap<::std::string::String>,
        }
        impl rt::ToValue for Metadata {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("values", rt::ToValue::to_value(&self.values)),])
            }
        }
        impl rt::FromValue for Metadata {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    values: rt::FromValue::from_value(rt::required_field(value, 0usize, "values")?)
                        .map_err(|e| e.at("values"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ChoiceContext {
            #[serde(rename = "values")]
            pub values: rt::TextMap<
                crate::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::AnyValue,
            >,
        }
        impl rt::ToValue for ChoiceContext {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("values", rt::ToValue::to_value(&self.values)),])
            }
        }
        impl rt::FromValue for ChoiceContext {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    values: rt::FromValue::from_value(rt::required_field(value, 0usize, "values")?)
                        .map_err(|e| e.at("values"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AnyContractView {}
        impl rt::ToValue for AnyContractView {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for AnyContractView {
            fn from_value(_value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum AnyValue {
            #[serde(rename = "AV_Text")]
            AV_Text(::std::string::String),
            #[serde(rename = "AV_Int")]
            AV_Int(rt::Int64),
            #[serde(rename = "AV_Decimal")]
            AV_Decimal(rt::Numeric),
            #[serde(rename = "AV_Bool")]
            AV_Bool(bool),
            #[serde(rename = "AV_Date")]
            AV_Date(rt::Date),
            #[serde(rename = "AV_Time")]
            AV_Time(rt::Timestamp),
            #[serde(rename = "AV_RelTime")]
            AV_RelTime(::canton_daml_stdlib::daml_stdlib_DA_Time_Types::DA_Time_Types::RelTime),
            #[serde(rename = "AV_Party")]
            AV_Party(rt::Party),
            #[serde(rename = "AV_ContractId")]
            AV_ContractId(
                rt::ContractId<
                    crate::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::AnyContract,
                >,
            ),
            #[serde(rename = "AV_List")]
            AV_List(
                ::std::vec::Vec<
                    crate::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::AnyValue,
                >,
            ),
            #[serde(rename = "AV_Map")]
            AV_Map(
                rt::TextMap<
                    crate::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::AnyValue,
                >,
            ),
        }
        impl rt::ToValue for AnyValue {
            fn to_value(&self) -> rt::Value {
                match self {
                    AnyValue::AV_Text(inner) => {
                        rt::variant_value("AV_Text", rt::ToValue::to_value(inner))
                    }
                    AnyValue::AV_Int(inner) => {
                        rt::variant_value("AV_Int", rt::ToValue::to_value(inner))
                    }
                    AnyValue::AV_Decimal(inner) => {
                        rt::variant_value("AV_Decimal", rt::ToValue::to_value(inner))
                    }
                    AnyValue::AV_Bool(inner) => {
                        rt::variant_value("AV_Bool", rt::ToValue::to_value(inner))
                    }
                    AnyValue::AV_Date(inner) => {
                        rt::variant_value("AV_Date", rt::ToValue::to_value(inner))
                    }
                    AnyValue::AV_Time(inner) => {
                        rt::variant_value("AV_Time", rt::ToValue::to_value(inner))
                    }
                    AnyValue::AV_RelTime(inner) => {
                        rt::variant_value("AV_RelTime", rt::ToValue::to_value(inner))
                    }
                    AnyValue::AV_Party(inner) => {
                        rt::variant_value("AV_Party", rt::ToValue::to_value(inner))
                    }
                    AnyValue::AV_ContractId(inner) => {
                        rt::variant_value("AV_ContractId", rt::ToValue::to_value(inner))
                    }
                    AnyValue::AV_List(inner) => {
                        rt::variant_value("AV_List", rt::ToValue::to_value(inner))
                    }
                    AnyValue::AV_Map(inner) => {
                        rt::variant_value("AV_Map", rt::ToValue::to_value(inner))
                    }
                }
            }
        }
        impl rt::FromValue for AnyValue {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                let (constructor, payload) = rt::variant_parts(value)?;
                match constructor {
                    "AV_Text" => ::core::result::Result::Ok(AnyValue::AV_Text(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("AV_Text"))?,
                    )),
                    "AV_Int" => ::core::result::Result::Ok(AnyValue::AV_Int(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("AV_Int"))?,
                    )),
                    "AV_Decimal" => ::core::result::Result::Ok(AnyValue::AV_Decimal(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("AV_Decimal"))?,
                    )),
                    "AV_Bool" => ::core::result::Result::Ok(AnyValue::AV_Bool(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("AV_Bool"))?,
                    )),
                    "AV_Date" => ::core::result::Result::Ok(AnyValue::AV_Date(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("AV_Date"))?,
                    )),
                    "AV_Time" => ::core::result::Result::Ok(AnyValue::AV_Time(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("AV_Time"))?,
                    )),
                    "AV_RelTime" => ::core::result::Result::Ok(AnyValue::AV_RelTime(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("AV_RelTime"))?,
                    )),
                    "AV_Party" => ::core::result::Result::Ok(AnyValue::AV_Party(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("AV_Party"))?,
                    )),
                    "AV_ContractId" => ::core::result::Result::Ok(AnyValue::AV_ContractId(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("AV_ContractId"))?,
                    )),
                    "AV_List" => ::core::result::Result::Ok(AnyValue::AV_List(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("AV_List"))?,
                    )),
                    "AV_Map" => ::core::result::Result::Ok(AnyValue::AV_Map(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("AV_Map"))?,
                    )),
                    other => {
                        ::core::result::Result::Err(rt::unexpected_constructor("AnyValue", other))
                    }
                }
            }
        }
        ///Marker for the Daml interface `AnyContract` (held via `ContractId`).
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct AnyContract;
        impl rt::Contract for AnyContract {
            const PACKAGE_ID: &'static str =
                "4ded6b668cb3b64f7a88a30874cd41c75829f5e064b3fbbadf41ec7e8363354f";
            const PACKAGE_NAME: &'static str = "splice-api-token-metadata-v1";
            const MODULE_NAME: &'static str = "Splice.Api.Token.MetadataV1";
            const ENTITY_NAME: &'static str = "AnyContract";
        }
        impl rt::Interface for AnyContract {
            type View =
                crate::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::AnyContractView;
        }
        ///The `Archive` choice on [`AnyContract`] (consuming).
        impl rt::Choice<AnyContract>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
    }
}
