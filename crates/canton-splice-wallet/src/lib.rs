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
//! - [`splice_wallet`] — 5 Daml modules

pub mod splice_wallet {
    pub mod Splice_Wallet_TopUpState {
        use canton_daml as rt;
        ///The Daml template `Splice.Wallet.TopUpState:ValidatorTopUpState`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-wallet:Splice.Wallet.TopUpState:ValidatorTopUpState`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorTopUpState {
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            #[serde(rename = "validator")]
            pub validator: rt::Party,
            ///Daml field `memberId`.
            #[serde(rename = "memberId")]
            pub member_id: ::std::string::String,
            ///Daml field `synchronizerId`.
            #[serde(rename = "synchronizerId")]
            pub synchronizer_id: ::std::string::String,
            ///Daml field `migrationId`.
            #[serde(rename = "migrationId")]
            pub migration_id: rt::Int64,
            ///Daml field `lastPurchasedAt`.
            #[serde(rename = "lastPurchasedAt")]
            pub last_purchased_at: rt::Timestamp,
        }
        impl rt::ToValue for ValidatorTopUpState {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("validator", rt::ToValue::to_value(&self.validator)),
                    ("memberId", rt::ToValue::to_value(&self.member_id)),
                    (
                        "synchronizerId",
                        rt::ToValue::to_value(&self.synchronizer_id)
                    ),
                    ("migrationId", rt::ToValue::to_value(&self.migration_id)),
                    (
                        "lastPurchasedAt",
                        rt::ToValue::to_value(&self.last_purchased_at)
                    ),
                ])
            }
        }
        impl rt::FromValue for ValidatorTopUpState {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                    validator: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "validator",
                    )?)
                    .map_err(|e| e.at("validator"))?,
                    member_id: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "memberId",
                    )?)
                    .map_err(|e| e.at("memberId"))?,
                    synchronizer_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "synchronizerId",
                    )?)
                    .map_err(|e| e.at("synchronizerId"))?,
                    migration_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "migrationId",
                    )?)
                    .map_err(|e| e.at("migrationId"))?,
                    last_purchased_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "lastPurchasedAt",
                    )?)
                    .map_err(|e| e.at("lastPurchasedAt"))?,
                })
            }
        }
        impl rt::Contract for ValidatorTopUpState {
            const PACKAGE_ID: &'static str =
                "690c1d47bac06db419db344d59a7a30c53fa3f5d961943fe1782cfc6c78794d8";
            const PACKAGE_NAME: &'static str = "splice-wallet";
            const MODULE_NAME: &'static str = "Splice.Wallet.TopUpState";
            const ENTITY_NAME: &'static str = "ValidatorTopUpState";
        }
        impl rt::Template for ValidatorTopUpState {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("validator", rt::ToValue::to_value(&self.validator)),
                    ("memberId", rt::ToValue::to_value(&self.member_id)),
                    (
                        "synchronizerId",
                        rt::ToValue::to_value(&self.synchronizer_id)
                    ),
                    ("migrationId", rt::ToValue::to_value(&self.migration_id)),
                    (
                        "lastPurchasedAt",
                        rt::ToValue::to_value(&self.last_purchased_at)
                    ),
                ])
            }
        }
        ///The `Archive` choice on [`ValidatorTopUpState`] (consuming).
        impl rt::Choice<ValidatorTopUpState>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
    }
    pub mod Splice_Wallet_Install {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_Allocation_Withdraw {
            ///Daml field `allocationCid`.
            #[serde(rename = "allocationCid")]
            pub allocation_cid: rt::ContractId<
                ::canton_splice_api_token_allocation_v1::splice_api_token_allocation_v1::Splice_Api_Token_AllocationV1::Allocation,
            >,
            ///Daml field `withdrawArg`.
            #[serde(rename = "withdrawArg")]
            pub withdraw_arg: ::canton_splice_api_token_allocation_v1::splice_api_token_allocation_v1::Splice_Api_Token_AllocationV1::Allocation_Withdraw,
        }
        impl rt::ToValue for WalletAppInstall_Allocation_Withdraw {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("allocationCid", rt::ToValue::to_value(&self.allocation_cid)),
                    ("withdrawArg", rt::ToValue::to_value(&self.withdraw_arg)),
                ])
            }
        }
        impl rt::FromValue for WalletAppInstall_Allocation_Withdraw {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    allocation_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "allocationCid",
                    )?)
                    .map_err(|e| e.at("allocationCid"))?,
                    withdraw_arg: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "withdrawArg",
                    )?)
                    .map_err(|e| e.at("withdrawArg"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_AllocationFactory_Allocate {
            ///Daml field `allocationFactory`.
            #[serde(rename = "allocationFactory")]
            pub allocation_factory: rt::ContractId<
                ::canton_splice_api_token_allocation_instruction_v1::splice_api_token_allocation_instruction_v1::Splice_Api_Token_AllocationInstructionV1::AllocationFactory,
            >,
            ///Daml field `allocateArg`.
            #[serde(rename = "allocateArg")]
            pub allocate_arg: ::canton_splice_api_token_allocation_instruction_v1::splice_api_token_allocation_instruction_v1::Splice_Api_Token_AllocationInstructionV1::AllocationFactory_Allocate,
        }
        impl rt::ToValue for WalletAppInstall_AllocationFactory_Allocate {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "allocationFactory",
                        rt::ToValue::to_value(&self.allocation_factory)
                    ),
                    ("allocateArg", rt::ToValue::to_value(&self.allocate_arg)),
                ])
            }
        }
        impl rt::FromValue for WalletAppInstall_AllocationFactory_Allocate {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    allocation_factory: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "allocationFactory",
                    )?)
                    .map_err(|e| e.at("allocationFactory"))?,
                    allocate_arg: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "allocateArg",
                    )?)
                    .map_err(|e| e.at("allocateArg"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_TransferInstruction_Withdraw {
            ///Daml field `transferInstructionCid`.
            #[serde(rename = "transferInstructionCid")]
            pub transfer_instruction_cid: rt::ContractId<
                ::canton_splice_api_token_transfer_instruction_v1::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferInstruction,
            >,
            ///Daml field `withdrawArg`.
            #[serde(rename = "withdrawArg")]
            pub withdraw_arg: ::canton_splice_api_token_transfer_instruction_v1::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferInstruction_Withdraw,
        }
        impl rt::ToValue for WalletAppInstall_TransferInstruction_Withdraw {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "transferInstructionCid",
                        rt::ToValue::to_value(&self.transfer_instruction_cid)
                    ),
                    ("withdrawArg", rt::ToValue::to_value(&self.withdraw_arg)),
                ])
            }
        }
        impl rt::FromValue for WalletAppInstall_TransferInstruction_Withdraw {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    transfer_instruction_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "transferInstructionCid",
                    )?)
                    .map_err(|e| e.at("transferInstructionCid"))?,
                    withdraw_arg: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "withdrawArg",
                    )?)
                    .map_err(|e| e.at("withdrawArg"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_TransferInstruction_Reject {
            ///Daml field `transferInstructionCid`.
            #[serde(rename = "transferInstructionCid")]
            pub transfer_instruction_cid: rt::ContractId<
                ::canton_splice_api_token_transfer_instruction_v1::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferInstruction,
            >,
            ///Daml field `rejectArg`.
            #[serde(rename = "rejectArg")]
            pub reject_arg: ::canton_splice_api_token_transfer_instruction_v1::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferInstruction_Reject,
        }
        impl rt::ToValue for WalletAppInstall_TransferInstruction_Reject {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "transferInstructionCid",
                        rt::ToValue::to_value(&self.transfer_instruction_cid)
                    ),
                    ("rejectArg", rt::ToValue::to_value(&self.reject_arg)),
                ])
            }
        }
        impl rt::FromValue for WalletAppInstall_TransferInstruction_Reject {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    transfer_instruction_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "transferInstructionCid",
                    )?)
                    .map_err(|e| e.at("transferInstructionCid"))?,
                    reject_arg: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "rejectArg",
                    )?)
                    .map_err(|e| e.at("rejectArg"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_TransferInstruction_Accept {
            ///Daml field `transferInstructionCid`.
            #[serde(rename = "transferInstructionCid")]
            pub transfer_instruction_cid: rt::ContractId<
                ::canton_splice_api_token_transfer_instruction_v1::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferInstruction,
            >,
            ///Daml field `acceptArg`.
            #[serde(rename = "acceptArg")]
            pub accept_arg: ::canton_splice_api_token_transfer_instruction_v1::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferInstruction_Accept,
        }
        impl rt::ToValue for WalletAppInstall_TransferInstruction_Accept {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "transferInstructionCid",
                        rt::ToValue::to_value(&self.transfer_instruction_cid)
                    ),
                    ("acceptArg", rt::ToValue::to_value(&self.accept_arg)),
                ])
            }
        }
        impl rt::FromValue for WalletAppInstall_TransferInstruction_Accept {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    transfer_instruction_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "transferInstructionCid",
                    )?)
                    .map_err(|e| e.at("transferInstructionCid"))?,
                    accept_arg: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "acceptArg",
                    )?)
                    .map_err(|e| e.at("acceptArg"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_TransferFactory_Transfer {
            ///Daml field `transferFactoryCid`.
            #[serde(rename = "transferFactoryCid")]
            pub transfer_factory_cid: rt::ContractId<
                ::canton_splice_api_token_transfer_instruction_v1::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferFactory,
            >,
            ///Daml field `transferArg`.
            #[serde(rename = "transferArg")]
            pub transfer_arg: ::canton_splice_api_token_transfer_instruction_v1::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferFactory_Transfer,
        }
        impl rt::ToValue for WalletAppInstall_TransferFactory_Transfer {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "transferFactoryCid",
                        rt::ToValue::to_value(&self.transfer_factory_cid)
                    ),
                    ("transferArg", rt::ToValue::to_value(&self.transfer_arg)),
                ])
            }
        }
        impl rt::FromValue for WalletAppInstall_TransferFactory_Transfer {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    transfer_factory_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "transferFactoryCid",
                    )?)
                    .map_err(|e| e.at("transferFactoryCid"))?,
                    transfer_arg: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "transferArg",
                    )?)
                    .map_err(|e| e.at("transferArg"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_FeaturedAppRights_SelfGrant {
            ///Daml field `amuletRulesCid`.
            #[serde(rename = "amuletRulesCid")]
            pub amulet_rules_cid: rt::ContractId<
                ::canton_splice_amulet::splice_amulet::Splice_AmuletRules::AmuletRules,
            >,
        }
        impl rt::ToValue for WalletAppInstall_FeaturedAppRights_SelfGrant {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "amuletRulesCid",
                    rt::ToValue::to_value(&self.amulet_rules_cid)
                ),])
            }
        }
        impl rt::FromValue for WalletAppInstall_FeaturedAppRights_SelfGrant {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    amulet_rules_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "amuletRulesCid",
                    )?)
                    .map_err(|e| e.at("amuletRulesCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_FeaturedAppRights_Cancel {
            #[serde(rename = "cid")]
            pub cid: rt::ContractId<
                ::canton_splice_amulet::splice_amulet::Splice_Amulet::FeaturedAppRight,
            >,
        }
        impl rt::ToValue for WalletAppInstall_FeaturedAppRights_Cancel {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("cid", rt::ToValue::to_value(&self.cid)),])
            }
        }
        impl rt::FromValue for WalletAppInstall_FeaturedAppRights_Cancel {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    cid: rt::FromValue::from_value(rt::required_field(value, 0usize, "cid")?)
                        .map_err(|e| e.at("cid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_BuyTrafficRequest_Expire {
            ///Daml field `requestCid`.
            #[serde(rename = "requestCid")]
            pub request_cid: rt::ContractId<
                crate::splice_wallet::Splice_Wallet_BuyTrafficRequest::BuyTrafficRequest,
            >,
        }
        impl rt::ToValue for WalletAppInstall_BuyTrafficRequest_Expire {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "requestCid",
                    rt::ToValue::to_value(&self.request_cid)
                ),])
            }
        }
        impl rt::FromValue for WalletAppInstall_BuyTrafficRequest_Expire {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    request_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "requestCid",
                    )?)
                    .map_err(|e| e.at("requestCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_BuyTrafficRequest_Cancel {
            ///Daml field `requestCid`.
            #[serde(rename = "requestCid")]
            pub request_cid: rt::ContractId<
                crate::splice_wallet::Splice_Wallet_BuyTrafficRequest::BuyTrafficRequest,
            >,
            #[serde(rename = "reason")]
            pub reason: ::std::string::String,
        }
        impl rt::ToValue for WalletAppInstall_BuyTrafficRequest_Cancel {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("requestCid", rt::ToValue::to_value(&self.request_cid)),
                    ("reason", rt::ToValue::to_value(&self.reason)),
                ])
            }
        }
        impl rt::FromValue for WalletAppInstall_BuyTrafficRequest_Cancel {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    request_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "requestCid",
                    )?)
                    .map_err(|e| e.at("requestCid"))?,
                    reason: rt::FromValue::from_value(rt::required_field(value, 1usize, "reason")?)
                        .map_err(|e| e.at("reason"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_CreateBuyTrafficRequest {
            ///Daml field `memberId`.
            #[serde(rename = "memberId")]
            pub member_id: ::std::string::String,
            ///Daml field `synchronizerId`.
            #[serde(rename = "synchronizerId")]
            pub synchronizer_id: ::std::string::String,
            ///Daml field `migrationId`.
            #[serde(rename = "migrationId")]
            pub migration_id: rt::Int64,
            ///Daml field `trafficAmount`.
            #[serde(rename = "trafficAmount")]
            pub traffic_amount: rt::Int64,
            ///Daml field `expiresAt`.
            #[serde(rename = "expiresAt")]
            pub expires_at: rt::Timestamp,
            ///Daml field `trackingId`.
            #[serde(rename = "trackingId")]
            pub tracking_id: ::std::string::String,
        }
        impl rt::ToValue for WalletAppInstall_CreateBuyTrafficRequest {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("memberId", rt::ToValue::to_value(&self.member_id)),
                    (
                        "synchronizerId",
                        rt::ToValue::to_value(&self.synchronizer_id)
                    ),
                    ("migrationId", rt::ToValue::to_value(&self.migration_id)),
                    ("trafficAmount", rt::ToValue::to_value(&self.traffic_amount)),
                    ("expiresAt", rt::ToValue::to_value(&self.expires_at)),
                    ("trackingId", rt::ToValue::to_value(&self.tracking_id)),
                ])
            }
        }
        impl rt::FromValue for WalletAppInstall_CreateBuyTrafficRequest {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    member_id: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "memberId",
                    )?)
                    .map_err(|e| e.at("memberId"))?,
                    synchronizer_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "synchronizerId",
                    )?)
                    .map_err(|e| e.at("synchronizerId"))?,
                    migration_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "migrationId",
                    )?)
                    .map_err(|e| e.at("migrationId"))?,
                    traffic_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "trafficAmount",
                    )?)
                    .map_err(|e| e.at("trafficAmount"))?,
                    expires_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "expiresAt",
                    )?)
                    .map_err(|e| e.at("expiresAt"))?,
                    tracking_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "trackingId",
                    )?)
                    .map_err(|e| e.at("trackingId"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_AcceptedTransferOffer_Expire {
            #[serde(rename = "cid")]
            pub cid: rt::ContractId<
                crate::splice_wallet::Splice_Wallet_TransferOffer::AcceptedTransferOffer,
            >,
        }
        impl rt::ToValue for WalletAppInstall_AcceptedTransferOffer_Expire {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("cid", rt::ToValue::to_value(&self.cid)),])
            }
        }
        impl rt::FromValue for WalletAppInstall_AcceptedTransferOffer_Expire {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    cid: rt::FromValue::from_value(rt::required_field(value, 0usize, "cid")?)
                        .map_err(|e| e.at("cid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_AcceptedTransferOffer_Withdraw {
            #[serde(rename = "cid")]
            pub cid: rt::ContractId<
                crate::splice_wallet::Splice_Wallet_TransferOffer::AcceptedTransferOffer,
            >,
            #[serde(rename = "reason")]
            pub reason: ::std::string::String,
        }
        impl rt::ToValue for WalletAppInstall_AcceptedTransferOffer_Withdraw {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("cid", rt::ToValue::to_value(&self.cid)),
                    ("reason", rt::ToValue::to_value(&self.reason)),
                ])
            }
        }
        impl rt::FromValue for WalletAppInstall_AcceptedTransferOffer_Withdraw {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    cid: rt::FromValue::from_value(rt::required_field(value, 0usize, "cid")?)
                        .map_err(|e| e.at("cid"))?,
                    reason: rt::FromValue::from_value(rt::required_field(value, 1usize, "reason")?)
                        .map_err(|e| e.at("reason"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_AcceptedTransferOffer_Abort {
            #[serde(rename = "cid")]
            pub cid: rt::ContractId<
                crate::splice_wallet::Splice_Wallet_TransferOffer::AcceptedTransferOffer,
            >,
            #[serde(rename = "reason")]
            pub reason: ::std::string::String,
        }
        impl rt::ToValue for WalletAppInstall_AcceptedTransferOffer_Abort {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("cid", rt::ToValue::to_value(&self.cid)),
                    ("reason", rt::ToValue::to_value(&self.reason)),
                ])
            }
        }
        impl rt::FromValue for WalletAppInstall_AcceptedTransferOffer_Abort {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    cid: rt::FromValue::from_value(rt::required_field(value, 0usize, "cid")?)
                        .map_err(|e| e.at("cid"))?,
                    reason: rt::FromValue::from_value(rt::required_field(value, 1usize, "reason")?)
                        .map_err(|e| e.at("reason"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_TransferOffer_Expire {
            #[serde(rename = "cid")]
            pub cid:
                rt::ContractId<crate::splice_wallet::Splice_Wallet_TransferOffer::TransferOffer>,
        }
        impl rt::ToValue for WalletAppInstall_TransferOffer_Expire {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("cid", rt::ToValue::to_value(&self.cid)),])
            }
        }
        impl rt::FromValue for WalletAppInstall_TransferOffer_Expire {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    cid: rt::FromValue::from_value(rt::required_field(value, 0usize, "cid")?)
                        .map_err(|e| e.at("cid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_TransferOffer_Withdraw {
            #[serde(rename = "cid")]
            pub cid:
                rt::ContractId<crate::splice_wallet::Splice_Wallet_TransferOffer::TransferOffer>,
            #[serde(rename = "reason")]
            pub reason: ::std::string::String,
        }
        impl rt::ToValue for WalletAppInstall_TransferOffer_Withdraw {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("cid", rt::ToValue::to_value(&self.cid)),
                    ("reason", rt::ToValue::to_value(&self.reason)),
                ])
            }
        }
        impl rt::FromValue for WalletAppInstall_TransferOffer_Withdraw {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    cid: rt::FromValue::from_value(rt::required_field(value, 0usize, "cid")?)
                        .map_err(|e| e.at("cid"))?,
                    reason: rt::FromValue::from_value(rt::required_field(value, 1usize, "reason")?)
                        .map_err(|e| e.at("reason"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_TransferOffer_Reject {
            #[serde(rename = "cid")]
            pub cid:
                rt::ContractId<crate::splice_wallet::Splice_Wallet_TransferOffer::TransferOffer>,
        }
        impl rt::ToValue for WalletAppInstall_TransferOffer_Reject {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("cid", rt::ToValue::to_value(&self.cid)),])
            }
        }
        impl rt::FromValue for WalletAppInstall_TransferOffer_Reject {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    cid: rt::FromValue::from_value(rt::required_field(value, 0usize, "cid")?)
                        .map_err(|e| e.at("cid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_TransferOffer_Accept {
            #[serde(rename = "cid")]
            pub cid:
                rt::ContractId<crate::splice_wallet::Splice_Wallet_TransferOffer::TransferOffer>,
        }
        impl rt::ToValue for WalletAppInstall_TransferOffer_Accept {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("cid", rt::ToValue::to_value(&self.cid)),])
            }
        }
        impl rt::FromValue for WalletAppInstall_TransferOffer_Accept {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    cid: rt::FromValue::from_value(rt::required_field(value, 0usize, "cid")?)
                        .map_err(|e| e.at("cid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_CreateTransferOffer {
            #[serde(rename = "receiver")]
            pub receiver: rt::Party,
            #[serde(rename = "amount")]
            pub amount: ::canton_splice_wallet_payments::splice_wallet_payments::Splice_Wallet_Payment::PaymentAmount,
            #[serde(rename = "description")]
            pub description: ::std::string::String,
            ///Daml field `expiresAt`.
            #[serde(rename = "expiresAt")]
            pub expires_at: rt::Timestamp,
            ///Daml field `trackingId`.
            #[serde(rename = "trackingId")]
            pub tracking_id: ::std::string::String,
        }
        impl rt::ToValue for WalletAppInstall_CreateTransferOffer {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("receiver", rt::ToValue::to_value(&self.receiver)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                    ("description", rt::ToValue::to_value(&self.description)),
                    ("expiresAt", rt::ToValue::to_value(&self.expires_at)),
                    ("trackingId", rt::ToValue::to_value(&self.tracking_id)),
                ])
            }
        }
        impl rt::FromValue for WalletAppInstall_CreateTransferOffer {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    receiver: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "receiver",
                    )?)
                    .map_err(|e| e.at("receiver"))?,
                    amount: rt::FromValue::from_value(rt::required_field(value, 1usize, "amount")?)
                        .map_err(|e| e.at("amount"))?,
                    description: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "description",
                    )?)
                    .map_err(|e| e.at("description"))?,
                    expires_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "expiresAt",
                    )?)
                    .map_err(|e| e.at("expiresAt"))?,
                    tracking_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "trackingId",
                    )?)
                    .map_err(|e| e.at("trackingId"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_SubscriptionIdleState_CancelSubscription {
            #[serde(rename = "cid")]
            pub cid: rt::ContractId<
                ::canton_splice_wallet_payments::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionIdleState,
            >,
        }
        impl rt::ToValue for WalletAppInstall_SubscriptionIdleState_CancelSubscription {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("cid", rt::ToValue::to_value(&self.cid)),])
            }
        }
        impl rt::FromValue for WalletAppInstall_SubscriptionIdleState_CancelSubscription {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    cid: rt::FromValue::from_value(rt::required_field(value, 0usize, "cid")?)
                        .map_err(|e| e.at("cid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_SubscriptionRequest_Reject {
            #[serde(rename = "cid")]
            pub cid: rt::ContractId<
                ::canton_splice_wallet_payments::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionRequest,
            >,
        }
        impl rt::ToValue for WalletAppInstall_SubscriptionRequest_Reject {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("cid", rt::ToValue::to_value(&self.cid)),])
            }
        }
        impl rt::FromValue for WalletAppInstall_SubscriptionRequest_Reject {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    cid: rt::FromValue::from_value(rt::required_field(value, 0usize, "cid")?)
                        .map_err(|e| e.at("cid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_AppPaymentRequest_Expire {
            #[serde(rename = "cid")]
            pub cid: rt::ContractId<
                ::canton_splice_wallet_payments::splice_wallet_payments::Splice_Wallet_Payment::AppPaymentRequest,
            >,
        }
        impl rt::ToValue for WalletAppInstall_AppPaymentRequest_Expire {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("cid", rt::ToValue::to_value(&self.cid)),])
            }
        }
        impl rt::FromValue for WalletAppInstall_AppPaymentRequest_Expire {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    cid: rt::FromValue::from_value(rt::required_field(value, 0usize, "cid")?)
                        .map_err(|e| e.at("cid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_AppPaymentRequest_Reject {
            #[serde(rename = "cid")]
            pub cid: rt::ContractId<
                ::canton_splice_wallet_payments::splice_wallet_payments::Splice_Wallet_Payment::AppPaymentRequest,
            >,
        }
        impl rt::ToValue for WalletAppInstall_AppPaymentRequest_Reject {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("cid", rt::ToValue::to_value(&self.cid)),])
            }
        }
        impl rt::FromValue for WalletAppInstall_AppPaymentRequest_Reject {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    cid: rt::FromValue::from_value(rt::required_field(value, 0usize, "cid")?)
                        .map_err(|e| e.at("cid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_ExecuteBatch {
            #[serde(rename = "context")]
            pub context:
                ::canton_splice_amulet::splice_amulet::Splice_AmuletRules::PaymentTransferContext,
            #[serde(rename = "inputs")]
            pub inputs: ::std::vec::Vec<
                ::canton_splice_amulet::splice_amulet::Splice_AmuletRules::TransferInput,
            >,
            #[serde(rename = "operations")]
            pub operations:
                ::std::vec::Vec<crate::splice_wallet::Splice_Wallet_Install::AmuletOperation>,
        }
        impl rt::ToValue for WalletAppInstall_ExecuteBatch {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("context", rt::ToValue::to_value(&self.context)),
                    ("inputs", rt::ToValue::to_value(&self.inputs)),
                    ("operations", rt::ToValue::to_value(&self.operations)),
                ])
            }
        }
        impl rt::FromValue for WalletAppInstall_ExecuteBatch {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    context: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "context",
                    )?)
                    .map_err(|e| e.at("context"))?,
                    inputs: rt::FromValue::from_value(rt::required_field(value, 1usize, "inputs")?)
                        .map_err(|e| e.at("inputs"))?,
                    operations: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "operations",
                    )?)
                    .map_err(|e| e.at("operations"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_TransferPreapprovalProposal_CreateResult {
            ///Daml field `preapprovalProposalCid`.
            #[serde(rename = "preapprovalProposalCid")]
            pub preapproval_proposal_cid: rt::ContractId<
                crate::splice_wallet::Splice_Wallet_TransferPreapproval::TransferPreapprovalProposal,
            >,
        }
        impl rt::ToValue for WalletAppInstall_TransferPreapprovalProposal_CreateResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "preapprovalProposalCid",
                    rt::ToValue::to_value(&self.preapproval_proposal_cid)
                ),])
            }
        }
        impl rt::FromValue for WalletAppInstall_TransferPreapprovalProposal_CreateResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    preapproval_proposal_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "preapprovalProposalCid",
                    )?)
                    .map_err(|e| e.at("preapprovalProposalCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_FeaturedAppRights_SelfGrantResult {
            ///Daml field `featuredAppRight`.
            #[serde(rename = "featuredAppRight")]
            pub featured_app_right: rt::ContractId<
                ::canton_splice_amulet::splice_amulet::Splice_Amulet::FeaturedAppRight,
            >,
        }
        impl rt::ToValue for WalletAppInstall_FeaturedAppRights_SelfGrantResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "featuredAppRight",
                    rt::ToValue::to_value(&self.featured_app_right)
                ),])
            }
        }
        impl rt::FromValue for WalletAppInstall_FeaturedAppRights_SelfGrantResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    featured_app_right: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "featuredAppRight",
                    )?)
                    .map_err(|e| e.at("featuredAppRight"))?,
                })
            }
        }
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize,
        )]
        #[serde(crate = "rt::serde")]
        pub enum WalletAppInstall_FeaturedAppRights_CancelResult {
            #[serde(rename = "WalletAppInstall_FeaturedAppRights_CancelResult")]
            WalletAppInstall_FeaturedAppRights_CancelResult,
        }
        impl rt::ToValue for WalletAppInstall_FeaturedAppRights_CancelResult {
            fn to_value(&self) -> rt::Value {
                rt::enum_value(
                    match self {
                        WalletAppInstall_FeaturedAppRights_CancelResult::WalletAppInstall_FeaturedAppRights_CancelResult => {
                            "WalletAppInstall_FeaturedAppRights_CancelResult"
                        }
                    },
                )
            }
        }
        impl rt::FromValue for WalletAppInstall_FeaturedAppRights_CancelResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                match rt::enum_constructor(value)? {
                    "WalletAppInstall_FeaturedAppRights_CancelResult" => {
                        ::core::result::Result::Ok(
                            WalletAppInstall_FeaturedAppRights_CancelResult::WalletAppInstall_FeaturedAppRights_CancelResult,
                        )
                    }
                    other => {
                        ::core::result::Result::Err(
                            rt::unexpected_constructor(
                                "WalletAppInstall_FeaturedAppRights_CancelResult",
                                other,
                            ),
                        )
                    }
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_BuyTrafficRequest_ExpireResult {
            ///Daml field `trackingInfo`.
            #[serde(rename = "trackingInfo")]
            pub tracking_info: crate::splice_wallet::Splice_Wallet_BuyTrafficRequest::BuyTrafficRequestTrackingInfo,
        }
        impl rt::ToValue for WalletAppInstall_BuyTrafficRequest_ExpireResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "trackingInfo",
                    rt::ToValue::to_value(&self.tracking_info)
                ),])
            }
        }
        impl rt::FromValue for WalletAppInstall_BuyTrafficRequest_ExpireResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    tracking_info: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "trackingInfo",
                    )?)
                    .map_err(|e| e.at("trackingInfo"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_BuyTrafficRequest_CancelResult {
            ///Daml field `trackingInfo`.
            #[serde(rename = "trackingInfo")]
            pub tracking_info: crate::splice_wallet::Splice_Wallet_BuyTrafficRequest::BuyTrafficRequestTrackingInfo,
        }
        impl rt::ToValue for WalletAppInstall_BuyTrafficRequest_CancelResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "trackingInfo",
                    rt::ToValue::to_value(&self.tracking_info)
                ),])
            }
        }
        impl rt::FromValue for WalletAppInstall_BuyTrafficRequest_CancelResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    tracking_info: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "trackingInfo",
                    )?)
                    .map_err(|e| e.at("trackingInfo"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_CreateBuyTrafficRequestResult {
            ///Daml field `buyTrafficRequest`.
            #[serde(rename = "buyTrafficRequest")]
            pub buy_traffic_request: rt::ContractId<
                crate::splice_wallet::Splice_Wallet_BuyTrafficRequest::BuyTrafficRequest,
            >,
        }
        impl rt::ToValue for WalletAppInstall_CreateBuyTrafficRequestResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "buyTrafficRequest",
                    rt::ToValue::to_value(&self.buy_traffic_request)
                ),])
            }
        }
        impl rt::FromValue for WalletAppInstall_CreateBuyTrafficRequestResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    buy_traffic_request: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "buyTrafficRequest",
                    )?)
                    .map_err(|e| e.at("buyTrafficRequest"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_AcceptedTransferOffer_ExpireResult {
            ///Daml field `trackingInfo`.
            #[serde(rename = "trackingInfo")]
            pub tracking_info:
                crate::splice_wallet::Splice_Wallet_TransferOffer::TransferOfferTrackingInfo,
        }
        impl rt::ToValue for WalletAppInstall_AcceptedTransferOffer_ExpireResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "trackingInfo",
                    rt::ToValue::to_value(&self.tracking_info)
                ),])
            }
        }
        impl rt::FromValue for WalletAppInstall_AcceptedTransferOffer_ExpireResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    tracking_info: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "trackingInfo",
                    )?)
                    .map_err(|e| e.at("trackingInfo"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_AcceptedTransferOffer_WithdrawResult {
            ///Daml field `trackingInfo`.
            #[serde(rename = "trackingInfo")]
            pub tracking_info:
                crate::splice_wallet::Splice_Wallet_TransferOffer::TransferOfferTrackingInfo,
        }
        impl rt::ToValue for WalletAppInstall_AcceptedTransferOffer_WithdrawResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "trackingInfo",
                    rt::ToValue::to_value(&self.tracking_info)
                ),])
            }
        }
        impl rt::FromValue for WalletAppInstall_AcceptedTransferOffer_WithdrawResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    tracking_info: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "trackingInfo",
                    )?)
                    .map_err(|e| e.at("trackingInfo"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_AcceptedTransferOffer_AbortResult {
            ///Daml field `trackingInfo`.
            #[serde(rename = "trackingInfo")]
            pub tracking_info:
                crate::splice_wallet::Splice_Wallet_TransferOffer::TransferOfferTrackingInfo,
        }
        impl rt::ToValue for WalletAppInstall_AcceptedTransferOffer_AbortResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "trackingInfo",
                    rt::ToValue::to_value(&self.tracking_info)
                ),])
            }
        }
        impl rt::FromValue for WalletAppInstall_AcceptedTransferOffer_AbortResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    tracking_info: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "trackingInfo",
                    )?)
                    .map_err(|e| e.at("trackingInfo"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_TransferOffer_ExpireResult {
            ///Daml field `trackingInfo`.
            #[serde(rename = "trackingInfo")]
            pub tracking_info:
                crate::splice_wallet::Splice_Wallet_TransferOffer::TransferOfferTrackingInfo,
        }
        impl rt::ToValue for WalletAppInstall_TransferOffer_ExpireResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "trackingInfo",
                    rt::ToValue::to_value(&self.tracking_info)
                ),])
            }
        }
        impl rt::FromValue for WalletAppInstall_TransferOffer_ExpireResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    tracking_info: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "trackingInfo",
                    )?)
                    .map_err(|e| e.at("trackingInfo"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_TransferOffer_WithdrawResult {
            ///Daml field `trackingInfo`.
            #[serde(rename = "trackingInfo")]
            pub tracking_info:
                crate::splice_wallet::Splice_Wallet_TransferOffer::TransferOfferTrackingInfo,
        }
        impl rt::ToValue for WalletAppInstall_TransferOffer_WithdrawResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "trackingInfo",
                    rt::ToValue::to_value(&self.tracking_info)
                ),])
            }
        }
        impl rt::FromValue for WalletAppInstall_TransferOffer_WithdrawResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    tracking_info: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "trackingInfo",
                    )?)
                    .map_err(|e| e.at("trackingInfo"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_TransferOffer_RejectResult {
            ///Daml field `trackingInfo`.
            #[serde(rename = "trackingInfo")]
            pub tracking_info:
                crate::splice_wallet::Splice_Wallet_TransferOffer::TransferOfferTrackingInfo,
        }
        impl rt::ToValue for WalletAppInstall_TransferOffer_RejectResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "trackingInfo",
                    rt::ToValue::to_value(&self.tracking_info)
                ),])
            }
        }
        impl rt::FromValue for WalletAppInstall_TransferOffer_RejectResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    tracking_info: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "trackingInfo",
                    )?)
                    .map_err(|e| e.at("trackingInfo"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_TransferOffer_AcceptResult {
            ///Daml field `acceptedTransferOffer`.
            #[serde(rename = "acceptedTransferOffer")]
            pub accepted_transfer_offer: rt::ContractId<
                crate::splice_wallet::Splice_Wallet_TransferOffer::AcceptedTransferOffer,
            >,
        }
        impl rt::ToValue for WalletAppInstall_TransferOffer_AcceptResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "acceptedTransferOffer",
                    rt::ToValue::to_value(&self.accepted_transfer_offer)
                ),])
            }
        }
        impl rt::FromValue for WalletAppInstall_TransferOffer_AcceptResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    accepted_transfer_offer: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "acceptedTransferOffer",
                    )?)
                    .map_err(|e| e.at("acceptedTransferOffer"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_CreateTransferOfferResult {
            ///Daml field `transferOffer`.
            #[serde(rename = "transferOffer")]
            pub transfer_offer:
                rt::ContractId<crate::splice_wallet::Splice_Wallet_TransferOffer::TransferOffer>,
        }
        impl rt::ToValue for WalletAppInstall_CreateTransferOfferResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "transferOffer",
                    rt::ToValue::to_value(&self.transfer_offer)
                ),])
            }
        }
        impl rt::FromValue for WalletAppInstall_CreateTransferOfferResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    transfer_offer: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "transferOffer",
                    )?)
                    .map_err(|e| e.at("transferOffer"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_SubscriptionIdleState_CancelSubscriptionResult {
            ///Daml field `terminatedSubscription`.
            #[serde(rename = "terminatedSubscription")]
            pub terminated_subscription: rt::ContractId<
                ::canton_splice_wallet_payments::splice_wallet_payments::Splice_Wallet_Subscriptions::TerminatedSubscription,
            >,
        }
        impl rt::ToValue for WalletAppInstall_SubscriptionIdleState_CancelSubscriptionResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "terminatedSubscription",
                    rt::ToValue::to_value(&self.terminated_subscription)
                ),])
            }
        }
        impl rt::FromValue for WalletAppInstall_SubscriptionIdleState_CancelSubscriptionResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    terminated_subscription: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "terminatedSubscription",
                    )?)
                    .map_err(|e| e.at("terminatedSubscription"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_SubscriptionRequest_RejectResult {
            ///Daml field `terminatedSubscription`.
            #[serde(rename = "terminatedSubscription")]
            pub terminated_subscription: rt::ContractId<
                ::canton_splice_wallet_payments::splice_wallet_payments::Splice_Wallet_Subscriptions::TerminatedSubscription,
            >,
        }
        impl rt::ToValue for WalletAppInstall_SubscriptionRequest_RejectResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "terminatedSubscription",
                    rt::ToValue::to_value(&self.terminated_subscription)
                ),])
            }
        }
        impl rt::FromValue for WalletAppInstall_SubscriptionRequest_RejectResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    terminated_subscription: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "terminatedSubscription",
                    )?)
                    .map_err(|e| e.at("terminatedSubscription"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_AppPaymentRequest_ExpireResult {
            ///Daml field `terminatedAppPayment`.
            #[serde(rename = "terminatedAppPayment")]
            pub terminated_app_payment: rt::ContractId<
                ::canton_splice_wallet_payments::splice_wallet_payments::Splice_Wallet_Payment::TerminatedAppPayment,
            >,
        }
        impl rt::ToValue for WalletAppInstall_AppPaymentRequest_ExpireResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "terminatedAppPayment",
                    rt::ToValue::to_value(&self.terminated_app_payment)
                ),])
            }
        }
        impl rt::FromValue for WalletAppInstall_AppPaymentRequest_ExpireResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    terminated_app_payment: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "terminatedAppPayment",
                    )?)
                    .map_err(|e| e.at("terminatedAppPayment"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_AppPaymentRequest_RejectResult {
            ///Daml field `terminatedAppPayment`.
            #[serde(rename = "terminatedAppPayment")]
            pub terminated_app_payment: rt::ContractId<
                ::canton_splice_wallet_payments::splice_wallet_payments::Splice_Wallet_Payment::TerminatedAppPayment,
            >,
        }
        impl rt::ToValue for WalletAppInstall_AppPaymentRequest_RejectResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "terminatedAppPayment",
                    rt::ToValue::to_value(&self.terminated_app_payment)
                ),])
            }
        }
        impl rt::FromValue for WalletAppInstall_AppPaymentRequest_RejectResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    terminated_app_payment: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "terminatedAppPayment",
                    )?)
                    .map_err(|e| e.at("terminatedAppPayment"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall_ExecuteBatchResult {
            ///Daml field `endUserName`.
            #[serde(rename = "endUserName")]
            pub end_user_name: ::std::string::String,
            #[serde(rename = "outcomes")]
            pub outcomes: ::std::vec::Vec<
                crate::splice_wallet::Splice_Wallet_Install::AmuletOperationOutcome,
            >,
            ///Daml field `optEndUserParty`.
            #[serde(rename = "optEndUserParty")]
            pub opt_end_user_party: ::core::option::Option<rt::Party>,
        }
        impl rt::ToValue for WalletAppInstall_ExecuteBatchResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("endUserName", rt::ToValue::to_value(&self.end_user_name)),
                    ("outcomes", rt::ToValue::to_value(&self.outcomes)),
                    (
                        "optEndUserParty",
                        rt::ToValue::to_value(&self.opt_end_user_party)
                    ),
                ])
            }
        }
        impl rt::FromValue for WalletAppInstall_ExecuteBatchResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    end_user_name: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "endUserName",
                    )?)
                    .map_err(|e| e.at("endUserName"))?,
                    outcomes: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "outcomes",
                    )?)
                    .map_err(|e| e.at("outcomes"))?,
                    opt_end_user_party: rt::optional_field(value, 2usize, "optEndUserParty")
                        .map_err(|e| e.at("optEndUserParty"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum AmuletOperationOutcome {
            #[serde(rename = "COO_AcceptedAppPayment")]
            COO_AcceptedAppPayment(
                rt::ContractId<
                    ::canton_splice_wallet_payments::splice_wallet_payments::Splice_Wallet_Payment::AcceptedAppPayment,
                >,
            ),
            #[serde(rename = "COO_CompleteAcceptedTransfer")]
            COO_CompleteAcceptedTransfer(
                ::canton_daml_stdlib::daml_prim_DA_Types::DA_Types::Tuple2<
                    ::canton_splice_amulet::splice_amulet::Splice_AmuletRules::TransferResult,
                    crate::splice_wallet::Splice_Wallet_TransferOffer::TransferOfferTrackingInfo,
                >,
            ),
            #[serde(rename = "COO_SubscriptionInitialPayment")]
            COO_SubscriptionInitialPayment(
                rt::ContractId<
                    ::canton_splice_wallet_payments::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionInitialPayment,
                >,
            ),
            #[serde(rename = "COO_SubscriptionPayment")]
            COO_SubscriptionPayment(
                rt::ContractId<
                    ::canton_splice_wallet_payments::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionPayment,
                >,
            ),
            #[serde(rename = "COO_MergeTransferInputs")]
            COO_MergeTransferInputs(
                ::core::option::Option<
                    rt::ContractId<
                        ::canton_splice_amulet::splice_amulet::Splice_Amulet::Amulet,
                    >,
                >,
            ),
            #[serde(rename = "COO_BuyMemberTraffic")]
            COO_BuyMemberTraffic(
                rt::ContractId<
                    ::canton_splice_amulet::splice_amulet::Splice_DecentralizedSynchronizer::MemberTraffic,
                >,
            ),
            #[serde(rename = "COO_CompleteBuyTrafficRequest")]
            COO_CompleteBuyTrafficRequest(
                ::canton_daml_stdlib::daml_prim_DA_Types::DA_Types::Tuple2<
                    rt::ContractId<
                        ::canton_splice_amulet::splice_amulet::Splice_DecentralizedSynchronizer::MemberTraffic,
                    >,
                    crate::splice_wallet::Splice_Wallet_BuyTrafficRequest::BuyTrafficRequestTrackingInfo,
                >,
            ),
            #[serde(rename = "COO_Tap")]
            COO_Tap(
                rt::ContractId<
                    ::canton_splice_amulet::splice_amulet::Splice_Amulet::Amulet,
                >,
            ),
            #[serde(rename = "COO_Error")]
            COO_Error(
                ::canton_splice_amulet::splice_amulet::Splice_AmuletRules::InvalidTransferReason,
            ),
            #[serde(rename = "ExtAmuletOperationOutcome")]
            ExtAmuletOperationOutcome(
                crate::splice_wallet::Splice_Wallet_Install::AmuletOperationOutcome_ExtAmuletOperationOutcome,
            ),
            #[serde(rename = "COO_CreateExternalPartySetupProposal")]
            COO_CreateExternalPartySetupProposal(
                rt::ContractId<
                    ::canton_splice_amulet::splice_amulet::Splice_AmuletRules::ExternalPartySetupProposal,
                >,
            ),
            #[serde(rename = "COO_AcceptTransferPreapprovalProposal")]
            COO_AcceptTransferPreapprovalProposal(
                rt::ContractId<
                    ::canton_splice_amulet::splice_amulet::Splice_AmuletRules::TransferPreapproval,
                >,
            ),
            #[serde(rename = "COO_RenewTransferPreapproval")]
            COO_RenewTransferPreapproval(
                rt::ContractId<
                    ::canton_splice_amulet::splice_amulet::Splice_AmuletRules::TransferPreapproval,
                >,
            ),
            #[serde(rename = "COO_TransferPreapprovalSend")]
            COO_TransferPreapprovalSend(
                ::core::option::Option<
                    rt::ContractId<
                        ::canton_splice_amulet::splice_amulet::Splice_Amulet::Amulet,
                    >,
                >,
            ),
        }
        impl rt::ToValue for AmuletOperationOutcome {
            fn to_value(&self) -> rt::Value {
                match self {
                    AmuletOperationOutcome::COO_AcceptedAppPayment(inner) => {
                        rt::variant_value("COO_AcceptedAppPayment", rt::ToValue::to_value(inner))
                    }
                    AmuletOperationOutcome::COO_CompleteAcceptedTransfer(inner) => {
                        rt::variant_value(
                            "COO_CompleteAcceptedTransfer",
                            rt::ToValue::to_value(inner),
                        )
                    }
                    AmuletOperationOutcome::COO_SubscriptionInitialPayment(inner) => {
                        rt::variant_value(
                            "COO_SubscriptionInitialPayment",
                            rt::ToValue::to_value(inner),
                        )
                    }
                    AmuletOperationOutcome::COO_SubscriptionPayment(inner) => {
                        rt::variant_value("COO_SubscriptionPayment", rt::ToValue::to_value(inner))
                    }
                    AmuletOperationOutcome::COO_MergeTransferInputs(inner) => {
                        rt::variant_value("COO_MergeTransferInputs", rt::ToValue::to_value(inner))
                    }
                    AmuletOperationOutcome::COO_BuyMemberTraffic(inner) => {
                        rt::variant_value("COO_BuyMemberTraffic", rt::ToValue::to_value(inner))
                    }
                    AmuletOperationOutcome::COO_CompleteBuyTrafficRequest(inner) => {
                        rt::variant_value(
                            "COO_CompleteBuyTrafficRequest",
                            rt::ToValue::to_value(inner),
                        )
                    }
                    AmuletOperationOutcome::COO_Tap(inner) => {
                        rt::variant_value("COO_Tap", rt::ToValue::to_value(inner))
                    }
                    AmuletOperationOutcome::COO_Error(inner) => {
                        rt::variant_value("COO_Error", rt::ToValue::to_value(inner))
                    }
                    AmuletOperationOutcome::ExtAmuletOperationOutcome(inner) => {
                        rt::variant_value("ExtAmuletOperationOutcome", rt::ToValue::to_value(inner))
                    }
                    AmuletOperationOutcome::COO_CreateExternalPartySetupProposal(inner) => {
                        rt::variant_value(
                            "COO_CreateExternalPartySetupProposal",
                            rt::ToValue::to_value(inner),
                        )
                    }
                    AmuletOperationOutcome::COO_AcceptTransferPreapprovalProposal(inner) => {
                        rt::variant_value(
                            "COO_AcceptTransferPreapprovalProposal",
                            rt::ToValue::to_value(inner),
                        )
                    }
                    AmuletOperationOutcome::COO_RenewTransferPreapproval(inner) => {
                        rt::variant_value(
                            "COO_RenewTransferPreapproval",
                            rt::ToValue::to_value(inner),
                        )
                    }
                    AmuletOperationOutcome::COO_TransferPreapprovalSend(inner) => {
                        rt::variant_value(
                            "COO_TransferPreapprovalSend",
                            rt::ToValue::to_value(inner),
                        )
                    }
                }
            }
        }
        impl rt::FromValue for AmuletOperationOutcome {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                let (constructor, payload) = rt::variant_parts(value)?;
                match constructor {
                    "COO_AcceptedAppPayment" => {
                        ::core::result::Result::Ok(AmuletOperationOutcome::COO_AcceptedAppPayment(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("COO_AcceptedAppPayment"))?,
                        ))
                    }
                    "COO_CompleteAcceptedTransfer" => ::core::result::Result::Ok(
                        AmuletOperationOutcome::COO_CompleteAcceptedTransfer(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("COO_CompleteAcceptedTransfer"))?,
                        ),
                    ),
                    "COO_SubscriptionInitialPayment" => ::core::result::Result::Ok(
                        AmuletOperationOutcome::COO_SubscriptionInitialPayment(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("COO_SubscriptionInitialPayment"))?,
                        ),
                    ),
                    "COO_SubscriptionPayment" => {
                        ::core::result::Result::Ok(AmuletOperationOutcome::COO_SubscriptionPayment(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("COO_SubscriptionPayment"))?,
                        ))
                    }
                    "COO_MergeTransferInputs" => {
                        ::core::result::Result::Ok(AmuletOperationOutcome::COO_MergeTransferInputs(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("COO_MergeTransferInputs"))?,
                        ))
                    }
                    "COO_BuyMemberTraffic" => {
                        ::core::result::Result::Ok(AmuletOperationOutcome::COO_BuyMemberTraffic(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("COO_BuyMemberTraffic"))?,
                        ))
                    }
                    "COO_CompleteBuyTrafficRequest" => ::core::result::Result::Ok(
                        AmuletOperationOutcome::COO_CompleteBuyTrafficRequest(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("COO_CompleteBuyTrafficRequest"))?,
                        ),
                    ),
                    "COO_Tap" => ::core::result::Result::Ok(AmuletOperationOutcome::COO_Tap(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("COO_Tap"))?,
                    )),
                    "COO_Error" => ::core::result::Result::Ok(AmuletOperationOutcome::COO_Error(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("COO_Error"))?,
                    )),
                    "ExtAmuletOperationOutcome" => ::core::result::Result::Ok(
                        AmuletOperationOutcome::ExtAmuletOperationOutcome(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("ExtAmuletOperationOutcome"))?,
                        ),
                    ),
                    "COO_CreateExternalPartySetupProposal" => ::core::result::Result::Ok(
                        AmuletOperationOutcome::COO_CreateExternalPartySetupProposal(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("COO_CreateExternalPartySetupProposal"))?,
                        ),
                    ),
                    "COO_AcceptTransferPreapprovalProposal" => ::core::result::Result::Ok(
                        AmuletOperationOutcome::COO_AcceptTransferPreapprovalProposal(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("COO_AcceptTransferPreapprovalProposal"))?,
                        ),
                    ),
                    "COO_RenewTransferPreapproval" => ::core::result::Result::Ok(
                        AmuletOperationOutcome::COO_RenewTransferPreapproval(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("COO_RenewTransferPreapproval"))?,
                        ),
                    ),
                    "COO_TransferPreapprovalSend" => ::core::result::Result::Ok(
                        AmuletOperationOutcome::COO_TransferPreapprovalSend(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("COO_TransferPreapprovalSend"))?,
                        ),
                    ),
                    other => ::core::result::Result::Err(rt::unexpected_constructor(
                        "AmuletOperationOutcome",
                        other,
                    )),
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletOperationOutcome_ExtAmuletOperationOutcome {
            ///Daml field `dummyUnitField`.
            #[serde(rename = "dummyUnitField")]
            pub dummy_unit_field: rt::Unit,
        }
        impl rt::ToValue for AmuletOperationOutcome_ExtAmuletOperationOutcome {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "dummyUnitField",
                    rt::ToValue::to_value(&self.dummy_unit_field)
                ),])
            }
        }
        impl rt::FromValue for AmuletOperationOutcome_ExtAmuletOperationOutcome {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dummy_unit_field: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "dummyUnitField",
                    )?)
                    .map_err(|e| e.at("dummyUnitField"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum AmuletOperation {
            #[serde(rename = "CO_AppPayment")]
            CO_AppPayment(
                rt::ContractId<
                    ::canton_splice_wallet_payments::splice_wallet_payments::Splice_Wallet_Payment::AppPaymentRequest,
                >,
            ),
            #[serde(rename = "CO_CompleteAcceptedTransfer")]
            CO_CompleteAcceptedTransfer(
                rt::ContractId<
                    crate::splice_wallet::Splice_Wallet_TransferOffer::AcceptedTransferOffer,
                >,
            ),
            #[serde(rename = "CO_SubscriptionAcceptAndMakeInitialPayment")]
            CO_SubscriptionAcceptAndMakeInitialPayment(
                rt::ContractId<
                    ::canton_splice_wallet_payments::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionRequest,
                >,
            ),
            #[serde(rename = "CO_SubscriptionMakePayment")]
            CO_SubscriptionMakePayment(
                rt::ContractId<
                    ::canton_splice_wallet_payments::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionIdleState,
                >,
            ),
            #[serde(rename = "CO_MergeTransferInputs")]
            CO_MergeTransferInputs(rt::Unit),
            #[serde(rename = "CO_BuyMemberTraffic")]
            CO_BuyMemberTraffic(
                crate::splice_wallet::Splice_Wallet_Install::AmuletOperation_CO_BuyMemberTraffic,
            ),
            #[serde(rename = "CO_CompleteBuyTrafficRequest")]
            CO_CompleteBuyTrafficRequest(
                crate::splice_wallet::Splice_Wallet_Install::AmuletOperation_CO_CompleteBuyTrafficRequest,
            ),
            #[serde(rename = "CO_Tap")]
            CO_Tap(crate::splice_wallet::Splice_Wallet_Install::AmuletOperation_CO_Tap),
            #[serde(rename = "ExtAmuletOperation")]
            ExtAmuletOperation(
                crate::splice_wallet::Splice_Wallet_Install::AmuletOperation_ExtAmuletOperation,
            ),
            #[serde(rename = "CO_CreateExternalPartySetupProposal")]
            CO_CreateExternalPartySetupProposal(
                crate::splice_wallet::Splice_Wallet_Install::AmuletOperation_CO_CreateExternalPartySetupProposal,
            ),
            #[serde(rename = "CO_AcceptTransferPreapprovalProposal")]
            CO_AcceptTransferPreapprovalProposal(
                crate::splice_wallet::Splice_Wallet_Install::AmuletOperation_CO_AcceptTransferPreapprovalProposal,
            ),
            #[serde(rename = "CO_RenewTransferPreapproval")]
            CO_RenewTransferPreapproval(
                crate::splice_wallet::Splice_Wallet_Install::AmuletOperation_CO_RenewTransferPreapproval,
            ),
            #[serde(rename = "CO_TransferPreapprovalSend")]
            CO_TransferPreapprovalSend(
                crate::splice_wallet::Splice_Wallet_Install::AmuletOperation_CO_TransferPreapprovalSend,
            ),
        }
        impl rt::ToValue for AmuletOperation {
            fn to_value(&self) -> rt::Value {
                match self {
                    AmuletOperation::CO_AppPayment(inner) => {
                        rt::variant_value("CO_AppPayment", rt::ToValue::to_value(inner))
                    }
                    AmuletOperation::CO_CompleteAcceptedTransfer(inner) => rt::variant_value(
                        "CO_CompleteAcceptedTransfer",
                        rt::ToValue::to_value(inner),
                    ),
                    AmuletOperation::CO_SubscriptionAcceptAndMakeInitialPayment(inner) => {
                        rt::variant_value(
                            "CO_SubscriptionAcceptAndMakeInitialPayment",
                            rt::ToValue::to_value(inner),
                        )
                    }
                    AmuletOperation::CO_SubscriptionMakePayment(inner) => rt::variant_value(
                        "CO_SubscriptionMakePayment",
                        rt::ToValue::to_value(inner),
                    ),
                    AmuletOperation::CO_MergeTransferInputs(inner) => {
                        rt::variant_value("CO_MergeTransferInputs", rt::ToValue::to_value(inner))
                    }
                    AmuletOperation::CO_BuyMemberTraffic(inner) => {
                        rt::variant_value("CO_BuyMemberTraffic", rt::ToValue::to_value(inner))
                    }
                    AmuletOperation::CO_CompleteBuyTrafficRequest(inner) => rt::variant_value(
                        "CO_CompleteBuyTrafficRequest",
                        rt::ToValue::to_value(inner),
                    ),
                    AmuletOperation::CO_Tap(inner) => {
                        rt::variant_value("CO_Tap", rt::ToValue::to_value(inner))
                    }
                    AmuletOperation::ExtAmuletOperation(inner) => {
                        rt::variant_value("ExtAmuletOperation", rt::ToValue::to_value(inner))
                    }
                    AmuletOperation::CO_CreateExternalPartySetupProposal(inner) => {
                        rt::variant_value(
                            "CO_CreateExternalPartySetupProposal",
                            rt::ToValue::to_value(inner),
                        )
                    }
                    AmuletOperation::CO_AcceptTransferPreapprovalProposal(inner) => {
                        rt::variant_value(
                            "CO_AcceptTransferPreapprovalProposal",
                            rt::ToValue::to_value(inner),
                        )
                    }
                    AmuletOperation::CO_RenewTransferPreapproval(inner) => rt::variant_value(
                        "CO_RenewTransferPreapproval",
                        rt::ToValue::to_value(inner),
                    ),
                    AmuletOperation::CO_TransferPreapprovalSend(inner) => rt::variant_value(
                        "CO_TransferPreapprovalSend",
                        rt::ToValue::to_value(inner),
                    ),
                }
            }
        }
        impl rt::FromValue for AmuletOperation {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                let (constructor, payload) = rt::variant_parts(value)?;
                match constructor {
                    "CO_AppPayment" => ::core::result::Result::Ok(AmuletOperation::CO_AppPayment(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("CO_AppPayment"))?,
                    )),
                    "CO_CompleteAcceptedTransfer" => {
                        ::core::result::Result::Ok(AmuletOperation::CO_CompleteAcceptedTransfer(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("CO_CompleteAcceptedTransfer"))?,
                        ))
                    }
                    "CO_SubscriptionAcceptAndMakeInitialPayment" => ::core::result::Result::Ok(
                        AmuletOperation::CO_SubscriptionAcceptAndMakeInitialPayment(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("CO_SubscriptionAcceptAndMakeInitialPayment"))?,
                        ),
                    ),
                    "CO_SubscriptionMakePayment" => {
                        ::core::result::Result::Ok(AmuletOperation::CO_SubscriptionMakePayment(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("CO_SubscriptionMakePayment"))?,
                        ))
                    }
                    "CO_MergeTransferInputs" => {
                        ::core::result::Result::Ok(AmuletOperation::CO_MergeTransferInputs(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("CO_MergeTransferInputs"))?,
                        ))
                    }
                    "CO_BuyMemberTraffic" => {
                        ::core::result::Result::Ok(AmuletOperation::CO_BuyMemberTraffic(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("CO_BuyMemberTraffic"))?,
                        ))
                    }
                    "CO_CompleteBuyTrafficRequest" => {
                        ::core::result::Result::Ok(AmuletOperation::CO_CompleteBuyTrafficRequest(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("CO_CompleteBuyTrafficRequest"))?,
                        ))
                    }
                    "CO_Tap" => ::core::result::Result::Ok(AmuletOperation::CO_Tap(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("CO_Tap"))?,
                    )),
                    "ExtAmuletOperation" => {
                        ::core::result::Result::Ok(AmuletOperation::ExtAmuletOperation(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("ExtAmuletOperation"))?,
                        ))
                    }
                    "CO_CreateExternalPartySetupProposal" => ::core::result::Result::Ok(
                        AmuletOperation::CO_CreateExternalPartySetupProposal(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("CO_CreateExternalPartySetupProposal"))?,
                        ),
                    ),
                    "CO_AcceptTransferPreapprovalProposal" => ::core::result::Result::Ok(
                        AmuletOperation::CO_AcceptTransferPreapprovalProposal(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("CO_AcceptTransferPreapprovalProposal"))?,
                        ),
                    ),
                    "CO_RenewTransferPreapproval" => {
                        ::core::result::Result::Ok(AmuletOperation::CO_RenewTransferPreapproval(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("CO_RenewTransferPreapproval"))?,
                        ))
                    }
                    "CO_TransferPreapprovalSend" => {
                        ::core::result::Result::Ok(AmuletOperation::CO_TransferPreapprovalSend(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("CO_TransferPreapprovalSend"))?,
                        ))
                    }
                    other => ::core::result::Result::Err(rt::unexpected_constructor(
                        "AmuletOperation",
                        other,
                    )),
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletOperation_CO_BuyMemberTraffic {
            ///Daml field `trafficAmount`.
            #[serde(rename = "trafficAmount")]
            pub traffic_amount: rt::Int64,
            ///Daml field `memberId`.
            #[serde(rename = "memberId")]
            pub member_id: ::std::string::String,
            ///Daml field `synchronizerId`.
            #[serde(rename = "synchronizerId")]
            pub synchronizer_id: ::std::string::String,
            ///Daml field `migrationId`.
            #[serde(rename = "migrationId")]
            pub migration_id: rt::Int64,
            ///Daml field `minTopupInterval`.
            #[serde(rename = "minTopupInterval")]
            pub min_topup_interval:
                ::canton_daml_stdlib::daml_stdlib_DA_Time_Types::DA_Time_Types::RelTime,
            ///Daml field `topupStateCid`.
            #[serde(rename = "topupStateCid")]
            pub topup_state_cid: ::core::option::Option<
                rt::ContractId<crate::splice_wallet::Splice_Wallet_TopUpState::ValidatorTopUpState>,
            >,
        }
        impl rt::ToValue for AmuletOperation_CO_BuyMemberTraffic {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("trafficAmount", rt::ToValue::to_value(&self.traffic_amount)),
                    ("memberId", rt::ToValue::to_value(&self.member_id)),
                    (
                        "synchronizerId",
                        rt::ToValue::to_value(&self.synchronizer_id)
                    ),
                    ("migrationId", rt::ToValue::to_value(&self.migration_id)),
                    (
                        "minTopupInterval",
                        rt::ToValue::to_value(&self.min_topup_interval)
                    ),
                    (
                        "topupStateCid",
                        rt::ToValue::to_value(&self.topup_state_cid)
                    ),
                ])
            }
        }
        impl rt::FromValue for AmuletOperation_CO_BuyMemberTraffic {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    traffic_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "trafficAmount",
                    )?)
                    .map_err(|e| e.at("trafficAmount"))?,
                    member_id: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "memberId",
                    )?)
                    .map_err(|e| e.at("memberId"))?,
                    synchronizer_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "synchronizerId",
                    )?)
                    .map_err(|e| e.at("synchronizerId"))?,
                    migration_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "migrationId",
                    )?)
                    .map_err(|e| e.at("migrationId"))?,
                    min_topup_interval: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "minTopupInterval",
                    )?)
                    .map_err(|e| e.at("minTopupInterval"))?,
                    topup_state_cid: rt::optional_field(value, 5usize, "topupStateCid")
                        .map_err(|e| e.at("topupStateCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletOperation_CO_CompleteBuyTrafficRequest {
            ///Daml field `trafficRequestCid`.
            #[serde(rename = "trafficRequestCid")]
            pub traffic_request_cid: rt::ContractId<
                crate::splice_wallet::Splice_Wallet_BuyTrafficRequest::BuyTrafficRequest,
            >,
        }
        impl rt::ToValue for AmuletOperation_CO_CompleteBuyTrafficRequest {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "trafficRequestCid",
                    rt::ToValue::to_value(&self.traffic_request_cid)
                ),])
            }
        }
        impl rt::FromValue for AmuletOperation_CO_CompleteBuyTrafficRequest {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    traffic_request_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "trafficRequestCid",
                    )?)
                    .map_err(|e| e.at("trafficRequestCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletOperation_CO_Tap {
            ///Daml field `tapAmount`.
            #[serde(rename = "tapAmount")]
            pub tap_amount: rt::Numeric,
        }
        impl rt::ToValue for AmuletOperation_CO_Tap {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "tapAmount",
                    rt::ToValue::to_value(&self.tap_amount)
                ),])
            }
        }
        impl rt::FromValue for AmuletOperation_CO_Tap {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    tap_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "tapAmount",
                    )?)
                    .map_err(|e| e.at("tapAmount"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletOperation_ExtAmuletOperation {
            ///Daml field `dummyUnitField`.
            #[serde(rename = "dummyUnitField")]
            pub dummy_unit_field: rt::Unit,
        }
        impl rt::ToValue for AmuletOperation_ExtAmuletOperation {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "dummyUnitField",
                    rt::ToValue::to_value(&self.dummy_unit_field)
                ),])
            }
        }
        impl rt::FromValue for AmuletOperation_ExtAmuletOperation {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dummy_unit_field: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "dummyUnitField",
                    )?)
                    .map_err(|e| e.at("dummyUnitField"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletOperation_CO_CreateExternalPartySetupProposal {
            ///Daml field `externalParty`.
            #[serde(rename = "externalParty")]
            pub external_party: rt::Party,
            ///Daml field `preapprovalExpiresAt`.
            #[serde(rename = "preapprovalExpiresAt")]
            pub preapproval_expires_at: rt::Timestamp,
        }
        impl rt::ToValue for AmuletOperation_CO_CreateExternalPartySetupProposal {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("externalParty", rt::ToValue::to_value(&self.external_party)),
                    (
                        "preapprovalExpiresAt",
                        rt::ToValue::to_value(&self.preapproval_expires_at)
                    ),
                ])
            }
        }
        impl rt::FromValue for AmuletOperation_CO_CreateExternalPartySetupProposal {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    external_party: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "externalParty",
                    )?)
                    .map_err(|e| e.at("externalParty"))?,
                    preapproval_expires_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "preapprovalExpiresAt",
                    )?)
                    .map_err(|e| e.at("preapprovalExpiresAt"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletOperation_CO_AcceptTransferPreapprovalProposal {
            ///Daml field `preapprovalProposalCid`.
            #[serde(rename = "preapprovalProposalCid")]
            pub preapproval_proposal_cid: rt::ContractId<
                crate::splice_wallet::Splice_Wallet_TransferPreapproval::TransferPreapprovalProposal,
            >,
            ///Daml field `expiresAt`.
            #[serde(rename = "expiresAt")]
            pub expires_at: rt::Timestamp,
        }
        impl rt::ToValue for AmuletOperation_CO_AcceptTransferPreapprovalProposal {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "preapprovalProposalCid",
                        rt::ToValue::to_value(&self.preapproval_proposal_cid)
                    ),
                    ("expiresAt", rt::ToValue::to_value(&self.expires_at)),
                ])
            }
        }
        impl rt::FromValue for AmuletOperation_CO_AcceptTransferPreapprovalProposal {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    preapproval_proposal_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "preapprovalProposalCid",
                    )?)
                    .map_err(|e| e.at("preapprovalProposalCid"))?,
                    expires_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "expiresAt",
                    )?)
                    .map_err(|e| e.at("expiresAt"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletOperation_CO_RenewTransferPreapproval {
            ///Daml field `previousApprovalCid`.
            #[serde(rename = "previousApprovalCid")]
            pub previous_approval_cid: rt::ContractId<
                ::canton_splice_amulet::splice_amulet::Splice_AmuletRules::TransferPreapproval,
            >,
            ///Daml field `newExpiresAt`.
            #[serde(rename = "newExpiresAt")]
            pub new_expires_at: rt::Timestamp,
        }
        impl rt::ToValue for AmuletOperation_CO_RenewTransferPreapproval {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "previousApprovalCid",
                        rt::ToValue::to_value(&self.previous_approval_cid)
                    ),
                    ("newExpiresAt", rt::ToValue::to_value(&self.new_expires_at)),
                ])
            }
        }
        impl rt::FromValue for AmuletOperation_CO_RenewTransferPreapproval {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    previous_approval_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "previousApprovalCid",
                    )?)
                    .map_err(|e| e.at("previousApprovalCid"))?,
                    new_expires_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "newExpiresAt",
                    )?)
                    .map_err(|e| e.at("newExpiresAt"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletOperation_CO_TransferPreapprovalSend {
            ///Daml field `transferPreapprovalCid`.
            #[serde(rename = "transferPreapprovalCid")]
            pub transfer_preapproval_cid: rt::ContractId<
                ::canton_splice_amulet::splice_amulet::Splice_AmuletRules::TransferPreapproval,
            >,
            ///Daml field `providerFeaturedAppRightCid`.
            #[serde(rename = "providerFeaturedAppRightCid")]
            pub provider_featured_app_right_cid: ::core::option::Option<
                rt::ContractId<
                    ::canton_splice_amulet::splice_amulet::Splice_Amulet::FeaturedAppRight,
                >,
            >,
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            #[serde(rename = "description")]
            pub description: ::core::option::Option<::std::string::String>,
        }
        impl rt::ToValue for AmuletOperation_CO_TransferPreapprovalSend {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "transferPreapprovalCid",
                        rt::ToValue::to_value(&self.transfer_preapproval_cid)
                    ),
                    (
                        "providerFeaturedAppRightCid",
                        rt::ToValue::to_value(&self.provider_featured_app_right_cid)
                    ),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                    ("description", rt::ToValue::to_value(&self.description)),
                ])
            }
        }
        impl rt::FromValue for AmuletOperation_CO_TransferPreapprovalSend {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    transfer_preapproval_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "transferPreapprovalCid",
                    )?)
                    .map_err(|e| e.at("transferPreapprovalCid"))?,
                    provider_featured_app_right_cid: rt::optional_field(
                        value,
                        1usize,
                        "providerFeaturedAppRightCid",
                    )
                    .map_err(|e| e.at("providerFeaturedAppRightCid"))?,
                    amount: rt::FromValue::from_value(rt::required_field(value, 2usize, "amount")?)
                        .map_err(|e| e.at("amount"))?,
                    description: rt::optional_field(value, 3usize, "description")
                        .map_err(|e| e.at("description"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ExecutionContext {
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///Daml field `endUser`.
            #[serde(rename = "endUser")]
            pub end_user: rt::Party,
            #[serde(rename = "validator")]
            pub validator: rt::Party,
            ///Daml field `paymentContext`.
            #[serde(rename = "paymentContext")]
            pub payment_context:
                ::canton_splice_amulet::splice_amulet::Splice_AmuletRules::PaymentTransferContext,
        }
        impl rt::ToValue for ExecutionContext {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("endUser", rt::ToValue::to_value(&self.end_user)),
                    ("validator", rt::ToValue::to_value(&self.validator)),
                    (
                        "paymentContext",
                        rt::ToValue::to_value(&self.payment_context)
                    ),
                ])
            }
        }
        impl rt::FromValue for ExecutionContext {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                    end_user: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "endUser",
                    )?)
                    .map_err(|e| e.at("endUser"))?,
                    validator: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "validator",
                    )?)
                    .map_err(|e| e.at("validator"))?,
                    payment_context: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "paymentContext",
                    )?)
                    .map_err(|e| e.at("paymentContext"))?,
                })
            }
        }
        ///The Daml template `Splice.Wallet.Install:WalletAppInstall`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-wallet:Splice.Wallet.Install:WalletAppInstall`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `WalletAppInstall_ExecuteBatch` — non-consuming
        ///- `WalletAppInstall_AppPaymentRequest_Reject` — non-consuming
        ///- `WalletAppInstall_AppPaymentRequest_Expire` — non-consuming
        ///- `WalletAppInstall_SubscriptionRequest_Reject` — non-consuming
        ///- `WalletAppInstall_SubscriptionIdleState_CancelSubscription` — non-consuming
        ///- `WalletAppInstall_CreateTransferOffer` — non-consuming
        ///- `WalletAppInstall_TransferOffer_Accept` — non-consuming
        ///- `WalletAppInstall_TransferOffer_Reject` — non-consuming
        ///- `WalletAppInstall_TransferOffer_Withdraw` — non-consuming
        ///- `WalletAppInstall_TransferOffer_Expire` — non-consuming
        ///- `WalletAppInstall_AcceptedTransferOffer_Abort` — non-consuming
        ///- `WalletAppInstall_AcceptedTransferOffer_Withdraw` — non-consuming
        ///- `WalletAppInstall_AcceptedTransferOffer_Expire` — non-consuming
        ///- `WalletAppInstall_CreateBuyTrafficRequest` — non-consuming
        ///- `WalletAppInstall_BuyTrafficRequest_Cancel` — non-consuming
        ///- `WalletAppInstall_BuyTrafficRequest_Expire` — non-consuming
        ///- `WalletAppInstall_FeaturedAppRights_Cancel` — non-consuming
        ///- `WalletAppInstall_FeaturedAppRights_SelfGrant` — non-consuming
        ///- `WalletAppInstall_TransferFactory_Transfer` — non-consuming
        ///- `WalletAppInstall_TransferInstruction_Accept` — non-consuming
        ///- `WalletAppInstall_TransferInstruction_Reject` — non-consuming
        ///- `WalletAppInstall_TransferInstruction_Withdraw` — non-consuming
        ///- `WalletAppInstall_AllocationFactory_Allocate` — non-consuming
        ///- `Archive` — consuming
        ///- `WalletAppInstall_Allocation_Withdraw` — non-consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct WalletAppInstall {
            ///Daml field `dsoParty`.
            #[serde(rename = "dsoParty")]
            pub dso_party: rt::Party,
            ///Daml field `validatorParty`.
            #[serde(rename = "validatorParty")]
            pub validator_party: rt::Party,
            ///Daml field `endUserName`.
            #[serde(rename = "endUserName")]
            pub end_user_name: ::std::string::String,
            ///Daml field `endUserParty`.
            #[serde(rename = "endUserParty")]
            pub end_user_party: rt::Party,
        }
        impl rt::ToValue for WalletAppInstall {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("dsoParty", rt::ToValue::to_value(&self.dso_party)),
                    (
                        "validatorParty",
                        rt::ToValue::to_value(&self.validator_party)
                    ),
                    ("endUserName", rt::ToValue::to_value(&self.end_user_name)),
                    ("endUserParty", rt::ToValue::to_value(&self.end_user_party)),
                ])
            }
        }
        impl rt::FromValue for WalletAppInstall {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dso_party: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "dsoParty",
                    )?)
                    .map_err(|e| e.at("dsoParty"))?,
                    validator_party: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "validatorParty",
                    )?)
                    .map_err(|e| e.at("validatorParty"))?,
                    end_user_name: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "endUserName",
                    )?)
                    .map_err(|e| e.at("endUserName"))?,
                    end_user_party: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "endUserParty",
                    )?)
                    .map_err(|e| e.at("endUserParty"))?,
                })
            }
        }
        impl rt::Contract for WalletAppInstall {
            const PACKAGE_ID: &'static str =
                "690c1d47bac06db419db344d59a7a30c53fa3f5d961943fe1782cfc6c78794d8";
            const PACKAGE_NAME: &'static str = "splice-wallet";
            const MODULE_NAME: &'static str = "Splice.Wallet.Install";
            const ENTITY_NAME: &'static str = "WalletAppInstall";
        }
        impl rt::Template for WalletAppInstall {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("dsoParty", rt::ToValue::to_value(&self.dso_party)),
                    (
                        "validatorParty",
                        rt::ToValue::to_value(&self.validator_party)
                    ),
                    ("endUserName", rt::ToValue::to_value(&self.end_user_name)),
                    ("endUserParty", rt::ToValue::to_value(&self.end_user_party)),
                ])
            }
        }
        ///The `WalletAppInstall_ExecuteBatch` choice on [`WalletAppInstall`] (non-consuming).
        impl rt::Choice<WalletAppInstall>
            for crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_ExecuteBatch
        {
            type Return =
                crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_ExecuteBatchResult;
            const NAME: &'static str = "WalletAppInstall_ExecuteBatch";
            const CONSUMING: bool = false;
        }
        ///The `WalletAppInstall_AppPaymentRequest_Reject` choice on [`WalletAppInstall`] (non-consuming).
        impl rt::Choice<WalletAppInstall>
        for crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_AppPaymentRequest_Reject {
            type Return = crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_AppPaymentRequest_RejectResult;
            const NAME: &'static str = "WalletAppInstall_AppPaymentRequest_Reject";
            const CONSUMING: bool = false;
        }
        ///The `WalletAppInstall_AppPaymentRequest_Expire` choice on [`WalletAppInstall`] (non-consuming).
        impl rt::Choice<WalletAppInstall>
        for crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_AppPaymentRequest_Expire {
            type Return = crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_AppPaymentRequest_ExpireResult;
            const NAME: &'static str = "WalletAppInstall_AppPaymentRequest_Expire";
            const CONSUMING: bool = false;
        }
        ///The `WalletAppInstall_SubscriptionRequest_Reject` choice on [`WalletAppInstall`] (non-consuming).
        impl rt::Choice<WalletAppInstall>
        for crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_SubscriptionRequest_Reject {
            type Return = crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_SubscriptionRequest_RejectResult;
            const NAME: &'static str = "WalletAppInstall_SubscriptionRequest_Reject";
            const CONSUMING: bool = false;
        }
        ///The `WalletAppInstall_SubscriptionIdleState_CancelSubscription` choice on [`WalletAppInstall`] (non-consuming).
        impl rt::Choice<WalletAppInstall>
        for crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_SubscriptionIdleState_CancelSubscription {
            type Return = crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_SubscriptionIdleState_CancelSubscriptionResult;
            const NAME: &'static str = "WalletAppInstall_SubscriptionIdleState_CancelSubscription";
            const CONSUMING: bool = false;
        }
        ///The `WalletAppInstall_CreateTransferOffer` choice on [`WalletAppInstall`] (non-consuming).
        impl rt::Choice<WalletAppInstall>
            for crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_CreateTransferOffer
        {
            type Return = crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_CreateTransferOfferResult;
            const NAME: &'static str = "WalletAppInstall_CreateTransferOffer";
            const CONSUMING: bool = false;
        }
        ///The `WalletAppInstall_TransferOffer_Accept` choice on [`WalletAppInstall`] (non-consuming).
        impl rt::Choice<WalletAppInstall>
            for crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_TransferOffer_Accept
        {
            type Return = crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_TransferOffer_AcceptResult;
            const NAME: &'static str = "WalletAppInstall_TransferOffer_Accept";
            const CONSUMING: bool = false;
        }
        ///The `WalletAppInstall_TransferOffer_Reject` choice on [`WalletAppInstall`] (non-consuming).
        impl rt::Choice<WalletAppInstall>
            for crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_TransferOffer_Reject
        {
            type Return = crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_TransferOffer_RejectResult;
            const NAME: &'static str = "WalletAppInstall_TransferOffer_Reject";
            const CONSUMING: bool = false;
        }
        ///The `WalletAppInstall_TransferOffer_Withdraw` choice on [`WalletAppInstall`] (non-consuming).
        impl rt::Choice<WalletAppInstall>
            for crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_TransferOffer_Withdraw
        {
            type Return = crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_TransferOffer_WithdrawResult;
            const NAME: &'static str = "WalletAppInstall_TransferOffer_Withdraw";
            const CONSUMING: bool = false;
        }
        ///The `WalletAppInstall_TransferOffer_Expire` choice on [`WalletAppInstall`] (non-consuming).
        impl rt::Choice<WalletAppInstall>
            for crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_TransferOffer_Expire
        {
            type Return = crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_TransferOffer_ExpireResult;
            const NAME: &'static str = "WalletAppInstall_TransferOffer_Expire";
            const CONSUMING: bool = false;
        }
        ///The `WalletAppInstall_AcceptedTransferOffer_Abort` choice on [`WalletAppInstall`] (non-consuming).
        impl rt::Choice<WalletAppInstall>
        for crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_AcceptedTransferOffer_Abort {
            type Return = crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_AcceptedTransferOffer_AbortResult;
            const NAME: &'static str = "WalletAppInstall_AcceptedTransferOffer_Abort";
            const CONSUMING: bool = false;
        }
        ///The `WalletAppInstall_AcceptedTransferOffer_Withdraw` choice on [`WalletAppInstall`] (non-consuming).
        impl rt::Choice<WalletAppInstall>
        for crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_AcceptedTransferOffer_Withdraw {
            type Return = crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_AcceptedTransferOffer_WithdrawResult;
            const NAME: &'static str = "WalletAppInstall_AcceptedTransferOffer_Withdraw";
            const CONSUMING: bool = false;
        }
        ///The `WalletAppInstall_AcceptedTransferOffer_Expire` choice on [`WalletAppInstall`] (non-consuming).
        impl rt::Choice<WalletAppInstall>
        for crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_AcceptedTransferOffer_Expire {
            type Return = crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_AcceptedTransferOffer_ExpireResult;
            const NAME: &'static str = "WalletAppInstall_AcceptedTransferOffer_Expire";
            const CONSUMING: bool = false;
        }
        ///The `WalletAppInstall_CreateBuyTrafficRequest` choice on [`WalletAppInstall`] (non-consuming).
        impl rt::Choice<WalletAppInstall>
        for crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_CreateBuyTrafficRequest {
            type Return = crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_CreateBuyTrafficRequestResult;
            const NAME: &'static str = "WalletAppInstall_CreateBuyTrafficRequest";
            const CONSUMING: bool = false;
        }
        ///The `WalletAppInstall_BuyTrafficRequest_Cancel` choice on [`WalletAppInstall`] (non-consuming).
        impl rt::Choice<WalletAppInstall>
        for crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_BuyTrafficRequest_Cancel {
            type Return = crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_BuyTrafficRequest_CancelResult;
            const NAME: &'static str = "WalletAppInstall_BuyTrafficRequest_Cancel";
            const CONSUMING: bool = false;
        }
        ///The `WalletAppInstall_BuyTrafficRequest_Expire` choice on [`WalletAppInstall`] (non-consuming).
        impl rt::Choice<WalletAppInstall>
        for crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_BuyTrafficRequest_Expire {
            type Return = crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_BuyTrafficRequest_ExpireResult;
            const NAME: &'static str = "WalletAppInstall_BuyTrafficRequest_Expire";
            const CONSUMING: bool = false;
        }
        ///The `WalletAppInstall_FeaturedAppRights_Cancel` choice on [`WalletAppInstall`] (non-consuming).
        impl rt::Choice<WalletAppInstall>
        for crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_FeaturedAppRights_Cancel {
            type Return = crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_FeaturedAppRights_CancelResult;
            const NAME: &'static str = "WalletAppInstall_FeaturedAppRights_Cancel";
            const CONSUMING: bool = false;
        }
        ///The `WalletAppInstall_FeaturedAppRights_SelfGrant` choice on [`WalletAppInstall`] (non-consuming).
        impl rt::Choice<WalletAppInstall>
        for crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_FeaturedAppRights_SelfGrant {
            type Return = crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_FeaturedAppRights_SelfGrantResult;
            const NAME: &'static str = "WalletAppInstall_FeaturedAppRights_SelfGrant";
            const CONSUMING: bool = false;
        }
        ///The `WalletAppInstall_TransferFactory_Transfer` choice on [`WalletAppInstall`] (non-consuming).
        impl rt::Choice<WalletAppInstall>
        for crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_TransferFactory_Transfer {
            type Return = ::canton_splice_api_token_transfer_instruction_v1::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferInstructionResult;
            const NAME: &'static str = "WalletAppInstall_TransferFactory_Transfer";
            const CONSUMING: bool = false;
        }
        ///The `WalletAppInstall_TransferInstruction_Accept` choice on [`WalletAppInstall`] (non-consuming).
        impl rt::Choice<WalletAppInstall>
        for crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_TransferInstruction_Accept {
            type Return = ::canton_splice_api_token_transfer_instruction_v1::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferInstructionResult;
            const NAME: &'static str = "WalletAppInstall_TransferInstruction_Accept";
            const CONSUMING: bool = false;
        }
        ///The `WalletAppInstall_TransferInstruction_Reject` choice on [`WalletAppInstall`] (non-consuming).
        impl rt::Choice<WalletAppInstall>
        for crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_TransferInstruction_Reject {
            type Return = ::canton_splice_api_token_transfer_instruction_v1::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferInstructionResult;
            const NAME: &'static str = "WalletAppInstall_TransferInstruction_Reject";
            const CONSUMING: bool = false;
        }
        ///The `WalletAppInstall_TransferInstruction_Withdraw` choice on [`WalletAppInstall`] (non-consuming).
        impl rt::Choice<WalletAppInstall>
        for crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_TransferInstruction_Withdraw {
            type Return = ::canton_splice_api_token_transfer_instruction_v1::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferInstructionResult;
            const NAME: &'static str = "WalletAppInstall_TransferInstruction_Withdraw";
            const CONSUMING: bool = false;
        }
        ///The `WalletAppInstall_AllocationFactory_Allocate` choice on [`WalletAppInstall`] (non-consuming).
        impl rt::Choice<WalletAppInstall>
        for crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_AllocationFactory_Allocate {
            type Return = ::canton_splice_api_token_allocation_instruction_v1::splice_api_token_allocation_instruction_v1::Splice_Api_Token_AllocationInstructionV1::AllocationInstructionResult;
            const NAME: &'static str = "WalletAppInstall_AllocationFactory_Allocate";
            const CONSUMING: bool = false;
        }
        ///The `Archive` choice on [`WalletAppInstall`] (consuming).
        impl rt::Choice<WalletAppInstall>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `WalletAppInstall_Allocation_Withdraw` choice on [`WalletAppInstall`] (non-consuming).
        impl rt::Choice<WalletAppInstall>
            for crate::splice_wallet::Splice_Wallet_Install::WalletAppInstall_Allocation_Withdraw
        {
            type Return = ::canton_splice_api_token_allocation_v1::splice_api_token_allocation_v1::Splice_Api_Token_AllocationV1::Allocation_WithdrawResult;
            const NAME: &'static str = "WalletAppInstall_Allocation_Withdraw";
            const CONSUMING: bool = false;
        }
    }
    pub mod Splice_Wallet_TransferOffer {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AcceptedTransferOffer_Expire {
            #[serde(rename = "actor")]
            pub actor: rt::Party,
        }
        impl rt::ToValue for AcceptedTransferOffer_Expire {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("actor", rt::ToValue::to_value(&self.actor)),])
            }
        }
        impl rt::FromValue for AcceptedTransferOffer_Expire {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    actor: rt::FromValue::from_value(rt::required_field(value, 0usize, "actor")?)
                        .map_err(|e| e.at("actor"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AcceptedTransferOffer_Abort {
            #[serde(rename = "reason")]
            pub reason: ::std::string::String,
        }
        impl rt::ToValue for AcceptedTransferOffer_Abort {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("reason", rt::ToValue::to_value(&self.reason)),])
            }
        }
        impl rt::FromValue for AcceptedTransferOffer_Abort {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    reason: rt::FromValue::from_value(rt::required_field(value, 0usize, "reason")?)
                        .map_err(|e| e.at("reason"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AcceptedTransferOffer_Withdraw {
            #[serde(rename = "reason")]
            pub reason: ::std::string::String,
        }
        impl rt::ToValue for AcceptedTransferOffer_Withdraw {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("reason", rt::ToValue::to_value(&self.reason)),])
            }
        }
        impl rt::FromValue for AcceptedTransferOffer_Withdraw {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    reason: rt::FromValue::from_value(rt::required_field(value, 0usize, "reason")?)
                        .map_err(|e| e.at("reason"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AcceptedTransferOffer_Complete {
            #[serde(rename = "inputs")]
            pub inputs: ::std::vec::Vec<
                ::canton_splice_amulet::splice_amulet::Splice_AmuletRules::TransferInput,
            >,
            ///Daml field `transferContext`.
            #[serde(rename = "transferContext")]
            pub transfer_context:
                ::canton_splice_amulet::splice_amulet::Splice_AmuletRules::PaymentTransferContext,
            ///Daml field `walletProvider`.
            #[serde(rename = "walletProvider")]
            pub wallet_provider: rt::Party,
        }
        impl rt::ToValue for AcceptedTransferOffer_Complete {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("inputs", rt::ToValue::to_value(&self.inputs)),
                    (
                        "transferContext",
                        rt::ToValue::to_value(&self.transfer_context)
                    ),
                    (
                        "walletProvider",
                        rt::ToValue::to_value(&self.wallet_provider)
                    ),
                ])
            }
        }
        impl rt::FromValue for AcceptedTransferOffer_Complete {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    inputs: rt::FromValue::from_value(rt::required_field(value, 0usize, "inputs")?)
                        .map_err(|e| e.at("inputs"))?,
                    transfer_context: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "transferContext",
                    )?)
                    .map_err(|e| e.at("transferContext"))?,
                    wallet_provider: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "walletProvider",
                    )?)
                    .map_err(|e| e.at("walletProvider"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AcceptedTransferOffer_CompleteResult {
            ///Daml field `transferResult`.
            #[serde(rename = "transferResult")]
            pub transfer_result:
                ::canton_splice_amulet::splice_amulet::Splice_AmuletRules::TransferResult,
            ///Daml field `trackingInfo`.
            #[serde(rename = "trackingInfo")]
            pub tracking_info:
                crate::splice_wallet::Splice_Wallet_TransferOffer::TransferOfferTrackingInfo,
            ///Daml field `senderChangeAmulet`.
            #[serde(rename = "senderChangeAmulet")]
            pub sender_change_amulet: ::core::option::Option<
                rt::ContractId<::canton_splice_amulet::splice_amulet::Splice_Amulet::Amulet>,
            >,
        }
        impl rt::ToValue for AcceptedTransferOffer_CompleteResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "transferResult",
                        rt::ToValue::to_value(&self.transfer_result)
                    ),
                    ("trackingInfo", rt::ToValue::to_value(&self.tracking_info)),
                    (
                        "senderChangeAmulet",
                        rt::ToValue::to_value(&self.sender_change_amulet)
                    ),
                ])
            }
        }
        impl rt::FromValue for AcceptedTransferOffer_CompleteResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    transfer_result: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "transferResult",
                    )?)
                    .map_err(|e| e.at("transferResult"))?,
                    tracking_info: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "trackingInfo",
                    )?)
                    .map_err(|e| e.at("trackingInfo"))?,
                    sender_change_amulet: rt::optional_field(value, 2usize, "senderChangeAmulet")
                        .map_err(|e| e.at("senderChangeAmulet"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferOfferTrackingInfo {
            ///Daml field `trackingId`.
            #[serde(rename = "trackingId")]
            pub tracking_id: ::std::string::String,
            #[serde(rename = "sender")]
            pub sender: rt::Party,
            #[serde(rename = "receiver")]
            pub receiver: rt::Party,
        }
        impl rt::ToValue for TransferOfferTrackingInfo {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("trackingId", rt::ToValue::to_value(&self.tracking_id)),
                    ("sender", rt::ToValue::to_value(&self.sender)),
                    ("receiver", rt::ToValue::to_value(&self.receiver)),
                ])
            }
        }
        impl rt::FromValue for TransferOfferTrackingInfo {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    tracking_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "trackingId",
                    )?)
                    .map_err(|e| e.at("trackingId"))?,
                    sender: rt::FromValue::from_value(rt::required_field(value, 1usize, "sender")?)
                        .map_err(|e| e.at("sender"))?,
                    receiver: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "receiver",
                    )?)
                    .map_err(|e| e.at("receiver"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferOffer_Expire {
            #[serde(rename = "actor")]
            pub actor: rt::Party,
        }
        impl rt::ToValue for TransferOffer_Expire {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("actor", rt::ToValue::to_value(&self.actor)),])
            }
        }
        impl rt::FromValue for TransferOffer_Expire {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    actor: rt::FromValue::from_value(rt::required_field(value, 0usize, "actor")?)
                        .map_err(|e| e.at("actor"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferOffer_Withdraw {
            #[serde(rename = "reason")]
            pub reason: ::std::string::String,
        }
        impl rt::ToValue for TransferOffer_Withdraw {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("reason", rt::ToValue::to_value(&self.reason)),])
            }
        }
        impl rt::FromValue for TransferOffer_Withdraw {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    reason: rt::FromValue::from_value(rt::required_field(value, 0usize, "reason")?)
                        .map_err(|e| e.at("reason"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferOffer_Reject {}
        impl rt::ToValue for TransferOffer_Reject {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for TransferOffer_Reject {
            fn from_value(_value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferOffer_Accept {}
        impl rt::ToValue for TransferOffer_Accept {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for TransferOffer_Accept {
            fn from_value(_value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AcceptedTransferOffer_ExpireResult {
            ///Daml field `trackingInfo`.
            #[serde(rename = "trackingInfo")]
            pub tracking_info:
                crate::splice_wallet::Splice_Wallet_TransferOffer::TransferOfferTrackingInfo,
        }
        impl rt::ToValue for AcceptedTransferOffer_ExpireResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "trackingInfo",
                    rt::ToValue::to_value(&self.tracking_info)
                ),])
            }
        }
        impl rt::FromValue for AcceptedTransferOffer_ExpireResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    tracking_info: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "trackingInfo",
                    )?)
                    .map_err(|e| e.at("trackingInfo"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AcceptedTransferOffer_AbortResult {
            ///Daml field `trackingInfo`.
            #[serde(rename = "trackingInfo")]
            pub tracking_info:
                crate::splice_wallet::Splice_Wallet_TransferOffer::TransferOfferTrackingInfo,
        }
        impl rt::ToValue for AcceptedTransferOffer_AbortResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "trackingInfo",
                    rt::ToValue::to_value(&self.tracking_info)
                ),])
            }
        }
        impl rt::FromValue for AcceptedTransferOffer_AbortResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    tracking_info: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "trackingInfo",
                    )?)
                    .map_err(|e| e.at("trackingInfo"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AcceptedTransferOffer_WithdrawResult {
            ///Daml field `trackingInfo`.
            #[serde(rename = "trackingInfo")]
            pub tracking_info:
                crate::splice_wallet::Splice_Wallet_TransferOffer::TransferOfferTrackingInfo,
        }
        impl rt::ToValue for AcceptedTransferOffer_WithdrawResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "trackingInfo",
                    rt::ToValue::to_value(&self.tracking_info)
                ),])
            }
        }
        impl rt::FromValue for AcceptedTransferOffer_WithdrawResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    tracking_info: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "trackingInfo",
                    )?)
                    .map_err(|e| e.at("trackingInfo"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferOffer_ExpireResult {
            ///Daml field `trackingInfo`.
            #[serde(rename = "trackingInfo")]
            pub tracking_info:
                crate::splice_wallet::Splice_Wallet_TransferOffer::TransferOfferTrackingInfo,
        }
        impl rt::ToValue for TransferOffer_ExpireResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "trackingInfo",
                    rt::ToValue::to_value(&self.tracking_info)
                ),])
            }
        }
        impl rt::FromValue for TransferOffer_ExpireResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    tracking_info: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "trackingInfo",
                    )?)
                    .map_err(|e| e.at("trackingInfo"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferOffer_WithdrawResult {
            ///Daml field `trackingInfo`.
            #[serde(rename = "trackingInfo")]
            pub tracking_info:
                crate::splice_wallet::Splice_Wallet_TransferOffer::TransferOfferTrackingInfo,
        }
        impl rt::ToValue for TransferOffer_WithdrawResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "trackingInfo",
                    rt::ToValue::to_value(&self.tracking_info)
                ),])
            }
        }
        impl rt::FromValue for TransferOffer_WithdrawResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    tracking_info: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "trackingInfo",
                    )?)
                    .map_err(|e| e.at("trackingInfo"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferOffer_RejectResult {
            ///Daml field `trackingInfo`.
            #[serde(rename = "trackingInfo")]
            pub tracking_info:
                crate::splice_wallet::Splice_Wallet_TransferOffer::TransferOfferTrackingInfo,
        }
        impl rt::ToValue for TransferOffer_RejectResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "trackingInfo",
                    rt::ToValue::to_value(&self.tracking_info)
                ),])
            }
        }
        impl rt::FromValue for TransferOffer_RejectResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    tracking_info: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "trackingInfo",
                    )?)
                    .map_err(|e| e.at("trackingInfo"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferOffer_AcceptResult {
            ///Daml field `acceptedTransferOffer`.
            #[serde(rename = "acceptedTransferOffer")]
            pub accepted_transfer_offer: rt::ContractId<
                crate::splice_wallet::Splice_Wallet_TransferOffer::AcceptedTransferOffer,
            >,
        }
        impl rt::ToValue for TransferOffer_AcceptResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "acceptedTransferOffer",
                    rt::ToValue::to_value(&self.accepted_transfer_offer)
                ),])
            }
        }
        impl rt::FromValue for TransferOffer_AcceptResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    accepted_transfer_offer: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "acceptedTransferOffer",
                    )?)
                    .map_err(|e| e.at("acceptedTransferOffer"))?,
                })
            }
        }
        ///The Daml template `Splice.Wallet.TransferOffer:AcceptedTransferOffer`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-wallet:Splice.Wallet.TransferOffer:AcceptedTransferOffer`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `AcceptedTransferOffer_Complete` — consuming
        ///- `AcceptedTransferOffer_Withdraw` — consuming
        ///- `AcceptedTransferOffer_Abort` — consuming
        ///- `AcceptedTransferOffer_Expire` — consuming
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AcceptedTransferOffer {
            #[serde(rename = "sender")]
            pub sender: rt::Party,
            #[serde(rename = "receiver")]
            pub receiver: rt::Party,
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            #[serde(rename = "amount")]
            pub amount: ::canton_splice_wallet_payments::splice_wallet_payments::Splice_Wallet_Payment::PaymentAmount,
            ///Daml field `expiresAt`.
            #[serde(rename = "expiresAt")]
            pub expires_at: rt::Timestamp,
            ///Daml field `trackingId`.
            #[serde(rename = "trackingId")]
            pub tracking_id: ::std::string::String,
        }
        impl rt::ToValue for AcceptedTransferOffer {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("sender", rt::ToValue::to_value(&self.sender)),
                    ("receiver", rt::ToValue::to_value(&self.receiver)),
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                    ("expiresAt", rt::ToValue::to_value(&self.expires_at)),
                    ("trackingId", rt::ToValue::to_value(&self.tracking_id)),
                ])
            }
        }
        impl rt::FromValue for AcceptedTransferOffer {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    sender: rt::FromValue::from_value(rt::required_field(value, 0usize, "sender")?)
                        .map_err(|e| e.at("sender"))?,
                    receiver: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "receiver",
                    )?)
                    .map_err(|e| e.at("receiver"))?,
                    dso: rt::FromValue::from_value(rt::required_field(value, 2usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                    amount: rt::FromValue::from_value(rt::required_field(value, 3usize, "amount")?)
                        .map_err(|e| e.at("amount"))?,
                    expires_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "expiresAt",
                    )?)
                    .map_err(|e| e.at("expiresAt"))?,
                    tracking_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "trackingId",
                    )?)
                    .map_err(|e| e.at("trackingId"))?,
                })
            }
        }
        impl rt::Contract for AcceptedTransferOffer {
            const PACKAGE_ID: &'static str =
                "690c1d47bac06db419db344d59a7a30c53fa3f5d961943fe1782cfc6c78794d8";
            const PACKAGE_NAME: &'static str = "splice-wallet";
            const MODULE_NAME: &'static str = "Splice.Wallet.TransferOffer";
            const ENTITY_NAME: &'static str = "AcceptedTransferOffer";
        }
        impl rt::Template for AcceptedTransferOffer {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("sender", rt::ToValue::to_value(&self.sender)),
                    ("receiver", rt::ToValue::to_value(&self.receiver)),
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                    ("expiresAt", rt::ToValue::to_value(&self.expires_at)),
                    ("trackingId", rt::ToValue::to_value(&self.tracking_id)),
                ])
            }
        }
        ///The `AcceptedTransferOffer_Complete` choice on [`AcceptedTransferOffer`] (consuming).
        impl rt::Choice<AcceptedTransferOffer>
            for crate::splice_wallet::Splice_Wallet_TransferOffer::AcceptedTransferOffer_Complete
        {
            type Return = crate::splice_wallet::Splice_Wallet_TransferOffer::AcceptedTransferOffer_CompleteResult;
            const NAME: &'static str = "AcceptedTransferOffer_Complete";
            const CONSUMING: bool = true;
        }
        ///The `AcceptedTransferOffer_Withdraw` choice on [`AcceptedTransferOffer`] (consuming).
        impl rt::Choice<AcceptedTransferOffer>
            for crate::splice_wallet::Splice_Wallet_TransferOffer::AcceptedTransferOffer_Withdraw
        {
            type Return = crate::splice_wallet::Splice_Wallet_TransferOffer::AcceptedTransferOffer_WithdrawResult;
            const NAME: &'static str = "AcceptedTransferOffer_Withdraw";
            const CONSUMING: bool = true;
        }
        ///The `AcceptedTransferOffer_Abort` choice on [`AcceptedTransferOffer`] (consuming).
        impl rt::Choice<AcceptedTransferOffer>
            for crate::splice_wallet::Splice_Wallet_TransferOffer::AcceptedTransferOffer_Abort
        {
            type Return = crate::splice_wallet::Splice_Wallet_TransferOffer::AcceptedTransferOffer_AbortResult;
            const NAME: &'static str = "AcceptedTransferOffer_Abort";
            const CONSUMING: bool = true;
        }
        ///The `AcceptedTransferOffer_Expire` choice on [`AcceptedTransferOffer`] (consuming).
        impl rt::Choice<AcceptedTransferOffer>
            for crate::splice_wallet::Splice_Wallet_TransferOffer::AcceptedTransferOffer_Expire
        {
            type Return = crate::splice_wallet::Splice_Wallet_TransferOffer::AcceptedTransferOffer_ExpireResult;
            const NAME: &'static str = "AcceptedTransferOffer_Expire";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`AcceptedTransferOffer`] (consuming).
        impl rt::Choice<AcceptedTransferOffer>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The Daml template `Splice.Wallet.TransferOffer:TransferOffer`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-wallet:Splice.Wallet.TransferOffer:TransferOffer`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `TransferOffer_Accept` — consuming
        ///- `TransferOffer_Reject` — consuming
        ///- `TransferOffer_Withdraw` — consuming
        ///- `TransferOffer_Expire` — consuming
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferOffer {
            #[serde(rename = "sender")]
            pub sender: rt::Party,
            #[serde(rename = "receiver")]
            pub receiver: rt::Party,
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            #[serde(rename = "amount")]
            pub amount: ::canton_splice_wallet_payments::splice_wallet_payments::Splice_Wallet_Payment::PaymentAmount,
            #[serde(rename = "description")]
            pub description: ::std::string::String,
            ///Daml field `expiresAt`.
            #[serde(rename = "expiresAt")]
            pub expires_at: rt::Timestamp,
            ///Daml field `trackingId`.
            #[serde(rename = "trackingId")]
            pub tracking_id: ::std::string::String,
        }
        impl rt::ToValue for TransferOffer {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("sender", rt::ToValue::to_value(&self.sender)),
                    ("receiver", rt::ToValue::to_value(&self.receiver)),
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                    ("description", rt::ToValue::to_value(&self.description)),
                    ("expiresAt", rt::ToValue::to_value(&self.expires_at)),
                    ("trackingId", rt::ToValue::to_value(&self.tracking_id)),
                ])
            }
        }
        impl rt::FromValue for TransferOffer {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    sender: rt::FromValue::from_value(rt::required_field(value, 0usize, "sender")?)
                        .map_err(|e| e.at("sender"))?,
                    receiver: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "receiver",
                    )?)
                    .map_err(|e| e.at("receiver"))?,
                    dso: rt::FromValue::from_value(rt::required_field(value, 2usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                    amount: rt::FromValue::from_value(rt::required_field(value, 3usize, "amount")?)
                        .map_err(|e| e.at("amount"))?,
                    description: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "description",
                    )?)
                    .map_err(|e| e.at("description"))?,
                    expires_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "expiresAt",
                    )?)
                    .map_err(|e| e.at("expiresAt"))?,
                    tracking_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        6usize,
                        "trackingId",
                    )?)
                    .map_err(|e| e.at("trackingId"))?,
                })
            }
        }
        impl rt::Contract for TransferOffer {
            const PACKAGE_ID: &'static str =
                "690c1d47bac06db419db344d59a7a30c53fa3f5d961943fe1782cfc6c78794d8";
            const PACKAGE_NAME: &'static str = "splice-wallet";
            const MODULE_NAME: &'static str = "Splice.Wallet.TransferOffer";
            const ENTITY_NAME: &'static str = "TransferOffer";
        }
        impl rt::Template for TransferOffer {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("sender", rt::ToValue::to_value(&self.sender)),
                    ("receiver", rt::ToValue::to_value(&self.receiver)),
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                    ("description", rt::ToValue::to_value(&self.description)),
                    ("expiresAt", rt::ToValue::to_value(&self.expires_at)),
                    ("trackingId", rt::ToValue::to_value(&self.tracking_id)),
                ])
            }
        }
        ///The `TransferOffer_Accept` choice on [`TransferOffer`] (consuming).
        impl rt::Choice<TransferOffer>
            for crate::splice_wallet::Splice_Wallet_TransferOffer::TransferOffer_Accept
        {
            type Return =
                crate::splice_wallet::Splice_Wallet_TransferOffer::TransferOffer_AcceptResult;
            const NAME: &'static str = "TransferOffer_Accept";
            const CONSUMING: bool = true;
        }
        ///The `TransferOffer_Reject` choice on [`TransferOffer`] (consuming).
        impl rt::Choice<TransferOffer>
            for crate::splice_wallet::Splice_Wallet_TransferOffer::TransferOffer_Reject
        {
            type Return =
                crate::splice_wallet::Splice_Wallet_TransferOffer::TransferOffer_RejectResult;
            const NAME: &'static str = "TransferOffer_Reject";
            const CONSUMING: bool = true;
        }
        ///The `TransferOffer_Withdraw` choice on [`TransferOffer`] (consuming).
        impl rt::Choice<TransferOffer>
            for crate::splice_wallet::Splice_Wallet_TransferOffer::TransferOffer_Withdraw
        {
            type Return =
                crate::splice_wallet::Splice_Wallet_TransferOffer::TransferOffer_WithdrawResult;
            const NAME: &'static str = "TransferOffer_Withdraw";
            const CONSUMING: bool = true;
        }
        ///The `TransferOffer_Expire` choice on [`TransferOffer`] (consuming).
        impl rt::Choice<TransferOffer>
            for crate::splice_wallet::Splice_Wallet_TransferOffer::TransferOffer_Expire
        {
            type Return =
                crate::splice_wallet::Splice_Wallet_TransferOffer::TransferOffer_ExpireResult;
            const NAME: &'static str = "TransferOffer_Expire";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`TransferOffer`] (consuming).
        impl rt::Choice<TransferOffer>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
    }
    pub mod Splice_Wallet_BuyTrafficRequest {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct BuyTrafficRequest_Expire {}
        impl rt::ToValue for BuyTrafficRequest_Expire {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for BuyTrafficRequest_Expire {
            fn from_value(_value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct BuyTrafficRequest_Cancel {
            #[serde(rename = "reason")]
            pub reason: ::std::string::String,
        }
        impl rt::ToValue for BuyTrafficRequest_Cancel {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("reason", rt::ToValue::to_value(&self.reason)),])
            }
        }
        impl rt::FromValue for BuyTrafficRequest_Cancel {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    reason: rt::FromValue::from_value(rt::required_field(value, 0usize, "reason")?)
                        .map_err(|e| e.at("reason"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct BuyTrafficRequest_Complete {
            #[serde(rename = "inputs")]
            pub inputs: ::std::vec::Vec<
                ::canton_splice_amulet::splice_amulet::Splice_AmuletRules::TransferInput,
            >,
            #[serde(rename = "context")]
            pub context:
                ::canton_splice_amulet::splice_amulet::Splice_AmuletRules::PaymentTransferContext,
            ///Daml field `walletProvider`.
            #[serde(rename = "walletProvider")]
            pub wallet_provider: rt::Party,
        }
        impl rt::ToValue for BuyTrafficRequest_Complete {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("inputs", rt::ToValue::to_value(&self.inputs)),
                    ("context", rt::ToValue::to_value(&self.context)),
                    (
                        "walletProvider",
                        rt::ToValue::to_value(&self.wallet_provider)
                    ),
                ])
            }
        }
        impl rt::FromValue for BuyTrafficRequest_Complete {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    inputs: rt::FromValue::from_value(rt::required_field(value, 0usize, "inputs")?)
                        .map_err(|e| e.at("inputs"))?,
                    context: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "context",
                    )?)
                    .map_err(|e| e.at("context"))?,
                    wallet_provider: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "walletProvider",
                    )?)
                    .map_err(|e| e.at("walletProvider"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct BuyTrafficRequest_ExpireResult {
            ///Daml field `trackingInfo`.
            #[serde(rename = "trackingInfo")]
            pub tracking_info: crate::splice_wallet::Splice_Wallet_BuyTrafficRequest::BuyTrafficRequestTrackingInfo,
        }
        impl rt::ToValue for BuyTrafficRequest_ExpireResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "trackingInfo",
                    rt::ToValue::to_value(&self.tracking_info)
                ),])
            }
        }
        impl rt::FromValue for BuyTrafficRequest_ExpireResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    tracking_info: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "trackingInfo",
                    )?)
                    .map_err(|e| e.at("trackingInfo"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct BuyTrafficRequest_CancelResult {
            ///Daml field `trackingInfo`.
            #[serde(rename = "trackingInfo")]
            pub tracking_info: crate::splice_wallet::Splice_Wallet_BuyTrafficRequest::BuyTrafficRequestTrackingInfo,
        }
        impl rt::ToValue for BuyTrafficRequest_CancelResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "trackingInfo",
                    rt::ToValue::to_value(&self.tracking_info)
                ),])
            }
        }
        impl rt::FromValue for BuyTrafficRequest_CancelResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    tracking_info: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "trackingInfo",
                    )?)
                    .map_err(|e| e.at("trackingInfo"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct BuyTrafficRequest_CompleteResult {
            ///Daml field `purchasedTraffic`.
            #[serde(rename = "purchasedTraffic")]
            pub purchased_traffic: rt::ContractId<
                ::canton_splice_amulet::splice_amulet::Splice_DecentralizedSynchronizer::MemberTraffic,
            >,
            ///Daml field `trackingInfo`.
            #[serde(rename = "trackingInfo")]
            pub tracking_info: crate::splice_wallet::Splice_Wallet_BuyTrafficRequest::BuyTrafficRequestTrackingInfo,
            ///Daml field `senderChangeAmulet`.
            #[serde(rename = "senderChangeAmulet")]
            pub sender_change_amulet: ::core::option::Option<
                rt::ContractId<
                    ::canton_splice_amulet::splice_amulet::Splice_Amulet::Amulet,
                >,
            >,
        }
        impl rt::ToValue for BuyTrafficRequest_CompleteResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "purchasedTraffic",
                        rt::ToValue::to_value(&self.purchased_traffic)
                    ),
                    ("trackingInfo", rt::ToValue::to_value(&self.tracking_info)),
                    (
                        "senderChangeAmulet",
                        rt::ToValue::to_value(&self.sender_change_amulet)
                    ),
                ])
            }
        }
        impl rt::FromValue for BuyTrafficRequest_CompleteResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    purchased_traffic: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "purchasedTraffic",
                    )?)
                    .map_err(|e| e.at("purchasedTraffic"))?,
                    tracking_info: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "trackingInfo",
                    )?)
                    .map_err(|e| e.at("trackingInfo"))?,
                    sender_change_amulet: rt::optional_field(value, 2usize, "senderChangeAmulet")
                        .map_err(|e| e.at("senderChangeAmulet"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct BuyTrafficRequestTrackingInfo {
            ///Daml field `trackingId`.
            #[serde(rename = "trackingId")]
            pub tracking_id: ::std::string::String,
            ///Daml field `endUserParty`.
            #[serde(rename = "endUserParty")]
            pub end_user_party: rt::Party,
        }
        impl rt::ToValue for BuyTrafficRequestTrackingInfo {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("trackingId", rt::ToValue::to_value(&self.tracking_id)),
                    ("endUserParty", rt::ToValue::to_value(&self.end_user_party)),
                ])
            }
        }
        impl rt::FromValue for BuyTrafficRequestTrackingInfo {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    tracking_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "trackingId",
                    )?)
                    .map_err(|e| e.at("trackingId"))?,
                    end_user_party: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "endUserParty",
                    )?)
                    .map_err(|e| e.at("endUserParty"))?,
                })
            }
        }
        ///The Daml template `Splice.Wallet.BuyTrafficRequest:BuyTrafficRequest`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-wallet:Splice.Wallet.BuyTrafficRequest:BuyTrafficRequest`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `BuyTrafficRequest_Complete` — consuming
        ///- `BuyTrafficRequest_Cancel` — consuming
        ///- `Archive` — consuming
        ///- `BuyTrafficRequest_Expire` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct BuyTrafficRequest {
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///Daml field `endUserParty`.
            #[serde(rename = "endUserParty")]
            pub end_user_party: rt::Party,
            ///Daml field `expiresAt`.
            #[serde(rename = "expiresAt")]
            pub expires_at: rt::Timestamp,
            ///Daml field `trackingId`.
            #[serde(rename = "trackingId")]
            pub tracking_id: ::std::string::String,
            ///Daml field `trafficAmount`.
            #[serde(rename = "trafficAmount")]
            pub traffic_amount: rt::Int64,
            ///Daml field `memberId`.
            #[serde(rename = "memberId")]
            pub member_id: ::std::string::String,
            ///Daml field `synchronizerId`.
            #[serde(rename = "synchronizerId")]
            pub synchronizer_id: ::std::string::String,
            ///Daml field `migrationId`.
            #[serde(rename = "migrationId")]
            pub migration_id: rt::Int64,
        }
        impl rt::ToValue for BuyTrafficRequest {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("endUserParty", rt::ToValue::to_value(&self.end_user_party)),
                    ("expiresAt", rt::ToValue::to_value(&self.expires_at)),
                    ("trackingId", rt::ToValue::to_value(&self.tracking_id)),
                    ("trafficAmount", rt::ToValue::to_value(&self.traffic_amount)),
                    ("memberId", rt::ToValue::to_value(&self.member_id)),
                    (
                        "synchronizerId",
                        rt::ToValue::to_value(&self.synchronizer_id)
                    ),
                    ("migrationId", rt::ToValue::to_value(&self.migration_id)),
                ])
            }
        }
        impl rt::FromValue for BuyTrafficRequest {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                    end_user_party: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "endUserParty",
                    )?)
                    .map_err(|e| e.at("endUserParty"))?,
                    expires_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "expiresAt",
                    )?)
                    .map_err(|e| e.at("expiresAt"))?,
                    tracking_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "trackingId",
                    )?)
                    .map_err(|e| e.at("trackingId"))?,
                    traffic_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "trafficAmount",
                    )?)
                    .map_err(|e| e.at("trafficAmount"))?,
                    member_id: rt::FromValue::from_value(rt::required_field(
                        value, 5usize, "memberId",
                    )?)
                    .map_err(|e| e.at("memberId"))?,
                    synchronizer_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        6usize,
                        "synchronizerId",
                    )?)
                    .map_err(|e| e.at("synchronizerId"))?,
                    migration_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        7usize,
                        "migrationId",
                    )?)
                    .map_err(|e| e.at("migrationId"))?,
                })
            }
        }
        impl rt::Contract for BuyTrafficRequest {
            const PACKAGE_ID: &'static str =
                "690c1d47bac06db419db344d59a7a30c53fa3f5d961943fe1782cfc6c78794d8";
            const PACKAGE_NAME: &'static str = "splice-wallet";
            const MODULE_NAME: &'static str = "Splice.Wallet.BuyTrafficRequest";
            const ENTITY_NAME: &'static str = "BuyTrafficRequest";
        }
        impl rt::Template for BuyTrafficRequest {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("endUserParty", rt::ToValue::to_value(&self.end_user_party)),
                    ("expiresAt", rt::ToValue::to_value(&self.expires_at)),
                    ("trackingId", rt::ToValue::to_value(&self.tracking_id)),
                    ("trafficAmount", rt::ToValue::to_value(&self.traffic_amount)),
                    ("memberId", rt::ToValue::to_value(&self.member_id)),
                    (
                        "synchronizerId",
                        rt::ToValue::to_value(&self.synchronizer_id)
                    ),
                    ("migrationId", rt::ToValue::to_value(&self.migration_id)),
                ])
            }
        }
        ///The `BuyTrafficRequest_Complete` choice on [`BuyTrafficRequest`] (consuming).
        impl rt::Choice<BuyTrafficRequest>
            for crate::splice_wallet::Splice_Wallet_BuyTrafficRequest::BuyTrafficRequest_Complete
        {
            type Return = crate::splice_wallet::Splice_Wallet_BuyTrafficRequest::BuyTrafficRequest_CompleteResult;
            const NAME: &'static str = "BuyTrafficRequest_Complete";
            const CONSUMING: bool = true;
        }
        ///The `BuyTrafficRequest_Cancel` choice on [`BuyTrafficRequest`] (consuming).
        impl rt::Choice<BuyTrafficRequest>
            for crate::splice_wallet::Splice_Wallet_BuyTrafficRequest::BuyTrafficRequest_Cancel
        {
            type Return = crate::splice_wallet::Splice_Wallet_BuyTrafficRequest::BuyTrafficRequest_CancelResult;
            const NAME: &'static str = "BuyTrafficRequest_Cancel";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`BuyTrafficRequest`] (consuming).
        impl rt::Choice<BuyTrafficRequest>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `BuyTrafficRequest_Expire` choice on [`BuyTrafficRequest`] (consuming).
        impl rt::Choice<BuyTrafficRequest>
            for crate::splice_wallet::Splice_Wallet_BuyTrafficRequest::BuyTrafficRequest_Expire
        {
            type Return = crate::splice_wallet::Splice_Wallet_BuyTrafficRequest::BuyTrafficRequest_ExpireResult;
            const NAME: &'static str = "BuyTrafficRequest_Expire";
            const CONSUMING: bool = true;
        }
    }
    pub mod Splice_Wallet_TransferPreapproval {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferPreapprovalProposal_AcceptResult {
            ///Daml field `transferPreapprovalCid`.
            #[serde(rename = "transferPreapprovalCid")]
            pub transfer_preapproval_cid: rt::ContractId<
                ::canton_splice_amulet::splice_amulet::Splice_AmuletRules::TransferPreapproval,
            >,
            ///Daml field `transferResult`.
            #[serde(rename = "transferResult")]
            pub transfer_result:
                ::canton_splice_amulet::splice_amulet::Splice_AmuletRules::TransferResult,
            ///Daml field `amuletPaid`.
            #[serde(rename = "amuletPaid")]
            pub amulet_paid: rt::Numeric,
        }
        impl rt::ToValue for TransferPreapprovalProposal_AcceptResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "transferPreapprovalCid",
                        rt::ToValue::to_value(&self.transfer_preapproval_cid)
                    ),
                    (
                        "transferResult",
                        rt::ToValue::to_value(&self.transfer_result)
                    ),
                    ("amuletPaid", rt::ToValue::to_value(&self.amulet_paid)),
                ])
            }
        }
        impl rt::FromValue for TransferPreapprovalProposal_AcceptResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    transfer_preapproval_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "transferPreapprovalCid",
                    )?)
                    .map_err(|e| e.at("transferPreapprovalCid"))?,
                    transfer_result: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "transferResult",
                    )?)
                    .map_err(|e| e.at("transferResult"))?,
                    amulet_paid: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "amuletPaid",
                    )?)
                    .map_err(|e| e.at("amuletPaid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferPreapprovalProposal_Accept {
            #[serde(rename = "context")]
            pub context:
                ::canton_splice_amulet::splice_amulet::Splice_AmuletRules::PaymentTransferContext,
            #[serde(rename = "inputs")]
            pub inputs: ::std::vec::Vec<
                ::canton_splice_amulet::splice_amulet::Splice_AmuletRules::TransferInput,
            >,
            ///Daml field `expiresAt`.
            #[serde(rename = "expiresAt")]
            pub expires_at: rt::Timestamp,
        }
        impl rt::ToValue for TransferPreapprovalProposal_Accept {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("context", rt::ToValue::to_value(&self.context)),
                    ("inputs", rt::ToValue::to_value(&self.inputs)),
                    ("expiresAt", rt::ToValue::to_value(&self.expires_at)),
                ])
            }
        }
        impl rt::FromValue for TransferPreapprovalProposal_Accept {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    context: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "context",
                    )?)
                    .map_err(|e| e.at("context"))?,
                    inputs: rt::FromValue::from_value(rt::required_field(value, 1usize, "inputs")?)
                        .map_err(|e| e.at("inputs"))?,
                    expires_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "expiresAt",
                    )?)
                    .map_err(|e| e.at("expiresAt"))?,
                })
            }
        }
        ///The Daml template `Splice.Wallet.TransferPreapproval:TransferPreapprovalProposal`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-wallet:Splice.Wallet.TransferPreapproval:TransferPreapprovalProposal`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `Archive` — consuming
        ///- `TransferPreapprovalProposal_Accept` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferPreapprovalProposal {
            #[serde(rename = "receiver")]
            pub receiver: rt::Party,
            #[serde(rename = "provider")]
            pub provider: rt::Party,
            ///Daml field `expectedDso`.
            #[serde(rename = "expectedDso")]
            pub expected_dso: ::core::option::Option<rt::Party>,
        }
        impl rt::ToValue for TransferPreapprovalProposal {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("receiver", rt::ToValue::to_value(&self.receiver)),
                    ("provider", rt::ToValue::to_value(&self.provider)),
                    ("expectedDso", rt::ToValue::to_value(&self.expected_dso)),
                ])
            }
        }
        impl rt::FromValue for TransferPreapprovalProposal {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    receiver: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "receiver",
                    )?)
                    .map_err(|e| e.at("receiver"))?,
                    provider: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "provider",
                    )?)
                    .map_err(|e| e.at("provider"))?,
                    expected_dso: rt::optional_field(value, 2usize, "expectedDso")
                        .map_err(|e| e.at("expectedDso"))?,
                })
            }
        }
        impl rt::Contract for TransferPreapprovalProposal {
            const PACKAGE_ID: &'static str =
                "690c1d47bac06db419db344d59a7a30c53fa3f5d961943fe1782cfc6c78794d8";
            const PACKAGE_NAME: &'static str = "splice-wallet";
            const MODULE_NAME: &'static str = "Splice.Wallet.TransferPreapproval";
            const ENTITY_NAME: &'static str = "TransferPreapprovalProposal";
        }
        impl rt::Template for TransferPreapprovalProposal {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("receiver", rt::ToValue::to_value(&self.receiver)),
                    ("provider", rt::ToValue::to_value(&self.provider)),
                    ("expectedDso", rt::ToValue::to_value(&self.expected_dso)),
                ])
            }
        }
        ///The `Archive` choice on [`TransferPreapprovalProposal`] (consuming).
        impl rt::Choice<TransferPreapprovalProposal>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `TransferPreapprovalProposal_Accept` choice on [`TransferPreapprovalProposal`] (consuming).
        impl rt::Choice<TransferPreapprovalProposal>
        for crate::splice_wallet::Splice_Wallet_TransferPreapproval::TransferPreapprovalProposal_Accept {
            type Return = crate::splice_wallet::Splice_Wallet_TransferPreapproval::TransferPreapprovalProposal_AcceptResult;
            const NAME: &'static str = "TransferPreapprovalProposal_Accept";
            const CONSUMING: bool = true;
        }
    }
}
