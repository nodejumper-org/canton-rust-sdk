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
//! - [`quickstart_licensing`] — 2 Daml modules

pub mod quickstart_licensing {
    pub mod Licensing_License {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct LicenseRenewalRequest_CompleteRenewal {
            ///Daml field `allocationCid`.
            #[serde(rename = "allocationCid")]
            pub allocation_cid: rt::ContractId<
                ::canton_splice_api_token_allocation_v1::splice_api_token_allocation_v1::Splice_Api_Token_AllocationV1::Allocation,
            >,
            ///Daml field `licenseCid`.
            #[serde(rename = "licenseCid")]
            pub license_cid: rt::ContractId<
                crate::quickstart_licensing::Licensing_License::License,
            >,
            ///Daml field `extraArgs`.
            #[serde(rename = "extraArgs")]
            pub extra_args: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for LicenseRenewalRequest_CompleteRenewal {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("allocationCid", rt::ToValue::to_value(&self.allocation_cid)),
                    ("licenseCid", rt::ToValue::to_value(&self.license_cid)),
                    ("extraArgs", rt::ToValue::to_value(&self.extra_args)),
                ])
            }
        }
        impl rt::FromValue for LicenseRenewalRequest_CompleteRenewal {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    allocation_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "allocationCid",
                    )?)
                    .map_err(|e| e.at("allocationCid"))?,
                    license_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "licenseCid",
                    )?)
                    .map_err(|e| e.at("licenseCid"))?,
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
        pub struct License_Expire {
            #[serde(rename = "actor")]
            pub actor: rt::Party,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for License_Expire {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("actor", rt::ToValue::to_value(&self.actor)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for License_Expire {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    actor: rt::FromValue::from_value(rt::required_field(value, 0usize, "actor")?)
                        .map_err(|e| e.at("actor"))?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 1usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct License_Renew {
            ///Daml field `requestId`.
            #[serde(rename = "requestId")]
            pub request_id: ::std::string::String,
            ///Daml field `licenseFeeInstrumentId`.
            #[serde(rename = "licenseFeeInstrumentId")]
            pub license_fee_instrument_id: ::canton_splice_api_token_holding_v1::splice_api_token_holding_v1::Splice_Api_Token_HoldingV1::InstrumentId,
            ///Daml field `licenseFeeAmount`.
            #[serde(rename = "licenseFeeAmount")]
            pub license_fee_amount: rt::Numeric,
            ///Daml field `licenseExtensionDuration`.
            #[serde(rename = "licenseExtensionDuration")]
            pub license_extension_duration: ::canton_daml_stdlib::daml_stdlib_DA_Time_Types::DA_Time_Types::RelTime,
            ///Daml field `requestedAt`.
            #[serde(rename = "requestedAt")]
            pub requested_at: rt::Timestamp,
            ///Daml field `prepareUntil`.
            #[serde(rename = "prepareUntil")]
            pub prepare_until: rt::Timestamp,
            ///Daml field `settleBefore`.
            #[serde(rename = "settleBefore")]
            pub settle_before: rt::Timestamp,
            #[serde(rename = "description")]
            pub description: ::std::string::String,
        }
        impl rt::ToValue for License_Renew {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("requestId", rt::ToValue::to_value(&self.request_id)),
                    (
                        "licenseFeeInstrumentId",
                        rt::ToValue::to_value(&self.license_fee_instrument_id)
                    ),
                    (
                        "licenseFeeAmount",
                        rt::ToValue::to_value(&self.license_fee_amount)
                    ),
                    (
                        "licenseExtensionDuration",
                        rt::ToValue::to_value(&self.license_extension_duration)
                    ),
                    ("requestedAt", rt::ToValue::to_value(&self.requested_at)),
                    ("prepareUntil", rt::ToValue::to_value(&self.prepare_until)),
                    ("settleBefore", rt::ToValue::to_value(&self.settle_before)),
                    ("description", rt::ToValue::to_value(&self.description)),
                ])
            }
        }
        impl rt::FromValue for License_Renew {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    request_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "requestId",
                    )?)
                    .map_err(|e| e.at("requestId"))?,
                    license_fee_instrument_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "licenseFeeInstrumentId",
                    )?)
                    .map_err(|e| e.at("licenseFeeInstrumentId"))?,
                    license_fee_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "licenseFeeAmount",
                    )?)
                    .map_err(|e| e.at("licenseFeeAmount"))?,
                    license_extension_duration: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "licenseExtensionDuration",
                    )?)
                    .map_err(|e| e.at("licenseExtensionDuration"))?,
                    requested_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "requestedAt",
                    )?)
                    .map_err(|e| e.at("requestedAt"))?,
                    prepare_until: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "prepareUntil",
                    )?)
                    .map_err(|e| e.at("prepareUntil"))?,
                    settle_before: rt::FromValue::from_value(rt::required_field(
                        value,
                        6usize,
                        "settleBefore",
                    )?)
                    .map_err(|e| e.at("settleBefore"))?,
                    description: rt::FromValue::from_value(rt::required_field(
                        value,
                        7usize,
                        "description",
                    )?)
                    .map_err(|e| e.at("description"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct LicenseParams {
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for LicenseParams {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("meta", rt::ToValue::to_value(&self.meta)),])
            }
        }
        impl rt::FromValue for LicenseParams {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    meta: rt::FromValue::from_value(rt::required_field(value, 0usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        ///The Daml template `Licensing.License:License`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#quickstart-licensing:Licensing.License:License`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `Archive` — consuming
        ///- `License_Renew` — non-consuming
        ///- `License_Expire` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct License {
            #[serde(rename = "provider")]
            pub provider: rt::Party,
            #[serde(rename = "user")]
            pub user: rt::Party,
            ///Daml field `expiresAt`.
            #[serde(rename = "expiresAt")]
            pub expires_at: rt::Timestamp,
            ///Daml field `licenseNum`.
            #[serde(rename = "licenseNum")]
            pub license_num: rt::Int64,
            #[serde(rename = "params")]
            pub params: crate::quickstart_licensing::Licensing_License::LicenseParams,
        }
        impl rt::ToValue for License {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("provider", rt::ToValue::to_value(&self.provider)),
                    ("user", rt::ToValue::to_value(&self.user)),
                    ("expiresAt", rt::ToValue::to_value(&self.expires_at)),
                    ("licenseNum", rt::ToValue::to_value(&self.license_num)),
                    ("params", rt::ToValue::to_value(&self.params)),
                ])
            }
        }
        impl rt::FromValue for License {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    provider: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "provider",
                    )?)
                    .map_err(|e| e.at("provider"))?,
                    user: rt::FromValue::from_value(rt::required_field(value, 1usize, "user")?)
                        .map_err(|e| e.at("user"))?,
                    expires_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "expiresAt",
                    )?)
                    .map_err(|e| e.at("expiresAt"))?,
                    license_num: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "licenseNum",
                    )?)
                    .map_err(|e| e.at("licenseNum"))?,
                    params: rt::FromValue::from_value(rt::required_field(value, 4usize, "params")?)
                        .map_err(|e| e.at("params"))?,
                })
            }
        }
        impl rt::Contract for License {
            const PACKAGE_ID: &'static str =
                "edd5a8d857f6ece9b0b3b21b1096448fc5292e7614044b916746927cbefa919a";
            const PACKAGE_NAME: &'static str = "quickstart-licensing";
            const MODULE_NAME: &'static str = "Licensing.License";
            const ENTITY_NAME: &'static str = "License";
        }
        impl rt::Template for License {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("provider", rt::ToValue::to_value(&self.provider)),
                    ("user", rt::ToValue::to_value(&self.user)),
                    ("expiresAt", rt::ToValue::to_value(&self.expires_at)),
                    ("licenseNum", rt::ToValue::to_value(&self.license_num)),
                    ("params", rt::ToValue::to_value(&self.params)),
                ])
            }
        }
        ///The `Archive` choice on [`License`] (consuming).
        impl rt::Choice<License>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `License_Renew` choice on [`License`] (non-consuming).
        impl rt::Choice<License> for crate::quickstart_licensing::Licensing_License::License_Renew {
            type Return = rt::ContractId<
                crate::quickstart_licensing::Licensing_License::LicenseRenewalRequest,
            >;
            const NAME: &'static str = "License_Renew";
            const CONSUMING: bool = false;
        }
        ///The `License_Expire` choice on [`License`] (consuming).
        impl rt::Choice<License> for crate::quickstart_licensing::Licensing_License::License_Expire {
            type Return = rt::Unit;
            const NAME: &'static str = "License_Expire";
            const CONSUMING: bool = true;
        }
        ///The Daml template `Licensing.License:LicenseRenewalRequest`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#quickstart-licensing:Licensing.License:LicenseRenewalRequest`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `LicenseRenewalRequest_CompleteRenewal` — non-consuming
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct LicenseRenewalRequest {
            ///Daml field `requestId`.
            #[serde(rename = "requestId")]
            pub request_id: ::std::string::String,
            #[serde(rename = "provider")]
            pub provider: rt::Party,
            #[serde(rename = "user")]
            pub user: rt::Party,
            ///Daml field `licenseNum`.
            #[serde(rename = "licenseNum")]
            pub license_num: rt::Int64,
            ///Daml field `licenseFeeAmount`.
            #[serde(rename = "licenseFeeAmount")]
            pub license_fee_amount: rt::Numeric,
            ///Daml field `licenseFeeInstrumentId`.
            #[serde(rename = "licenseFeeInstrumentId")]
            pub license_fee_instrument_id: ::canton_splice_api_token_holding_v1::splice_api_token_holding_v1::Splice_Api_Token_HoldingV1::InstrumentId,
            ///Daml field `licenseExtensionDuration`.
            #[serde(rename = "licenseExtensionDuration")]
            pub license_extension_duration: ::canton_daml_stdlib::daml_stdlib_DA_Time_Types::DA_Time_Types::RelTime,
            ///Daml field `prepareUntil`.
            #[serde(rename = "prepareUntil")]
            pub prepare_until: rt::Timestamp,
            ///Daml field `settleBefore`.
            #[serde(rename = "settleBefore")]
            pub settle_before: rt::Timestamp,
            ///Daml field `requestedAt`.
            #[serde(rename = "requestedAt")]
            pub requested_at: rt::Timestamp,
            #[serde(rename = "description")]
            pub description: ::std::string::String,
        }
        impl rt::ToValue for LicenseRenewalRequest {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("requestId", rt::ToValue::to_value(&self.request_id)),
                    ("provider", rt::ToValue::to_value(&self.provider)),
                    ("user", rt::ToValue::to_value(&self.user)),
                    ("licenseNum", rt::ToValue::to_value(&self.license_num)),
                    (
                        "licenseFeeAmount",
                        rt::ToValue::to_value(&self.license_fee_amount)
                    ),
                    (
                        "licenseFeeInstrumentId",
                        rt::ToValue::to_value(&self.license_fee_instrument_id)
                    ),
                    (
                        "licenseExtensionDuration",
                        rt::ToValue::to_value(&self.license_extension_duration)
                    ),
                    ("prepareUntil", rt::ToValue::to_value(&self.prepare_until)),
                    ("settleBefore", rt::ToValue::to_value(&self.settle_before)),
                    ("requestedAt", rt::ToValue::to_value(&self.requested_at)),
                    ("description", rt::ToValue::to_value(&self.description)),
                ])
            }
        }
        impl rt::FromValue for LicenseRenewalRequest {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    request_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "requestId",
                    )?)
                    .map_err(|e| e.at("requestId"))?,
                    provider: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "provider",
                    )?)
                    .map_err(|e| e.at("provider"))?,
                    user: rt::FromValue::from_value(rt::required_field(value, 2usize, "user")?)
                        .map_err(|e| e.at("user"))?,
                    license_num: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "licenseNum",
                    )?)
                    .map_err(|e| e.at("licenseNum"))?,
                    license_fee_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "licenseFeeAmount",
                    )?)
                    .map_err(|e| e.at("licenseFeeAmount"))?,
                    license_fee_instrument_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "licenseFeeInstrumentId",
                    )?)
                    .map_err(|e| e.at("licenseFeeInstrumentId"))?,
                    license_extension_duration: rt::FromValue::from_value(rt::required_field(
                        value,
                        6usize,
                        "licenseExtensionDuration",
                    )?)
                    .map_err(|e| e.at("licenseExtensionDuration"))?,
                    prepare_until: rt::FromValue::from_value(rt::required_field(
                        value,
                        7usize,
                        "prepareUntil",
                    )?)
                    .map_err(|e| e.at("prepareUntil"))?,
                    settle_before: rt::FromValue::from_value(rt::required_field(
                        value,
                        8usize,
                        "settleBefore",
                    )?)
                    .map_err(|e| e.at("settleBefore"))?,
                    requested_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        9usize,
                        "requestedAt",
                    )?)
                    .map_err(|e| e.at("requestedAt"))?,
                    description: rt::FromValue::from_value(rt::required_field(
                        value,
                        10usize,
                        "description",
                    )?)
                    .map_err(|e| e.at("description"))?,
                })
            }
        }
        impl rt::Contract for LicenseRenewalRequest {
            const PACKAGE_ID: &'static str =
                "edd5a8d857f6ece9b0b3b21b1096448fc5292e7614044b916746927cbefa919a";
            const PACKAGE_NAME: &'static str = "quickstart-licensing";
            const MODULE_NAME: &'static str = "Licensing.License";
            const ENTITY_NAME: &'static str = "LicenseRenewalRequest";
        }
        impl rt::Template for LicenseRenewalRequest {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("requestId", rt::ToValue::to_value(&self.request_id)),
                    ("provider", rt::ToValue::to_value(&self.provider)),
                    ("user", rt::ToValue::to_value(&self.user)),
                    ("licenseNum", rt::ToValue::to_value(&self.license_num)),
                    (
                        "licenseFeeAmount",
                        rt::ToValue::to_value(&self.license_fee_amount)
                    ),
                    (
                        "licenseFeeInstrumentId",
                        rt::ToValue::to_value(&self.license_fee_instrument_id)
                    ),
                    (
                        "licenseExtensionDuration",
                        rt::ToValue::to_value(&self.license_extension_duration)
                    ),
                    ("prepareUntil", rt::ToValue::to_value(&self.prepare_until)),
                    ("settleBefore", rt::ToValue::to_value(&self.settle_before)),
                    ("requestedAt", rt::ToValue::to_value(&self.requested_at)),
                    ("description", rt::ToValue::to_value(&self.description)),
                ])
            }
        }
        ///The `LicenseRenewalRequest_CompleteRenewal` choice on [`LicenseRenewalRequest`] (non-consuming).
        impl rt::Choice<LicenseRenewalRequest>
        for crate::quickstart_licensing::Licensing_License::LicenseRenewalRequest_CompleteRenewal {
            type Return = rt::ContractId<
                crate::quickstart_licensing::Licensing_License::License,
            >;
            const NAME: &'static str = "LicenseRenewalRequest_CompleteRenewal";
            const CONSUMING: bool = false;
        }
        ///The `Archive` choice on [`LicenseRenewalRequest`] (consuming).
        impl rt::Choice<LicenseRenewalRequest>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
    }
    pub mod Licensing_AppInstall {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AppInstallRequest_Reject {
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for AppInstallRequest_Reject {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("meta", rt::ToValue::to_value(&self.meta)),])
            }
        }
        impl rt::FromValue for AppInstallRequest_Reject {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    meta: rt::FromValue::from_value(rt::required_field(value, 0usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AppInstallRequest_Accept {
            ///Daml field `installMeta`.
            #[serde(rename = "installMeta")]
            pub install_meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for AppInstallRequest_Accept {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("installMeta", rt::ToValue::to_value(&self.install_meta)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for AppInstallRequest_Accept {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    install_meta: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "installMeta",
                    )?)
                    .map_err(|e| e.at("installMeta"))?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 1usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AppInstall_Cancel {
            #[serde(rename = "actor")]
            pub actor: rt::Party,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for AppInstall_Cancel {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("actor", rt::ToValue::to_value(&self.actor)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for AppInstall_Cancel {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    actor: rt::FromValue::from_value(rt::required_field(value, 0usize, "actor")?)
                        .map_err(|e| e.at("actor"))?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 1usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AppInstall_CreateLicense {
            #[serde(rename = "params")]
            pub params: crate::quickstart_licensing::Licensing_License::LicenseParams,
        }
        impl rt::ToValue for AppInstall_CreateLicense {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("params", rt::ToValue::to_value(&self.params)),])
            }
        }
        impl rt::FromValue for AppInstall_CreateLicense {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    params: rt::FromValue::from_value(rt::required_field(value, 0usize, "params")?)
                        .map_err(|e| e.at("params"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AppInstall_CreateLicense_Result {
            ///Daml field `installId`.
            #[serde(rename = "installId")]
            pub install_id:
                rt::ContractId<crate::quickstart_licensing::Licensing_AppInstall::AppInstall>,
            ///Daml field `licenseId`.
            #[serde(rename = "licenseId")]
            pub license_id: rt::ContractId<crate::quickstart_licensing::Licensing_License::License>,
        }
        impl rt::ToValue for AppInstall_CreateLicense_Result {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("installId", rt::ToValue::to_value(&self.install_id)),
                    ("licenseId", rt::ToValue::to_value(&self.license_id)),
                ])
            }
        }
        impl rt::FromValue for AppInstall_CreateLicense_Result {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    install_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "installId",
                    )?)
                    .map_err(|e| e.at("installId"))?,
                    license_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "licenseId",
                    )?)
                    .map_err(|e| e.at("licenseId"))?,
                })
            }
        }
        ///The Daml template `Licensing.AppInstall:AppInstall`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#quickstart-licensing:Licensing.AppInstall:AppInstall`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `AppInstall_CreateLicense` — consuming
        ///- `AppInstall_Cancel` — consuming
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AppInstall {
            #[serde(rename = "provider")]
            pub provider: rt::Party,
            #[serde(rename = "user")]
            pub user: rt::Party,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
            ///Daml field `numLicensesCreated`.
            #[serde(rename = "numLicensesCreated")]
            pub num_licenses_created: rt::Int64,
        }
        impl rt::ToValue for AppInstall {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("provider", rt::ToValue::to_value(&self.provider)),
                    ("user", rt::ToValue::to_value(&self.user)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                    (
                        "numLicensesCreated",
                        rt::ToValue::to_value(&self.num_licenses_created)
                    ),
                ])
            }
        }
        impl rt::FromValue for AppInstall {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    provider: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "provider",
                    )?)
                    .map_err(|e| e.at("provider"))?,
                    user: rt::FromValue::from_value(rt::required_field(value, 1usize, "user")?)
                        .map_err(|e| e.at("user"))?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 2usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                    num_licenses_created: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "numLicensesCreated",
                    )?)
                    .map_err(|e| e.at("numLicensesCreated"))?,
                })
            }
        }
        impl rt::Contract for AppInstall {
            const PACKAGE_ID: &'static str =
                "edd5a8d857f6ece9b0b3b21b1096448fc5292e7614044b916746927cbefa919a";
            const PACKAGE_NAME: &'static str = "quickstart-licensing";
            const MODULE_NAME: &'static str = "Licensing.AppInstall";
            const ENTITY_NAME: &'static str = "AppInstall";
        }
        impl rt::Template for AppInstall {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("provider", rt::ToValue::to_value(&self.provider)),
                    ("user", rt::ToValue::to_value(&self.user)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                    (
                        "numLicensesCreated",
                        rt::ToValue::to_value(&self.num_licenses_created)
                    ),
                ])
            }
        }
        ///The `AppInstall_CreateLicense` choice on [`AppInstall`] (consuming).
        impl rt::Choice<AppInstall>
            for crate::quickstart_licensing::Licensing_AppInstall::AppInstall_CreateLicense
        {
            type Return =
                crate::quickstart_licensing::Licensing_AppInstall::AppInstall_CreateLicense_Result;
            const NAME: &'static str = "AppInstall_CreateLicense";
            const CONSUMING: bool = true;
        }
        ///The `AppInstall_Cancel` choice on [`AppInstall`] (consuming).
        impl rt::Choice<AppInstall>
            for crate::quickstart_licensing::Licensing_AppInstall::AppInstall_Cancel
        {
            type Return = rt::Unit;
            const NAME: &'static str = "AppInstall_Cancel";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`AppInstall`] (consuming).
        impl rt::Choice<AppInstall>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The Daml template `Licensing.AppInstall:AppInstallRequest`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#quickstart-licensing:Licensing.AppInstall:AppInstallRequest`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `AppInstallRequest_Accept` — consuming
        ///- `Archive` — consuming
        ///- `AppInstallRequest_Reject` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AppInstallRequest {
            #[serde(rename = "provider")]
            pub provider: rt::Party,
            #[serde(rename = "user")]
            pub user: rt::Party,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for AppInstallRequest {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("provider", rt::ToValue::to_value(&self.provider)),
                    ("user", rt::ToValue::to_value(&self.user)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for AppInstallRequest {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    provider: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "provider",
                    )?)
                    .map_err(|e| e.at("provider"))?,
                    user: rt::FromValue::from_value(rt::required_field(value, 1usize, "user")?)
                        .map_err(|e| e.at("user"))?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 2usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        impl rt::Contract for AppInstallRequest {
            const PACKAGE_ID: &'static str =
                "edd5a8d857f6ece9b0b3b21b1096448fc5292e7614044b916746927cbefa919a";
            const PACKAGE_NAME: &'static str = "quickstart-licensing";
            const MODULE_NAME: &'static str = "Licensing.AppInstall";
            const ENTITY_NAME: &'static str = "AppInstallRequest";
        }
        impl rt::Template for AppInstallRequest {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("provider", rt::ToValue::to_value(&self.provider)),
                    ("user", rt::ToValue::to_value(&self.user)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        ///The `AppInstallRequest_Accept` choice on [`AppInstallRequest`] (consuming).
        impl rt::Choice<AppInstallRequest>
            for crate::quickstart_licensing::Licensing_AppInstall::AppInstallRequest_Accept
        {
            type Return =
                rt::ContractId<crate::quickstart_licensing::Licensing_AppInstall::AppInstall>;
            const NAME: &'static str = "AppInstallRequest_Accept";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`AppInstallRequest`] (consuming).
        impl rt::Choice<AppInstallRequest>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `AppInstallRequest_Reject` choice on [`AppInstallRequest`] (consuming).
        impl rt::Choice<AppInstallRequest>
            for crate::quickstart_licensing::Licensing_AppInstall::AppInstallRequest_Reject
        {
            type Return = rt::Unit;
            const NAME: &'static str = "AppInstallRequest_Reject";
            const CONSUMING: bool = true;
        }
    }
}
