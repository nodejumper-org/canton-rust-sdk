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
//! - [`splice_api_token_burn_mint_v1`] — 1 Daml module

pub mod splice_api_token_burn_mint_v1 {
    pub mod Splice_Api_Token_BurnMintV1 {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct BurnMintFactoryView {
            #[serde(rename = "admin")]
            pub admin: rt::Party,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for BurnMintFactoryView {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("admin", rt::ToValue::to_value(&self.admin)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for BurnMintFactoryView {
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
        pub struct BurnMintFactory_BurnMintResult {
            ///Daml field `outputCids`.
            #[serde(rename = "outputCids")]
            pub output_cids: ::std::vec::Vec<
                rt::ContractId<
                    ::canton_splice_api_token_holding_v1::splice_api_token_holding_v1::Splice_Api_Token_HoldingV1::Holding,
                >,
            >,
        }
        impl rt::ToValue for BurnMintFactory_BurnMintResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "outputCids",
                    rt::ToValue::to_value(&self.output_cids)
                ),])
            }
        }
        impl rt::FromValue for BurnMintFactory_BurnMintResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    output_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "outputCids",
                    )?)
                    .map_err(|e| e.at("outputCids"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct BurnMintOutput {
            #[serde(rename = "owner")]
            pub owner: rt::Party,
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            #[serde(rename = "context")]
            pub context: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ChoiceContext,
        }
        impl rt::ToValue for BurnMintOutput {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("owner", rt::ToValue::to_value(&self.owner)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                    ("context", rt::ToValue::to_value(&self.context)),
                ])
            }
        }
        impl rt::FromValue for BurnMintOutput {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    owner: rt::FromValue::from_value(rt::required_field(value, 0usize, "owner")?)
                        .map_err(|e| e.at("owner"))?,
                    amount: rt::FromValue::from_value(rt::required_field(value, 1usize, "amount")?)
                        .map_err(|e| e.at("amount"))?,
                    context: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "context",
                    )?)
                    .map_err(|e| e.at("context"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct BurnMintFactory_PublicFetch {
            ///Daml field `expectedAdmin`.
            #[serde(rename = "expectedAdmin")]
            pub expected_admin: rt::Party,
            #[serde(rename = "actor")]
            pub actor: rt::Party,
        }
        impl rt::ToValue for BurnMintFactory_PublicFetch {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("expectedAdmin", rt::ToValue::to_value(&self.expected_admin)),
                    ("actor", rt::ToValue::to_value(&self.actor)),
                ])
            }
        }
        impl rt::FromValue for BurnMintFactory_PublicFetch {
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
        pub struct BurnMintFactory_BurnMint {
            ///Daml field `expectedAdmin`.
            #[serde(rename = "expectedAdmin")]
            pub expected_admin: rt::Party,
            ///Daml field `instrumentId`.
            #[serde(rename = "instrumentId")]
            pub instrument_id: ::canton_splice_api_token_holding_v1::splice_api_token_holding_v1::Splice_Api_Token_HoldingV1::InstrumentId,
            ///Daml field `inputHoldingCids`.
            #[serde(rename = "inputHoldingCids")]
            pub input_holding_cids: ::std::vec::Vec<
                rt::ContractId<
                    ::canton_splice_api_token_holding_v1::splice_api_token_holding_v1::Splice_Api_Token_HoldingV1::Holding,
                >,
            >,
            #[serde(rename = "outputs")]
            pub outputs: ::std::vec::Vec<
                crate::splice_api_token_burn_mint_v1::Splice_Api_Token_BurnMintV1::BurnMintOutput,
            >,
            ///Daml field `extraActors`.
            #[serde(rename = "extraActors")]
            pub extra_actors: ::std::vec::Vec<rt::Party>,
            ///Daml field `extraArgs`.
            #[serde(rename = "extraArgs")]
            pub extra_args: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for BurnMintFactory_BurnMint {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("expectedAdmin", rt::ToValue::to_value(&self.expected_admin)),
                    ("instrumentId", rt::ToValue::to_value(&self.instrument_id)),
                    (
                        "inputHoldingCids",
                        rt::ToValue::to_value(&self.input_holding_cids)
                    ),
                    ("outputs", rt::ToValue::to_value(&self.outputs)),
                    ("extraActors", rt::ToValue::to_value(&self.extra_actors)),
                    ("extraArgs", rt::ToValue::to_value(&self.extra_args)),
                ])
            }
        }
        impl rt::FromValue for BurnMintFactory_BurnMint {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    expected_admin: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "expectedAdmin",
                    )?)
                    .map_err(|e| e.at("expectedAdmin"))?,
                    instrument_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "instrumentId",
                    )?)
                    .map_err(|e| e.at("instrumentId"))?,
                    input_holding_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "inputHoldingCids",
                    )?)
                    .map_err(|e| e.at("inputHoldingCids"))?,
                    outputs: rt::FromValue::from_value(rt::required_field(
                        value, 3usize, "outputs",
                    )?)
                    .map_err(|e| e.at("outputs"))?,
                    extra_actors: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "extraActors",
                    )?)
                    .map_err(|e| e.at("extraActors"))?,
                    extra_args: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "extraArgs",
                    )?)
                    .map_err(|e| e.at("extraArgs"))?,
                })
            }
        }
        ///Marker for the Daml interface `BurnMintFactory` (held via `ContractId`).
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct BurnMintFactory;
        impl rt::Contract for BurnMintFactory {
            const PACKAGE_ID: &'static str =
                "9cc2cbc838ef38dc2c7f34014c9c452bcf71b8e2a4f939235fc0b5d0924b185e";
            const PACKAGE_NAME: &'static str = "splice-api-token-burn-mint-v1";
            const MODULE_NAME: &'static str = "Splice.Api.Token.BurnMintV1";
            const ENTITY_NAME: &'static str = "BurnMintFactory";
        }
        impl rt::Interface for BurnMintFactory {
            type View = crate::splice_api_token_burn_mint_v1::Splice_Api_Token_BurnMintV1::BurnMintFactoryView;
        }
        ///The `Archive` choice on [`BurnMintFactory`] (consuming).
        impl rt::Choice<BurnMintFactory>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `BurnMintFactory_PublicFetch` choice on [`BurnMintFactory`] (non-consuming).
        impl rt::Choice<BurnMintFactory>
        for crate::splice_api_token_burn_mint_v1::Splice_Api_Token_BurnMintV1::BurnMintFactory_PublicFetch {
            type Return = crate::splice_api_token_burn_mint_v1::Splice_Api_Token_BurnMintV1::BurnMintFactoryView;
            const NAME: &'static str = "BurnMintFactory_PublicFetch";
            const CONSUMING: bool = false;
        }
        ///The `BurnMintFactory_BurnMint` choice on [`BurnMintFactory`] (non-consuming).
        impl rt::Choice<BurnMintFactory>
        for crate::splice_api_token_burn_mint_v1::Splice_Api_Token_BurnMintV1::BurnMintFactory_BurnMint {
            type Return = crate::splice_api_token_burn_mint_v1::Splice_Api_Token_BurnMintV1::BurnMintFactory_BurnMintResult;
            const NAME: &'static str = "BurnMintFactory_BurnMint";
            const CONSUMING: bool = false;
        }
    }
}
