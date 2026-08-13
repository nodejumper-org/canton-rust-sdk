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
//! - [`splice_api_featured_app_v1`] — 1 Daml module

pub mod splice_api_featured_app_v1 {
    pub mod Splice_Api_FeaturedAppRightV1 {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct FeaturedAppActivityMarkerView {
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            #[serde(rename = "provider")]
            pub provider: rt::Party,
            #[serde(rename = "beneficiary")]
            pub beneficiary: rt::Party,
            #[serde(rename = "weight")]
            pub weight: rt::Numeric,
        }
        impl rt::ToValue for FeaturedAppActivityMarkerView {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("provider", rt::ToValue::to_value(&self.provider)),
                    ("beneficiary", rt::ToValue::to_value(&self.beneficiary)),
                    ("weight", rt::ToValue::to_value(&self.weight)),
                ])
            }
        }
        impl rt::FromValue for FeaturedAppActivityMarkerView {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                    provider: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "provider",
                    )?)
                    .map_err(|e| e.at("provider"))?,
                    beneficiary: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "beneficiary",
                    )?)
                    .map_err(|e| e.at("beneficiary"))?,
                    weight: rt::FromValue::from_value(rt::required_field(value, 3usize, "weight")?)
                        .map_err(|e| e.at("weight"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct FeaturedAppRightView {
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            #[serde(rename = "provider")]
            pub provider: rt::Party,
        }
        impl rt::ToValue for FeaturedAppRightView {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("provider", rt::ToValue::to_value(&self.provider)),
                ])
            }
        }
        impl rt::FromValue for FeaturedAppRightView {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                    provider: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "provider",
                    )?)
                    .map_err(|e| e.at("provider"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct FeaturedAppRight_CreateActivityMarkerResult {
            ///Daml field `activityMarkerCids`.
            #[serde(rename = "activityMarkerCids")]
            pub activity_marker_cids: ::std::vec::Vec<
                rt::ContractId<
                    crate::splice_api_featured_app_v1::Splice_Api_FeaturedAppRightV1::FeaturedAppActivityMarker,
                >,
            >,
        }
        impl rt::ToValue for FeaturedAppRight_CreateActivityMarkerResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "activityMarkerCids",
                    rt::ToValue::to_value(&self.activity_marker_cids)
                ),])
            }
        }
        impl rt::FromValue for FeaturedAppRight_CreateActivityMarkerResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    activity_marker_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "activityMarkerCids",
                    )?)
                    .map_err(|e| e.at("activityMarkerCids"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct FeaturedAppRight_CreateActivityMarker {
            #[serde(rename = "beneficiaries")]
            pub beneficiaries: ::std::vec::Vec<
                crate::splice_api_featured_app_v1::Splice_Api_FeaturedAppRightV1::AppRewardBeneficiary,
            >,
        }
        impl rt::ToValue for FeaturedAppRight_CreateActivityMarker {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "beneficiaries",
                    rt::ToValue::to_value(&self.beneficiaries)
                ),])
            }
        }
        impl rt::FromValue for FeaturedAppRight_CreateActivityMarker {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    beneficiaries: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "beneficiaries",
                    )?)
                    .map_err(|e| e.at("beneficiaries"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AppRewardBeneficiary {
            #[serde(rename = "beneficiary")]
            pub beneficiary: rt::Party,
            #[serde(rename = "weight")]
            pub weight: rt::Numeric,
        }
        impl rt::ToValue for AppRewardBeneficiary {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("beneficiary", rt::ToValue::to_value(&self.beneficiary)),
                    ("weight", rt::ToValue::to_value(&self.weight)),
                ])
            }
        }
        impl rt::FromValue for AppRewardBeneficiary {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    beneficiary: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "beneficiary",
                    )?)
                    .map_err(|e| e.at("beneficiary"))?,
                    weight: rt::FromValue::from_value(rt::required_field(value, 1usize, "weight")?)
                        .map_err(|e| e.at("weight"))?,
                })
            }
        }
        ///Marker for the Daml interface `FeaturedAppActivityMarker` (held via `ContractId`).
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct FeaturedAppActivityMarker;
        ///Marker for the Daml interface `FeaturedAppRight` (held via `ContractId`).
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct FeaturedAppRight;
        impl rt::Contract for FeaturedAppActivityMarker {
            const PACKAGE_ID: &'static str =
                "7804375fe5e4c6d5afe067bd314c42fe0b7d005a1300019c73154dd939da4dda";
            const PACKAGE_NAME: &'static str = "splice-api-featured-app-v1";
            const MODULE_NAME: &'static str = "Splice.Api.FeaturedAppRightV1";
            const ENTITY_NAME: &'static str = "FeaturedAppActivityMarker";
        }
        impl rt::Interface for FeaturedAppActivityMarker {
            type View = crate::splice_api_featured_app_v1::Splice_Api_FeaturedAppRightV1::FeaturedAppActivityMarkerView;
        }
        ///The `Archive` choice on [`FeaturedAppActivityMarker`] (consuming).
        impl rt::Choice<FeaturedAppActivityMarker>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        impl rt::Contract for FeaturedAppRight {
            const PACKAGE_ID: &'static str =
                "7804375fe5e4c6d5afe067bd314c42fe0b7d005a1300019c73154dd939da4dda";
            const PACKAGE_NAME: &'static str = "splice-api-featured-app-v1";
            const MODULE_NAME: &'static str = "Splice.Api.FeaturedAppRightV1";
            const ENTITY_NAME: &'static str = "FeaturedAppRight";
        }
        impl rt::Interface for FeaturedAppRight {
            type View = crate::splice_api_featured_app_v1::Splice_Api_FeaturedAppRightV1::FeaturedAppRightView;
        }
        ///The `Archive` choice on [`FeaturedAppRight`] (consuming).
        impl rt::Choice<FeaturedAppRight>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `FeaturedAppRight_CreateActivityMarker` choice on [`FeaturedAppRight`] (non-consuming).
        impl rt::Choice<FeaturedAppRight>
        for crate::splice_api_featured_app_v1::Splice_Api_FeaturedAppRightV1::FeaturedAppRight_CreateActivityMarker {
            type Return = crate::splice_api_featured_app_v1::Splice_Api_FeaturedAppRightV1::FeaturedAppRight_CreateActivityMarkerResult;
            const NAME: &'static str = "FeaturedAppRight_CreateActivityMarker";
            const CONSUMING: bool = false;
        }
    }
}
