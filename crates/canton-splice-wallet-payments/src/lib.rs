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
//! - [`splice_wallet_payments`] — 2 Daml modules

pub mod splice_wallet_payments {
    pub mod Splice_Wallet_Payment {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AcceptedAppPayment_Expire {
            #[serde(rename = "context")]
            pub context:
                ::canton_splice_amulet::splice_amulet::Splice_AmuletRules::AppTransferContext,
        }
        impl rt::ToValue for AcceptedAppPayment_Expire {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "context",
                    rt::ToValue::to_value(&self.context)
                ),])
            }
        }
        impl rt::FromValue for AcceptedAppPayment_Expire {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    context: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "context",
                    )?)
                    .map_err(|e| e.at("context"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AcceptedAppPayment_Reject {
            #[serde(rename = "context")]
            pub context:
                ::canton_splice_amulet::splice_amulet::Splice_AmuletRules::AppTransferContext,
        }
        impl rt::ToValue for AcceptedAppPayment_Reject {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "context",
                    rt::ToValue::to_value(&self.context)
                ),])
            }
        }
        impl rt::FromValue for AcceptedAppPayment_Reject {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    context: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "context",
                    )?)
                    .map_err(|e| e.at("context"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AcceptedAppPayment_Collect {
            #[serde(rename = "context")]
            pub context:
                ::canton_splice_amulet::splice_amulet::Splice_AmuletRules::AppTransferContext,
        }
        impl rt::ToValue for AcceptedAppPayment_Collect {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "context",
                    rt::ToValue::to_value(&self.context)
                ),])
            }
        }
        impl rt::FromValue for AcceptedAppPayment_Collect {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    context: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "context",
                    )?)
                    .map_err(|e| e.at("context"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AcceptedAppPayment_CollectResult {
            ///Daml field `receiverAmulets`.
            #[serde(rename = "receiverAmulets")]
            pub receiver_amulets: ::std::vec::Vec<
                ::canton_daml_stdlib::daml_prim_DA_Types::DA_Types::Tuple2<
                    rt::Party,
                    rt::ContractId<::canton_splice_amulet::splice_amulet::Splice_Amulet::Amulet>,
                >,
            >,
        }
        impl rt::ToValue for AcceptedAppPayment_CollectResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "receiverAmulets",
                    rt::ToValue::to_value(&self.receiver_amulets)
                ),])
            }
        }
        impl rt::FromValue for AcceptedAppPayment_CollectResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    receiver_amulets: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "receiverAmulets",
                    )?)
                    .map_err(|e| e.at("receiverAmulets"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ReceiverAmulet {
            #[serde(rename = "receiver")]
            pub receiver: rt::Party,
            ///Daml field `lockedAmulet`.
            #[serde(rename = "lockedAmulet")]
            pub locked_amulet:
                rt::ContractId<::canton_splice_amulet::splice_amulet::Splice_Amulet::LockedAmulet>,
        }
        impl rt::ToValue for ReceiverAmulet {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("receiver", rt::ToValue::to_value(&self.receiver)),
                    ("lockedAmulet", rt::ToValue::to_value(&self.locked_amulet)),
                ])
            }
        }
        impl rt::FromValue for ReceiverAmulet {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    receiver: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "receiver",
                    )?)
                    .map_err(|e| e.at("receiver"))?,
                    locked_amulet: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "lockedAmulet",
                    )?)
                    .map_err(|e| e.at("lockedAmulet"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AppPaymentRequest_Reject {}
        impl rt::ToValue for AppPaymentRequest_Reject {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for AppPaymentRequest_Reject {
            fn from_value(_value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AppPaymentRequest_Withdraw {}
        impl rt::ToValue for AppPaymentRequest_Withdraw {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for AppPaymentRequest_Withdraw {
            fn from_value(_value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AppPaymentRequest_Accept {
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
        impl rt::ToValue for AppPaymentRequest_Accept {
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
        impl rt::FromValue for AppPaymentRequest_Accept {
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
        pub struct AppPaymentRequest_Expire {
            #[serde(rename = "actor")]
            pub actor: rt::Party,
        }
        impl rt::ToValue for AppPaymentRequest_Expire {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("actor", rt::ToValue::to_value(&self.actor)),])
            }
        }
        impl rt::FromValue for AppPaymentRequest_Expire {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    actor: rt::FromValue::from_value(rt::required_field(value, 0usize, "actor")?)
                        .map_err(|e| e.at("actor"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ReceiverAmuletAmount {
            #[serde(rename = "receiver")]
            pub receiver: rt::Party,
            ///Daml field `amuletAmount`.
            #[serde(rename = "amuletAmount")]
            pub amulet_amount: rt::Numeric,
        }
        impl rt::ToValue for ReceiverAmuletAmount {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("receiver", rt::ToValue::to_value(&self.receiver)),
                    ("amuletAmount", rt::ToValue::to_value(&self.amulet_amount)),
                ])
            }
        }
        impl rt::FromValue for ReceiverAmuletAmount {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    receiver: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "receiver",
                    )?)
                    .map_err(|e| e.at("receiver"))?,
                    amulet_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "amuletAmount",
                    )?)
                    .map_err(|e| e.at("amuletAmount"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ReceiverAmount {
            #[serde(rename = "receiver")]
            pub receiver: rt::Party,
            #[serde(rename = "amount")]
            pub amount: crate::splice_wallet_payments::Splice_Wallet_Payment::PaymentAmount,
        }
        impl rt::ToValue for ReceiverAmount {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("receiver", rt::ToValue::to_value(&self.receiver)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                ])
            }
        }
        impl rt::FromValue for ReceiverAmount {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    receiver: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "receiver",
                    )?)
                    .map_err(|e| e.at("receiver"))?,
                    amount: rt::FromValue::from_value(rt::required_field(value, 1usize, "amount")?)
                        .map_err(|e| e.at("amount"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct PaymentAmount {
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            #[serde(rename = "unit")]
            pub unit: crate::splice_wallet_payments::Splice_Wallet_Payment::Unit,
        }
        impl rt::ToValue for PaymentAmount {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("amount", rt::ToValue::to_value(&self.amount)),
                    ("unit", rt::ToValue::to_value(&self.unit)),
                ])
            }
        }
        impl rt::FromValue for PaymentAmount {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    amount: rt::FromValue::from_value(rt::required_field(value, 0usize, "amount")?)
                        .map_err(|e| e.at("amount"))?,
                    unit: rt::FromValue::from_value(rt::required_field(value, 1usize, "unit")?)
                        .map_err(|e| e.at("unit"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AcceptedAppPayment_ExpireResult {
            #[serde(rename = "amulet")]
            pub amulet: ::canton_splice_amulet::splice_amulet::Splice_Amulet::AmuletCreateSummary<
                rt::ContractId<::canton_splice_amulet::splice_amulet::Splice_Amulet::Amulet>,
            >,
        }
        impl rt::ToValue for AcceptedAppPayment_ExpireResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("amulet", rt::ToValue::to_value(&self.amulet)),])
            }
        }
        impl rt::FromValue for AcceptedAppPayment_ExpireResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    amulet: rt::FromValue::from_value(rt::required_field(value, 0usize, "amulet")?)
                        .map_err(|e| e.at("amulet"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AcceptedAppPayment_RejectResult {
            #[serde(rename = "amulet")]
            pub amulet: ::canton_splice_amulet::splice_amulet::Splice_Amulet::AmuletCreateSummary<
                rt::ContractId<::canton_splice_amulet::splice_amulet::Splice_Amulet::Amulet>,
            >,
        }
        impl rt::ToValue for AcceptedAppPayment_RejectResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("amulet", rt::ToValue::to_value(&self.amulet)),])
            }
        }
        impl rt::FromValue for AcceptedAppPayment_RejectResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    amulet: rt::FromValue::from_value(rt::required_field(value, 0usize, "amulet")?)
                        .map_err(|e| e.at("amulet"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AppPaymentRequest_RejectResult {
            ///Daml field `terminatedAppPayment`.
            #[serde(rename = "terminatedAppPayment")]
            pub terminated_app_payment: rt::ContractId<
                crate::splice_wallet_payments::Splice_Wallet_Payment::TerminatedAppPayment,
            >,
        }
        impl rt::ToValue for AppPaymentRequest_RejectResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "terminatedAppPayment",
                    rt::ToValue::to_value(&self.terminated_app_payment)
                ),])
            }
        }
        impl rt::FromValue for AppPaymentRequest_RejectResult {
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
        pub struct AppPaymentRequest_WithdrawResult {
            ///Daml field `terminatedAppPayment`.
            #[serde(rename = "terminatedAppPayment")]
            pub terminated_app_payment: rt::ContractId<
                crate::splice_wallet_payments::Splice_Wallet_Payment::TerminatedAppPayment,
            >,
        }
        impl rt::ToValue for AppPaymentRequest_WithdrawResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "terminatedAppPayment",
                    rt::ToValue::to_value(&self.terminated_app_payment)
                ),])
            }
        }
        impl rt::FromValue for AppPaymentRequest_WithdrawResult {
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
        pub struct AppPaymentRequest_AcceptResult {
            ///Daml field `acceptedPayment`.
            #[serde(rename = "acceptedPayment")]
            pub accepted_payment: rt::ContractId<
                crate::splice_wallet_payments::Splice_Wallet_Payment::AcceptedAppPayment,
            >,
            ///Daml field `senderChangeAmulet`.
            #[serde(rename = "senderChangeAmulet")]
            pub sender_change_amulet: ::core::option::Option<
                rt::ContractId<::canton_splice_amulet::splice_amulet::Splice_Amulet::Amulet>,
            >,
        }
        impl rt::ToValue for AppPaymentRequest_AcceptResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "acceptedPayment",
                        rt::ToValue::to_value(&self.accepted_payment)
                    ),
                    (
                        "senderChangeAmulet",
                        rt::ToValue::to_value(&self.sender_change_amulet)
                    ),
                ])
            }
        }
        impl rt::FromValue for AppPaymentRequest_AcceptResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    accepted_payment: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "acceptedPayment",
                    )?)
                    .map_err(|e| e.at("acceptedPayment"))?,
                    sender_change_amulet: rt::optional_field(value, 1usize, "senderChangeAmulet")
                        .map_err(|e| e.at("senderChangeAmulet"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AppPaymentRequest_ExpireResult {
            ///Daml field `terminatedAppPayment`.
            #[serde(rename = "terminatedAppPayment")]
            pub terminated_app_payment: rt::ContractId<
                crate::splice_wallet_payments::Splice_Wallet_Payment::TerminatedAppPayment,
            >,
        }
        impl rt::ToValue for AppPaymentRequest_ExpireResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "terminatedAppPayment",
                    rt::ToValue::to_value(&self.terminated_app_payment)
                ),])
            }
        }
        impl rt::FromValue for AppPaymentRequest_ExpireResult {
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
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize,
        )]
        #[serde(crate = "rt::serde")]
        pub enum Unit {
            #[serde(rename = "USDUnit")]
            USDUnit,
            #[serde(rename = "AmuletUnit")]
            AmuletUnit,
            #[serde(rename = "ExtUnit")]
            ExtUnit,
        }
        impl rt::ToValue for Unit {
            fn to_value(&self) -> rt::Value {
                rt::enum_value(match self {
                    Unit::USDUnit => "USDUnit",
                    Unit::AmuletUnit => "AmuletUnit",
                    Unit::ExtUnit => "ExtUnit",
                })
            }
        }
        impl rt::FromValue for Unit {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                match rt::enum_constructor(value)? {
                    "USDUnit" => ::core::result::Result::Ok(Unit::USDUnit),
                    "AmuletUnit" => ::core::result::Result::Ok(Unit::AmuletUnit),
                    "ExtUnit" => ::core::result::Result::Ok(Unit::ExtUnit),
                    other => ::core::result::Result::Err(rt::unexpected_constructor("Unit", other)),
                }
            }
        }
        ///The Daml template `Splice.Wallet.Payment:AcceptedAppPayment`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-wallet-payments:Splice.Wallet.Payment:AcceptedAppPayment`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `Archive` — consuming
        ///- `AcceptedAppPayment_Collect` — consuming
        ///- `AcceptedAppPayment_Expire` — consuming
        ///- `AcceptedAppPayment_Reject` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AcceptedAppPayment {
            #[serde(rename = "sender")]
            pub sender: rt::Party,
            ///Daml field `amuletReceiverAmounts`.
            #[serde(rename = "amuletReceiverAmounts")]
            pub amulet_receiver_amounts: ::std::vec::Vec<
                crate::splice_wallet_payments::Splice_Wallet_Payment::ReceiverAmuletAmount,
            >,
            #[serde(rename = "provider")]
            pub provider: rt::Party,
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///Daml field `lockedAmulet`.
            #[serde(rename = "lockedAmulet")]
            pub locked_amulet:
                rt::ContractId<::canton_splice_amulet::splice_amulet::Splice_Amulet::LockedAmulet>,
            #[serde(rename = "round")]
            pub round: ::canton_splice_amulet::splice_amulet::Splice_Types::Round,
            #[serde(rename = "reference")]
            pub reference: rt::ContractId<
                crate::splice_wallet_payments::Splice_Wallet_Payment::AppPaymentRequest,
            >,
        }
        impl rt::ToValue for AcceptedAppPayment {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("sender", rt::ToValue::to_value(&self.sender)),
                    (
                        "amuletReceiverAmounts",
                        rt::ToValue::to_value(&self.amulet_receiver_amounts)
                    ),
                    ("provider", rt::ToValue::to_value(&self.provider)),
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("lockedAmulet", rt::ToValue::to_value(&self.locked_amulet)),
                    ("round", rt::ToValue::to_value(&self.round)),
                    ("reference", rt::ToValue::to_value(&self.reference)),
                ])
            }
        }
        impl rt::FromValue for AcceptedAppPayment {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    sender: rt::FromValue::from_value(rt::required_field(value, 0usize, "sender")?)
                        .map_err(|e| e.at("sender"))?,
                    amulet_receiver_amounts: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "amuletReceiverAmounts",
                    )?)
                    .map_err(|e| e.at("amuletReceiverAmounts"))?,
                    provider: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "provider",
                    )?)
                    .map_err(|e| e.at("provider"))?,
                    dso: rt::FromValue::from_value(rt::required_field(value, 3usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                    locked_amulet: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "lockedAmulet",
                    )?)
                    .map_err(|e| e.at("lockedAmulet"))?,
                    round: rt::FromValue::from_value(rt::required_field(value, 5usize, "round")?)
                        .map_err(|e| e.at("round"))?,
                    reference: rt::FromValue::from_value(rt::required_field(
                        value,
                        6usize,
                        "reference",
                    )?)
                    .map_err(|e| e.at("reference"))?,
                })
            }
        }
        impl rt::Contract for AcceptedAppPayment {
            const PACKAGE_ID: &'static str =
                "45b29d6e05b5352c39edde850c66b4535c682b9991b06eec312176b1a48ecab5";
            const PACKAGE_NAME: &'static str = "splice-wallet-payments";
            const MODULE_NAME: &'static str = "Splice.Wallet.Payment";
            const ENTITY_NAME: &'static str = "AcceptedAppPayment";
        }
        impl rt::Template for AcceptedAppPayment {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("sender", rt::ToValue::to_value(&self.sender)),
                    (
                        "amuletReceiverAmounts",
                        rt::ToValue::to_value(&self.amulet_receiver_amounts)
                    ),
                    ("provider", rt::ToValue::to_value(&self.provider)),
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("lockedAmulet", rt::ToValue::to_value(&self.locked_amulet)),
                    ("round", rt::ToValue::to_value(&self.round)),
                    ("reference", rt::ToValue::to_value(&self.reference)),
                ])
            }
        }
        ///The `Archive` choice on [`AcceptedAppPayment`] (consuming).
        impl rt::Choice<AcceptedAppPayment>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `AcceptedAppPayment_Collect` choice on [`AcceptedAppPayment`] (consuming).
        impl rt::Choice<AcceptedAppPayment>
            for crate::splice_wallet_payments::Splice_Wallet_Payment::AcceptedAppPayment_Collect
        {
            type Return = crate::splice_wallet_payments::Splice_Wallet_Payment::AcceptedAppPayment_CollectResult;
            const NAME: &'static str = "AcceptedAppPayment_Collect";
            const CONSUMING: bool = true;
        }
        ///The `AcceptedAppPayment_Expire` choice on [`AcceptedAppPayment`] (consuming).
        impl rt::Choice<AcceptedAppPayment>
            for crate::splice_wallet_payments::Splice_Wallet_Payment::AcceptedAppPayment_Expire
        {
            type Return = crate::splice_wallet_payments::Splice_Wallet_Payment::AcceptedAppPayment_ExpireResult;
            const NAME: &'static str = "AcceptedAppPayment_Expire";
            const CONSUMING: bool = true;
        }
        ///The `AcceptedAppPayment_Reject` choice on [`AcceptedAppPayment`] (consuming).
        impl rt::Choice<AcceptedAppPayment>
            for crate::splice_wallet_payments::Splice_Wallet_Payment::AcceptedAppPayment_Reject
        {
            type Return = crate::splice_wallet_payments::Splice_Wallet_Payment::AcceptedAppPayment_RejectResult;
            const NAME: &'static str = "AcceptedAppPayment_Reject";
            const CONSUMING: bool = true;
        }
        ///The Daml template `Splice.Wallet.Payment:AppPaymentRequest`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-wallet-payments:Splice.Wallet.Payment:AppPaymentRequest`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `AppPaymentRequest_Accept` — consuming
        ///- `AppPaymentRequest_Expire` — consuming
        ///- `AppPaymentRequest_Reject` — consuming
        ///- `Archive` — consuming
        ///- `AppPaymentRequest_Withdraw` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AppPaymentRequest {
            #[serde(rename = "sender")]
            pub sender: rt::Party,
            ///Daml field `receiverAmounts`.
            #[serde(rename = "receiverAmounts")]
            pub receiver_amounts: ::std::vec::Vec<
                crate::splice_wallet_payments::Splice_Wallet_Payment::ReceiverAmount,
            >,
            #[serde(rename = "provider")]
            pub provider: rt::Party,
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///Daml field `expiresAt`.
            #[serde(rename = "expiresAt")]
            pub expires_at: rt::Timestamp,
            #[serde(rename = "description")]
            pub description: ::std::string::String,
        }
        impl rt::ToValue for AppPaymentRequest {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("sender", rt::ToValue::to_value(&self.sender)),
                    (
                        "receiverAmounts",
                        rt::ToValue::to_value(&self.receiver_amounts)
                    ),
                    ("provider", rt::ToValue::to_value(&self.provider)),
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("expiresAt", rt::ToValue::to_value(&self.expires_at)),
                    ("description", rt::ToValue::to_value(&self.description)),
                ])
            }
        }
        impl rt::FromValue for AppPaymentRequest {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    sender: rt::FromValue::from_value(rt::required_field(value, 0usize, "sender")?)
                        .map_err(|e| e.at("sender"))?,
                    receiver_amounts: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "receiverAmounts",
                    )?)
                    .map_err(|e| e.at("receiverAmounts"))?,
                    provider: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "provider",
                    )?)
                    .map_err(|e| e.at("provider"))?,
                    dso: rt::FromValue::from_value(rt::required_field(value, 3usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                    expires_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "expiresAt",
                    )?)
                    .map_err(|e| e.at("expiresAt"))?,
                    description: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "description",
                    )?)
                    .map_err(|e| e.at("description"))?,
                })
            }
        }
        impl rt::Contract for AppPaymentRequest {
            const PACKAGE_ID: &'static str =
                "45b29d6e05b5352c39edde850c66b4535c682b9991b06eec312176b1a48ecab5";
            const PACKAGE_NAME: &'static str = "splice-wallet-payments";
            const MODULE_NAME: &'static str = "Splice.Wallet.Payment";
            const ENTITY_NAME: &'static str = "AppPaymentRequest";
        }
        impl rt::Template for AppPaymentRequest {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("sender", rt::ToValue::to_value(&self.sender)),
                    (
                        "receiverAmounts",
                        rt::ToValue::to_value(&self.receiver_amounts)
                    ),
                    ("provider", rt::ToValue::to_value(&self.provider)),
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("expiresAt", rt::ToValue::to_value(&self.expires_at)),
                    ("description", rt::ToValue::to_value(&self.description)),
                ])
            }
        }
        ///The `AppPaymentRequest_Accept` choice on [`AppPaymentRequest`] (consuming).
        impl rt::Choice<AppPaymentRequest>
            for crate::splice_wallet_payments::Splice_Wallet_Payment::AppPaymentRequest_Accept
        {
            type Return = crate::splice_wallet_payments::Splice_Wallet_Payment::AppPaymentRequest_AcceptResult;
            const NAME: &'static str = "AppPaymentRequest_Accept";
            const CONSUMING: bool = true;
        }
        ///The `AppPaymentRequest_Expire` choice on [`AppPaymentRequest`] (consuming).
        impl rt::Choice<AppPaymentRequest>
            for crate::splice_wallet_payments::Splice_Wallet_Payment::AppPaymentRequest_Expire
        {
            type Return = crate::splice_wallet_payments::Splice_Wallet_Payment::AppPaymentRequest_ExpireResult;
            const NAME: &'static str = "AppPaymentRequest_Expire";
            const CONSUMING: bool = true;
        }
        ///The `AppPaymentRequest_Reject` choice on [`AppPaymentRequest`] (consuming).
        impl rt::Choice<AppPaymentRequest>
            for crate::splice_wallet_payments::Splice_Wallet_Payment::AppPaymentRequest_Reject
        {
            type Return = crate::splice_wallet_payments::Splice_Wallet_Payment::AppPaymentRequest_RejectResult;
            const NAME: &'static str = "AppPaymentRequest_Reject";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`AppPaymentRequest`] (consuming).
        impl rt::Choice<AppPaymentRequest>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `AppPaymentRequest_Withdraw` choice on [`AppPaymentRequest`] (consuming).
        impl rt::Choice<AppPaymentRequest>
            for crate::splice_wallet_payments::Splice_Wallet_Payment::AppPaymentRequest_Withdraw
        {
            type Return = crate::splice_wallet_payments::Splice_Wallet_Payment::AppPaymentRequest_WithdrawResult;
            const NAME: &'static str = "AppPaymentRequest_Withdraw";
            const CONSUMING: bool = true;
        }
        ///The Daml template `Splice.Wallet.Payment:TerminatedAppPayment`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-wallet-payments:Splice.Wallet.Payment:TerminatedAppPayment`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TerminatedAppPayment {
            #[serde(rename = "sender")]
            pub sender: rt::Party,
            #[serde(rename = "provider")]
            pub provider: rt::Party,
            #[serde(rename = "receivers")]
            pub receivers: ::std::vec::Vec<rt::Party>,
            #[serde(rename = "reference")]
            pub reference: rt::ContractId<
                crate::splice_wallet_payments::Splice_Wallet_Payment::AppPaymentRequest,
            >,
        }
        impl rt::ToValue for TerminatedAppPayment {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("sender", rt::ToValue::to_value(&self.sender)),
                    ("provider", rt::ToValue::to_value(&self.provider)),
                    ("receivers", rt::ToValue::to_value(&self.receivers)),
                    ("reference", rt::ToValue::to_value(&self.reference)),
                ])
            }
        }
        impl rt::FromValue for TerminatedAppPayment {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    sender: rt::FromValue::from_value(rt::required_field(value, 0usize, "sender")?)
                        .map_err(|e| e.at("sender"))?,
                    provider: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "provider",
                    )?)
                    .map_err(|e| e.at("provider"))?,
                    receivers: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "receivers",
                    )?)
                    .map_err(|e| e.at("receivers"))?,
                    reference: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "reference",
                    )?)
                    .map_err(|e| e.at("reference"))?,
                })
            }
        }
        impl rt::Contract for TerminatedAppPayment {
            const PACKAGE_ID: &'static str =
                "45b29d6e05b5352c39edde850c66b4535c682b9991b06eec312176b1a48ecab5";
            const PACKAGE_NAME: &'static str = "splice-wallet-payments";
            const MODULE_NAME: &'static str = "Splice.Wallet.Payment";
            const ENTITY_NAME: &'static str = "TerminatedAppPayment";
        }
        impl rt::Template for TerminatedAppPayment {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("sender", rt::ToValue::to_value(&self.sender)),
                    ("provider", rt::ToValue::to_value(&self.provider)),
                    ("receivers", rt::ToValue::to_value(&self.receivers)),
                    ("reference", rt::ToValue::to_value(&self.reference)),
                ])
            }
        }
        ///The `Archive` choice on [`TerminatedAppPayment`] (consuming).
        impl rt::Choice<TerminatedAppPayment>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
    }
    pub mod Splice_Wallet_Subscriptions {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SubscriptionPayment_Expire {
            #[serde(rename = "actor")]
            pub actor: rt::Party,
            ///Daml field `transferContext`.
            #[serde(rename = "transferContext")]
            pub transfer_context:
                ::canton_splice_amulet::splice_amulet::Splice_AmuletRules::AppTransferContext,
        }
        impl rt::ToValue for SubscriptionPayment_Expire {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("actor", rt::ToValue::to_value(&self.actor)),
                    (
                        "transferContext",
                        rt::ToValue::to_value(&self.transfer_context)
                    ),
                ])
            }
        }
        impl rt::FromValue for SubscriptionPayment_Expire {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    actor: rt::FromValue::from_value(rt::required_field(value, 0usize, "actor")?)
                        .map_err(|e| e.at("actor"))?,
                    transfer_context: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "transferContext",
                    )?)
                    .map_err(|e| e.at("transferContext"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SubscriptionPayment_Reject {
            ///Daml field `transferContext`.
            #[serde(rename = "transferContext")]
            pub transfer_context:
                ::canton_splice_amulet::splice_amulet::Splice_AmuletRules::AppTransferContext,
        }
        impl rt::ToValue for SubscriptionPayment_Reject {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "transferContext",
                    rt::ToValue::to_value(&self.transfer_context)
                ),])
            }
        }
        impl rt::FromValue for SubscriptionPayment_Reject {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    transfer_context: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "transferContext",
                    )?)
                    .map_err(|e| e.at("transferContext"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SubscriptionPayment_Collect {
            ///Daml field `transferContext`.
            #[serde(rename = "transferContext")]
            pub transfer_context:
                ::canton_splice_amulet::splice_amulet::Splice_AmuletRules::AppTransferContext,
        }
        impl rt::ToValue for SubscriptionPayment_Collect {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "transferContext",
                    rt::ToValue::to_value(&self.transfer_context)
                ),])
            }
        }
        impl rt::FromValue for SubscriptionPayment_Collect {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    transfer_context: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "transferContext",
                    )?)
                    .map_err(|e| e.at("transferContext"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SubscriptionIdleState_CancelSubscription {}
        impl rt::ToValue for SubscriptionIdleState_CancelSubscription {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for SubscriptionIdleState_CancelSubscription {
            fn from_value(_value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SubscriptionIdleState_ExpireSubscription {
            #[serde(rename = "actor")]
            pub actor: rt::Party,
        }
        impl rt::ToValue for SubscriptionIdleState_ExpireSubscription {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("actor", rt::ToValue::to_value(&self.actor)),])
            }
        }
        impl rt::FromValue for SubscriptionIdleState_ExpireSubscription {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    actor: rt::FromValue::from_value(rt::required_field(value, 0usize, "actor")?)
                        .map_err(|e| e.at("actor"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SubscriptionIdleState_MakePayment {
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
        impl rt::ToValue for SubscriptionIdleState_MakePayment {
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
        impl rt::FromValue for SubscriptionIdleState_MakePayment {
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
        pub struct SubscriptionInitialPayment_Expire {
            #[serde(rename = "actor")]
            pub actor: rt::Party,
            ///Daml field `transferContext`.
            #[serde(rename = "transferContext")]
            pub transfer_context:
                ::canton_splice_amulet::splice_amulet::Splice_AmuletRules::AppTransferContext,
        }
        impl rt::ToValue for SubscriptionInitialPayment_Expire {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("actor", rt::ToValue::to_value(&self.actor)),
                    (
                        "transferContext",
                        rt::ToValue::to_value(&self.transfer_context)
                    ),
                ])
            }
        }
        impl rt::FromValue for SubscriptionInitialPayment_Expire {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    actor: rt::FromValue::from_value(rt::required_field(value, 0usize, "actor")?)
                        .map_err(|e| e.at("actor"))?,
                    transfer_context: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "transferContext",
                    )?)
                    .map_err(|e| e.at("transferContext"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SubscriptionInitialPayment_Reject {
            ///Daml field `transferContext`.
            #[serde(rename = "transferContext")]
            pub transfer_context:
                ::canton_splice_amulet::splice_amulet::Splice_AmuletRules::AppTransferContext,
        }
        impl rt::ToValue for SubscriptionInitialPayment_Reject {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "transferContext",
                    rt::ToValue::to_value(&self.transfer_context)
                ),])
            }
        }
        impl rt::FromValue for SubscriptionInitialPayment_Reject {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    transfer_context: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "transferContext",
                    )?)
                    .map_err(|e| e.at("transferContext"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SubscriptionInitialPayment_Collect {
            ///Daml field `transferContext`.
            #[serde(rename = "transferContext")]
            pub transfer_context:
                ::canton_splice_amulet::splice_amulet::Splice_AmuletRules::AppTransferContext,
        }
        impl rt::ToValue for SubscriptionInitialPayment_Collect {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "transferContext",
                    rt::ToValue::to_value(&self.transfer_context)
                ),])
            }
        }
        impl rt::FromValue for SubscriptionInitialPayment_Collect {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    transfer_context: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "transferContext",
                    )?)
                    .map_err(|e| e.at("transferContext"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SubscriptionRequest_Reject {}
        impl rt::ToValue for SubscriptionRequest_Reject {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for SubscriptionRequest_Reject {
            fn from_value(_value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SubscriptionRequest_Withdraw {}
        impl rt::ToValue for SubscriptionRequest_Withdraw {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for SubscriptionRequest_Withdraw {
            fn from_value(_value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SubscriptionRequest_AcceptAndMakePayment {
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
        impl rt::ToValue for SubscriptionRequest_AcceptAndMakePayment {
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
        impl rt::FromValue for SubscriptionRequest_AcceptAndMakePayment {
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
        pub struct SubscriptionPayData {
            ///Daml field `paymentAmount`.
            #[serde(rename = "paymentAmount")]
            pub payment_amount: crate::splice_wallet_payments::Splice_Wallet_Payment::PaymentAmount,
            ///Daml field `paymentInterval`.
            #[serde(rename = "paymentInterval")]
            pub payment_interval:
                ::canton_daml_stdlib::daml_stdlib_DA_Time_Types::DA_Time_Types::RelTime,
            ///Daml field `paymentDuration`.
            #[serde(rename = "paymentDuration")]
            pub payment_duration:
                ::canton_daml_stdlib::daml_stdlib_DA_Time_Types::DA_Time_Types::RelTime,
        }
        impl rt::ToValue for SubscriptionPayData {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("paymentAmount", rt::ToValue::to_value(&self.payment_amount)),
                    (
                        "paymentInterval",
                        rt::ToValue::to_value(&self.payment_interval)
                    ),
                    (
                        "paymentDuration",
                        rt::ToValue::to_value(&self.payment_duration)
                    ),
                ])
            }
        }
        impl rt::FromValue for SubscriptionPayData {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    payment_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "paymentAmount",
                    )?)
                    .map_err(|e| e.at("paymentAmount"))?,
                    payment_interval: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "paymentInterval",
                    )?)
                    .map_err(|e| e.at("paymentInterval"))?,
                    payment_duration: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "paymentDuration",
                    )?)
                    .map_err(|e| e.at("paymentDuration"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Subscription_Archive {}
        impl rt::ToValue for Subscription_Archive {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for Subscription_Archive {
            fn from_value(_value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SubscriptionPayment_ExpireResult {
            ///Daml field `subscriptionState`.
            #[serde(rename = "subscriptionState")]
            pub subscription_state: rt::ContractId<
                crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionIdleState,
            >,
            ///Daml field `amuletSum`.
            #[serde(rename = "amuletSum")]
            pub amulet_sum:
                ::canton_splice_amulet::splice_amulet::Splice_Amulet::AmuletCreateSummary<
                    rt::ContractId<::canton_splice_amulet::splice_amulet::Splice_Amulet::Amulet>,
                >,
        }
        impl rt::ToValue for SubscriptionPayment_ExpireResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "subscriptionState",
                        rt::ToValue::to_value(&self.subscription_state)
                    ),
                    ("amuletSum", rt::ToValue::to_value(&self.amulet_sum)),
                ])
            }
        }
        impl rt::FromValue for SubscriptionPayment_ExpireResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    subscription_state: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "subscriptionState",
                    )?)
                    .map_err(|e| e.at("subscriptionState"))?,
                    amulet_sum: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "amuletSum",
                    )?)
                    .map_err(|e| e.at("amuletSum"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SubscriptionPayment_RejectResult {
            ///Daml field `subscriptionState`.
            #[serde(rename = "subscriptionState")]
            pub subscription_state: rt::ContractId<
                crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionIdleState,
            >,
            ///Daml field `amuletSum`.
            #[serde(rename = "amuletSum")]
            pub amulet_sum:
                ::canton_splice_amulet::splice_amulet::Splice_Amulet::AmuletCreateSummary<
                    rt::ContractId<::canton_splice_amulet::splice_amulet::Splice_Amulet::Amulet>,
                >,
        }
        impl rt::ToValue for SubscriptionPayment_RejectResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "subscriptionState",
                        rt::ToValue::to_value(&self.subscription_state)
                    ),
                    ("amuletSum", rt::ToValue::to_value(&self.amulet_sum)),
                ])
            }
        }
        impl rt::FromValue for SubscriptionPayment_RejectResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    subscription_state: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "subscriptionState",
                    )?)
                    .map_err(|e| e.at("subscriptionState"))?,
                    amulet_sum: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "amuletSum",
                    )?)
                    .map_err(|e| e.at("amuletSum"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SubscriptionPayment_CollectResult {
            ///Daml field `subscriptionState`.
            #[serde(rename = "subscriptionState")]
            pub subscription_state: rt::ContractId<
                crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionIdleState,
            >,
            #[serde(rename = "amulet")]
            pub amulet:
                rt::ContractId<::canton_splice_amulet::splice_amulet::Splice_Amulet::Amulet>,
        }
        impl rt::ToValue for SubscriptionPayment_CollectResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "subscriptionState",
                        rt::ToValue::to_value(&self.subscription_state)
                    ),
                    ("amulet", rt::ToValue::to_value(&self.amulet)),
                ])
            }
        }
        impl rt::FromValue for SubscriptionPayment_CollectResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    subscription_state: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "subscriptionState",
                    )?)
                    .map_err(|e| e.at("subscriptionState"))?,
                    amulet: rt::FromValue::from_value(rt::required_field(value, 1usize, "amulet")?)
                        .map_err(|e| e.at("amulet"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SubscriptionIdleState_MakePaymentResult {
            ///Daml field `subscriptionPayment`.
            #[serde(rename = "subscriptionPayment")]
            pub subscription_payment: rt::ContractId<
                crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionPayment,
            >,
            ///Daml field `senderChange`.
            #[serde(rename = "senderChange")]
            pub sender_change: ::core::option::Option<
                rt::ContractId<::canton_splice_amulet::splice_amulet::Splice_Amulet::Amulet>,
            >,
        }
        impl rt::ToValue for SubscriptionIdleState_MakePaymentResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "subscriptionPayment",
                        rt::ToValue::to_value(&self.subscription_payment)
                    ),
                    ("senderChange", rt::ToValue::to_value(&self.sender_change)),
                ])
            }
        }
        impl rt::FromValue for SubscriptionIdleState_MakePaymentResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    subscription_payment: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "subscriptionPayment",
                    )?)
                    .map_err(|e| e.at("subscriptionPayment"))?,
                    sender_change: rt::optional_field(value, 1usize, "senderChange")
                        .map_err(|e| e.at("senderChange"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SubscriptionIdleState_CancelSubscriptionResult {
            ///Daml field `terminatedSubscription`.
            #[serde(rename = "terminatedSubscription")]
            pub terminated_subscription: rt::ContractId<
                crate::splice_wallet_payments::Splice_Wallet_Subscriptions::TerminatedSubscription,
            >,
        }
        impl rt::ToValue for SubscriptionIdleState_CancelSubscriptionResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "terminatedSubscription",
                    rt::ToValue::to_value(&self.terminated_subscription)
                ),])
            }
        }
        impl rt::FromValue for SubscriptionIdleState_CancelSubscriptionResult {
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
        pub struct SubscriptionIdleState_ExpireSubscriptionResult {
            ///Daml field `terminatedSubscription`.
            #[serde(rename = "terminatedSubscription")]
            pub terminated_subscription: rt::ContractId<
                crate::splice_wallet_payments::Splice_Wallet_Subscriptions::TerminatedSubscription,
            >,
        }
        impl rt::ToValue for SubscriptionIdleState_ExpireSubscriptionResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "terminatedSubscription",
                    rt::ToValue::to_value(&self.terminated_subscription)
                ),])
            }
        }
        impl rt::FromValue for SubscriptionIdleState_ExpireSubscriptionResult {
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
        pub struct SubscriptionInitialPayment_ExpireResult {
            ///Daml field `amuletSum`.
            #[serde(rename = "amuletSum")]
            pub amulet_sum:
                ::canton_splice_amulet::splice_amulet::Splice_Amulet::AmuletCreateSummary<
                    rt::ContractId<::canton_splice_amulet::splice_amulet::Splice_Amulet::Amulet>,
                >,
        }
        impl rt::ToValue for SubscriptionInitialPayment_ExpireResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "amuletSum",
                    rt::ToValue::to_value(&self.amulet_sum)
                ),])
            }
        }
        impl rt::FromValue for SubscriptionInitialPayment_ExpireResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    amulet_sum: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "amuletSum",
                    )?)
                    .map_err(|e| e.at("amuletSum"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SubscriptionInitialPayment_RejectResult {
            ///Daml field `amuletSum`.
            #[serde(rename = "amuletSum")]
            pub amulet_sum:
                ::canton_splice_amulet::splice_amulet::Splice_Amulet::AmuletCreateSummary<
                    rt::ContractId<::canton_splice_amulet::splice_amulet::Splice_Amulet::Amulet>,
                >,
        }
        impl rt::ToValue for SubscriptionInitialPayment_RejectResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "amuletSum",
                    rt::ToValue::to_value(&self.amulet_sum)
                ),])
            }
        }
        impl rt::FromValue for SubscriptionInitialPayment_RejectResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    amulet_sum: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "amuletSum",
                    )?)
                    .map_err(|e| e.at("amuletSum"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SubscriptionInitialPayment_CollectResult {
            #[serde(rename = "subscription")]
            pub subscription: rt::ContractId<
                crate::splice_wallet_payments::Splice_Wallet_Subscriptions::Subscription,
            >,
            ///Daml field `subscriptionState`.
            #[serde(rename = "subscriptionState")]
            pub subscription_state: rt::ContractId<
                crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionIdleState,
            >,
            #[serde(rename = "amulet")]
            pub amulet:
                rt::ContractId<::canton_splice_amulet::splice_amulet::Splice_Amulet::Amulet>,
        }
        impl rt::ToValue for SubscriptionInitialPayment_CollectResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("subscription", rt::ToValue::to_value(&self.subscription)),
                    (
                        "subscriptionState",
                        rt::ToValue::to_value(&self.subscription_state)
                    ),
                    ("amulet", rt::ToValue::to_value(&self.amulet)),
                ])
            }
        }
        impl rt::FromValue for SubscriptionInitialPayment_CollectResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    subscription: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "subscription",
                    )?)
                    .map_err(|e| e.at("subscription"))?,
                    subscription_state: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "subscriptionState",
                    )?)
                    .map_err(|e| e.at("subscriptionState"))?,
                    amulet: rt::FromValue::from_value(rt::required_field(value, 2usize, "amulet")?)
                        .map_err(|e| e.at("amulet"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SubscriptionRequest_RejectResult {
            ///Daml field `terminatedSubscription`.
            #[serde(rename = "terminatedSubscription")]
            pub terminated_subscription: rt::ContractId<
                crate::splice_wallet_payments::Splice_Wallet_Subscriptions::TerminatedSubscription,
            >,
        }
        impl rt::ToValue for SubscriptionRequest_RejectResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "terminatedSubscription",
                    rt::ToValue::to_value(&self.terminated_subscription)
                ),])
            }
        }
        impl rt::FromValue for SubscriptionRequest_RejectResult {
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
        pub struct SubscriptionRequest_WithdrawResult {
            ///Daml field `terminatedSubscription`.
            #[serde(rename = "terminatedSubscription")]
            pub terminated_subscription: rt::ContractId<
                crate::splice_wallet_payments::Splice_Wallet_Subscriptions::TerminatedSubscription,
            >,
        }
        impl rt::ToValue for SubscriptionRequest_WithdrawResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "terminatedSubscription",
                    rt::ToValue::to_value(&self.terminated_subscription)
                ),])
            }
        }
        impl rt::FromValue for SubscriptionRequest_WithdrawResult {
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
        pub struct SubscriptionRequest_AcceptAndMakePaymentResult {
            ///Daml field `subscriptionPayment`.
            #[serde(rename = "subscriptionPayment")]
            pub subscription_payment: rt::ContractId<
                crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionInitialPayment,
            >,
            ///Daml field `senderChange`.
            #[serde(rename = "senderChange")]
            pub sender_change: ::core::option::Option<
                rt::ContractId<
                    ::canton_splice_amulet::splice_amulet::Splice_Amulet::Amulet,
                >,
            >,
        }
        impl rt::ToValue for SubscriptionRequest_AcceptAndMakePaymentResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "subscriptionPayment",
                        rt::ToValue::to_value(&self.subscription_payment)
                    ),
                    ("senderChange", rt::ToValue::to_value(&self.sender_change)),
                ])
            }
        }
        impl rt::FromValue for SubscriptionRequest_AcceptAndMakePaymentResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    subscription_payment: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "subscriptionPayment",
                    )?)
                    .map_err(|e| e.at("subscriptionPayment"))?,
                    sender_change: rt::optional_field(value, 1usize, "senderChange")
                        .map_err(|e| e.at("senderChange"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Subscription_ArchiveResult {
            ///Daml field `terminatedSubscription`.
            #[serde(rename = "terminatedSubscription")]
            pub terminated_subscription: rt::ContractId<
                crate::splice_wallet_payments::Splice_Wallet_Subscriptions::TerminatedSubscription,
            >,
        }
        impl rt::ToValue for Subscription_ArchiveResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "terminatedSubscription",
                    rt::ToValue::to_value(&self.terminated_subscription)
                ),])
            }
        }
        impl rt::FromValue for Subscription_ArchiveResult {
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
        pub struct SubscriptionData {
            #[serde(rename = "sender")]
            pub sender: rt::Party,
            #[serde(rename = "receiver")]
            pub receiver: rt::Party,
            #[serde(rename = "provider")]
            pub provider: rt::Party,
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            #[serde(rename = "description")]
            pub description: ::std::string::String,
        }
        impl rt::ToValue for SubscriptionData {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("sender", rt::ToValue::to_value(&self.sender)),
                    ("receiver", rt::ToValue::to_value(&self.receiver)),
                    ("provider", rt::ToValue::to_value(&self.provider)),
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("description", rt::ToValue::to_value(&self.description)),
                ])
            }
        }
        impl rt::FromValue for SubscriptionData {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    sender: rt::FromValue::from_value(rt::required_field(value, 0usize, "sender")?)
                        .map_err(|e| e.at("sender"))?,
                    receiver: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "receiver",
                    )?)
                    .map_err(|e| e.at("receiver"))?,
                    provider: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "provider",
                    )?)
                    .map_err(|e| e.at("provider"))?,
                    dso: rt::FromValue::from_value(rt::required_field(value, 3usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                    description: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "description",
                    )?)
                    .map_err(|e| e.at("description"))?,
                })
            }
        }
        ///The Daml template `Splice.Wallet.Subscriptions:Subscription`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-wallet-payments:Splice.Wallet.Subscriptions:Subscription`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `Subscription_Archive` — consuming
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Subscription {
            ///Daml field `subscriptionData`.
            #[serde(rename = "subscriptionData")]
            pub subscription_data:
                crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionData,
            #[serde(rename = "reference")]
            pub reference: rt::ContractId<
                crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionRequest,
            >,
        }
        impl rt::ToValue for Subscription {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "subscriptionData",
                        rt::ToValue::to_value(&self.subscription_data)
                    ),
                    ("reference", rt::ToValue::to_value(&self.reference)),
                ])
            }
        }
        impl rt::FromValue for Subscription {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    subscription_data: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "subscriptionData",
                    )?)
                    .map_err(|e| e.at("subscriptionData"))?,
                    reference: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "reference",
                    )?)
                    .map_err(|e| e.at("reference"))?,
                })
            }
        }
        impl rt::Contract for Subscription {
            const PACKAGE_ID: &'static str =
                "45b29d6e05b5352c39edde850c66b4535c682b9991b06eec312176b1a48ecab5";
            const PACKAGE_NAME: &'static str = "splice-wallet-payments";
            const MODULE_NAME: &'static str = "Splice.Wallet.Subscriptions";
            const ENTITY_NAME: &'static str = "Subscription";
        }
        impl rt::Template for Subscription {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    (
                        "subscriptionData",
                        rt::ToValue::to_value(&self.subscription_data)
                    ),
                    ("reference", rt::ToValue::to_value(&self.reference)),
                ])
            }
        }
        ///The `Subscription_Archive` choice on [`Subscription`] (consuming).
        impl rt::Choice<Subscription>
            for crate::splice_wallet_payments::Splice_Wallet_Subscriptions::Subscription_Archive
        {
            type Return = crate::splice_wallet_payments::Splice_Wallet_Subscriptions::Subscription_ArchiveResult;
            const NAME: &'static str = "Subscription_Archive";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`Subscription`] (consuming).
        impl rt::Choice<Subscription>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The Daml template `Splice.Wallet.Subscriptions:SubscriptionIdleState`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-wallet-payments:Splice.Wallet.Subscriptions:SubscriptionIdleState`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `SubscriptionIdleState_MakePayment` — consuming
        ///- `SubscriptionIdleState_ExpireSubscription` — consuming
        ///- `SubscriptionIdleState_CancelSubscription` — consuming
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SubscriptionIdleState {
            #[serde(rename = "subscription")]
            pub subscription: rt::ContractId<
                crate::splice_wallet_payments::Splice_Wallet_Subscriptions::Subscription,
            >,
            ///Daml field `subscriptionData`.
            #[serde(rename = "subscriptionData")]
            pub subscription_data:
                crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionData,
            ///Daml field `payData`.
            #[serde(rename = "payData")]
            pub pay_data:
                crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionPayData,
            ///Daml field `nextPaymentDueAt`.
            #[serde(rename = "nextPaymentDueAt")]
            pub next_payment_due_at: rt::Timestamp,
            #[serde(rename = "reference")]
            pub reference: rt::ContractId<
                crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionRequest,
            >,
        }
        impl rt::ToValue for SubscriptionIdleState {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("subscription", rt::ToValue::to_value(&self.subscription)),
                    (
                        "subscriptionData",
                        rt::ToValue::to_value(&self.subscription_data)
                    ),
                    ("payData", rt::ToValue::to_value(&self.pay_data)),
                    (
                        "nextPaymentDueAt",
                        rt::ToValue::to_value(&self.next_payment_due_at)
                    ),
                    ("reference", rt::ToValue::to_value(&self.reference)),
                ])
            }
        }
        impl rt::FromValue for SubscriptionIdleState {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    subscription: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "subscription",
                    )?)
                    .map_err(|e| e.at("subscription"))?,
                    subscription_data: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "subscriptionData",
                    )?)
                    .map_err(|e| e.at("subscriptionData"))?,
                    pay_data: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "payData",
                    )?)
                    .map_err(|e| e.at("payData"))?,
                    next_payment_due_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "nextPaymentDueAt",
                    )?)
                    .map_err(|e| e.at("nextPaymentDueAt"))?,
                    reference: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "reference",
                    )?)
                    .map_err(|e| e.at("reference"))?,
                })
            }
        }
        impl rt::Contract for SubscriptionIdleState {
            const PACKAGE_ID: &'static str =
                "45b29d6e05b5352c39edde850c66b4535c682b9991b06eec312176b1a48ecab5";
            const PACKAGE_NAME: &'static str = "splice-wallet-payments";
            const MODULE_NAME: &'static str = "Splice.Wallet.Subscriptions";
            const ENTITY_NAME: &'static str = "SubscriptionIdleState";
        }
        impl rt::Template for SubscriptionIdleState {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("subscription", rt::ToValue::to_value(&self.subscription)),
                    (
                        "subscriptionData",
                        rt::ToValue::to_value(&self.subscription_data)
                    ),
                    ("payData", rt::ToValue::to_value(&self.pay_data)),
                    (
                        "nextPaymentDueAt",
                        rt::ToValue::to_value(&self.next_payment_due_at)
                    ),
                    ("reference", rt::ToValue::to_value(&self.reference)),
                ])
            }
        }
        ///The `SubscriptionIdleState_MakePayment` choice on [`SubscriptionIdleState`] (consuming).
        impl rt::Choice<SubscriptionIdleState>
        for crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionIdleState_MakePayment {
            type Return = crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionIdleState_MakePaymentResult;
            const NAME: &'static str = "SubscriptionIdleState_MakePayment";
            const CONSUMING: bool = true;
        }
        ///The `SubscriptionIdleState_ExpireSubscription` choice on [`SubscriptionIdleState`] (consuming).
        impl rt::Choice<SubscriptionIdleState>
        for crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionIdleState_ExpireSubscription {
            type Return = crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionIdleState_ExpireSubscriptionResult;
            const NAME: &'static str = "SubscriptionIdleState_ExpireSubscription";
            const CONSUMING: bool = true;
        }
        ///The `SubscriptionIdleState_CancelSubscription` choice on [`SubscriptionIdleState`] (consuming).
        impl rt::Choice<SubscriptionIdleState>
        for crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionIdleState_CancelSubscription {
            type Return = crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionIdleState_CancelSubscriptionResult;
            const NAME: &'static str = "SubscriptionIdleState_CancelSubscription";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`SubscriptionIdleState`] (consuming).
        impl rt::Choice<SubscriptionIdleState>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The Daml template `Splice.Wallet.Subscriptions:SubscriptionInitialPayment`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-wallet-payments:Splice.Wallet.Subscriptions:SubscriptionInitialPayment`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `SubscriptionInitialPayment_Collect` — consuming
        ///- `SubscriptionInitialPayment_Reject` — consuming
        ///- `SubscriptionInitialPayment_Expire` — consuming
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SubscriptionInitialPayment {
            ///Daml field `subscriptionData`.
            #[serde(rename = "subscriptionData")]
            pub subscription_data:
                crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionData,
            ///Daml field `payData`.
            #[serde(rename = "payData")]
            pub pay_data:
                crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionPayData,
            ///Daml field `targetAmount`.
            #[serde(rename = "targetAmount")]
            pub target_amount: rt::Numeric,
            ///Daml field `lockedAmulet`.
            #[serde(rename = "lockedAmulet")]
            pub locked_amulet:
                rt::ContractId<::canton_splice_amulet::splice_amulet::Splice_Amulet::LockedAmulet>,
            #[serde(rename = "round")]
            pub round: ::canton_splice_amulet::splice_amulet::Splice_Types::Round,
            #[serde(rename = "reference")]
            pub reference: rt::ContractId<
                crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionRequest,
            >,
        }
        impl rt::ToValue for SubscriptionInitialPayment {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "subscriptionData",
                        rt::ToValue::to_value(&self.subscription_data)
                    ),
                    ("payData", rt::ToValue::to_value(&self.pay_data)),
                    ("targetAmount", rt::ToValue::to_value(&self.target_amount)),
                    ("lockedAmulet", rt::ToValue::to_value(&self.locked_amulet)),
                    ("round", rt::ToValue::to_value(&self.round)),
                    ("reference", rt::ToValue::to_value(&self.reference)),
                ])
            }
        }
        impl rt::FromValue for SubscriptionInitialPayment {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    subscription_data: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "subscriptionData",
                    )?)
                    .map_err(|e| e.at("subscriptionData"))?,
                    pay_data: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "payData",
                    )?)
                    .map_err(|e| e.at("payData"))?,
                    target_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "targetAmount",
                    )?)
                    .map_err(|e| e.at("targetAmount"))?,
                    locked_amulet: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "lockedAmulet",
                    )?)
                    .map_err(|e| e.at("lockedAmulet"))?,
                    round: rt::FromValue::from_value(rt::required_field(value, 4usize, "round")?)
                        .map_err(|e| e.at("round"))?,
                    reference: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "reference",
                    )?)
                    .map_err(|e| e.at("reference"))?,
                })
            }
        }
        impl rt::Contract for SubscriptionInitialPayment {
            const PACKAGE_ID: &'static str =
                "45b29d6e05b5352c39edde850c66b4535c682b9991b06eec312176b1a48ecab5";
            const PACKAGE_NAME: &'static str = "splice-wallet-payments";
            const MODULE_NAME: &'static str = "Splice.Wallet.Subscriptions";
            const ENTITY_NAME: &'static str = "SubscriptionInitialPayment";
        }
        impl rt::Template for SubscriptionInitialPayment {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    (
                        "subscriptionData",
                        rt::ToValue::to_value(&self.subscription_data)
                    ),
                    ("payData", rt::ToValue::to_value(&self.pay_data)),
                    ("targetAmount", rt::ToValue::to_value(&self.target_amount)),
                    ("lockedAmulet", rt::ToValue::to_value(&self.locked_amulet)),
                    ("round", rt::ToValue::to_value(&self.round)),
                    ("reference", rt::ToValue::to_value(&self.reference)),
                ])
            }
        }
        ///The `SubscriptionInitialPayment_Collect` choice on [`SubscriptionInitialPayment`] (consuming).
        impl rt::Choice<SubscriptionInitialPayment>
        for crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionInitialPayment_Collect {
            type Return = crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionInitialPayment_CollectResult;
            const NAME: &'static str = "SubscriptionInitialPayment_Collect";
            const CONSUMING: bool = true;
        }
        ///The `SubscriptionInitialPayment_Reject` choice on [`SubscriptionInitialPayment`] (consuming).
        impl rt::Choice<SubscriptionInitialPayment>
        for crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionInitialPayment_Reject {
            type Return = crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionInitialPayment_RejectResult;
            const NAME: &'static str = "SubscriptionInitialPayment_Reject";
            const CONSUMING: bool = true;
        }
        ///The `SubscriptionInitialPayment_Expire` choice on [`SubscriptionInitialPayment`] (consuming).
        impl rt::Choice<SubscriptionInitialPayment>
        for crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionInitialPayment_Expire {
            type Return = crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionInitialPayment_ExpireResult;
            const NAME: &'static str = "SubscriptionInitialPayment_Expire";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`SubscriptionInitialPayment`] (consuming).
        impl rt::Choice<SubscriptionInitialPayment>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The Daml template `Splice.Wallet.Subscriptions:SubscriptionPayment`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-wallet-payments:Splice.Wallet.Subscriptions:SubscriptionPayment`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `SubscriptionPayment_Collect` — consuming
        ///- `SubscriptionPayment_Reject` — consuming
        ///- `SubscriptionPayment_Expire` — consuming
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SubscriptionPayment {
            #[serde(rename = "subscription")]
            pub subscription: rt::ContractId<
                crate::splice_wallet_payments::Splice_Wallet_Subscriptions::Subscription,
            >,
            ///Daml field `subscriptionData`.
            #[serde(rename = "subscriptionData")]
            pub subscription_data:
                crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionData,
            ///Daml field `payData`.
            #[serde(rename = "payData")]
            pub pay_data:
                crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionPayData,
            ///Daml field `thisPaymentDueAt`.
            #[serde(rename = "thisPaymentDueAt")]
            pub this_payment_due_at: rt::Timestamp,
            ///Daml field `targetAmount`.
            #[serde(rename = "targetAmount")]
            pub target_amount: rt::Numeric,
            ///Daml field `lockedAmulet`.
            #[serde(rename = "lockedAmulet")]
            pub locked_amulet:
                rt::ContractId<::canton_splice_amulet::splice_amulet::Splice_Amulet::LockedAmulet>,
            #[serde(rename = "round")]
            pub round: ::canton_splice_amulet::splice_amulet::Splice_Types::Round,
            #[serde(rename = "reference")]
            pub reference: rt::ContractId<
                crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionRequest,
            >,
        }
        impl rt::ToValue for SubscriptionPayment {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("subscription", rt::ToValue::to_value(&self.subscription)),
                    (
                        "subscriptionData",
                        rt::ToValue::to_value(&self.subscription_data)
                    ),
                    ("payData", rt::ToValue::to_value(&self.pay_data)),
                    (
                        "thisPaymentDueAt",
                        rt::ToValue::to_value(&self.this_payment_due_at)
                    ),
                    ("targetAmount", rt::ToValue::to_value(&self.target_amount)),
                    ("lockedAmulet", rt::ToValue::to_value(&self.locked_amulet)),
                    ("round", rt::ToValue::to_value(&self.round)),
                    ("reference", rt::ToValue::to_value(&self.reference)),
                ])
            }
        }
        impl rt::FromValue for SubscriptionPayment {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    subscription: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "subscription",
                    )?)
                    .map_err(|e| e.at("subscription"))?,
                    subscription_data: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "subscriptionData",
                    )?)
                    .map_err(|e| e.at("subscriptionData"))?,
                    pay_data: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "payData",
                    )?)
                    .map_err(|e| e.at("payData"))?,
                    this_payment_due_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "thisPaymentDueAt",
                    )?)
                    .map_err(|e| e.at("thisPaymentDueAt"))?,
                    target_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "targetAmount",
                    )?)
                    .map_err(|e| e.at("targetAmount"))?,
                    locked_amulet: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "lockedAmulet",
                    )?)
                    .map_err(|e| e.at("lockedAmulet"))?,
                    round: rt::FromValue::from_value(rt::required_field(value, 6usize, "round")?)
                        .map_err(|e| e.at("round"))?,
                    reference: rt::FromValue::from_value(rt::required_field(
                        value,
                        7usize,
                        "reference",
                    )?)
                    .map_err(|e| e.at("reference"))?,
                })
            }
        }
        impl rt::Contract for SubscriptionPayment {
            const PACKAGE_ID: &'static str =
                "45b29d6e05b5352c39edde850c66b4535c682b9991b06eec312176b1a48ecab5";
            const PACKAGE_NAME: &'static str = "splice-wallet-payments";
            const MODULE_NAME: &'static str = "Splice.Wallet.Subscriptions";
            const ENTITY_NAME: &'static str = "SubscriptionPayment";
        }
        impl rt::Template for SubscriptionPayment {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("subscription", rt::ToValue::to_value(&self.subscription)),
                    (
                        "subscriptionData",
                        rt::ToValue::to_value(&self.subscription_data)
                    ),
                    ("payData", rt::ToValue::to_value(&self.pay_data)),
                    (
                        "thisPaymentDueAt",
                        rt::ToValue::to_value(&self.this_payment_due_at)
                    ),
                    ("targetAmount", rt::ToValue::to_value(&self.target_amount)),
                    ("lockedAmulet", rt::ToValue::to_value(&self.locked_amulet)),
                    ("round", rt::ToValue::to_value(&self.round)),
                    ("reference", rt::ToValue::to_value(&self.reference)),
                ])
            }
        }
        ///The `SubscriptionPayment_Collect` choice on [`SubscriptionPayment`] (consuming).
        impl rt::Choice<SubscriptionPayment>
        for crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionPayment_Collect {
            type Return = crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionPayment_CollectResult;
            const NAME: &'static str = "SubscriptionPayment_Collect";
            const CONSUMING: bool = true;
        }
        ///The `SubscriptionPayment_Reject` choice on [`SubscriptionPayment`] (consuming).
        impl rt::Choice<SubscriptionPayment>
        for crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionPayment_Reject {
            type Return = crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionPayment_RejectResult;
            const NAME: &'static str = "SubscriptionPayment_Reject";
            const CONSUMING: bool = true;
        }
        ///The `SubscriptionPayment_Expire` choice on [`SubscriptionPayment`] (consuming).
        impl rt::Choice<SubscriptionPayment>
        for crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionPayment_Expire {
            type Return = crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionPayment_ExpireResult;
            const NAME: &'static str = "SubscriptionPayment_Expire";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`SubscriptionPayment`] (consuming).
        impl rt::Choice<SubscriptionPayment>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The Daml template `Splice.Wallet.Subscriptions:SubscriptionRequest`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-wallet-payments:Splice.Wallet.Subscriptions:SubscriptionRequest`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `SubscriptionRequest_AcceptAndMakePayment` — consuming
        ///- `SubscriptionRequest_Withdraw` — consuming
        ///- `SubscriptionRequest_Reject` — consuming
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SubscriptionRequest {
            ///Daml field `subscriptionData`.
            #[serde(rename = "subscriptionData")]
            pub subscription_data:
                crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionData,
            ///Daml field `payData`.
            #[serde(rename = "payData")]
            pub pay_data:
                crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionPayData,
        }
        impl rt::ToValue for SubscriptionRequest {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "subscriptionData",
                        rt::ToValue::to_value(&self.subscription_data)
                    ),
                    ("payData", rt::ToValue::to_value(&self.pay_data)),
                ])
            }
        }
        impl rt::FromValue for SubscriptionRequest {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    subscription_data: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "subscriptionData",
                    )?)
                    .map_err(|e| e.at("subscriptionData"))?,
                    pay_data: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "payData",
                    )?)
                    .map_err(|e| e.at("payData"))?,
                })
            }
        }
        impl rt::Contract for SubscriptionRequest {
            const PACKAGE_ID: &'static str =
                "45b29d6e05b5352c39edde850c66b4535c682b9991b06eec312176b1a48ecab5";
            const PACKAGE_NAME: &'static str = "splice-wallet-payments";
            const MODULE_NAME: &'static str = "Splice.Wallet.Subscriptions";
            const ENTITY_NAME: &'static str = "SubscriptionRequest";
        }
        impl rt::Template for SubscriptionRequest {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    (
                        "subscriptionData",
                        rt::ToValue::to_value(&self.subscription_data)
                    ),
                    ("payData", rt::ToValue::to_value(&self.pay_data)),
                ])
            }
        }
        ///The `SubscriptionRequest_AcceptAndMakePayment` choice on [`SubscriptionRequest`] (consuming).
        impl rt::Choice<SubscriptionRequest>
        for crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionRequest_AcceptAndMakePayment {
            type Return = crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionRequest_AcceptAndMakePaymentResult;
            const NAME: &'static str = "SubscriptionRequest_AcceptAndMakePayment";
            const CONSUMING: bool = true;
        }
        ///The `SubscriptionRequest_Withdraw` choice on [`SubscriptionRequest`] (consuming).
        impl rt::Choice<SubscriptionRequest>
        for crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionRequest_Withdraw {
            type Return = crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionRequest_WithdrawResult;
            const NAME: &'static str = "SubscriptionRequest_Withdraw";
            const CONSUMING: bool = true;
        }
        ///The `SubscriptionRequest_Reject` choice on [`SubscriptionRequest`] (consuming).
        impl rt::Choice<SubscriptionRequest>
        for crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionRequest_Reject {
            type Return = crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionRequest_RejectResult;
            const NAME: &'static str = "SubscriptionRequest_Reject";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`SubscriptionRequest`] (consuming).
        impl rt::Choice<SubscriptionRequest>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The Daml template `Splice.Wallet.Subscriptions:TerminatedSubscription`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-wallet-payments:Splice.Wallet.Subscriptions:TerminatedSubscription`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TerminatedSubscription {
            ///Daml field `subscriptionData`.
            #[serde(rename = "subscriptionData")]
            pub subscription_data:
                crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionData,
            #[serde(rename = "reference")]
            pub reference: rt::ContractId<
                crate::splice_wallet_payments::Splice_Wallet_Subscriptions::SubscriptionRequest,
            >,
        }
        impl rt::ToValue for TerminatedSubscription {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "subscriptionData",
                        rt::ToValue::to_value(&self.subscription_data)
                    ),
                    ("reference", rt::ToValue::to_value(&self.reference)),
                ])
            }
        }
        impl rt::FromValue for TerminatedSubscription {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    subscription_data: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "subscriptionData",
                    )?)
                    .map_err(|e| e.at("subscriptionData"))?,
                    reference: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "reference",
                    )?)
                    .map_err(|e| e.at("reference"))?,
                })
            }
        }
        impl rt::Contract for TerminatedSubscription {
            const PACKAGE_ID: &'static str =
                "45b29d6e05b5352c39edde850c66b4535c682b9991b06eec312176b1a48ecab5";
            const PACKAGE_NAME: &'static str = "splice-wallet-payments";
            const MODULE_NAME: &'static str = "Splice.Wallet.Subscriptions";
            const ENTITY_NAME: &'static str = "TerminatedSubscription";
        }
        impl rt::Template for TerminatedSubscription {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    (
                        "subscriptionData",
                        rt::ToValue::to_value(&self.subscription_data)
                    ),
                    ("reference", rt::ToValue::to_value(&self.reference)),
                ])
            }
        }
        ///The `Archive` choice on [`TerminatedSubscription`] (consuming).
        impl rt::Choice<TerminatedSubscription>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
    }
}
