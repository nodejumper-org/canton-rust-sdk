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
//! - [`splice_amulet`] — 17 Daml modules

pub mod splice_amulet {
    pub mod Splice_Types {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ForOwner {
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            #[serde(rename = "owner")]
            pub owner: rt::Party,
        }
        impl rt::ToValue for ForOwner {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("owner", rt::ToValue::to_value(&self.owner)),
                ])
            }
        }
        impl rt::FromValue for ForOwner {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                    owner: rt::FromValue::from_value(rt::required_field(value, 1usize, "owner")?)
                        .map_err(|e| e.at("owner"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ForRound {
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            #[serde(rename = "round")]
            pub round: crate::splice_amulet::Splice_Types::Round,
        }
        impl rt::ToValue for ForRound {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("round", rt::ToValue::to_value(&self.round)),
                ])
            }
        }
        impl rt::FromValue for ForRound {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                    round: rt::FromValue::from_value(rt::required_field(value, 1usize, "round")?)
                        .map_err(|e| e.at("round"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ForDso {
            #[serde(rename = "dso")]
            pub dso: rt::Party,
        }
        impl rt::ToValue for ForDso {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("dso", rt::ToValue::to_value(&self.dso)),])
            }
        }
        impl rt::FromValue for ForDso {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Round {
            #[serde(rename = "number")]
            pub number: rt::Int64,
        }
        impl rt::ToValue for Round {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("number", rt::ToValue::to_value(&self.number)),])
            }
        }
        impl rt::FromValue for Round {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    number: rt::FromValue::from_value(rt::required_field(value, 0usize, "number")?)
                        .map_err(|e| e.at("number"))?,
                })
            }
        }
    }
    pub mod Splice_AmuletRules {
        use canton_daml as rt;
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize,
        )]
        #[serde(crate = "rt::serde")]
        pub enum TransferPreapproval_CancelResult {
            #[serde(rename = "TransferPreapproval_CancelResult")]
            TransferPreapproval_CancelResult,
        }
        impl rt::ToValue for TransferPreapproval_CancelResult {
            fn to_value(&self) -> rt::Value {
                rt::enum_value(match self {
                    TransferPreapproval_CancelResult::TransferPreapproval_CancelResult => {
                        "TransferPreapproval_CancelResult"
                    }
                })
            }
        }
        impl rt::FromValue for TransferPreapproval_CancelResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                match rt::enum_constructor(value)? {
                    "TransferPreapproval_CancelResult" => ::core::result::Result::Ok(
                        TransferPreapproval_CancelResult::TransferPreapproval_CancelResult,
                    ),
                    other => ::core::result::Result::Err(rt::unexpected_constructor(
                        "TransferPreapproval_CancelResult",
                        other,
                    )),
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferPreapproval_ExpireResult {}
        impl rt::ToValue for TransferPreapproval_ExpireResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for TransferPreapproval_ExpireResult {
            fn from_value(_value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferPreapproval_RenewResult {
            ///Daml field `transferPreapprovalCid`.
            #[serde(rename = "transferPreapprovalCid")]
            pub transfer_preapproval_cid: rt::ContractId<
                crate::splice_amulet::Splice_AmuletRules::TransferPreapproval,
            >,
            ///Daml field `transferResult`.
            #[serde(rename = "transferResult")]
            pub transfer_result: crate::splice_amulet::Splice_AmuletRules::TransferResult,
            #[serde(rename = "receiver")]
            pub receiver: rt::Party,
            #[serde(rename = "provider")]
            pub provider: rt::Party,
            ///Daml field `amuletPaid`.
            #[serde(rename = "amuletPaid")]
            pub amulet_paid: rt::Numeric,
            #[serde(rename = "meta")]
            pub meta: ::core::option::Option<
                ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
            >,
        }
        impl rt::ToValue for TransferPreapproval_RenewResult {
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
                    ("receiver", rt::ToValue::to_value(&self.receiver)),
                    ("provider", rt::ToValue::to_value(&self.provider)),
                    ("amuletPaid", rt::ToValue::to_value(&self.amulet_paid)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for TransferPreapproval_RenewResult {
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
                    receiver: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "receiver",
                    )?)
                    .map_err(|e| e.at("receiver"))?,
                    provider: rt::FromValue::from_value(rt::required_field(
                        value, 3usize, "provider",
                    )?)
                    .map_err(|e| e.at("provider"))?,
                    amulet_paid: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "amuletPaid",
                    )?)
                    .map_err(|e| e.at("amuletPaid"))?,
                    meta: rt::optional_field(value, 5usize, "meta").map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferPreapproval_SendResult {
            #[serde(rename = "result")]
            pub result: crate::splice_amulet::Splice_AmuletRules::TransferResult,
            #[serde(rename = "meta")]
            pub meta: ::core::option::Option<
                ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
            >,
        }
        impl rt::ToValue for TransferPreapproval_SendResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("result", rt::ToValue::to_value(&self.result)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for TransferPreapproval_SendResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    result: rt::FromValue::from_value(rt::required_field(value, 0usize, "result")?)
                        .map_err(|e| e.at("result"))?,
                    meta: rt::optional_field(value, 1usize, "meta").map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferPreapproval_Cancel {
            #[serde(rename = "p")]
            pub p: rt::Party,
        }
        impl rt::ToValue for TransferPreapproval_Cancel {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("p", rt::ToValue::to_value(&self.p)),])
            }
        }
        impl rt::FromValue for TransferPreapproval_Cancel {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    p: rt::FromValue::from_value(rt::required_field(value, 0usize, "p")?)
                        .map_err(|e| e.at("p"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferPreapproval_Expire {}
        impl rt::ToValue for TransferPreapproval_Expire {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for TransferPreapproval_Expire {
            fn from_value(_value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferPreapproval_Renew {
            #[serde(rename = "context")]
            pub context: crate::splice_amulet::Splice_AmuletRules::PaymentTransferContext,
            #[serde(rename = "inputs")]
            pub inputs: ::std::vec::Vec<crate::splice_amulet::Splice_AmuletRules::TransferInput>,
            ///Daml field `newExpiresAt`.
            #[serde(rename = "newExpiresAt")]
            pub new_expires_at: rt::Timestamp,
        }
        impl rt::ToValue for TransferPreapproval_Renew {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("context", rt::ToValue::to_value(&self.context)),
                    ("inputs", rt::ToValue::to_value(&self.inputs)),
                    ("newExpiresAt", rt::ToValue::to_value(&self.new_expires_at)),
                ])
            }
        }
        impl rt::FromValue for TransferPreapproval_Renew {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    context: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "context",
                    )?)
                    .map_err(|e| e.at("context"))?,
                    inputs: rt::FromValue::from_value(rt::required_field(value, 1usize, "inputs")?)
                        .map_err(|e| e.at("inputs"))?,
                    new_expires_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "newExpiresAt",
                    )?)
                    .map_err(|e| e.at("newExpiresAt"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferPreapproval_Send {
            #[serde(rename = "context")]
            pub context: crate::splice_amulet::Splice_AmuletRules::PaymentTransferContext,
            #[serde(rename = "inputs")]
            pub inputs: ::std::vec::Vec<crate::splice_amulet::Splice_AmuletRules::TransferInput>,
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            #[serde(rename = "sender")]
            pub sender: rt::Party,
            #[serde(rename = "description")]
            pub description: ::core::option::Option<::std::string::String>,
        }
        impl rt::ToValue for TransferPreapproval_Send {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("context", rt::ToValue::to_value(&self.context)),
                    ("inputs", rt::ToValue::to_value(&self.inputs)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                    ("sender", rt::ToValue::to_value(&self.sender)),
                    ("description", rt::ToValue::to_value(&self.description)),
                ])
            }
        }
        impl rt::FromValue for TransferPreapproval_Send {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    context: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "context",
                    )?)
                    .map_err(|e| e.at("context"))?,
                    inputs: rt::FromValue::from_value(rt::required_field(value, 1usize, "inputs")?)
                        .map_err(|e| e.at("inputs"))?,
                    amount: rt::FromValue::from_value(rt::required_field(value, 2usize, "amount")?)
                        .map_err(|e| e.at("amount"))?,
                    sender: rt::FromValue::from_value(rt::required_field(value, 3usize, "sender")?)
                        .map_err(|e| e.at("sender"))?,
                    description: rt::optional_field(value, 4usize, "description")
                        .map_err(|e| e.at("description"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferPreapproval_Fetch {
            #[serde(rename = "p")]
            pub p: rt::Party,
        }
        impl rt::ToValue for TransferPreapproval_Fetch {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("p", rt::ToValue::to_value(&self.p)),])
            }
        }
        impl rt::FromValue for TransferPreapproval_Fetch {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    p: rt::FromValue::from_value(rt::required_field(value, 0usize, "p")?)
                        .map_err(|e| e.at("p"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ExternalPartySetupProposal_WithdrawResult {
            ///Daml field `dummyArg`.
            #[serde(rename = "dummyArg")]
            pub dummy_arg: rt::Unit,
        }
        impl rt::ToValue for ExternalPartySetupProposal_WithdrawResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "dummyArg",
                    rt::ToValue::to_value(&self.dummy_arg)
                ),])
            }
        }
        impl rt::FromValue for ExternalPartySetupProposal_WithdrawResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dummy_arg: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "dummyArg",
                    )?)
                    .map_err(|e| e.at("dummyArg"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ExternalPartySetupProposal_RejectResult {
            ///Daml field `dummyArg`.
            #[serde(rename = "dummyArg")]
            pub dummy_arg: rt::Unit,
        }
        impl rt::ToValue for ExternalPartySetupProposal_RejectResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "dummyArg",
                    rt::ToValue::to_value(&self.dummy_arg)
                ),])
            }
        }
        impl rt::FromValue for ExternalPartySetupProposal_RejectResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dummy_arg: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "dummyArg",
                    )?)
                    .map_err(|e| e.at("dummyArg"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ExternalPartySetupProposal_AcceptResult {
            ///Daml field `validatorRightCid`.
            #[serde(rename = "validatorRightCid")]
            pub validator_right_cid:
                rt::ContractId<crate::splice_amulet::Splice_Amulet::ValidatorRight>,
            ///Daml field `transferPreapprovalCid`.
            #[serde(rename = "transferPreapprovalCid")]
            pub transfer_preapproval_cid:
                rt::ContractId<crate::splice_amulet::Splice_AmuletRules::TransferPreapproval>,
        }
        impl rt::ToValue for ExternalPartySetupProposal_AcceptResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "validatorRightCid",
                        rt::ToValue::to_value(&self.validator_right_cid)
                    ),
                    (
                        "transferPreapprovalCid",
                        rt::ToValue::to_value(&self.transfer_preapproval_cid)
                    ),
                ])
            }
        }
        impl rt::FromValue for ExternalPartySetupProposal_AcceptResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    validator_right_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "validatorRightCid",
                    )?)
                    .map_err(|e| e.at("validatorRightCid"))?,
                    transfer_preapproval_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "transferPreapprovalCid",
                    )?)
                    .map_err(|e| e.at("transferPreapprovalCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ExternalPartySetupProposal_Withdraw {
            #[serde(rename = "reason")]
            pub reason: ::std::string::String,
        }
        impl rt::ToValue for ExternalPartySetupProposal_Withdraw {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("reason", rt::ToValue::to_value(&self.reason)),])
            }
        }
        impl rt::FromValue for ExternalPartySetupProposal_Withdraw {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    reason: rt::FromValue::from_value(rt::required_field(value, 0usize, "reason")?)
                        .map_err(|e| e.at("reason"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ExternalPartySetupProposal_Reject {
            #[serde(rename = "reason")]
            pub reason: ::std::string::String,
        }
        impl rt::ToValue for ExternalPartySetupProposal_Reject {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("reason", rt::ToValue::to_value(&self.reason)),])
            }
        }
        impl rt::FromValue for ExternalPartySetupProposal_Reject {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    reason: rt::FromValue::from_value(rt::required_field(value, 0usize, "reason")?)
                        .map_err(|e| e.at("reason"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ExternalPartySetupProposal_Accept {}
        impl rt::ToValue for ExternalPartySetupProposal_Accept {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for ExternalPartySetupProposal_Accept {
            fn from_value(_value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct BalanceChange {
            ///Daml field `changeToInitialAmountAsOfRoundZero`.
            #[serde(rename = "changeToInitialAmountAsOfRoundZero")]
            pub change_to_initial_amount_as_of_round_zero: rt::Numeric,
            ///Daml field `changeToHoldingFeesRate`.
            #[serde(rename = "changeToHoldingFeesRate")]
            pub change_to_holding_fees_rate: rt::Numeric,
        }
        impl rt::ToValue for BalanceChange {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "changeToInitialAmountAsOfRoundZero",
                        rt::ToValue::to_value(&self.change_to_initial_amount_as_of_round_zero)
                    ),
                    (
                        "changeToHoldingFeesRate",
                        rt::ToValue::to_value(&self.change_to_holding_fees_rate)
                    ),
                ])
            }
        }
        impl rt::FromValue for BalanceChange {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    change_to_initial_amount_as_of_round_zero: rt::FromValue::from_value(
                        rt::required_field(value, 0usize, "changeToInitialAmountAsOfRoundZero")?,
                    )
                    .map_err(|e| e.at("changeToInitialAmountAsOfRoundZero"))?,
                    change_to_holding_fees_rate: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "changeToHoldingFeesRate",
                    )?)
                    .map_err(|e| e.at("changeToHoldingFeesRate"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferSummary {
            ///Daml field `inputAppRewardAmount`.
            #[serde(rename = "inputAppRewardAmount")]
            pub input_app_reward_amount: rt::Numeric,
            ///Daml field `inputValidatorRewardAmount`.
            #[serde(rename = "inputValidatorRewardAmount")]
            pub input_validator_reward_amount: rt::Numeric,
            ///Daml field `inputSvRewardAmount`.
            #[serde(rename = "inputSvRewardAmount")]
            pub input_sv_reward_amount: rt::Numeric,
            ///Daml field `inputAmuletAmount`.
            #[serde(rename = "inputAmuletAmount")]
            pub input_amulet_amount: rt::Numeric,
            ///Daml field `balanceChanges`.
            #[serde(rename = "balanceChanges")]
            pub balance_changes:
                rt::GenMap<rt::Party, crate::splice_amulet::Splice_AmuletRules::BalanceChange>,
            ///Daml field `holdingFees`.
            #[serde(rename = "holdingFees")]
            pub holding_fees: rt::Numeric,
            ///Daml field `outputFees`.
            #[serde(rename = "outputFees")]
            pub output_fees: ::std::vec::Vec<rt::Numeric>,
            ///Daml field `senderChangeFee`.
            #[serde(rename = "senderChangeFee")]
            pub sender_change_fee: rt::Numeric,
            ///Daml field `senderChangeAmount`.
            #[serde(rename = "senderChangeAmount")]
            pub sender_change_amount: rt::Numeric,
            ///Daml field `amuletPrice`.
            #[serde(rename = "amuletPrice")]
            pub amulet_price: rt::Numeric,
            ///Daml field `inputValidatorFaucetAmount`.
            #[serde(rename = "inputValidatorFaucetAmount")]
            pub input_validator_faucet_amount: ::core::option::Option<rt::Numeric>,
            ///Daml field `inputUnclaimedActivityRecordAmount`.
            #[serde(rename = "inputUnclaimedActivityRecordAmount")]
            pub input_unclaimed_activity_record_amount: ::core::option::Option<rt::Numeric>,
        }
        impl rt::ToValue for TransferSummary {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "inputAppRewardAmount",
                        rt::ToValue::to_value(&self.input_app_reward_amount)
                    ),
                    (
                        "inputValidatorRewardAmount",
                        rt::ToValue::to_value(&self.input_validator_reward_amount)
                    ),
                    (
                        "inputSvRewardAmount",
                        rt::ToValue::to_value(&self.input_sv_reward_amount)
                    ),
                    (
                        "inputAmuletAmount",
                        rt::ToValue::to_value(&self.input_amulet_amount)
                    ),
                    (
                        "balanceChanges",
                        rt::ToValue::to_value(&self.balance_changes)
                    ),
                    ("holdingFees", rt::ToValue::to_value(&self.holding_fees)),
                    ("outputFees", rt::ToValue::to_value(&self.output_fees)),
                    (
                        "senderChangeFee",
                        rt::ToValue::to_value(&self.sender_change_fee)
                    ),
                    (
                        "senderChangeAmount",
                        rt::ToValue::to_value(&self.sender_change_amount)
                    ),
                    ("amuletPrice", rt::ToValue::to_value(&self.amulet_price)),
                    (
                        "inputValidatorFaucetAmount",
                        rt::ToValue::to_value(&self.input_validator_faucet_amount)
                    ),
                    (
                        "inputUnclaimedActivityRecordAmount",
                        rt::ToValue::to_value(&self.input_unclaimed_activity_record_amount)
                    ),
                ])
            }
        }
        impl rt::FromValue for TransferSummary {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    input_app_reward_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "inputAppRewardAmount",
                    )?)
                    .map_err(|e| e.at("inputAppRewardAmount"))?,
                    input_validator_reward_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "inputValidatorRewardAmount",
                    )?)
                    .map_err(|e| e.at("inputValidatorRewardAmount"))?,
                    input_sv_reward_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "inputSvRewardAmount",
                    )?)
                    .map_err(|e| e.at("inputSvRewardAmount"))?,
                    input_amulet_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "inputAmuletAmount",
                    )?)
                    .map_err(|e| e.at("inputAmuletAmount"))?,
                    balance_changes: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "balanceChanges",
                    )?)
                    .map_err(|e| e.at("balanceChanges"))?,
                    holding_fees: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "holdingFees",
                    )?)
                    .map_err(|e| e.at("holdingFees"))?,
                    output_fees: rt::FromValue::from_value(rt::required_field(
                        value,
                        6usize,
                        "outputFees",
                    )?)
                    .map_err(|e| e.at("outputFees"))?,
                    sender_change_fee: rt::FromValue::from_value(rt::required_field(
                        value,
                        7usize,
                        "senderChangeFee",
                    )?)
                    .map_err(|e| e.at("senderChangeFee"))?,
                    sender_change_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        8usize,
                        "senderChangeAmount",
                    )?)
                    .map_err(|e| e.at("senderChangeAmount"))?,
                    amulet_price: rt::FromValue::from_value(rt::required_field(
                        value,
                        9usize,
                        "amuletPrice",
                    )?)
                    .map_err(|e| e.at("amuletPrice"))?,
                    input_validator_faucet_amount: rt::optional_field(
                        value,
                        10usize,
                        "inputValidatorFaucetAmount",
                    )
                    .map_err(|e| e.at("inputValidatorFaucetAmount"))?,
                    input_unclaimed_activity_record_amount: rt::optional_field(
                        value,
                        11usize,
                        "inputUnclaimedActivityRecordAmount",
                    )
                    .map_err(|e| e.at("inputUnclaimedActivityRecordAmount"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_CreateTransferPreapprovalResult {
            ///Daml field `transferPreapprovalCid`.
            #[serde(rename = "transferPreapprovalCid")]
            pub transfer_preapproval_cid: rt::ContractId<
                crate::splice_amulet::Splice_AmuletRules::TransferPreapproval,
            >,
            ///Daml field `transferResult`.
            #[serde(rename = "transferResult")]
            pub transfer_result: crate::splice_amulet::Splice_AmuletRules::TransferResult,
            ///Daml field `amuletPaid`.
            #[serde(rename = "amuletPaid")]
            pub amulet_paid: rt::Numeric,
            #[serde(rename = "meta")]
            pub meta: ::core::option::Option<
                ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
            >,
        }
        impl rt::ToValue for AmuletRules_CreateTransferPreapprovalResult {
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
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for AmuletRules_CreateTransferPreapprovalResult {
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
                    meta: rt::optional_field(value, 3usize, "meta").map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_CreateExternalPartySetupProposalResult {
            ///Daml field `proposalCid`.
            #[serde(rename = "proposalCid")]
            pub proposal_cid: rt::ContractId<
                crate::splice_amulet::Splice_AmuletRules::ExternalPartySetupProposal,
            >,
            #[serde(rename = "user")]
            pub user: rt::Party,
            #[serde(rename = "validator")]
            pub validator: rt::Party,
            ///Daml field `transferResult`.
            #[serde(rename = "transferResult")]
            pub transfer_result: crate::splice_amulet::Splice_AmuletRules::TransferResult,
            ///Daml field `amuletPaid`.
            #[serde(rename = "amuletPaid")]
            pub amulet_paid: rt::Numeric,
            #[serde(rename = "meta")]
            pub meta: ::core::option::Option<
                ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
            >,
        }
        impl rt::ToValue for AmuletRules_CreateExternalPartySetupProposalResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("proposalCid", rt::ToValue::to_value(&self.proposal_cid)),
                    ("user", rt::ToValue::to_value(&self.user)),
                    ("validator", rt::ToValue::to_value(&self.validator)),
                    (
                        "transferResult",
                        rt::ToValue::to_value(&self.transfer_result)
                    ),
                    ("amuletPaid", rt::ToValue::to_value(&self.amulet_paid)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for AmuletRules_CreateExternalPartySetupProposalResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    proposal_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "proposalCid",
                    )?)
                    .map_err(|e| e.at("proposalCid"))?,
                    user: rt::FromValue::from_value(rt::required_field(value, 1usize, "user")?)
                        .map_err(|e| e.at("user"))?,
                    validator: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "validator",
                    )?)
                    .map_err(|e| e.at("validator"))?,
                    transfer_result: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "transferResult",
                    )?)
                    .map_err(|e| e.at("transferResult"))?,
                    amulet_paid: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "amuletPaid",
                    )?)
                    .map_err(|e| e.at("amuletPaid"))?,
                    meta: rt::optional_field(value, 5usize, "meta").map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_BuyMemberTrafficResult {
            #[serde(rename = "round")]
            pub round: crate::splice_amulet::Splice_Types::Round,
            #[serde(rename = "summary")]
            pub summary: crate::splice_amulet::Splice_AmuletRules::TransferSummary,
            ///Daml field `amuletPaid`.
            #[serde(rename = "amuletPaid")]
            pub amulet_paid: rt::Numeric,
            ///Daml field `purchasedTraffic`.
            #[serde(rename = "purchasedTraffic")]
            pub purchased_traffic: rt::ContractId<
                crate::splice_amulet::Splice_DecentralizedSynchronizer::MemberTraffic,
            >,
            ///Daml field `senderChangeAmulet`.
            #[serde(rename = "senderChangeAmulet")]
            pub sender_change_amulet: ::core::option::Option<
                rt::ContractId<crate::splice_amulet::Splice_Amulet::Amulet>,
            >,
            #[serde(rename = "meta")]
            pub meta: ::core::option::Option<
                ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
            >,
        }
        impl rt::ToValue for AmuletRules_BuyMemberTrafficResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("round", rt::ToValue::to_value(&self.round)),
                    ("summary", rt::ToValue::to_value(&self.summary)),
                    ("amuletPaid", rt::ToValue::to_value(&self.amulet_paid)),
                    (
                        "purchasedTraffic",
                        rt::ToValue::to_value(&self.purchased_traffic)
                    ),
                    (
                        "senderChangeAmulet",
                        rt::ToValue::to_value(&self.sender_change_amulet)
                    ),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for AmuletRules_BuyMemberTrafficResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    round: rt::FromValue::from_value(rt::required_field(value, 0usize, "round")?)
                        .map_err(|e| e.at("round"))?,
                    summary: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "summary",
                    )?)
                    .map_err(|e| e.at("summary"))?,
                    amulet_paid: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "amuletPaid",
                    )?)
                    .map_err(|e| e.at("amuletPaid"))?,
                    purchased_traffic: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "purchasedTraffic",
                    )?)
                    .map_err(|e| e.at("purchasedTraffic"))?,
                    sender_change_amulet: rt::optional_field(value, 4usize, "senderChangeAmulet")
                        .map_err(|e| e.at("senderChangeAmulet"))?,
                    meta: rt::optional_field(value, 5usize, "meta").map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferResult {
            #[serde(rename = "round")]
            pub round: crate::splice_amulet::Splice_Types::Round,
            #[serde(rename = "summary")]
            pub summary: crate::splice_amulet::Splice_AmuletRules::TransferSummary,
            ///Daml field `createdAmulets`.
            #[serde(rename = "createdAmulets")]
            pub created_amulets: ::std::vec::Vec<
                crate::splice_amulet::Splice_AmuletRules::CreatedAmulet,
            >,
            ///Daml field `senderChangeAmulet`.
            #[serde(rename = "senderChangeAmulet")]
            pub sender_change_amulet: ::core::option::Option<
                rt::ContractId<crate::splice_amulet::Splice_Amulet::Amulet>,
            >,
            #[serde(rename = "meta")]
            pub meta: ::core::option::Option<
                ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
            >,
        }
        impl rt::ToValue for TransferResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("round", rt::ToValue::to_value(&self.round)),
                    ("summary", rt::ToValue::to_value(&self.summary)),
                    (
                        "createdAmulets",
                        rt::ToValue::to_value(&self.created_amulets)
                    ),
                    (
                        "senderChangeAmulet",
                        rt::ToValue::to_value(&self.sender_change_amulet)
                    ),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for TransferResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    round: rt::FromValue::from_value(rt::required_field(value, 0usize, "round")?)
                        .map_err(|e| e.at("round"))?,
                    summary: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "summary",
                    )?)
                    .map_err(|e| e.at("summary"))?,
                    created_amulets: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "createdAmulets",
                    )?)
                    .map_err(|e| e.at("createdAmulets"))?,
                    sender_change_amulet: rt::optional_field(value, 3usize, "senderChangeAmulet")
                        .map_err(|e| e.at("senderChangeAmulet"))?,
                    meta: rt::optional_field(value, 4usize, "meta").map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferOutput {
            #[serde(rename = "receiver")]
            pub receiver: rt::Party,
            ///Daml field `receiverFeeRatio`.
            #[serde(rename = "receiverFeeRatio")]
            pub receiver_fee_ratio: rt::Numeric,
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            #[serde(rename = "lock")]
            pub lock: ::core::option::Option<crate::splice_amulet::Splice_Expiry::TimeLock>,
        }
        impl rt::ToValue for TransferOutput {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("receiver", rt::ToValue::to_value(&self.receiver)),
                    (
                        "receiverFeeRatio",
                        rt::ToValue::to_value(&self.receiver_fee_ratio)
                    ),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                    ("lock", rt::ToValue::to_value(&self.lock)),
                ])
            }
        }
        impl rt::FromValue for TransferOutput {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    receiver: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "receiver",
                    )?)
                    .map_err(|e| e.at("receiver"))?,
                    receiver_fee_ratio: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "receiverFeeRatio",
                    )?)
                    .map_err(|e| e.at("receiverFeeRatio"))?,
                    amount: rt::FromValue::from_value(rt::required_field(value, 2usize, "amount")?)
                        .map_err(|e| e.at("amount"))?,
                    lock: rt::optional_field(value, 3usize, "lock").map_err(|e| e.at("lock"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Transfer {
            #[serde(rename = "sender")]
            pub sender: rt::Party,
            #[serde(rename = "provider")]
            pub provider: rt::Party,
            #[serde(rename = "inputs")]
            pub inputs: ::std::vec::Vec<
                crate::splice_amulet::Splice_AmuletRules::TransferInput,
            >,
            #[serde(rename = "outputs")]
            pub outputs: ::std::vec::Vec<
                crate::splice_amulet::Splice_AmuletRules::TransferOutput,
            >,
            #[serde(rename = "beneficiaries")]
            pub beneficiaries: ::core::option::Option<
                ::std::vec::Vec<
                    ::canton_splice_api_featured_app_v1::splice_api_featured_app_v1::Splice_Api_FeaturedAppRightV1::AppRewardBeneficiary,
                >,
            >,
        }
        impl rt::ToValue for Transfer {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("sender", rt::ToValue::to_value(&self.sender)),
                    ("provider", rt::ToValue::to_value(&self.provider)),
                    ("inputs", rt::ToValue::to_value(&self.inputs)),
                    ("outputs", rt::ToValue::to_value(&self.outputs)),
                    ("beneficiaries", rt::ToValue::to_value(&self.beneficiaries)),
                ])
            }
        }
        impl rt::FromValue for Transfer {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    sender: rt::FromValue::from_value(rt::required_field(value, 0usize, "sender")?)
                        .map_err(|e| e.at("sender"))?,
                    provider: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "provider",
                    )?)
                    .map_err(|e| e.at("provider"))?,
                    inputs: rt::FromValue::from_value(rt::required_field(value, 2usize, "inputs")?)
                        .map_err(|e| e.at("inputs"))?,
                    outputs: rt::FromValue::from_value(rt::required_field(
                        value, 3usize, "outputs",
                    )?)
                    .map_err(|e| e.at("outputs"))?,
                    beneficiaries: rt::optional_field(value, 4usize, "beneficiaries")
                        .map_err(|e| e.at("beneficiaries"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum TransferInput {
            #[serde(rename = "InputAppRewardCoupon")]
            InputAppRewardCoupon(
                rt::ContractId<crate::splice_amulet::Splice_Amulet::AppRewardCoupon>,
            ),
            #[serde(rename = "InputValidatorRewardCoupon")]
            InputValidatorRewardCoupon(
                rt::ContractId<crate::splice_amulet::Splice_Amulet::ValidatorRewardCoupon>,
            ),
            #[serde(rename = "InputSvRewardCoupon")]
            InputSvRewardCoupon(
                rt::ContractId<crate::splice_amulet::Splice_Amulet::SvRewardCoupon>,
            ),
            #[serde(rename = "InputAmulet")]
            InputAmulet(rt::ContractId<crate::splice_amulet::Splice_Amulet::Amulet>),
            #[serde(rename = "ExtTransferInput")]
            ExtTransferInput(
                crate::splice_amulet::Splice_AmuletRules::TransferInput_ExtTransferInput,
            ),
            #[serde(rename = "InputValidatorLivenessActivityRecord")]
            InputValidatorLivenessActivityRecord(
                rt::ContractId<
                    crate::splice_amulet::Splice_ValidatorLicense::ValidatorLivenessActivityRecord,
                >,
            ),
            #[serde(rename = "InputUnclaimedActivityRecord")]
            InputUnclaimedActivityRecord(
                rt::ContractId<crate::splice_amulet::Splice_Amulet::UnclaimedActivityRecord>,
            ),
        }
        impl rt::ToValue for TransferInput {
            fn to_value(&self) -> rt::Value {
                match self {
                    TransferInput::InputAppRewardCoupon(inner) => {
                        rt::variant_value("InputAppRewardCoupon", rt::ToValue::to_value(inner))
                    }
                    TransferInput::InputValidatorRewardCoupon(inner) => rt::variant_value(
                        "InputValidatorRewardCoupon",
                        rt::ToValue::to_value(inner),
                    ),
                    TransferInput::InputSvRewardCoupon(inner) => {
                        rt::variant_value("InputSvRewardCoupon", rt::ToValue::to_value(inner))
                    }
                    TransferInput::InputAmulet(inner) => {
                        rt::variant_value("InputAmulet", rt::ToValue::to_value(inner))
                    }
                    TransferInput::ExtTransferInput(inner) => {
                        rt::variant_value("ExtTransferInput", rt::ToValue::to_value(inner))
                    }
                    TransferInput::InputValidatorLivenessActivityRecord(inner) => {
                        rt::variant_value(
                            "InputValidatorLivenessActivityRecord",
                            rt::ToValue::to_value(inner),
                        )
                    }
                    TransferInput::InputUnclaimedActivityRecord(inner) => rt::variant_value(
                        "InputUnclaimedActivityRecord",
                        rt::ToValue::to_value(inner),
                    ),
                }
            }
        }
        impl rt::FromValue for TransferInput {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                let (constructor, payload) = rt::variant_parts(value)?;
                match constructor {
                    "InputAppRewardCoupon" => {
                        ::core::result::Result::Ok(TransferInput::InputAppRewardCoupon(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("InputAppRewardCoupon"))?,
                        ))
                    }
                    "InputValidatorRewardCoupon" => {
                        ::core::result::Result::Ok(TransferInput::InputValidatorRewardCoupon(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("InputValidatorRewardCoupon"))?,
                        ))
                    }
                    "InputSvRewardCoupon" => {
                        ::core::result::Result::Ok(TransferInput::InputSvRewardCoupon(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("InputSvRewardCoupon"))?,
                        ))
                    }
                    "InputAmulet" => ::core::result::Result::Ok(TransferInput::InputAmulet(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("InputAmulet"))?,
                    )),
                    "ExtTransferInput" => {
                        ::core::result::Result::Ok(TransferInput::ExtTransferInput(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("ExtTransferInput"))?,
                        ))
                    }
                    "InputValidatorLivenessActivityRecord" => ::core::result::Result::Ok(
                        TransferInput::InputValidatorLivenessActivityRecord(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("InputValidatorLivenessActivityRecord"))?,
                        ),
                    ),
                    "InputUnclaimedActivityRecord" => {
                        ::core::result::Result::Ok(TransferInput::InputUnclaimedActivityRecord(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("InputUnclaimedActivityRecord"))?,
                        ))
                    }
                    other => ::core::result::Result::Err(rt::unexpected_constructor(
                        "TransferInput",
                        other,
                    )),
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferInput_ExtTransferInput {
            ///Daml field `dummyUnitField`.
            #[serde(rename = "dummyUnitField")]
            pub dummy_unit_field: rt::Unit,
            ///Daml field `optInputValidatorFaucetCoupon`.
            #[serde(rename = "optInputValidatorFaucetCoupon")]
            pub opt_input_validator_faucet_coupon: ::core::option::Option<
                rt::ContractId<
                    crate::splice_amulet::Splice_ValidatorLicense::ValidatorFaucetCoupon,
                >,
            >,
        }
        impl rt::ToValue for TransferInput_ExtTransferInput {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "dummyUnitField",
                        rt::ToValue::to_value(&self.dummy_unit_field)
                    ),
                    (
                        "optInputValidatorFaucetCoupon",
                        rt::ToValue::to_value(&self.opt_input_validator_faucet_coupon)
                    ),
                ])
            }
        }
        impl rt::FromValue for TransferInput_ExtTransferInput {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dummy_unit_field: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "dummyUnitField",
                    )?)
                    .map_err(|e| e.at("dummyUnitField"))?,
                    opt_input_validator_faucet_coupon: rt::optional_field(
                        value,
                        1usize,
                        "optInputValidatorFaucetCoupon",
                    )
                    .map_err(|e| e.at("optInputValidatorFaucetCoupon"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum CreatedAmulet {
            #[serde(rename = "TransferResultAmulet")]
            TransferResultAmulet(rt::ContractId<crate::splice_amulet::Splice_Amulet::Amulet>),
            #[serde(rename = "TransferResultLockedAmulet")]
            TransferResultLockedAmulet(
                rt::ContractId<crate::splice_amulet::Splice_Amulet::LockedAmulet>,
            ),
            #[serde(rename = "ExtCreatedAmulet")]
            ExtCreatedAmulet(
                crate::splice_amulet::Splice_AmuletRules::CreatedAmulet_ExtCreatedAmulet,
            ),
        }
        impl rt::ToValue for CreatedAmulet {
            fn to_value(&self) -> rt::Value {
                match self {
                    CreatedAmulet::TransferResultAmulet(inner) => {
                        rt::variant_value("TransferResultAmulet", rt::ToValue::to_value(inner))
                    }
                    CreatedAmulet::TransferResultLockedAmulet(inner) => rt::variant_value(
                        "TransferResultLockedAmulet",
                        rt::ToValue::to_value(inner),
                    ),
                    CreatedAmulet::ExtCreatedAmulet(inner) => {
                        rt::variant_value("ExtCreatedAmulet", rt::ToValue::to_value(inner))
                    }
                }
            }
        }
        impl rt::FromValue for CreatedAmulet {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                let (constructor, payload) = rt::variant_parts(value)?;
                match constructor {
                    "TransferResultAmulet" => {
                        ::core::result::Result::Ok(CreatedAmulet::TransferResultAmulet(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("TransferResultAmulet"))?,
                        ))
                    }
                    "TransferResultLockedAmulet" => {
                        ::core::result::Result::Ok(CreatedAmulet::TransferResultLockedAmulet(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("TransferResultLockedAmulet"))?,
                        ))
                    }
                    "ExtCreatedAmulet" => {
                        ::core::result::Result::Ok(CreatedAmulet::ExtCreatedAmulet(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("ExtCreatedAmulet"))?,
                        ))
                    }
                    other => ::core::result::Result::Err(rt::unexpected_constructor(
                        "CreatedAmulet",
                        other,
                    )),
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct CreatedAmulet_ExtCreatedAmulet {
            ///Daml field `dummyUnitField`.
            #[serde(rename = "dummyUnitField")]
            pub dummy_unit_field: rt::Unit,
        }
        impl rt::ToValue for CreatedAmulet_ExtCreatedAmulet {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "dummyUnitField",
                    rt::ToValue::to_value(&self.dummy_unit_field)
                ),])
            }
        }
        impl rt::FromValue for CreatedAmulet_ExtCreatedAmulet {
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
        pub struct TransferContext {
            ///Daml field `openMiningRound`.
            #[serde(rename = "openMiningRound")]
            pub open_mining_round:
                rt::ContractId<crate::splice_amulet::Splice_Round::OpenMiningRound>,
            ///Daml field `issuingMiningRounds`.
            #[serde(rename = "issuingMiningRounds")]
            pub issuing_mining_rounds: rt::GenMap<
                crate::splice_amulet::Splice_Types::Round,
                rt::ContractId<crate::splice_amulet::Splice_Round::IssuingMiningRound>,
            >,
            ///Daml field `validatorRights`.
            #[serde(rename = "validatorRights")]
            pub validator_rights: rt::GenMap<
                rt::Party,
                rt::ContractId<crate::splice_amulet::Splice_Amulet::ValidatorRight>,
            >,
            ///Daml field `featuredAppRight`.
            #[serde(rename = "featuredAppRight")]
            pub featured_app_right: ::core::option::Option<
                rt::ContractId<crate::splice_amulet::Splice_Amulet::FeaturedAppRight>,
            >,
        }
        impl rt::ToValue for TransferContext {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "openMiningRound",
                        rt::ToValue::to_value(&self.open_mining_round)
                    ),
                    (
                        "issuingMiningRounds",
                        rt::ToValue::to_value(&self.issuing_mining_rounds)
                    ),
                    (
                        "validatorRights",
                        rt::ToValue::to_value(&self.validator_rights)
                    ),
                    (
                        "featuredAppRight",
                        rt::ToValue::to_value(&self.featured_app_right)
                    ),
                ])
            }
        }
        impl rt::FromValue for TransferContext {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    open_mining_round: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "openMiningRound",
                    )?)
                    .map_err(|e| e.at("openMiningRound"))?,
                    issuing_mining_rounds: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "issuingMiningRounds",
                    )?)
                    .map_err(|e| e.at("issuingMiningRounds"))?,
                    validator_rights: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "validatorRights",
                    )?)
                    .map_err(|e| e.at("validatorRights"))?,
                    featured_app_right: rt::optional_field(value, 3usize, "featuredAppRight")
                        .map_err(|e| e.at("featuredAppRight"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct PaymentTransferContext {
            ///Daml field `amuletRules`.
            #[serde(rename = "amuletRules")]
            pub amulet_rules: rt::ContractId<crate::splice_amulet::Splice_AmuletRules::AmuletRules>,
            #[serde(rename = "context")]
            pub context: crate::splice_amulet::Splice_AmuletRules::TransferContext,
        }
        impl rt::ToValue for PaymentTransferContext {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("amuletRules", rt::ToValue::to_value(&self.amulet_rules)),
                    ("context", rt::ToValue::to_value(&self.context)),
                ])
            }
        }
        impl rt::FromValue for PaymentTransferContext {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    amulet_rules: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "amuletRules",
                    )?)
                    .map_err(|e| e.at("amuletRules"))?,
                    context: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "context",
                    )?)
                    .map_err(|e| e.at("context"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AppTransferContext {
            ///Daml field `amuletRules`.
            #[serde(rename = "amuletRules")]
            pub amulet_rules: rt::ContractId<crate::splice_amulet::Splice_AmuletRules::AmuletRules>,
            ///Daml field `openMiningRound`.
            #[serde(rename = "openMiningRound")]
            pub open_mining_round:
                rt::ContractId<crate::splice_amulet::Splice_Round::OpenMiningRound>,
            ///Daml field `featuredAppRight`.
            #[serde(rename = "featuredAppRight")]
            pub featured_app_right: ::core::option::Option<
                rt::ContractId<crate::splice_amulet::Splice_Amulet::FeaturedAppRight>,
            >,
        }
        impl rt::ToValue for AppTransferContext {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("amuletRules", rt::ToValue::to_value(&self.amulet_rules)),
                    (
                        "openMiningRound",
                        rt::ToValue::to_value(&self.open_mining_round)
                    ),
                    (
                        "featuredAppRight",
                        rt::ToValue::to_value(&self.featured_app_right)
                    ),
                ])
            }
        }
        impl rt::FromValue for AppTransferContext {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    amulet_rules: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "amuletRules",
                    )?)
                    .map_err(|e| e.at("amuletRules"))?,
                    open_mining_round: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "openMiningRound",
                    )?)
                    .map_err(|e| e.at("openMiningRound"))?,
                    featured_app_right: rt::optional_field(value, 2usize, "featuredAppRight")
                        .map_err(|e| e.at("featuredAppRight"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct PreprocessedTransferOutput {
            #[serde(rename = "owner")]
            pub owner: rt::Party,
            ///Daml field `outputFee`.
            #[serde(rename = "outputFee")]
            pub output_fee: rt::Numeric,
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            #[serde(rename = "lock")]
            pub lock: ::core::option::Option<crate::splice_amulet::Splice_Expiry::TimeLock>,
        }
        impl rt::ToValue for PreprocessedTransferOutput {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("owner", rt::ToValue::to_value(&self.owner)),
                    ("outputFee", rt::ToValue::to_value(&self.output_fee)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                    ("lock", rt::ToValue::to_value(&self.lock)),
                ])
            }
        }
        impl rt::FromValue for PreprocessedTransferOutput {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    owner: rt::FromValue::from_value(rt::required_field(value, 0usize, "owner")?)
                        .map_err(|e| e.at("owner"))?,
                    output_fee: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "outputFee",
                    )?)
                    .map_err(|e| e.at("outputFee"))?,
                    amount: rt::FromValue::from_value(rt::required_field(value, 2usize, "amount")?)
                        .map_err(|e| e.at("amount"))?,
                    lock: rt::optional_field(value, 3usize, "lock").map_err(|e| e.at("lock"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferInputsSummary {
            ///Daml field `totalAmuletAmount`.
            #[serde(rename = "totalAmuletAmount")]
            pub total_amulet_amount: rt::Numeric,
            ///Daml field `totalAppRewardAmount`.
            #[serde(rename = "totalAppRewardAmount")]
            pub total_app_reward_amount: rt::Numeric,
            ///Daml field `totalValidatorRewardAmount`.
            #[serde(rename = "totalValidatorRewardAmount")]
            pub total_validator_reward_amount: rt::Numeric,
            ///Daml field `totalValidatorFaucetAmount`.
            #[serde(rename = "totalValidatorFaucetAmount")]
            pub total_validator_faucet_amount: rt::Numeric,
            ///Daml field `totalSvRewardAmount`.
            #[serde(rename = "totalSvRewardAmount")]
            pub total_sv_reward_amount: rt::Numeric,
            ///Daml field `totalHoldingFees`.
            #[serde(rename = "totalHoldingFees")]
            pub total_holding_fees: rt::Numeric,
            ///Daml field `amountArchivedAsOfRoundZero`.
            #[serde(rename = "amountArchivedAsOfRoundZero")]
            pub amount_archived_as_of_round_zero: rt::Numeric,
            ///Daml field `changeToHoldingFeesRate`.
            #[serde(rename = "changeToHoldingFeesRate")]
            pub change_to_holding_fees_rate: rt::Numeric,
            ///Daml field `totalUnclaimedActivityRecordAmount`.
            #[serde(rename = "totalUnclaimedActivityRecordAmount")]
            pub total_unclaimed_activity_record_amount: ::core::option::Option<rt::Numeric>,
        }
        impl rt::ToValue for TransferInputsSummary {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "totalAmuletAmount",
                        rt::ToValue::to_value(&self.total_amulet_amount)
                    ),
                    (
                        "totalAppRewardAmount",
                        rt::ToValue::to_value(&self.total_app_reward_amount)
                    ),
                    (
                        "totalValidatorRewardAmount",
                        rt::ToValue::to_value(&self.total_validator_reward_amount)
                    ),
                    (
                        "totalValidatorFaucetAmount",
                        rt::ToValue::to_value(&self.total_validator_faucet_amount)
                    ),
                    (
                        "totalSvRewardAmount",
                        rt::ToValue::to_value(&self.total_sv_reward_amount)
                    ),
                    (
                        "totalHoldingFees",
                        rt::ToValue::to_value(&self.total_holding_fees)
                    ),
                    (
                        "amountArchivedAsOfRoundZero",
                        rt::ToValue::to_value(&self.amount_archived_as_of_round_zero)
                    ),
                    (
                        "changeToHoldingFeesRate",
                        rt::ToValue::to_value(&self.change_to_holding_fees_rate)
                    ),
                    (
                        "totalUnclaimedActivityRecordAmount",
                        rt::ToValue::to_value(&self.total_unclaimed_activity_record_amount)
                    ),
                ])
            }
        }
        impl rt::FromValue for TransferInputsSummary {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    total_amulet_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "totalAmuletAmount",
                    )?)
                    .map_err(|e| e.at("totalAmuletAmount"))?,
                    total_app_reward_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "totalAppRewardAmount",
                    )?)
                    .map_err(|e| e.at("totalAppRewardAmount"))?,
                    total_validator_reward_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "totalValidatorRewardAmount",
                    )?)
                    .map_err(|e| e.at("totalValidatorRewardAmount"))?,
                    total_validator_faucet_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "totalValidatorFaucetAmount",
                    )?)
                    .map_err(|e| e.at("totalValidatorFaucetAmount"))?,
                    total_sv_reward_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "totalSvRewardAmount",
                    )?)
                    .map_err(|e| e.at("totalSvRewardAmount"))?,
                    total_holding_fees: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "totalHoldingFees",
                    )?)
                    .map_err(|e| e.at("totalHoldingFees"))?,
                    amount_archived_as_of_round_zero: rt::FromValue::from_value(
                        rt::required_field(value, 6usize, "amountArchivedAsOfRoundZero")?,
                    )
                    .map_err(|e| e.at("amountArchivedAsOfRoundZero"))?,
                    change_to_holding_fees_rate: rt::FromValue::from_value(rt::required_field(
                        value,
                        7usize,
                        "changeToHoldingFeesRate",
                    )?)
                    .map_err(|e| e.at("changeToHoldingFeesRate"))?,
                    total_unclaimed_activity_record_amount: rt::optional_field(
                        value,
                        8usize,
                        "totalUnclaimedActivityRecordAmount",
                    )
                    .map_err(|e| e.at("totalUnclaimedActivityRecordAmount"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferContextSummary {
            ///Daml field `featuredAppProvider`.
            #[serde(rename = "featuredAppProvider")]
            pub featured_app_provider: ::core::option::Option<rt::Party>,
            #[serde(rename = "config")]
            pub config: crate::splice_amulet::Splice_AmuletConfig::TransferConfig<
                crate::splice_amulet::Splice_Amulet::Amulet,
            >,
            ///Daml field `openRound`.
            #[serde(rename = "openRound")]
            pub open_round: crate::splice_amulet::Splice_Round::OpenMiningRound,
            ///Daml field `issuingMiningRounds`.
            #[serde(rename = "issuingMiningRounds")]
            pub issuing_mining_rounds: rt::GenMap<
                crate::splice_amulet::Splice_Types::Round,
                crate::splice_amulet::Splice_Round::IssuingMiningRound,
            >,
            ///Daml field `validatorRights`.
            #[serde(rename = "validatorRights")]
            pub validator_rights: rt::GenMap<
                rt::Party,
                rt::ContractId<crate::splice_amulet::Splice_Amulet::ValidatorRight>,
            >,
        }
        impl rt::ToValue for TransferContextSummary {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "featuredAppProvider",
                        rt::ToValue::to_value(&self.featured_app_provider)
                    ),
                    ("config", rt::ToValue::to_value(&self.config)),
                    ("openRound", rt::ToValue::to_value(&self.open_round)),
                    (
                        "issuingMiningRounds",
                        rt::ToValue::to_value(&self.issuing_mining_rounds)
                    ),
                    (
                        "validatorRights",
                        rt::ToValue::to_value(&self.validator_rights)
                    ),
                ])
            }
        }
        impl rt::FromValue for TransferContextSummary {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    featured_app_provider: rt::optional_field(value, 0usize, "featuredAppProvider")
                        .map_err(|e| e.at("featuredAppProvider"))?,
                    config: rt::FromValue::from_value(rt::required_field(value, 1usize, "config")?)
                        .map_err(|e| e.at("config"))?,
                    open_round: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "openRound",
                    )?)
                    .map_err(|e| e.at("openRound"))?,
                    issuing_mining_rounds: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "issuingMiningRounds",
                    )?)
                    .map_err(|e| e.at("issuingMiningRounds"))?,
                    validator_rights: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "validatorRights",
                    )?)
                    .map_err(|e| e.at("validatorRights"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct RewardsIssuanceConfig {
            ///Daml field `issueAppRewards`.
            #[serde(rename = "issueAppRewards")]
            pub issue_app_rewards: bool,
            ///Daml field `issueValidatorRewards`.
            #[serde(rename = "issueValidatorRewards")]
            pub issue_validator_rewards: bool,
        }
        impl rt::ToValue for RewardsIssuanceConfig {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "issueAppRewards",
                        rt::ToValue::to_value(&self.issue_app_rewards)
                    ),
                    (
                        "issueValidatorRewards",
                        rt::ToValue::to_value(&self.issue_validator_rewards)
                    ),
                ])
            }
        }
        impl rt::FromValue for RewardsIssuanceConfig {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    issue_app_rewards: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "issueAppRewards",
                    )?)
                    .map_err(|e| e.at("issueAppRewards"))?,
                    issue_validator_rewards: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "issueValidatorRewards",
                    )?)
                    .map_err(|e| e.at("issueValidatorRewards"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct InvalidTransfer {
            #[serde(rename = "reason")]
            pub reason: crate::splice_amulet::Splice_AmuletRules::InvalidTransferReason,
        }
        impl rt::ToValue for InvalidTransfer {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("reason", rt::ToValue::to_value(&self.reason)),])
            }
        }
        impl rt::FromValue for InvalidTransfer {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    reason: rt::FromValue::from_value(rt::required_field(value, 0usize, "reason")?)
                        .map_err(|e| e.at("reason"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum InvalidTransferReason {
            #[serde(rename = "ITR_InsufficientFunds")]
            ITR_InsufficientFunds(
                crate::splice_amulet::Splice_AmuletRules::InvalidTransferReason_ITR_InsufficientFunds,
            ),
            #[serde(rename = "ITR_UnknownSynchronizer")]
            ITR_UnknownSynchronizer(
                crate::splice_amulet::Splice_AmuletRules::InvalidTransferReason_ITR_UnknownSynchronizer,
            ),
            #[serde(rename = "ITR_InsufficientTopupAmount")]
            ITR_InsufficientTopupAmount(
                crate::splice_amulet::Splice_AmuletRules::InvalidTransferReason_ITR_InsufficientTopupAmount,
            ),
            #[serde(rename = "ITR_Other")]
            ITR_Other(
                crate::splice_amulet::Splice_AmuletRules::InvalidTransferReason_ITR_Other,
            ),
            #[serde(rename = "ExtInvalidTransferReason")]
            ExtInvalidTransferReason(
                crate::splice_amulet::Splice_AmuletRules::InvalidTransferReason_ExtInvalidTransferReason,
            ),
        }
        impl rt::ToValue for InvalidTransferReason {
            fn to_value(&self) -> rt::Value {
                match self {
                    InvalidTransferReason::ITR_InsufficientFunds(inner) => {
                        rt::variant_value("ITR_InsufficientFunds", rt::ToValue::to_value(inner))
                    }
                    InvalidTransferReason::ITR_UnknownSynchronizer(inner) => {
                        rt::variant_value("ITR_UnknownSynchronizer", rt::ToValue::to_value(inner))
                    }
                    InvalidTransferReason::ITR_InsufficientTopupAmount(inner) => rt::variant_value(
                        "ITR_InsufficientTopupAmount",
                        rt::ToValue::to_value(inner),
                    ),
                    InvalidTransferReason::ITR_Other(inner) => {
                        rt::variant_value("ITR_Other", rt::ToValue::to_value(inner))
                    }
                    InvalidTransferReason::ExtInvalidTransferReason(inner) => {
                        rt::variant_value("ExtInvalidTransferReason", rt::ToValue::to_value(inner))
                    }
                }
            }
        }
        impl rt::FromValue for InvalidTransferReason {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                let (constructor, payload) = rt::variant_parts(value)?;
                match constructor {
                    "ITR_InsufficientFunds" => {
                        ::core::result::Result::Ok(InvalidTransferReason::ITR_InsufficientFunds(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("ITR_InsufficientFunds"))?,
                        ))
                    }
                    "ITR_UnknownSynchronizer" => {
                        ::core::result::Result::Ok(InvalidTransferReason::ITR_UnknownSynchronizer(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("ITR_UnknownSynchronizer"))?,
                        ))
                    }
                    "ITR_InsufficientTopupAmount" => ::core::result::Result::Ok(
                        InvalidTransferReason::ITR_InsufficientTopupAmount(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("ITR_InsufficientTopupAmount"))?,
                        ),
                    ),
                    "ITR_Other" => ::core::result::Result::Ok(InvalidTransferReason::ITR_Other(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("ITR_Other"))?,
                    )),
                    "ExtInvalidTransferReason" => {
                        ::core::result::Result::Ok(InvalidTransferReason::ExtInvalidTransferReason(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("ExtInvalidTransferReason"))?,
                        ))
                    }
                    other => ::core::result::Result::Err(rt::unexpected_constructor(
                        "InvalidTransferReason",
                        other,
                    )),
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct InvalidTransferReason_ITR_InsufficientFunds {
            ///Daml field `missingAmount`.
            #[serde(rename = "missingAmount")]
            pub missing_amount: rt::Numeric,
        }
        impl rt::ToValue for InvalidTransferReason_ITR_InsufficientFunds {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "missingAmount",
                    rt::ToValue::to_value(&self.missing_amount)
                ),])
            }
        }
        impl rt::FromValue for InvalidTransferReason_ITR_InsufficientFunds {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    missing_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "missingAmount",
                    )?)
                    .map_err(|e| e.at("missingAmount"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct InvalidTransferReason_ITR_UnknownSynchronizer {
            ///Daml field `synchronizerId`.
            #[serde(rename = "synchronizerId")]
            pub synchronizer_id: ::std::string::String,
        }
        impl rt::ToValue for InvalidTransferReason_ITR_UnknownSynchronizer {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "synchronizerId",
                    rt::ToValue::to_value(&self.synchronizer_id)
                ),])
            }
        }
        impl rt::FromValue for InvalidTransferReason_ITR_UnknownSynchronizer {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    synchronizer_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "synchronizerId",
                    )?)
                    .map_err(|e| e.at("synchronizerId"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct InvalidTransferReason_ITR_InsufficientTopupAmount {
            ///Daml field `requestedTopupAmount`.
            #[serde(rename = "requestedTopupAmount")]
            pub requested_topup_amount: rt::Int64,
            ///Daml field `minTopupAmount`.
            #[serde(rename = "minTopupAmount")]
            pub min_topup_amount: rt::Int64,
        }
        impl rt::ToValue for InvalidTransferReason_ITR_InsufficientTopupAmount {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "requestedTopupAmount",
                        rt::ToValue::to_value(&self.requested_topup_amount)
                    ),
                    (
                        "minTopupAmount",
                        rt::ToValue::to_value(&self.min_topup_amount)
                    ),
                ])
            }
        }
        impl rt::FromValue for InvalidTransferReason_ITR_InsufficientTopupAmount {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    requested_topup_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "requestedTopupAmount",
                    )?)
                    .map_err(|e| e.at("requestedTopupAmount"))?,
                    min_topup_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "minTopupAmount",
                    )?)
                    .map_err(|e| e.at("minTopupAmount"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct InvalidTransferReason_ITR_Other {
            #[serde(rename = "description")]
            pub description: ::std::string::String,
        }
        impl rt::ToValue for InvalidTransferReason_ITR_Other {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "description",
                    rt::ToValue::to_value(&self.description)
                ),])
            }
        }
        impl rt::FromValue for InvalidTransferReason_ITR_Other {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    description: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "description",
                    )?)
                    .map_err(|e| e.at("description"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct InvalidTransferReason_ExtInvalidTransferReason {
            ///Daml field `dummyUnitField`.
            #[serde(rename = "dummyUnitField")]
            pub dummy_unit_field: rt::Unit,
        }
        impl rt::ToValue for InvalidTransferReason_ExtInvalidTransferReason {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "dummyUnitField",
                    rt::ToValue::to_value(&self.dummy_unit_field)
                ),])
            }
        }
        impl rt::FromValue for InvalidTransferReason_ExtInvalidTransferReason {
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
        pub struct AmuletRules_ConvertFeaturedAppActivityMarkers {
            ///Daml field `markerCids`.
            #[serde(rename = "markerCids")]
            pub marker_cids: ::std::vec::Vec<
                rt::ContractId<crate::splice_amulet::Splice_Amulet::FeaturedAppActivityMarker>,
            >,
            ///Daml field `openMiningRoundCid`.
            #[serde(rename = "openMiningRoundCid")]
            pub open_mining_round_cid:
                rt::ContractId<crate::splice_amulet::Splice_Round::OpenMiningRound>,
        }
        impl rt::ToValue for AmuletRules_ConvertFeaturedAppActivityMarkers {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("markerCids", rt::ToValue::to_value(&self.marker_cids)),
                    (
                        "openMiningRoundCid",
                        rt::ToValue::to_value(&self.open_mining_round_cid)
                    ),
                ])
            }
        }
        impl rt::FromValue for AmuletRules_ConvertFeaturedAppActivityMarkers {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    marker_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "markerCids",
                    )?)
                    .map_err(|e| e.at("markerCids"))?,
                    open_mining_round_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "openMiningRoundCid",
                    )?)
                    .map_err(|e| e.at("openMiningRoundCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_UpdateFutureAmuletConfigSchedule {
            ///Daml field `scheduleItem`.
            #[serde(rename = "scheduleItem")]
            pub schedule_item: ::canton_daml_stdlib::daml_prim_DA_Types::DA_Types::Tuple2<
                rt::Timestamp,
                crate::splice_amulet::Splice_AmuletConfig::AmuletConfig<
                    crate::splice_amulet::Splice_AmuletConfig::USD,
                >,
            >,
        }
        impl rt::ToValue for AmuletRules_UpdateFutureAmuletConfigSchedule {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "scheduleItem",
                    rt::ToValue::to_value(&self.schedule_item)
                ),])
            }
        }
        impl rt::FromValue for AmuletRules_UpdateFutureAmuletConfigSchedule {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    schedule_item: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "scheduleItem",
                    )?)
                    .map_err(|e| e.at("scheduleItem"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_RemoveFutureAmuletConfigSchedule {
            ///Daml field `scheduleTime`.
            #[serde(rename = "scheduleTime")]
            pub schedule_time: rt::Timestamp,
        }
        impl rt::ToValue for AmuletRules_RemoveFutureAmuletConfigSchedule {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "scheduleTime",
                    rt::ToValue::to_value(&self.schedule_time)
                ),])
            }
        }
        impl rt::FromValue for AmuletRules_RemoveFutureAmuletConfigSchedule {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    schedule_time: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "scheduleTime",
                    )?)
                    .map_err(|e| e.at("scheduleTime"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_AddFutureAmuletConfigSchedule {
            ///Daml field `newScheduleItem`.
            #[serde(rename = "newScheduleItem")]
            pub new_schedule_item: ::canton_daml_stdlib::daml_prim_DA_Types::DA_Types::Tuple2<
                rt::Timestamp,
                crate::splice_amulet::Splice_AmuletConfig::AmuletConfig<
                    crate::splice_amulet::Splice_AmuletConfig::USD,
                >,
            >,
        }
        impl rt::ToValue for AmuletRules_AddFutureAmuletConfigSchedule {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "newScheduleItem",
                    rt::ToValue::to_value(&self.new_schedule_item)
                ),])
            }
        }
        impl rt::FromValue for AmuletRules_AddFutureAmuletConfigSchedule {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    new_schedule_item: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "newScheduleItem",
                    )?)
                    .map_err(|e| e.at("newScheduleItem"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_SetConfig {
            ///Daml field `newConfig`.
            #[serde(rename = "newConfig")]
            pub new_config: crate::splice_amulet::Splice_AmuletConfig::AmuletConfig<
                crate::splice_amulet::Splice_AmuletConfig::USD,
            >,
            ///Daml field `baseConfig`.
            #[serde(rename = "baseConfig")]
            pub base_config: crate::splice_amulet::Splice_AmuletConfig::AmuletConfig<
                crate::splice_amulet::Splice_AmuletConfig::USD,
            >,
        }
        impl rt::ToValue for AmuletRules_SetConfig {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("newConfig", rt::ToValue::to_value(&self.new_config)),
                    ("baseConfig", rt::ToValue::to_value(&self.base_config)),
                ])
            }
        }
        impl rt::FromValue for AmuletRules_SetConfig {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    new_config: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "newConfig",
                    )?)
                    .map_err(|e| e.at("newConfig"))?,
                    base_config: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "baseConfig",
                    )?)
                    .map_err(|e| e.at("baseConfig"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_Fetch {
            #[serde(rename = "p")]
            pub p: rt::Party,
        }
        impl rt::ToValue for AmuletRules_Fetch {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("p", rt::ToValue::to_value(&self.p)),])
            }
        }
        impl rt::FromValue for AmuletRules_Fetch {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    p: rt::FromValue::from_value(rt::required_field(value, 0usize, "p")?)
                        .map_err(|e| e.at("p"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_MergeUnclaimedRewards {
            ///Daml field `unclaimedRewardCids`.
            #[serde(rename = "unclaimedRewardCids")]
            pub unclaimed_reward_cids: ::std::vec::Vec<
                rt::ContractId<crate::splice_amulet::Splice_Amulet::UnclaimedReward>,
            >,
        }
        impl rt::ToValue for AmuletRules_MergeUnclaimedRewards {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "unclaimedRewardCids",
                    rt::ToValue::to_value(&self.unclaimed_reward_cids)
                ),])
            }
        }
        impl rt::FromValue for AmuletRules_MergeUnclaimedRewards {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    unclaimed_reward_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "unclaimedRewardCids",
                    )?)
                    .map_err(|e| e.at("unclaimedRewardCids"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_ClaimExpiredRewards {
            ///Daml field `closedRoundCid`.
            #[serde(rename = "closedRoundCid")]
            pub closed_round_cid: rt::ContractId<
                crate::splice_amulet::Splice_Round::ClosedMiningRound,
            >,
            ///Daml field `validatorRewardCouponCids`.
            #[serde(rename = "validatorRewardCouponCids")]
            pub validator_reward_coupon_cids: ::std::vec::Vec<
                rt::ContractId<
                    crate::splice_amulet::Splice_Amulet::ValidatorRewardCoupon,
                >,
            >,
            ///Daml field `appCouponCids`.
            #[serde(rename = "appCouponCids")]
            pub app_coupon_cids: ::std::vec::Vec<
                rt::ContractId<crate::splice_amulet::Splice_Amulet::AppRewardCoupon>,
            >,
            ///Daml field `svRewardCouponCids`.
            #[serde(rename = "svRewardCouponCids")]
            pub sv_reward_coupon_cids: ::std::vec::Vec<
                rt::ContractId<crate::splice_amulet::Splice_Amulet::SvRewardCoupon>,
            >,
            ///Daml field `optValidatorFaucetCouponCids`.
            #[serde(rename = "optValidatorFaucetCouponCids")]
            pub opt_validator_faucet_coupon_cids: ::core::option::Option<
                ::std::vec::Vec<
                    rt::ContractId<
                        crate::splice_amulet::Splice_ValidatorLicense::ValidatorFaucetCoupon,
                    >,
                >,
            >,
            ///Daml field `optValidatorLivenessActivityRecordCids`.
            #[serde(rename = "optValidatorLivenessActivityRecordCids")]
            pub opt_validator_liveness_activity_record_cids: ::core::option::Option<
                ::std::vec::Vec<
                    rt::ContractId<
                        crate::splice_amulet::Splice_ValidatorLicense::ValidatorLivenessActivityRecord,
                    >,
                >,
            >,
        }
        impl rt::ToValue for AmuletRules_ClaimExpiredRewards {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "closedRoundCid",
                        rt::ToValue::to_value(&self.closed_round_cid)
                    ),
                    (
                        "validatorRewardCouponCids",
                        rt::ToValue::to_value(&self.validator_reward_coupon_cids)
                    ),
                    (
                        "appCouponCids",
                        rt::ToValue::to_value(&self.app_coupon_cids)
                    ),
                    (
                        "svRewardCouponCids",
                        rt::ToValue::to_value(&self.sv_reward_coupon_cids)
                    ),
                    (
                        "optValidatorFaucetCouponCids",
                        rt::ToValue::to_value(&self.opt_validator_faucet_coupon_cids)
                    ),
                    (
                        "optValidatorLivenessActivityRecordCids",
                        rt::ToValue::to_value(&self.opt_validator_liveness_activity_record_cids)
                    ),
                ])
            }
        }
        impl rt::FromValue for AmuletRules_ClaimExpiredRewards {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    closed_round_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "closedRoundCid",
                    )?)
                    .map_err(|e| e.at("closedRoundCid"))?,
                    validator_reward_coupon_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "validatorRewardCouponCids",
                    )?)
                    .map_err(|e| e.at("validatorRewardCouponCids"))?,
                    app_coupon_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "appCouponCids",
                    )?)
                    .map_err(|e| e.at("appCouponCids"))?,
                    sv_reward_coupon_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "svRewardCouponCids",
                    )?)
                    .map_err(|e| e.at("svRewardCouponCids"))?,
                    opt_validator_faucet_coupon_cids: rt::optional_field(
                        value,
                        4usize,
                        "optValidatorFaucetCouponCids",
                    )
                    .map_err(|e| e.at("optValidatorFaucetCouponCids"))?,
                    opt_validator_liveness_activity_record_cids: rt::optional_field(
                        value,
                        5usize,
                        "optValidatorLivenessActivityRecordCids",
                    )
                    .map_err(|e| e.at("optValidatorLivenessActivityRecordCids"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_MiningRound_Archive {
            ///Daml field `closedRoundCid`.
            #[serde(rename = "closedRoundCid")]
            pub closed_round_cid:
                rt::ContractId<crate::splice_amulet::Splice_Round::ClosedMiningRound>,
        }
        impl rt::ToValue for AmuletRules_MiningRound_Archive {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "closedRoundCid",
                    rt::ToValue::to_value(&self.closed_round_cid)
                ),])
            }
        }
        impl rt::FromValue for AmuletRules_MiningRound_Archive {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    closed_round_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "closedRoundCid",
                    )?)
                    .map_err(|e| e.at("closedRoundCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_MiningRound_Close {
            ///Daml field `issuingRoundCid`.
            #[serde(rename = "issuingRoundCid")]
            pub issuing_round_cid:
                rt::ContractId<crate::splice_amulet::Splice_Round::IssuingMiningRound>,
        }
        impl rt::ToValue for AmuletRules_MiningRound_Close {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "issuingRoundCid",
                    rt::ToValue::to_value(&self.issuing_round_cid)
                ),])
            }
        }
        impl rt::FromValue for AmuletRules_MiningRound_Close {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    issuing_round_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "issuingRoundCid",
                    )?)
                    .map_err(|e| e.at("issuingRoundCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_MiningRound_StartIssuing {
            ///Daml field `miningRoundCid`.
            #[serde(rename = "miningRoundCid")]
            pub mining_round_cid:
                rt::ContractId<crate::splice_amulet::Splice_Round::SummarizingMiningRound>,
            #[serde(rename = "summary")]
            pub summary: crate::splice_amulet::Splice_Issuance::OpenMiningRoundSummary,
        }
        impl rt::ToValue for AmuletRules_MiningRound_StartIssuing {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "miningRoundCid",
                        rt::ToValue::to_value(&self.mining_round_cid)
                    ),
                    ("summary", rt::ToValue::to_value(&self.summary)),
                ])
            }
        }
        impl rt::FromValue for AmuletRules_MiningRound_StartIssuing {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    mining_round_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "miningRoundCid",
                    )?)
                    .map_err(|e| e.at("miningRoundCid"))?,
                    summary: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "summary",
                    )?)
                    .map_err(|e| e.at("summary"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_AdvanceOpenMiningRounds {
            ///Daml field `amuletPrice`.
            #[serde(rename = "amuletPrice")]
            pub amulet_price: rt::Numeric,
            ///Daml field `roundToArchiveCid`.
            #[serde(rename = "roundToArchiveCid")]
            pub round_to_archive_cid:
                rt::ContractId<crate::splice_amulet::Splice_Round::OpenMiningRound>,
            ///Daml field `middleRoundCid`.
            #[serde(rename = "middleRoundCid")]
            pub middle_round_cid:
                rt::ContractId<crate::splice_amulet::Splice_Round::OpenMiningRound>,
            ///Daml field `latestRoundCid`.
            #[serde(rename = "latestRoundCid")]
            pub latest_round_cid:
                rt::ContractId<crate::splice_amulet::Splice_Round::OpenMiningRound>,
        }
        impl rt::ToValue for AmuletRules_AdvanceOpenMiningRounds {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("amuletPrice", rt::ToValue::to_value(&self.amulet_price)),
                    (
                        "roundToArchiveCid",
                        rt::ToValue::to_value(&self.round_to_archive_cid)
                    ),
                    (
                        "middleRoundCid",
                        rt::ToValue::to_value(&self.middle_round_cid)
                    ),
                    (
                        "latestRoundCid",
                        rt::ToValue::to_value(&self.latest_round_cid)
                    ),
                ])
            }
        }
        impl rt::FromValue for AmuletRules_AdvanceOpenMiningRounds {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    amulet_price: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "amuletPrice",
                    )?)
                    .map_err(|e| e.at("amuletPrice"))?,
                    round_to_archive_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "roundToArchiveCid",
                    )?)
                    .map_err(|e| e.at("roundToArchiveCid"))?,
                    middle_round_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "middleRoundCid",
                    )?)
                    .map_err(|e| e.at("middleRoundCid"))?,
                    latest_round_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "latestRoundCid",
                    )?)
                    .map_err(|e| e.at("latestRoundCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_Bootstrap_Rounds {
            ///Daml field `amuletPrice`.
            #[serde(rename = "amuletPrice")]
            pub amulet_price: rt::Numeric,
            ///Daml field `round0Duration`.
            #[serde(rename = "round0Duration")]
            pub round0_duration:
                ::canton_daml_stdlib::daml_stdlib_DA_Time_Types::DA_Time_Types::RelTime,
            ///Daml field `initialRound`.
            #[serde(rename = "initialRound")]
            pub initial_round: ::core::option::Option<rt::Int64>,
        }
        impl rt::ToValue for AmuletRules_Bootstrap_Rounds {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("amuletPrice", rt::ToValue::to_value(&self.amulet_price)),
                    (
                        "round0Duration",
                        rt::ToValue::to_value(&self.round0_duration)
                    ),
                    ("initialRound", rt::ToValue::to_value(&self.initial_round)),
                ])
            }
        }
        impl rt::FromValue for AmuletRules_Bootstrap_Rounds {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    amulet_price: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "amuletPrice",
                    )?)
                    .map_err(|e| e.at("amuletPrice"))?,
                    round0_duration: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "round0Duration",
                    )?)
                    .map_err(|e| e.at("round0Duration"))?,
                    initial_round: rt::optional_field(value, 2usize, "initialRound")
                        .map_err(|e| e.at("initialRound"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_DevNet_FeatureApp {
            #[serde(rename = "provider")]
            pub provider: rt::Party,
        }
        impl rt::ToValue for AmuletRules_DevNet_FeatureApp {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "provider",
                    rt::ToValue::to_value(&self.provider)
                ),])
            }
        }
        impl rt::FromValue for AmuletRules_DevNet_FeatureApp {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    provider: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "provider",
                    )?)
                    .map_err(|e| e.at("provider"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_DevNet_Tap {
            #[serde(rename = "receiver")]
            pub receiver: rt::Party,
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            ///Daml field `openRound`.
            #[serde(rename = "openRound")]
            pub open_round: rt::ContractId<crate::splice_amulet::Splice_Round::OpenMiningRound>,
        }
        impl rt::ToValue for AmuletRules_DevNet_Tap {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("receiver", rt::ToValue::to_value(&self.receiver)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                    ("openRound", rt::ToValue::to_value(&self.open_round)),
                ])
            }
        }
        impl rt::FromValue for AmuletRules_DevNet_Tap {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    receiver: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "receiver",
                    )?)
                    .map_err(|e| e.at("receiver"))?,
                    amount: rt::FromValue::from_value(rt::required_field(value, 1usize, "amount")?)
                        .map_err(|e| e.at("amount"))?,
                    open_round: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "openRound",
                    )?)
                    .map_err(|e| e.at("openRound"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_Mint {
            #[serde(rename = "receiver")]
            pub receiver: rt::Party,
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            ///Daml field `openRound`.
            #[serde(rename = "openRound")]
            pub open_round: rt::ContractId<crate::splice_amulet::Splice_Round::OpenMiningRound>,
        }
        impl rt::ToValue for AmuletRules_Mint {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("receiver", rt::ToValue::to_value(&self.receiver)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                    ("openRound", rt::ToValue::to_value(&self.open_round)),
                ])
            }
        }
        impl rt::FromValue for AmuletRules_Mint {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    receiver: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "receiver",
                    )?)
                    .map_err(|e| e.at("receiver"))?,
                    amount: rt::FromValue::from_value(rt::required_field(value, 1usize, "amount")?)
                        .map_err(|e| e.at("amount"))?,
                    open_round: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "openRound",
                    )?)
                    .map_err(|e| e.at("openRound"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_MergeMemberTrafficContracts {
            ///Daml field `trafficCids`.
            #[serde(rename = "trafficCids")]
            pub traffic_cids: ::std::vec::Vec<
                rt::ContractId<
                    crate::splice_amulet::Splice_DecentralizedSynchronizer::MemberTraffic,
                >,
            >,
        }
        impl rt::ToValue for AmuletRules_MergeMemberTrafficContracts {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "trafficCids",
                    rt::ToValue::to_value(&self.traffic_cids)
                ),])
            }
        }
        impl rt::FromValue for AmuletRules_MergeMemberTrafficContracts {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    traffic_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "trafficCids",
                    )?)
                    .map_err(|e| e.at("trafficCids"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_BuyMemberTraffic {
            #[serde(rename = "inputs")]
            pub inputs: ::std::vec::Vec<crate::splice_amulet::Splice_AmuletRules::TransferInput>,
            #[serde(rename = "context")]
            pub context: crate::splice_amulet::Splice_AmuletRules::TransferContext,
            #[serde(rename = "provider")]
            pub provider: rt::Party,
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
            ///Daml field `expectedDso`.
            #[serde(rename = "expectedDso")]
            pub expected_dso: ::core::option::Option<rt::Party>,
        }
        impl rt::ToValue for AmuletRules_BuyMemberTraffic {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("inputs", rt::ToValue::to_value(&self.inputs)),
                    ("context", rt::ToValue::to_value(&self.context)),
                    ("provider", rt::ToValue::to_value(&self.provider)),
                    ("memberId", rt::ToValue::to_value(&self.member_id)),
                    (
                        "synchronizerId",
                        rt::ToValue::to_value(&self.synchronizer_id)
                    ),
                    ("migrationId", rt::ToValue::to_value(&self.migration_id)),
                    ("trafficAmount", rt::ToValue::to_value(&self.traffic_amount)),
                    ("expectedDso", rt::ToValue::to_value(&self.expected_dso)),
                ])
            }
        }
        impl rt::FromValue for AmuletRules_BuyMemberTraffic {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    inputs: rt::FromValue::from_value(rt::required_field(value, 0usize, "inputs")?)
                        .map_err(|e| e.at("inputs"))?,
                    context: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "context",
                    )?)
                    .map_err(|e| e.at("context"))?,
                    provider: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "provider",
                    )?)
                    .map_err(|e| e.at("provider"))?,
                    member_id: rt::FromValue::from_value(rt::required_field(
                        value, 3usize, "memberId",
                    )?)
                    .map_err(|e| e.at("memberId"))?,
                    synchronizer_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "synchronizerId",
                    )?)
                    .map_err(|e| e.at("synchronizerId"))?,
                    migration_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "migrationId",
                    )?)
                    .map_err(|e| e.at("migrationId"))?,
                    traffic_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        6usize,
                        "trafficAmount",
                    )?)
                    .map_err(|e| e.at("trafficAmount"))?,
                    expected_dso: rt::optional_field(value, 7usize, "expectedDso")
                        .map_err(|e| e.at("expectedDso"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_CreateTransferPreapproval {
            #[serde(rename = "context")]
            pub context: crate::splice_amulet::Splice_AmuletRules::PaymentTransferContext,
            #[serde(rename = "inputs")]
            pub inputs: ::std::vec::Vec<crate::splice_amulet::Splice_AmuletRules::TransferInput>,
            #[serde(rename = "receiver")]
            pub receiver: rt::Party,
            #[serde(rename = "provider")]
            pub provider: rt::Party,
            ///Daml field `expiresAt`.
            #[serde(rename = "expiresAt")]
            pub expires_at: rt::Timestamp,
            ///Daml field `expectedDso`.
            #[serde(rename = "expectedDso")]
            pub expected_dso: ::core::option::Option<rt::Party>,
        }
        impl rt::ToValue for AmuletRules_CreateTransferPreapproval {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("context", rt::ToValue::to_value(&self.context)),
                    ("inputs", rt::ToValue::to_value(&self.inputs)),
                    ("receiver", rt::ToValue::to_value(&self.receiver)),
                    ("provider", rt::ToValue::to_value(&self.provider)),
                    ("expiresAt", rt::ToValue::to_value(&self.expires_at)),
                    ("expectedDso", rt::ToValue::to_value(&self.expected_dso)),
                ])
            }
        }
        impl rt::FromValue for AmuletRules_CreateTransferPreapproval {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    context: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "context",
                    )?)
                    .map_err(|e| e.at("context"))?,
                    inputs: rt::FromValue::from_value(rt::required_field(value, 1usize, "inputs")?)
                        .map_err(|e| e.at("inputs"))?,
                    receiver: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "receiver",
                    )?)
                    .map_err(|e| e.at("receiver"))?,
                    provider: rt::FromValue::from_value(rt::required_field(
                        value, 3usize, "provider",
                    )?)
                    .map_err(|e| e.at("provider"))?,
                    expires_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "expiresAt",
                    )?)
                    .map_err(|e| e.at("expiresAt"))?,
                    expected_dso: rt::optional_field(value, 5usize, "expectedDso")
                        .map_err(|e| e.at("expectedDso"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_CreateExternalPartySetupProposal {
            #[serde(rename = "context")]
            pub context: crate::splice_amulet::Splice_AmuletRules::PaymentTransferContext,
            #[serde(rename = "inputs")]
            pub inputs: ::std::vec::Vec<crate::splice_amulet::Splice_AmuletRules::TransferInput>,
            #[serde(rename = "user")]
            pub user: rt::Party,
            #[serde(rename = "validator")]
            pub validator: rt::Party,
            ///Daml field `preapprovalExpiresAt`.
            #[serde(rename = "preapprovalExpiresAt")]
            pub preapproval_expires_at: rt::Timestamp,
            ///Daml field `expectedDso`.
            #[serde(rename = "expectedDso")]
            pub expected_dso: ::core::option::Option<rt::Party>,
        }
        impl rt::ToValue for AmuletRules_CreateExternalPartySetupProposal {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("context", rt::ToValue::to_value(&self.context)),
                    ("inputs", rt::ToValue::to_value(&self.inputs)),
                    ("user", rt::ToValue::to_value(&self.user)),
                    ("validator", rt::ToValue::to_value(&self.validator)),
                    (
                        "preapprovalExpiresAt",
                        rt::ToValue::to_value(&self.preapproval_expires_at)
                    ),
                    ("expectedDso", rt::ToValue::to_value(&self.expected_dso)),
                ])
            }
        }
        impl rt::FromValue for AmuletRules_CreateExternalPartySetupProposal {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    context: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "context",
                    )?)
                    .map_err(|e| e.at("context"))?,
                    inputs: rt::FromValue::from_value(rt::required_field(value, 1usize, "inputs")?)
                        .map_err(|e| e.at("inputs"))?,
                    user: rt::FromValue::from_value(rt::required_field(value, 2usize, "user")?)
                        .map_err(|e| e.at("user"))?,
                    validator: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "validator",
                    )?)
                    .map_err(|e| e.at("validator"))?,
                    preapproval_expires_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "preapprovalExpiresAt",
                    )?)
                    .map_err(|e| e.at("preapprovalExpiresAt"))?,
                    expected_dso: rt::optional_field(value, 5usize, "expectedDso")
                        .map_err(|e| e.at("expectedDso"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_Transfer {
            #[serde(rename = "transfer")]
            pub transfer: crate::splice_amulet::Splice_AmuletRules::Transfer,
            #[serde(rename = "context")]
            pub context: crate::splice_amulet::Splice_AmuletRules::TransferContext,
            ///Daml field `expectedDso`.
            #[serde(rename = "expectedDso")]
            pub expected_dso: ::core::option::Option<rt::Party>,
        }
        impl rt::ToValue for AmuletRules_Transfer {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("transfer", rt::ToValue::to_value(&self.transfer)),
                    ("context", rt::ToValue::to_value(&self.context)),
                    ("expectedDso", rt::ToValue::to_value(&self.expected_dso)),
                ])
            }
        }
        impl rt::FromValue for AmuletRules_Transfer {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    transfer: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "transfer",
                    )?)
                    .map_err(|e| e.at("transfer"))?,
                    context: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "context",
                    )?)
                    .map_err(|e| e.at("context"))?,
                    expected_dso: rt::optional_field(value, 2usize, "expectedDso")
                        .map_err(|e| e.at("expectedDso"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_ComputeFees {
            #[serde(rename = "context")]
            pub context: crate::splice_amulet::Splice_AmuletRules::TransferContext,
            #[serde(rename = "sender")]
            pub sender: rt::Party,
            #[serde(rename = "outputs")]
            pub outputs: ::std::vec::Vec<crate::splice_amulet::Splice_AmuletRules::TransferOutput>,
            ///Daml field `expectedDso`.
            #[serde(rename = "expectedDso")]
            pub expected_dso: ::core::option::Option<rt::Party>,
        }
        impl rt::ToValue for AmuletRules_ComputeFees {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("context", rt::ToValue::to_value(&self.context)),
                    ("sender", rt::ToValue::to_value(&self.sender)),
                    ("outputs", rt::ToValue::to_value(&self.outputs)),
                    ("expectedDso", rt::ToValue::to_value(&self.expected_dso)),
                ])
            }
        }
        impl rt::FromValue for AmuletRules_ComputeFees {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    context: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "context",
                    )?)
                    .map_err(|e| e.at("context"))?,
                    sender: rt::FromValue::from_value(rt::required_field(value, 1usize, "sender")?)
                        .map_err(|e| e.at("sender"))?,
                    outputs: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "outputs",
                    )?)
                    .map_err(|e| e.at("outputs"))?,
                    expected_dso: rt::optional_field(value, 3usize, "expectedDso")
                        .map_err(|e| e.at("expectedDso"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_ConvertFeaturedAppActivityMarkersResult {
            ///Daml field `appRewardCouponCids`.
            #[serde(rename = "appRewardCouponCids")]
            pub app_reward_coupon_cids: ::std::vec::Vec<
                rt::ContractId<crate::splice_amulet::Splice_Amulet::AppRewardCoupon>,
            >,
        }
        impl rt::ToValue for AmuletRules_ConvertFeaturedAppActivityMarkersResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "appRewardCouponCids",
                    rt::ToValue::to_value(&self.app_reward_coupon_cids)
                ),])
            }
        }
        impl rt::FromValue for AmuletRules_ConvertFeaturedAppActivityMarkersResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    app_reward_coupon_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "appRewardCouponCids",
                    )?)
                    .map_err(|e| e.at("appRewardCouponCids"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_UpdateFutureAmuletConfigScheduleResult {
            ///Daml field `newAmuletRules`.
            #[serde(rename = "newAmuletRules")]
            pub new_amulet_rules:
                rt::ContractId<crate::splice_amulet::Splice_AmuletRules::AmuletRules>,
        }
        impl rt::ToValue for AmuletRules_UpdateFutureAmuletConfigScheduleResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "newAmuletRules",
                    rt::ToValue::to_value(&self.new_amulet_rules)
                ),])
            }
        }
        impl rt::FromValue for AmuletRules_UpdateFutureAmuletConfigScheduleResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    new_amulet_rules: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "newAmuletRules",
                    )?)
                    .map_err(|e| e.at("newAmuletRules"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_RemoveFutureAmuletConfigScheduleResult {
            ///Daml field `newAmuletRules`.
            #[serde(rename = "newAmuletRules")]
            pub new_amulet_rules:
                rt::ContractId<crate::splice_amulet::Splice_AmuletRules::AmuletRules>,
        }
        impl rt::ToValue for AmuletRules_RemoveFutureAmuletConfigScheduleResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "newAmuletRules",
                    rt::ToValue::to_value(&self.new_amulet_rules)
                ),])
            }
        }
        impl rt::FromValue for AmuletRules_RemoveFutureAmuletConfigScheduleResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    new_amulet_rules: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "newAmuletRules",
                    )?)
                    .map_err(|e| e.at("newAmuletRules"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_AddFutureAmuletConfigScheduleResult {
            ///Daml field `newAmuletRules`.
            #[serde(rename = "newAmuletRules")]
            pub new_amulet_rules:
                rt::ContractId<crate::splice_amulet::Splice_AmuletRules::AmuletRules>,
        }
        impl rt::ToValue for AmuletRules_AddFutureAmuletConfigScheduleResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "newAmuletRules",
                    rt::ToValue::to_value(&self.new_amulet_rules)
                ),])
            }
        }
        impl rt::FromValue for AmuletRules_AddFutureAmuletConfigScheduleResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    new_amulet_rules: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "newAmuletRules",
                    )?)
                    .map_err(|e| e.at("newAmuletRules"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_SetConfigResult {
            ///Daml field `newAmuletRules`.
            #[serde(rename = "newAmuletRules")]
            pub new_amulet_rules:
                rt::ContractId<crate::splice_amulet::Splice_AmuletRules::AmuletRules>,
        }
        impl rt::ToValue for AmuletRules_SetConfigResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "newAmuletRules",
                    rt::ToValue::to_value(&self.new_amulet_rules)
                ),])
            }
        }
        impl rt::FromValue for AmuletRules_SetConfigResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    new_amulet_rules: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "newAmuletRules",
                    )?)
                    .map_err(|e| e.at("newAmuletRules"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_MergeUnclaimedRewardsResult {
            ///Daml field `unclaimedRewardCid`.
            #[serde(rename = "unclaimedRewardCid")]
            pub unclaimed_reward_cid:
                rt::ContractId<crate::splice_amulet::Splice_Amulet::UnclaimedReward>,
        }
        impl rt::ToValue for AmuletRules_MergeUnclaimedRewardsResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "unclaimedRewardCid",
                    rt::ToValue::to_value(&self.unclaimed_reward_cid)
                ),])
            }
        }
        impl rt::FromValue for AmuletRules_MergeUnclaimedRewardsResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    unclaimed_reward_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "unclaimedRewardCid",
                    )?)
                    .map_err(|e| e.at("unclaimedRewardCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_ClaimExpiredRewardsResult {
            ///Daml field `unclaimedRewardCid`.
            #[serde(rename = "unclaimedRewardCid")]
            pub unclaimed_reward_cid: ::core::option::Option<
                rt::ContractId<crate::splice_amulet::Splice_Amulet::UnclaimedReward>,
            >,
        }
        impl rt::ToValue for AmuletRules_ClaimExpiredRewardsResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "unclaimedRewardCid",
                    rt::ToValue::to_value(&self.unclaimed_reward_cid)
                ),])
            }
        }
        impl rt::FromValue for AmuletRules_ClaimExpiredRewardsResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    unclaimed_reward_cid: rt::optional_field(value, 0usize, "unclaimedRewardCid")
                        .map_err(|e| e.at("unclaimedRewardCid"))?,
                })
            }
        }
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize,
        )]
        #[serde(crate = "rt::serde")]
        pub enum AmuletRules_MiningRound_ArchiveResult {
            #[serde(rename = "AmuletRules_MiningRound_ArchiveResult")]
            AmuletRules_MiningRound_ArchiveResult,
        }
        impl rt::ToValue for AmuletRules_MiningRound_ArchiveResult {
            fn to_value(&self) -> rt::Value {
                rt::enum_value(
                    match self {
                        AmuletRules_MiningRound_ArchiveResult::AmuletRules_MiningRound_ArchiveResult => {
                            "AmuletRules_MiningRound_ArchiveResult"
                        }
                    },
                )
            }
        }
        impl rt::FromValue for AmuletRules_MiningRound_ArchiveResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                match rt::enum_constructor(value)? {
                    "AmuletRules_MiningRound_ArchiveResult" => {
                        ::core::result::Result::Ok(
                            AmuletRules_MiningRound_ArchiveResult::AmuletRules_MiningRound_ArchiveResult,
                        )
                    }
                    other => {
                        ::core::result::Result::Err(
                            rt::unexpected_constructor(
                                "AmuletRules_MiningRound_ArchiveResult",
                                other,
                            ),
                        )
                    }
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_MiningRound_CloseResult {
            ///Daml field `closedRoundCid`.
            #[serde(rename = "closedRoundCid")]
            pub closed_round_cid:
                rt::ContractId<crate::splice_amulet::Splice_Round::ClosedMiningRound>,
        }
        impl rt::ToValue for AmuletRules_MiningRound_CloseResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "closedRoundCid",
                    rt::ToValue::to_value(&self.closed_round_cid)
                ),])
            }
        }
        impl rt::FromValue for AmuletRules_MiningRound_CloseResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    closed_round_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "closedRoundCid",
                    )?)
                    .map_err(|e| e.at("closedRoundCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_MiningRound_StartIssuingResult {
            ///Daml field `issuingRoundCid`.
            #[serde(rename = "issuingRoundCid")]
            pub issuing_round_cid:
                rt::ContractId<crate::splice_amulet::Splice_Round::IssuingMiningRound>,
        }
        impl rt::ToValue for AmuletRules_MiningRound_StartIssuingResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "issuingRoundCid",
                    rt::ToValue::to_value(&self.issuing_round_cid)
                ),])
            }
        }
        impl rt::FromValue for AmuletRules_MiningRound_StartIssuingResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    issuing_round_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "issuingRoundCid",
                    )?)
                    .map_err(|e| e.at("issuingRoundCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_AdvanceOpenMiningRoundsResult {
            ///Daml field `summarizingRoundCid`.
            #[serde(rename = "summarizingRoundCid")]
            pub summarizing_round_cid:
                rt::ContractId<crate::splice_amulet::Splice_Round::SummarizingMiningRound>,
            ///Daml field `openRoundCid`.
            #[serde(rename = "openRoundCid")]
            pub open_round_cid: rt::ContractId<crate::splice_amulet::Splice_Round::OpenMiningRound>,
        }
        impl rt::ToValue for AmuletRules_AdvanceOpenMiningRoundsResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "summarizingRoundCid",
                        rt::ToValue::to_value(&self.summarizing_round_cid)
                    ),
                    ("openRoundCid", rt::ToValue::to_value(&self.open_round_cid)),
                ])
            }
        }
        impl rt::FromValue for AmuletRules_AdvanceOpenMiningRoundsResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    summarizing_round_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "summarizingRoundCid",
                    )?)
                    .map_err(|e| e.at("summarizingRoundCid"))?,
                    open_round_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "openRoundCid",
                    )?)
                    .map_err(|e| e.at("openRoundCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_Bootstrap_RoundsResult {
            ///Daml field `openMiningRoundCid`.
            #[serde(rename = "openMiningRoundCid")]
            pub open_mining_round_cid:
                rt::ContractId<crate::splice_amulet::Splice_Round::OpenMiningRound>,
            ///Daml field `initialRound`.
            #[serde(rename = "initialRound")]
            pub initial_round: ::core::option::Option<crate::splice_amulet::Splice_Types::Round>,
        }
        impl rt::ToValue for AmuletRules_Bootstrap_RoundsResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "openMiningRoundCid",
                        rt::ToValue::to_value(&self.open_mining_round_cid)
                    ),
                    ("initialRound", rt::ToValue::to_value(&self.initial_round)),
                ])
            }
        }
        impl rt::FromValue for AmuletRules_Bootstrap_RoundsResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    open_mining_round_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "openMiningRoundCid",
                    )?)
                    .map_err(|e| e.at("openMiningRoundCid"))?,
                    initial_round: rt::optional_field(value, 1usize, "initialRound")
                        .map_err(|e| e.at("initialRound"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_DevNet_FeatureAppResult {
            ///Daml field `featuredAppRightCid`.
            #[serde(rename = "featuredAppRightCid")]
            pub featured_app_right_cid:
                rt::ContractId<crate::splice_amulet::Splice_Amulet::FeaturedAppRight>,
        }
        impl rt::ToValue for AmuletRules_DevNet_FeatureAppResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "featuredAppRightCid",
                    rt::ToValue::to_value(&self.featured_app_right_cid)
                ),])
            }
        }
        impl rt::FromValue for AmuletRules_DevNet_FeatureAppResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    featured_app_right_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "featuredAppRightCid",
                    )?)
                    .map_err(|e| e.at("featuredAppRightCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_DevNet_TapResult {
            ///Daml field `amuletSum`.
            #[serde(rename = "amuletSum")]
            pub amulet_sum: crate::splice_amulet::Splice_Amulet::AmuletCreateSummary<
                rt::ContractId<crate::splice_amulet::Splice_Amulet::Amulet>,
            >,
            #[serde(rename = "meta")]
            pub meta: ::core::option::Option<
                ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
            >,
        }
        impl rt::ToValue for AmuletRules_DevNet_TapResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("amuletSum", rt::ToValue::to_value(&self.amulet_sum)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for AmuletRules_DevNet_TapResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    amulet_sum: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "amuletSum",
                    )?)
                    .map_err(|e| e.at("amuletSum"))?,
                    meta: rt::optional_field(value, 1usize, "meta").map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_MintResult {
            ///Daml field `amuletSum`.
            #[serde(rename = "amuletSum")]
            pub amulet_sum: crate::splice_amulet::Splice_Amulet::AmuletCreateSummary<
                rt::ContractId<crate::splice_amulet::Splice_Amulet::Amulet>,
            >,
        }
        impl rt::ToValue for AmuletRules_MintResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "amuletSum",
                    rt::ToValue::to_value(&self.amulet_sum)
                ),])
            }
        }
        impl rt::FromValue for AmuletRules_MintResult {
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
        pub struct AmuletRules_MergeMemberTrafficContractsResult {
            ///Daml field `mergedTrafficCid`.
            #[serde(rename = "mergedTrafficCid")]
            pub merged_traffic_cid: rt::ContractId<
                crate::splice_amulet::Splice_DecentralizedSynchronizer::MemberTraffic,
            >,
        }
        impl rt::ToValue for AmuletRules_MergeMemberTrafficContractsResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "mergedTrafficCid",
                    rt::ToValue::to_value(&self.merged_traffic_cid)
                ),])
            }
        }
        impl rt::FromValue for AmuletRules_MergeMemberTrafficContractsResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    merged_traffic_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "mergedTrafficCid",
                    )?)
                    .map_err(|e| e.at("mergedTrafficCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_ComputeFeesResult {
            #[serde(rename = "fees")]
            pub fees: ::std::vec::Vec<rt::Numeric>,
        }
        impl rt::ToValue for AmuletRules_ComputeFeesResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("fees", rt::ToValue::to_value(&self.fees)),])
            }
        }
        impl rt::FromValue for AmuletRules_ComputeFeesResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    fees: rt::FromValue::from_value(rt::required_field(value, 0usize, "fees")?)
                        .map_err(|e| e.at("fees"))?,
                })
            }
        }
        ///The Daml template `Splice.AmuletRules:AmuletRules`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-amulet:Splice.AmuletRules:AmuletRules`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `AmuletRules_ComputeFees` — non-consuming
        ///- `AmuletRules_Transfer` — non-consuming
        ///- `AmuletRules_CreateExternalPartySetupProposal` — non-consuming
        ///- `AmuletRules_CreateTransferPreapproval` — non-consuming
        ///- `AmuletRules_BuyMemberTraffic` — non-consuming
        ///- `AmuletRules_MergeMemberTrafficContracts` — non-consuming
        ///- `AmuletRules_Mint` — non-consuming
        ///- `AmuletRules_DevNet_Tap` — non-consuming
        ///- `AmuletRules_DevNet_FeatureApp` — non-consuming
        ///- `AmuletRules_Bootstrap_Rounds` — non-consuming
        ///- `AmuletRules_AdvanceOpenMiningRounds` — non-consuming
        ///- `AmuletRules_MiningRound_StartIssuing` — non-consuming
        ///- `AmuletRules_MiningRound_Close` — non-consuming
        ///- `AmuletRules_MiningRound_Archive` — non-consuming
        ///- `AmuletRules_ClaimExpiredRewards` — non-consuming
        ///- `AmuletRules_MergeUnclaimedRewards` — non-consuming
        ///- `AmuletRules_SetConfig` — consuming
        ///- `AmuletRules_ConvertFeaturedAppActivityMarkers` — non-consuming
        ///- `Archive` — consuming
        ///- `AmuletRules_Fetch` — non-consuming
        ///- `AmuletRules_AddFutureAmuletConfigSchedule` — consuming
        ///- `AmuletRules_RemoveFutureAmuletConfigSchedule` — consuming
        ///- `AmuletRules_UpdateFutureAmuletConfigSchedule` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules {
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///Daml field `configSchedule`.
            #[serde(rename = "configSchedule")]
            pub config_schedule: crate::splice_amulet::Splice_Schedule::Schedule<
                rt::Timestamp,
                crate::splice_amulet::Splice_AmuletConfig::AmuletConfig<
                    crate::splice_amulet::Splice_AmuletConfig::USD,
                >,
            >,
            ///Daml field `isDevNet`.
            #[serde(rename = "isDevNet")]
            pub is_dev_net: bool,
        }
        impl rt::ToValue for AmuletRules {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    (
                        "configSchedule",
                        rt::ToValue::to_value(&self.config_schedule)
                    ),
                    ("isDevNet", rt::ToValue::to_value(&self.is_dev_net)),
                ])
            }
        }
        impl rt::FromValue for AmuletRules {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                    config_schedule: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "configSchedule",
                    )?)
                    .map_err(|e| e.at("configSchedule"))?,
                    is_dev_net: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "isDevNet",
                    )?)
                    .map_err(|e| e.at("isDevNet"))?,
                })
            }
        }
        impl rt::Contract for AmuletRules {
            const PACKAGE_ID: &'static str =
                "3ca1343ab26b453d38c8adb70dca5f1ead8440c42b59b68f070786955cbf9ec1";
            const PACKAGE_NAME: &'static str = "splice-amulet";
            const MODULE_NAME: &'static str = "Splice.AmuletRules";
            const ENTITY_NAME: &'static str = "AmuletRules";
        }
        impl rt::Template for AmuletRules {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    (
                        "configSchedule",
                        rt::ToValue::to_value(&self.config_schedule)
                    ),
                    ("isDevNet", rt::ToValue::to_value(&self.is_dev_net)),
                ])
            }
        }
        ///The `AmuletRules_ComputeFees` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules> for crate::splice_amulet::Splice_AmuletRules::AmuletRules_ComputeFees {
            type Return = crate::splice_amulet::Splice_AmuletRules::AmuletRules_ComputeFeesResult;
            const NAME: &'static str = "AmuletRules_ComputeFees";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_Transfer` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules> for crate::splice_amulet::Splice_AmuletRules::AmuletRules_Transfer {
            type Return = crate::splice_amulet::Splice_AmuletRules::TransferResult;
            const NAME: &'static str = "AmuletRules_Transfer";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_CreateExternalPartySetupProposal` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules>
        for crate::splice_amulet::Splice_AmuletRules::AmuletRules_CreateExternalPartySetupProposal {
            type Return = crate::splice_amulet::Splice_AmuletRules::AmuletRules_CreateExternalPartySetupProposalResult;
            const NAME: &'static str = "AmuletRules_CreateExternalPartySetupProposal";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_CreateTransferPreapproval` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules>
            for crate::splice_amulet::Splice_AmuletRules::AmuletRules_CreateTransferPreapproval
        {
            type Return = crate::splice_amulet::Splice_AmuletRules::AmuletRules_CreateTransferPreapprovalResult;
            const NAME: &'static str = "AmuletRules_CreateTransferPreapproval";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_BuyMemberTraffic` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules>
            for crate::splice_amulet::Splice_AmuletRules::AmuletRules_BuyMemberTraffic
        {
            type Return =
                crate::splice_amulet::Splice_AmuletRules::AmuletRules_BuyMemberTrafficResult;
            const NAME: &'static str = "AmuletRules_BuyMemberTraffic";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_MergeMemberTrafficContracts` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules>
            for crate::splice_amulet::Splice_AmuletRules::AmuletRules_MergeMemberTrafficContracts
        {
            type Return = crate::splice_amulet::Splice_AmuletRules::AmuletRules_MergeMemberTrafficContractsResult;
            const NAME: &'static str = "AmuletRules_MergeMemberTrafficContracts";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_Mint` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules> for crate::splice_amulet::Splice_AmuletRules::AmuletRules_Mint {
            type Return = crate::splice_amulet::Splice_AmuletRules::AmuletRules_MintResult;
            const NAME: &'static str = "AmuletRules_Mint";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_DevNet_Tap` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules> for crate::splice_amulet::Splice_AmuletRules::AmuletRules_DevNet_Tap {
            type Return = crate::splice_amulet::Splice_AmuletRules::AmuletRules_DevNet_TapResult;
            const NAME: &'static str = "AmuletRules_DevNet_Tap";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_DevNet_FeatureApp` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules>
            for crate::splice_amulet::Splice_AmuletRules::AmuletRules_DevNet_FeatureApp
        {
            type Return =
                crate::splice_amulet::Splice_AmuletRules::AmuletRules_DevNet_FeatureAppResult;
            const NAME: &'static str = "AmuletRules_DevNet_FeatureApp";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_Bootstrap_Rounds` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules>
            for crate::splice_amulet::Splice_AmuletRules::AmuletRules_Bootstrap_Rounds
        {
            type Return =
                crate::splice_amulet::Splice_AmuletRules::AmuletRules_Bootstrap_RoundsResult;
            const NAME: &'static str = "AmuletRules_Bootstrap_Rounds";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_AdvanceOpenMiningRounds` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules>
            for crate::splice_amulet::Splice_AmuletRules::AmuletRules_AdvanceOpenMiningRounds
        {
            type Return =
                crate::splice_amulet::Splice_AmuletRules::AmuletRules_AdvanceOpenMiningRoundsResult;
            const NAME: &'static str = "AmuletRules_AdvanceOpenMiningRounds";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_MiningRound_StartIssuing` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules>
            for crate::splice_amulet::Splice_AmuletRules::AmuletRules_MiningRound_StartIssuing
        {
            type Return = crate::splice_amulet::Splice_AmuletRules::AmuletRules_MiningRound_StartIssuingResult;
            const NAME: &'static str = "AmuletRules_MiningRound_StartIssuing";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_MiningRound_Close` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules>
            for crate::splice_amulet::Splice_AmuletRules::AmuletRules_MiningRound_Close
        {
            type Return =
                crate::splice_amulet::Splice_AmuletRules::AmuletRules_MiningRound_CloseResult;
            const NAME: &'static str = "AmuletRules_MiningRound_Close";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_MiningRound_Archive` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules>
            for crate::splice_amulet::Splice_AmuletRules::AmuletRules_MiningRound_Archive
        {
            type Return =
                crate::splice_amulet::Splice_AmuletRules::AmuletRules_MiningRound_ArchiveResult;
            const NAME: &'static str = "AmuletRules_MiningRound_Archive";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_ClaimExpiredRewards` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules>
            for crate::splice_amulet::Splice_AmuletRules::AmuletRules_ClaimExpiredRewards
        {
            type Return =
                crate::splice_amulet::Splice_AmuletRules::AmuletRules_ClaimExpiredRewardsResult;
            const NAME: &'static str = "AmuletRules_ClaimExpiredRewards";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_MergeUnclaimedRewards` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules>
            for crate::splice_amulet::Splice_AmuletRules::AmuletRules_MergeUnclaimedRewards
        {
            type Return =
                crate::splice_amulet::Splice_AmuletRules::AmuletRules_MergeUnclaimedRewardsResult;
            const NAME: &'static str = "AmuletRules_MergeUnclaimedRewards";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_SetConfig` choice on [`AmuletRules`] (consuming).
        impl rt::Choice<AmuletRules> for crate::splice_amulet::Splice_AmuletRules::AmuletRules_SetConfig {
            type Return = crate::splice_amulet::Splice_AmuletRules::AmuletRules_SetConfigResult;
            const NAME: &'static str = "AmuletRules_SetConfig";
            const CONSUMING: bool = true;
        }
        ///The `AmuletRules_ConvertFeaturedAppActivityMarkers` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules>
        for crate::splice_amulet::Splice_AmuletRules::AmuletRules_ConvertFeaturedAppActivityMarkers {
            type Return = crate::splice_amulet::Splice_AmuletRules::AmuletRules_ConvertFeaturedAppActivityMarkersResult;
            const NAME: &'static str = "AmuletRules_ConvertFeaturedAppActivityMarkers";
            const CONSUMING: bool = false;
        }
        ///The `Archive` choice on [`AmuletRules`] (consuming).
        impl rt::Choice<AmuletRules>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `AmuletRules_Fetch` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules> for crate::splice_amulet::Splice_AmuletRules::AmuletRules_Fetch {
            type Return = crate::splice_amulet::Splice_AmuletRules::AmuletRules;
            const NAME: &'static str = "AmuletRules_Fetch";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_AddFutureAmuletConfigSchedule` choice on [`AmuletRules`] (consuming).
        impl rt::Choice<AmuletRules>
            for crate::splice_amulet::Splice_AmuletRules::AmuletRules_AddFutureAmuletConfigSchedule
        {
            type Return = crate::splice_amulet::Splice_AmuletRules::AmuletRules_AddFutureAmuletConfigScheduleResult;
            const NAME: &'static str = "AmuletRules_AddFutureAmuletConfigSchedule";
            const CONSUMING: bool = true;
        }
        ///The `AmuletRules_RemoveFutureAmuletConfigSchedule` choice on [`AmuletRules`] (consuming).
        impl rt::Choice<AmuletRules>
        for crate::splice_amulet::Splice_AmuletRules::AmuletRules_RemoveFutureAmuletConfigSchedule {
            type Return = crate::splice_amulet::Splice_AmuletRules::AmuletRules_RemoveFutureAmuletConfigScheduleResult;
            const NAME: &'static str = "AmuletRules_RemoveFutureAmuletConfigSchedule";
            const CONSUMING: bool = true;
        }
        ///The `AmuletRules_UpdateFutureAmuletConfigSchedule` choice on [`AmuletRules`] (consuming).
        impl rt::Choice<AmuletRules>
        for crate::splice_amulet::Splice_AmuletRules::AmuletRules_UpdateFutureAmuletConfigSchedule {
            type Return = crate::splice_amulet::Splice_AmuletRules::AmuletRules_UpdateFutureAmuletConfigScheduleResult;
            const NAME: &'static str = "AmuletRules_UpdateFutureAmuletConfigSchedule";
            const CONSUMING: bool = true;
        }
        ///The Daml template `Splice.AmuletRules:ExternalPartySetupProposal`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-amulet:Splice.AmuletRules:ExternalPartySetupProposal`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `ExternalPartySetupProposal_Accept` — consuming
        ///- `Archive` — consuming
        ///- `ExternalPartySetupProposal_Reject` — consuming
        ///- `ExternalPartySetupProposal_Withdraw` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ExternalPartySetupProposal {
            #[serde(rename = "validator")]
            pub validator: rt::Party,
            #[serde(rename = "user")]
            pub user: rt::Party,
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///Daml field `createdAt`.
            #[serde(rename = "createdAt")]
            pub created_at: rt::Timestamp,
            ///Daml field `preapprovalExpiresAt`.
            #[serde(rename = "preapprovalExpiresAt")]
            pub preapproval_expires_at: rt::Timestamp,
        }
        impl rt::ToValue for ExternalPartySetupProposal {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("validator", rt::ToValue::to_value(&self.validator)),
                    ("user", rt::ToValue::to_value(&self.user)),
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("createdAt", rt::ToValue::to_value(&self.created_at)),
                    (
                        "preapprovalExpiresAt",
                        rt::ToValue::to_value(&self.preapproval_expires_at)
                    ),
                ])
            }
        }
        impl rt::FromValue for ExternalPartySetupProposal {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    validator: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "validator",
                    )?)
                    .map_err(|e| e.at("validator"))?,
                    user: rt::FromValue::from_value(rt::required_field(value, 1usize, "user")?)
                        .map_err(|e| e.at("user"))?,
                    dso: rt::FromValue::from_value(rt::required_field(value, 2usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                    created_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "createdAt",
                    )?)
                    .map_err(|e| e.at("createdAt"))?,
                    preapproval_expires_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "preapprovalExpiresAt",
                    )?)
                    .map_err(|e| e.at("preapprovalExpiresAt"))?,
                })
            }
        }
        impl rt::Contract for ExternalPartySetupProposal {
            const PACKAGE_ID: &'static str =
                "3ca1343ab26b453d38c8adb70dca5f1ead8440c42b59b68f070786955cbf9ec1";
            const PACKAGE_NAME: &'static str = "splice-amulet";
            const MODULE_NAME: &'static str = "Splice.AmuletRules";
            const ENTITY_NAME: &'static str = "ExternalPartySetupProposal";
        }
        impl rt::Template for ExternalPartySetupProposal {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("validator", rt::ToValue::to_value(&self.validator)),
                    ("user", rt::ToValue::to_value(&self.user)),
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("createdAt", rt::ToValue::to_value(&self.created_at)),
                    (
                        "preapprovalExpiresAt",
                        rt::ToValue::to_value(&self.preapproval_expires_at)
                    ),
                ])
            }
        }
        ///The `ExternalPartySetupProposal_Accept` choice on [`ExternalPartySetupProposal`] (consuming).
        impl rt::Choice<ExternalPartySetupProposal>
            for crate::splice_amulet::Splice_AmuletRules::ExternalPartySetupProposal_Accept
        {
            type Return =
                crate::splice_amulet::Splice_AmuletRules::ExternalPartySetupProposal_AcceptResult;
            const NAME: &'static str = "ExternalPartySetupProposal_Accept";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`ExternalPartySetupProposal`] (consuming).
        impl rt::Choice<ExternalPartySetupProposal>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `ExternalPartySetupProposal_Reject` choice on [`ExternalPartySetupProposal`] (consuming).
        impl rt::Choice<ExternalPartySetupProposal>
            for crate::splice_amulet::Splice_AmuletRules::ExternalPartySetupProposal_Reject
        {
            type Return =
                crate::splice_amulet::Splice_AmuletRules::ExternalPartySetupProposal_RejectResult;
            const NAME: &'static str = "ExternalPartySetupProposal_Reject";
            const CONSUMING: bool = true;
        }
        ///The `ExternalPartySetupProposal_Withdraw` choice on [`ExternalPartySetupProposal`] (consuming).
        impl rt::Choice<ExternalPartySetupProposal>
            for crate::splice_amulet::Splice_AmuletRules::ExternalPartySetupProposal_Withdraw
        {
            type Return =
                crate::splice_amulet::Splice_AmuletRules::ExternalPartySetupProposal_WithdrawResult;
            const NAME: &'static str = "ExternalPartySetupProposal_Withdraw";
            const CONSUMING: bool = true;
        }
        ///The Daml template `Splice.AmuletRules:TransferPreapproval`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-amulet:Splice.AmuletRules:TransferPreapproval`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `TransferPreapproval_Renew` — consuming
        ///- `TransferPreapproval_Send` — non-consuming
        ///- `TransferPreapproval_Expire` — consuming
        ///- `TransferPreapproval_Cancel` — consuming
        ///- `Archive` — consuming
        ///- `TransferPreapproval_Fetch` — non-consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferPreapproval {
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            #[serde(rename = "receiver")]
            pub receiver: rt::Party,
            #[serde(rename = "provider")]
            pub provider: rt::Party,
            ///Daml field `validFrom`.
            #[serde(rename = "validFrom")]
            pub valid_from: rt::Timestamp,
            ///Daml field `lastRenewedAt`.
            #[serde(rename = "lastRenewedAt")]
            pub last_renewed_at: rt::Timestamp,
            ///Daml field `expiresAt`.
            #[serde(rename = "expiresAt")]
            pub expires_at: rt::Timestamp,
        }
        impl rt::ToValue for TransferPreapproval {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("receiver", rt::ToValue::to_value(&self.receiver)),
                    ("provider", rt::ToValue::to_value(&self.provider)),
                    ("validFrom", rt::ToValue::to_value(&self.valid_from)),
                    (
                        "lastRenewedAt",
                        rt::ToValue::to_value(&self.last_renewed_at)
                    ),
                    ("expiresAt", rt::ToValue::to_value(&self.expires_at)),
                ])
            }
        }
        impl rt::FromValue for TransferPreapproval {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                    receiver: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "receiver",
                    )?)
                    .map_err(|e| e.at("receiver"))?,
                    provider: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "provider",
                    )?)
                    .map_err(|e| e.at("provider"))?,
                    valid_from: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "validFrom",
                    )?)
                    .map_err(|e| e.at("validFrom"))?,
                    last_renewed_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "lastRenewedAt",
                    )?)
                    .map_err(|e| e.at("lastRenewedAt"))?,
                    expires_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "expiresAt",
                    )?)
                    .map_err(|e| e.at("expiresAt"))?,
                })
            }
        }
        impl rt::Contract for TransferPreapproval {
            const PACKAGE_ID: &'static str =
                "3ca1343ab26b453d38c8adb70dca5f1ead8440c42b59b68f070786955cbf9ec1";
            const PACKAGE_NAME: &'static str = "splice-amulet";
            const MODULE_NAME: &'static str = "Splice.AmuletRules";
            const ENTITY_NAME: &'static str = "TransferPreapproval";
        }
        impl rt::Template for TransferPreapproval {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("receiver", rt::ToValue::to_value(&self.receiver)),
                    ("provider", rt::ToValue::to_value(&self.provider)),
                    ("validFrom", rt::ToValue::to_value(&self.valid_from)),
                    (
                        "lastRenewedAt",
                        rt::ToValue::to_value(&self.last_renewed_at)
                    ),
                    ("expiresAt", rt::ToValue::to_value(&self.expires_at)),
                ])
            }
        }
        ///The `TransferPreapproval_Renew` choice on [`TransferPreapproval`] (consuming).
        impl rt::Choice<TransferPreapproval>
            for crate::splice_amulet::Splice_AmuletRules::TransferPreapproval_Renew
        {
            type Return = crate::splice_amulet::Splice_AmuletRules::TransferPreapproval_RenewResult;
            const NAME: &'static str = "TransferPreapproval_Renew";
            const CONSUMING: bool = true;
        }
        ///The `TransferPreapproval_Send` choice on [`TransferPreapproval`] (non-consuming).
        impl rt::Choice<TransferPreapproval>
            for crate::splice_amulet::Splice_AmuletRules::TransferPreapproval_Send
        {
            type Return = crate::splice_amulet::Splice_AmuletRules::TransferPreapproval_SendResult;
            const NAME: &'static str = "TransferPreapproval_Send";
            const CONSUMING: bool = false;
        }
        ///The `TransferPreapproval_Expire` choice on [`TransferPreapproval`] (consuming).
        impl rt::Choice<TransferPreapproval>
            for crate::splice_amulet::Splice_AmuletRules::TransferPreapproval_Expire
        {
            type Return =
                crate::splice_amulet::Splice_AmuletRules::TransferPreapproval_ExpireResult;
            const NAME: &'static str = "TransferPreapproval_Expire";
            const CONSUMING: bool = true;
        }
        ///The `TransferPreapproval_Cancel` choice on [`TransferPreapproval`] (consuming).
        impl rt::Choice<TransferPreapproval>
            for crate::splice_amulet::Splice_AmuletRules::TransferPreapproval_Cancel
        {
            type Return =
                crate::splice_amulet::Splice_AmuletRules::TransferPreapproval_CancelResult;
            const NAME: &'static str = "TransferPreapproval_Cancel";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`TransferPreapproval`] (consuming).
        impl rt::Choice<TransferPreapproval>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `TransferPreapproval_Fetch` choice on [`TransferPreapproval`] (non-consuming).
        impl rt::Choice<TransferPreapproval>
            for crate::splice_amulet::Splice_AmuletRules::TransferPreapproval_Fetch
        {
            type Return = crate::splice_amulet::Splice_AmuletRules::TransferPreapproval;
            const NAME: &'static str = "TransferPreapproval_Fetch";
            const CONSUMING: bool = false;
        }
    }
    pub mod Splice_Amulet {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct UnclaimedActivityRecord_DsoExpire {}
        impl rt::ToValue for UnclaimedActivityRecord_DsoExpire {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for UnclaimedActivityRecord_DsoExpire {
            fn from_value(_value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SvRewardCoupon_ArchiveAsBeneficiary {}
        impl rt::ToValue for SvRewardCoupon_ArchiveAsBeneficiary {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for SvRewardCoupon_ArchiveAsBeneficiary {
            fn from_value(_value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SvRewardCoupon_DsoExpire {
            ///Daml field `closedRoundCid`.
            #[serde(rename = "closedRoundCid")]
            pub closed_round_cid:
                rt::ContractId<crate::splice_amulet::Splice_Round::ClosedMiningRound>,
        }
        impl rt::ToValue for SvRewardCoupon_DsoExpire {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "closedRoundCid",
                    rt::ToValue::to_value(&self.closed_round_cid)
                ),])
            }
        }
        impl rt::FromValue for SvRewardCoupon_DsoExpire {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    closed_round_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "closedRoundCid",
                    )?)
                    .map_err(|e| e.at("closedRoundCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorRewardCoupon_ArchiveAsValidator {
            #[serde(rename = "validator")]
            pub validator: rt::Party,
            ///Daml field `rightCid`.
            #[serde(rename = "rightCid")]
            pub right_cid: rt::ContractId<crate::splice_amulet::Splice_Amulet::ValidatorRight>,
        }
        impl rt::ToValue for ValidatorRewardCoupon_ArchiveAsValidator {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("validator", rt::ToValue::to_value(&self.validator)),
                    ("rightCid", rt::ToValue::to_value(&self.right_cid)),
                ])
            }
        }
        impl rt::FromValue for ValidatorRewardCoupon_ArchiveAsValidator {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    validator: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "validator",
                    )?)
                    .map_err(|e| e.at("validator"))?,
                    right_cid: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "rightCid",
                    )?)
                    .map_err(|e| e.at("rightCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorRewardCoupon_DsoExpire {
            ///Daml field `closedRoundCid`.
            #[serde(rename = "closedRoundCid")]
            pub closed_round_cid:
                rt::ContractId<crate::splice_amulet::Splice_Round::ClosedMiningRound>,
        }
        impl rt::ToValue for ValidatorRewardCoupon_DsoExpire {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "closedRoundCid",
                    rt::ToValue::to_value(&self.closed_round_cid)
                ),])
            }
        }
        impl rt::FromValue for ValidatorRewardCoupon_DsoExpire {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    closed_round_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "closedRoundCid",
                    )?)
                    .map_err(|e| e.at("closedRoundCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AppRewardCoupon_DsoExpire {
            ///Daml field `closedRoundCid`.
            #[serde(rename = "closedRoundCid")]
            pub closed_round_cid:
                rt::ContractId<crate::splice_amulet::Splice_Round::ClosedMiningRound>,
        }
        impl rt::ToValue for AppRewardCoupon_DsoExpire {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "closedRoundCid",
                    rt::ToValue::to_value(&self.closed_round_cid)
                ),])
            }
        }
        impl rt::FromValue for AppRewardCoupon_DsoExpire {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    closed_round_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "closedRoundCid",
                    )?)
                    .map_err(|e| e.at("closedRoundCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct FeaturedAppRight_Cancel {}
        impl rt::ToValue for FeaturedAppRight_Cancel {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for FeaturedAppRight_Cancel {
            fn from_value(_value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct FeaturedAppRight_Withdraw {
            #[serde(rename = "reason")]
            pub reason: ::std::string::String,
        }
        impl rt::ToValue for FeaturedAppRight_Withdraw {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("reason", rt::ToValue::to_value(&self.reason)),])
            }
        }
        impl rt::FromValue for FeaturedAppRight_Withdraw {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    reason: rt::FromValue::from_value(rt::required_field(value, 0usize, "reason")?)
                        .map_err(|e| e.at("reason"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorRight_ArchiveAsUser {}
        impl rt::ToValue for ValidatorRight_ArchiveAsUser {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for ValidatorRight_ArchiveAsUser {
            fn from_value(_value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorRight_ArchiveAsValidator {}
        impl rt::ToValue for ValidatorRight_ArchiveAsValidator {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for ValidatorRight_ArchiveAsValidator {
            fn from_value(_value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct LockedAmulet_ExpireAmulet {
            ///Daml field `roundCid`.
            #[serde(rename = "roundCid")]
            pub round_cid: rt::ContractId<crate::splice_amulet::Splice_Round::OpenMiningRound>,
        }
        impl rt::ToValue for LockedAmulet_ExpireAmulet {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "roundCid",
                    rt::ToValue::to_value(&self.round_cid)
                ),])
            }
        }
        impl rt::FromValue for LockedAmulet_ExpireAmulet {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    round_cid: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "roundCid",
                    )?)
                    .map_err(|e| e.at("roundCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct LockedAmulet_OwnerExpireLock {
            ///Daml field `openRoundCid`.
            #[serde(rename = "openRoundCid")]
            pub open_round_cid: rt::ContractId<crate::splice_amulet::Splice_Round::OpenMiningRound>,
        }
        impl rt::ToValue for LockedAmulet_OwnerExpireLock {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "openRoundCid",
                    rt::ToValue::to_value(&self.open_round_cid)
                ),])
            }
        }
        impl rt::FromValue for LockedAmulet_OwnerExpireLock {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    open_round_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "openRoundCid",
                    )?)
                    .map_err(|e| e.at("openRoundCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct LockedAmulet_Unlock {
            ///Daml field `openRoundCid`.
            #[serde(rename = "openRoundCid")]
            pub open_round_cid: rt::ContractId<crate::splice_amulet::Splice_Round::OpenMiningRound>,
        }
        impl rt::ToValue for LockedAmulet_Unlock {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "openRoundCid",
                    rt::ToValue::to_value(&self.open_round_cid)
                ),])
            }
        }
        impl rt::FromValue for LockedAmulet_Unlock {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    open_round_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "openRoundCid",
                    )?)
                    .map_err(|e| e.at("openRoundCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Amulet_Expire {
            ///Daml field `roundCid`.
            #[serde(rename = "roundCid")]
            pub round_cid: rt::ContractId<crate::splice_amulet::Splice_Round::OpenMiningRound>,
        }
        impl rt::ToValue for Amulet_Expire {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "roundCid",
                    rt::ToValue::to_value(&self.round_cid)
                ),])
            }
        }
        impl rt::FromValue for Amulet_Expire {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    round_cid: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "roundCid",
                    )?)
                    .map_err(|e| e.at("roundCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct UnclaimedActivityRecord_DsoExpireResult {
            ///Daml field `unclaimedRewardCid`.
            #[serde(rename = "unclaimedRewardCid")]
            pub unclaimed_reward_cid:
                rt::ContractId<crate::splice_amulet::Splice_Amulet::UnclaimedReward>,
        }
        impl rt::ToValue for UnclaimedActivityRecord_DsoExpireResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "unclaimedRewardCid",
                    rt::ToValue::to_value(&self.unclaimed_reward_cid)
                ),])
            }
        }
        impl rt::FromValue for UnclaimedActivityRecord_DsoExpireResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    unclaimed_reward_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "unclaimedRewardCid",
                    )?)
                    .map_err(|e| e.at("unclaimedRewardCid"))?,
                })
            }
        }
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize,
        )]
        #[serde(crate = "rt::serde")]
        pub enum UnclaimedActivityRecord_ArchiveAsBeneficiaryResult {
            #[serde(rename = "UnclaimedActivityRecord_ArchiveAsBeneficiaryResult")]
            UnclaimedActivityRecord_ArchiveAsBeneficiaryResult,
        }
        impl rt::ToValue for UnclaimedActivityRecord_ArchiveAsBeneficiaryResult {
            fn to_value(&self) -> rt::Value {
                rt::enum_value(
                    match self {
                        UnclaimedActivityRecord_ArchiveAsBeneficiaryResult::UnclaimedActivityRecord_ArchiveAsBeneficiaryResult => {
                            "UnclaimedActivityRecord_ArchiveAsBeneficiaryResult"
                        }
                    },
                )
            }
        }
        impl rt::FromValue for UnclaimedActivityRecord_ArchiveAsBeneficiaryResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                match rt::enum_constructor(value)? {
                    "UnclaimedActivityRecord_ArchiveAsBeneficiaryResult" => {
                        ::core::result::Result::Ok(
                            UnclaimedActivityRecord_ArchiveAsBeneficiaryResult::UnclaimedActivityRecord_ArchiveAsBeneficiaryResult,
                        )
                    }
                    other => {
                        ::core::result::Result::Err(
                            rt::unexpected_constructor(
                                "UnclaimedActivityRecord_ArchiveAsBeneficiaryResult",
                                other,
                            ),
                        )
                    }
                }
            }
        }
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize,
        )]
        #[serde(crate = "rt::serde")]
        pub enum SvRewardCoupon_ArchiveAsBeneficiaryResult {
            #[serde(rename = "SvRewardCoupon_ArchiveAsBeneficiaryResult")]
            SvRewardCoupon_ArchiveAsBeneficiaryResult,
        }
        impl rt::ToValue for SvRewardCoupon_ArchiveAsBeneficiaryResult {
            fn to_value(&self) -> rt::Value {
                rt::enum_value(
                    match self {
                        SvRewardCoupon_ArchiveAsBeneficiaryResult::SvRewardCoupon_ArchiveAsBeneficiaryResult => {
                            "SvRewardCoupon_ArchiveAsBeneficiaryResult"
                        }
                    },
                )
            }
        }
        impl rt::FromValue for SvRewardCoupon_ArchiveAsBeneficiaryResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                match rt::enum_constructor(value)? {
                    "SvRewardCoupon_ArchiveAsBeneficiaryResult" => {
                        ::core::result::Result::Ok(
                            SvRewardCoupon_ArchiveAsBeneficiaryResult::SvRewardCoupon_ArchiveAsBeneficiaryResult,
                        )
                    }
                    other => {
                        ::core::result::Result::Err(
                            rt::unexpected_constructor(
                                "SvRewardCoupon_ArchiveAsBeneficiaryResult",
                                other,
                            ),
                        )
                    }
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SvRewardCoupon_DsoExpireResult {
            #[serde(rename = "weight")]
            pub weight: rt::Int64,
        }
        impl rt::ToValue for SvRewardCoupon_DsoExpireResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("weight", rt::ToValue::to_value(&self.weight)),])
            }
        }
        impl rt::FromValue for SvRewardCoupon_DsoExpireResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    weight: rt::FromValue::from_value(rt::required_field(value, 0usize, "weight")?)
                        .map_err(|e| e.at("weight"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorRewardCoupon_ArchiveAsValidatorResult {}
        impl rt::ToValue for ValidatorRewardCoupon_ArchiveAsValidatorResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for ValidatorRewardCoupon_ArchiveAsValidatorResult {
            fn from_value(_value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorRewardCoupon_DsoExpireResult {
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
        }
        impl rt::ToValue for ValidatorRewardCoupon_DsoExpireResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("amount", rt::ToValue::to_value(&self.amount)),])
            }
        }
        impl rt::FromValue for ValidatorRewardCoupon_DsoExpireResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    amount: rt::FromValue::from_value(rt::required_field(value, 0usize, "amount")?)
                        .map_err(|e| e.at("amount"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AppRewardCoupon_DsoExpireResult {
            #[serde(rename = "featured")]
            pub featured: bool,
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
        }
        impl rt::ToValue for AppRewardCoupon_DsoExpireResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("featured", rt::ToValue::to_value(&self.featured)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                ])
            }
        }
        impl rt::FromValue for AppRewardCoupon_DsoExpireResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    featured: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "featured",
                    )?)
                    .map_err(|e| e.at("featured"))?,
                    amount: rt::FromValue::from_value(rt::required_field(value, 1usize, "amount")?)
                        .map_err(|e| e.at("amount"))?,
                })
            }
        }
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize,
        )]
        #[serde(crate = "rt::serde")]
        pub enum FeaturedAppRight_CancelResult {
            #[serde(rename = "FeaturedAppRight_CancelResult")]
            FeaturedAppRight_CancelResult,
        }
        impl rt::ToValue for FeaturedAppRight_CancelResult {
            fn to_value(&self) -> rt::Value {
                rt::enum_value(match self {
                    FeaturedAppRight_CancelResult::FeaturedAppRight_CancelResult => {
                        "FeaturedAppRight_CancelResult"
                    }
                })
            }
        }
        impl rt::FromValue for FeaturedAppRight_CancelResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                match rt::enum_constructor(value)? {
                    "FeaturedAppRight_CancelResult" => ::core::result::Result::Ok(
                        FeaturedAppRight_CancelResult::FeaturedAppRight_CancelResult,
                    ),
                    other => ::core::result::Result::Err(rt::unexpected_constructor(
                        "FeaturedAppRight_CancelResult",
                        other,
                    )),
                }
            }
        }
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize,
        )]
        #[serde(crate = "rt::serde")]
        pub enum FeaturedAppRight_WithdrawResult {
            #[serde(rename = "FeaturedAppRight_WithdrawResult")]
            FeaturedAppRight_WithdrawResult,
        }
        impl rt::ToValue for FeaturedAppRight_WithdrawResult {
            fn to_value(&self) -> rt::Value {
                rt::enum_value(match self {
                    FeaturedAppRight_WithdrawResult::FeaturedAppRight_WithdrawResult => {
                        "FeaturedAppRight_WithdrawResult"
                    }
                })
            }
        }
        impl rt::FromValue for FeaturedAppRight_WithdrawResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                match rt::enum_constructor(value)? {
                    "FeaturedAppRight_WithdrawResult" => ::core::result::Result::Ok(
                        FeaturedAppRight_WithdrawResult::FeaturedAppRight_WithdrawResult,
                    ),
                    other => ::core::result::Result::Err(rt::unexpected_constructor(
                        "FeaturedAppRight_WithdrawResult",
                        other,
                    )),
                }
            }
        }
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize,
        )]
        #[serde(crate = "rt::serde")]
        pub enum ValidatorRight_ArchiveAsUserResult {
            #[serde(rename = "ValidatorRight_ArchiveAsUserResult")]
            ValidatorRight_ArchiveAsUserResult,
        }
        impl rt::ToValue for ValidatorRight_ArchiveAsUserResult {
            fn to_value(&self) -> rt::Value {
                rt::enum_value(match self {
                    ValidatorRight_ArchiveAsUserResult::ValidatorRight_ArchiveAsUserResult => {
                        "ValidatorRight_ArchiveAsUserResult"
                    }
                })
            }
        }
        impl rt::FromValue for ValidatorRight_ArchiveAsUserResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                match rt::enum_constructor(value)? {
                    "ValidatorRight_ArchiveAsUserResult" => ::core::result::Result::Ok(
                        ValidatorRight_ArchiveAsUserResult::ValidatorRight_ArchiveAsUserResult,
                    ),
                    other => ::core::result::Result::Err(rt::unexpected_constructor(
                        "ValidatorRight_ArchiveAsUserResult",
                        other,
                    )),
                }
            }
        }
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize,
        )]
        #[serde(crate = "rt::serde")]
        pub enum ValidatorRight_ArchiveAsValidatorResult {
            #[serde(rename = "ValidatorRight_ArchiveAsValidatorResult")]
            ValidatorRight_ArchiveAsValidatorResult,
        }
        impl rt::ToValue for ValidatorRight_ArchiveAsValidatorResult {
            fn to_value(&self) -> rt::Value {
                rt::enum_value(
                    match self {
                        ValidatorRight_ArchiveAsValidatorResult::ValidatorRight_ArchiveAsValidatorResult => {
                            "ValidatorRight_ArchiveAsValidatorResult"
                        }
                    },
                )
            }
        }
        impl rt::FromValue for ValidatorRight_ArchiveAsValidatorResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                match rt::enum_constructor(value)? {
                    "ValidatorRight_ArchiveAsValidatorResult" => {
                        ::core::result::Result::Ok(
                            ValidatorRight_ArchiveAsValidatorResult::ValidatorRight_ArchiveAsValidatorResult,
                        )
                    }
                    other => {
                        ::core::result::Result::Err(
                            rt::unexpected_constructor(
                                "ValidatorRight_ArchiveAsValidatorResult",
                                other,
                            ),
                        )
                    }
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct LockedAmulet_ExpireAmuletResult {
            ///Daml field `expireSum`.
            #[serde(rename = "expireSum")]
            pub expire_sum: crate::splice_amulet::Splice_Amulet::AmuletExpireSummary,
            #[serde(rename = "meta")]
            pub meta: ::core::option::Option<
                ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
            >,
        }
        impl rt::ToValue for LockedAmulet_ExpireAmuletResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("expireSum", rt::ToValue::to_value(&self.expire_sum)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for LockedAmulet_ExpireAmuletResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    expire_sum: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "expireSum",
                    )?)
                    .map_err(|e| e.at("expireSum"))?,
                    meta: rt::optional_field(value, 1usize, "meta").map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct LockedAmulet_OwnerExpireLockResult {
            ///Daml field `amuletSum`.
            #[serde(rename = "amuletSum")]
            pub amulet_sum: crate::splice_amulet::Splice_Amulet::AmuletCreateSummary<
                rt::ContractId<crate::splice_amulet::Splice_Amulet::Amulet>,
            >,
            #[serde(rename = "meta")]
            pub meta: ::core::option::Option<
                ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
            >,
        }
        impl rt::ToValue for LockedAmulet_OwnerExpireLockResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("amuletSum", rt::ToValue::to_value(&self.amulet_sum)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for LockedAmulet_OwnerExpireLockResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    amulet_sum: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "amuletSum",
                    )?)
                    .map_err(|e| e.at("amuletSum"))?,
                    meta: rt::optional_field(value, 1usize, "meta").map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct LockedAmulet_UnlockResult {
            ///Daml field `amuletSum`.
            #[serde(rename = "amuletSum")]
            pub amulet_sum: crate::splice_amulet::Splice_Amulet::AmuletCreateSummary<
                rt::ContractId<crate::splice_amulet::Splice_Amulet::Amulet>,
            >,
            #[serde(rename = "meta")]
            pub meta: ::core::option::Option<
                ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
            >,
        }
        impl rt::ToValue for LockedAmulet_UnlockResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("amuletSum", rt::ToValue::to_value(&self.amulet_sum)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for LockedAmulet_UnlockResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    amulet_sum: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "amuletSum",
                    )?)
                    .map_err(|e| e.at("amuletSum"))?,
                    meta: rt::optional_field(value, 1usize, "meta").map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Amulet_ExpireResult {
            ///Daml field `expireSum`.
            #[serde(rename = "expireSum")]
            pub expire_sum: crate::splice_amulet::Splice_Amulet::AmuletExpireSummary,
            #[serde(rename = "meta")]
            pub meta: ::core::option::Option<
                ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
            >,
        }
        impl rt::ToValue for Amulet_ExpireResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("expireSum", rt::ToValue::to_value(&self.expire_sum)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for Amulet_ExpireResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    expire_sum: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "expireSum",
                    )?)
                    .map_err(|e| e.at("expireSum"))?,
                    meta: rt::optional_field(value, 1usize, "meta").map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletCreateSummary<AmuletContractId> {
            #[serde(rename = "amulet")]
            pub amulet: AmuletContractId,
            ///Daml field `amuletPrice`.
            #[serde(rename = "amuletPrice")]
            pub amulet_price: rt::Numeric,
            #[serde(rename = "round")]
            pub round: crate::splice_amulet::Splice_Types::Round,
        }
        impl<AmuletContractId> rt::ToValue for AmuletCreateSummary<AmuletContractId>
        where
            AmuletContractId: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("amulet", rt::ToValue::to_value(&self.amulet)),
                    ("amuletPrice", rt::ToValue::to_value(&self.amulet_price)),
                    ("round", rt::ToValue::to_value(&self.round)),
                ])
            }
        }
        impl<AmuletContractId> rt::FromValue for AmuletCreateSummary<AmuletContractId>
        where
            AmuletContractId: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    amulet: rt::FromValue::from_value(rt::required_field(value, 0usize, "amulet")?)
                        .map_err(|e| e.at("amulet"))?,
                    amulet_price: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "amuletPrice",
                    )?)
                    .map_err(|e| e.at("amuletPrice"))?,
                    round: rt::FromValue::from_value(rt::required_field(value, 2usize, "round")?)
                        .map_err(|e| e.at("round"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletExpireSummary {
            #[serde(rename = "owner")]
            pub owner: rt::Party,
            #[serde(rename = "round")]
            pub round: crate::splice_amulet::Splice_Types::Round,
            ///Daml field `changeToInitialAmountAsOfRoundZero`.
            #[serde(rename = "changeToInitialAmountAsOfRoundZero")]
            pub change_to_initial_amount_as_of_round_zero: rt::Numeric,
            ///Daml field `changeToHoldingFeesRate`.
            #[serde(rename = "changeToHoldingFeesRate")]
            pub change_to_holding_fees_rate: rt::Numeric,
        }
        impl rt::ToValue for AmuletExpireSummary {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("owner", rt::ToValue::to_value(&self.owner)),
                    ("round", rt::ToValue::to_value(&self.round)),
                    (
                        "changeToInitialAmountAsOfRoundZero",
                        rt::ToValue::to_value(&self.change_to_initial_amount_as_of_round_zero)
                    ),
                    (
                        "changeToHoldingFeesRate",
                        rt::ToValue::to_value(&self.change_to_holding_fees_rate)
                    ),
                ])
            }
        }
        impl rt::FromValue for AmuletExpireSummary {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    owner: rt::FromValue::from_value(rt::required_field(value, 0usize, "owner")?)
                        .map_err(|e| e.at("owner"))?,
                    round: rt::FromValue::from_value(rt::required_field(value, 1usize, "round")?)
                        .map_err(|e| e.at("round"))?,
                    change_to_initial_amount_as_of_round_zero: rt::FromValue::from_value(
                        rt::required_field(value, 2usize, "changeToInitialAmountAsOfRoundZero")?,
                    )
                    .map_err(|e| e.at("changeToInitialAmountAsOfRoundZero"))?,
                    change_to_holding_fees_rate: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "changeToHoldingFeesRate",
                    )?)
                    .map_err(|e| e.at("changeToHoldingFeesRate"))?,
                })
            }
        }
        ///The Daml template `Splice.Amulet:Amulet`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-amulet:Splice.Amulet:Amulet`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `Amulet_Expire` — consuming
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Amulet {
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            #[serde(rename = "owner")]
            pub owner: rt::Party,
            #[serde(rename = "amount")]
            pub amount: crate::splice_amulet::Splice_Fees::ExpiringAmount,
        }
        impl rt::ToValue for Amulet {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("owner", rt::ToValue::to_value(&self.owner)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                ])
            }
        }
        impl rt::FromValue for Amulet {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                    owner: rt::FromValue::from_value(rt::required_field(value, 1usize, "owner")?)
                        .map_err(|e| e.at("owner"))?,
                    amount: rt::FromValue::from_value(rt::required_field(value, 2usize, "amount")?)
                        .map_err(|e| e.at("amount"))?,
                })
            }
        }
        impl rt::Contract for Amulet {
            const PACKAGE_ID: &'static str =
                "3ca1343ab26b453d38c8adb70dca5f1ead8440c42b59b68f070786955cbf9ec1";
            const PACKAGE_NAME: &'static str = "splice-amulet";
            const MODULE_NAME: &'static str = "Splice.Amulet";
            const ENTITY_NAME: &'static str = "Amulet";
        }
        impl rt::Template for Amulet {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("owner", rt::ToValue::to_value(&self.owner)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                ])
            }
        }
        ///The `Amulet_Expire` choice on [`Amulet`] (consuming).
        impl rt::Choice<Amulet> for crate::splice_amulet::Splice_Amulet::Amulet_Expire {
            type Return = crate::splice_amulet::Splice_Amulet::Amulet_ExpireResult;
            const NAME: &'static str = "Amulet_Expire";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`Amulet`] (consuming).
        impl rt::Choice<Amulet>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The Daml template `Splice.Amulet:AppRewardCoupon`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-amulet:Splice.Amulet:AppRewardCoupon`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `AppRewardCoupon_DsoExpire` — consuming
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AppRewardCoupon {
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            #[serde(rename = "provider")]
            pub provider: rt::Party,
            #[serde(rename = "featured")]
            pub featured: bool,
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            #[serde(rename = "round")]
            pub round: crate::splice_amulet::Splice_Types::Round,
            #[serde(rename = "beneficiary")]
            pub beneficiary: ::core::option::Option<rt::Party>,
        }
        impl rt::ToValue for AppRewardCoupon {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("provider", rt::ToValue::to_value(&self.provider)),
                    ("featured", rt::ToValue::to_value(&self.featured)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                    ("round", rt::ToValue::to_value(&self.round)),
                    ("beneficiary", rt::ToValue::to_value(&self.beneficiary)),
                ])
            }
        }
        impl rt::FromValue for AppRewardCoupon {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                    provider: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "provider",
                    )?)
                    .map_err(|e| e.at("provider"))?,
                    featured: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "featured",
                    )?)
                    .map_err(|e| e.at("featured"))?,
                    amount: rt::FromValue::from_value(rt::required_field(value, 3usize, "amount")?)
                        .map_err(|e| e.at("amount"))?,
                    round: rt::FromValue::from_value(rt::required_field(value, 4usize, "round")?)
                        .map_err(|e| e.at("round"))?,
                    beneficiary: rt::optional_field(value, 5usize, "beneficiary")
                        .map_err(|e| e.at("beneficiary"))?,
                })
            }
        }
        impl rt::Contract for AppRewardCoupon {
            const PACKAGE_ID: &'static str =
                "3ca1343ab26b453d38c8adb70dca5f1ead8440c42b59b68f070786955cbf9ec1";
            const PACKAGE_NAME: &'static str = "splice-amulet";
            const MODULE_NAME: &'static str = "Splice.Amulet";
            const ENTITY_NAME: &'static str = "AppRewardCoupon";
        }
        impl rt::Template for AppRewardCoupon {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("provider", rt::ToValue::to_value(&self.provider)),
                    ("featured", rt::ToValue::to_value(&self.featured)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                    ("round", rt::ToValue::to_value(&self.round)),
                    ("beneficiary", rt::ToValue::to_value(&self.beneficiary)),
                ])
            }
        }
        ///The `AppRewardCoupon_DsoExpire` choice on [`AppRewardCoupon`] (consuming).
        impl rt::Choice<AppRewardCoupon>
            for crate::splice_amulet::Splice_Amulet::AppRewardCoupon_DsoExpire
        {
            type Return = crate::splice_amulet::Splice_Amulet::AppRewardCoupon_DsoExpireResult;
            const NAME: &'static str = "AppRewardCoupon_DsoExpire";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`AppRewardCoupon`] (consuming).
        impl rt::Choice<AppRewardCoupon>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The Daml template `Splice.Amulet:FeaturedAppActivityMarker`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-amulet:Splice.Amulet:FeaturedAppActivityMarker`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct FeaturedAppActivityMarker {
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            #[serde(rename = "provider")]
            pub provider: rt::Party,
            #[serde(rename = "beneficiary")]
            pub beneficiary: rt::Party,
            #[serde(rename = "weight")]
            pub weight: rt::Numeric,
        }
        impl rt::ToValue for FeaturedAppActivityMarker {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("provider", rt::ToValue::to_value(&self.provider)),
                    ("beneficiary", rt::ToValue::to_value(&self.beneficiary)),
                    ("weight", rt::ToValue::to_value(&self.weight)),
                ])
            }
        }
        impl rt::FromValue for FeaturedAppActivityMarker {
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
        impl rt::Contract for FeaturedAppActivityMarker {
            const PACKAGE_ID: &'static str =
                "3ca1343ab26b453d38c8adb70dca5f1ead8440c42b59b68f070786955cbf9ec1";
            const PACKAGE_NAME: &'static str = "splice-amulet";
            const MODULE_NAME: &'static str = "Splice.Amulet";
            const ENTITY_NAME: &'static str = "FeaturedAppActivityMarker";
        }
        impl rt::Template for FeaturedAppActivityMarker {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("provider", rt::ToValue::to_value(&self.provider)),
                    ("beneficiary", rt::ToValue::to_value(&self.beneficiary)),
                    ("weight", rt::ToValue::to_value(&self.weight)),
                ])
            }
        }
        ///The `Archive` choice on [`FeaturedAppActivityMarker`] (consuming).
        impl rt::Choice<FeaturedAppActivityMarker>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The Daml template `Splice.Amulet:FeaturedAppRight`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-amulet:Splice.Amulet:FeaturedAppRight`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `FeaturedAppRight_Withdraw` — consuming
        ///- `FeaturedAppRight_Cancel` — consuming
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct FeaturedAppRight {
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            #[serde(rename = "provider")]
            pub provider: rt::Party,
        }
        impl rt::ToValue for FeaturedAppRight {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("provider", rt::ToValue::to_value(&self.provider)),
                ])
            }
        }
        impl rt::FromValue for FeaturedAppRight {
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
        impl rt::Contract for FeaturedAppRight {
            const PACKAGE_ID: &'static str =
                "3ca1343ab26b453d38c8adb70dca5f1ead8440c42b59b68f070786955cbf9ec1";
            const PACKAGE_NAME: &'static str = "splice-amulet";
            const MODULE_NAME: &'static str = "Splice.Amulet";
            const ENTITY_NAME: &'static str = "FeaturedAppRight";
        }
        impl rt::Template for FeaturedAppRight {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("provider", rt::ToValue::to_value(&self.provider)),
                ])
            }
        }
        ///The `FeaturedAppRight_Withdraw` choice on [`FeaturedAppRight`] (consuming).
        impl rt::Choice<FeaturedAppRight>
            for crate::splice_amulet::Splice_Amulet::FeaturedAppRight_Withdraw
        {
            type Return = crate::splice_amulet::Splice_Amulet::FeaturedAppRight_WithdrawResult;
            const NAME: &'static str = "FeaturedAppRight_Withdraw";
            const CONSUMING: bool = true;
        }
        ///The `FeaturedAppRight_Cancel` choice on [`FeaturedAppRight`] (consuming).
        impl rt::Choice<FeaturedAppRight> for crate::splice_amulet::Splice_Amulet::FeaturedAppRight_Cancel {
            type Return = crate::splice_amulet::Splice_Amulet::FeaturedAppRight_CancelResult;
            const NAME: &'static str = "FeaturedAppRight_Cancel";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`FeaturedAppRight`] (consuming).
        impl rt::Choice<FeaturedAppRight>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The Daml template `Splice.Amulet:LockedAmulet`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-amulet:Splice.Amulet:LockedAmulet`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `LockedAmulet_Unlock` — consuming
        ///- `LockedAmulet_OwnerExpireLock` — consuming
        ///- `LockedAmulet_ExpireAmulet` — consuming
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct LockedAmulet {
            #[serde(rename = "amulet")]
            pub amulet: crate::splice_amulet::Splice_Amulet::Amulet,
            #[serde(rename = "lock")]
            pub lock: crate::splice_amulet::Splice_Expiry::TimeLock,
        }
        impl rt::ToValue for LockedAmulet {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("amulet", rt::ToValue::to_value(&self.amulet)),
                    ("lock", rt::ToValue::to_value(&self.lock)),
                ])
            }
        }
        impl rt::FromValue for LockedAmulet {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    amulet: rt::FromValue::from_value(rt::required_field(value, 0usize, "amulet")?)
                        .map_err(|e| e.at("amulet"))?,
                    lock: rt::FromValue::from_value(rt::required_field(value, 1usize, "lock")?)
                        .map_err(|e| e.at("lock"))?,
                })
            }
        }
        impl rt::Contract for LockedAmulet {
            const PACKAGE_ID: &'static str =
                "3ca1343ab26b453d38c8adb70dca5f1ead8440c42b59b68f070786955cbf9ec1";
            const PACKAGE_NAME: &'static str = "splice-amulet";
            const MODULE_NAME: &'static str = "Splice.Amulet";
            const ENTITY_NAME: &'static str = "LockedAmulet";
        }
        impl rt::Template for LockedAmulet {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("amulet", rt::ToValue::to_value(&self.amulet)),
                    ("lock", rt::ToValue::to_value(&self.lock)),
                ])
            }
        }
        ///The `LockedAmulet_Unlock` choice on [`LockedAmulet`] (consuming).
        impl rt::Choice<LockedAmulet> for crate::splice_amulet::Splice_Amulet::LockedAmulet_Unlock {
            type Return = crate::splice_amulet::Splice_Amulet::LockedAmulet_UnlockResult;
            const NAME: &'static str = "LockedAmulet_Unlock";
            const CONSUMING: bool = true;
        }
        ///The `LockedAmulet_OwnerExpireLock` choice on [`LockedAmulet`] (consuming).
        impl rt::Choice<LockedAmulet>
            for crate::splice_amulet::Splice_Amulet::LockedAmulet_OwnerExpireLock
        {
            type Return = crate::splice_amulet::Splice_Amulet::LockedAmulet_OwnerExpireLockResult;
            const NAME: &'static str = "LockedAmulet_OwnerExpireLock";
            const CONSUMING: bool = true;
        }
        ///The `LockedAmulet_ExpireAmulet` choice on [`LockedAmulet`] (consuming).
        impl rt::Choice<LockedAmulet> for crate::splice_amulet::Splice_Amulet::LockedAmulet_ExpireAmulet {
            type Return = crate::splice_amulet::Splice_Amulet::LockedAmulet_ExpireAmuletResult;
            const NAME: &'static str = "LockedAmulet_ExpireAmulet";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`LockedAmulet`] (consuming).
        impl rt::Choice<LockedAmulet>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The Daml template `Splice.Amulet:SvRewardCoupon`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-amulet:Splice.Amulet:SvRewardCoupon`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `SvRewardCoupon_DsoExpire` — consuming
        ///- `SvRewardCoupon_ArchiveAsBeneficiary` — consuming
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SvRewardCoupon {
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            #[serde(rename = "sv")]
            pub sv: rt::Party,
            #[serde(rename = "beneficiary")]
            pub beneficiary: rt::Party,
            #[serde(rename = "round")]
            pub round: crate::splice_amulet::Splice_Types::Round,
            #[serde(rename = "weight")]
            pub weight: rt::Int64,
        }
        impl rt::ToValue for SvRewardCoupon {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("sv", rt::ToValue::to_value(&self.sv)),
                    ("beneficiary", rt::ToValue::to_value(&self.beneficiary)),
                    ("round", rt::ToValue::to_value(&self.round)),
                    ("weight", rt::ToValue::to_value(&self.weight)),
                ])
            }
        }
        impl rt::FromValue for SvRewardCoupon {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                    sv: rt::FromValue::from_value(rt::required_field(value, 1usize, "sv")?)
                        .map_err(|e| e.at("sv"))?,
                    beneficiary: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "beneficiary",
                    )?)
                    .map_err(|e| e.at("beneficiary"))?,
                    round: rt::FromValue::from_value(rt::required_field(value, 3usize, "round")?)
                        .map_err(|e| e.at("round"))?,
                    weight: rt::FromValue::from_value(rt::required_field(value, 4usize, "weight")?)
                        .map_err(|e| e.at("weight"))?,
                })
            }
        }
        impl rt::Contract for SvRewardCoupon {
            const PACKAGE_ID: &'static str =
                "3ca1343ab26b453d38c8adb70dca5f1ead8440c42b59b68f070786955cbf9ec1";
            const PACKAGE_NAME: &'static str = "splice-amulet";
            const MODULE_NAME: &'static str = "Splice.Amulet";
            const ENTITY_NAME: &'static str = "SvRewardCoupon";
        }
        impl rt::Template for SvRewardCoupon {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("sv", rt::ToValue::to_value(&self.sv)),
                    ("beneficiary", rt::ToValue::to_value(&self.beneficiary)),
                    ("round", rt::ToValue::to_value(&self.round)),
                    ("weight", rt::ToValue::to_value(&self.weight)),
                ])
            }
        }
        ///The `SvRewardCoupon_DsoExpire` choice on [`SvRewardCoupon`] (consuming).
        impl rt::Choice<SvRewardCoupon> for crate::splice_amulet::Splice_Amulet::SvRewardCoupon_DsoExpire {
            type Return = crate::splice_amulet::Splice_Amulet::SvRewardCoupon_DsoExpireResult;
            const NAME: &'static str = "SvRewardCoupon_DsoExpire";
            const CONSUMING: bool = true;
        }
        ///The `SvRewardCoupon_ArchiveAsBeneficiary` choice on [`SvRewardCoupon`] (consuming).
        impl rt::Choice<SvRewardCoupon>
            for crate::splice_amulet::Splice_Amulet::SvRewardCoupon_ArchiveAsBeneficiary
        {
            type Return =
                crate::splice_amulet::Splice_Amulet::SvRewardCoupon_ArchiveAsBeneficiaryResult;
            const NAME: &'static str = "SvRewardCoupon_ArchiveAsBeneficiary";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`SvRewardCoupon`] (consuming).
        impl rt::Choice<SvRewardCoupon>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The Daml template `Splice.Amulet:UnclaimedActivityRecord`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-amulet:Splice.Amulet:UnclaimedActivityRecord`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `UnclaimedActivityRecord_DsoExpire` — consuming
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct UnclaimedActivityRecord {
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            #[serde(rename = "beneficiary")]
            pub beneficiary: rt::Party,
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            #[serde(rename = "reason")]
            pub reason: ::std::string::String,
            ///Daml field `expiresAt`.
            #[serde(rename = "expiresAt")]
            pub expires_at: rt::Timestamp,
        }
        impl rt::ToValue for UnclaimedActivityRecord {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("beneficiary", rt::ToValue::to_value(&self.beneficiary)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                    ("reason", rt::ToValue::to_value(&self.reason)),
                    ("expiresAt", rt::ToValue::to_value(&self.expires_at)),
                ])
            }
        }
        impl rt::FromValue for UnclaimedActivityRecord {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                    beneficiary: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "beneficiary",
                    )?)
                    .map_err(|e| e.at("beneficiary"))?,
                    amount: rt::FromValue::from_value(rt::required_field(value, 2usize, "amount")?)
                        .map_err(|e| e.at("amount"))?,
                    reason: rt::FromValue::from_value(rt::required_field(value, 3usize, "reason")?)
                        .map_err(|e| e.at("reason"))?,
                    expires_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "expiresAt",
                    )?)
                    .map_err(|e| e.at("expiresAt"))?,
                })
            }
        }
        impl rt::Contract for UnclaimedActivityRecord {
            const PACKAGE_ID: &'static str =
                "3ca1343ab26b453d38c8adb70dca5f1ead8440c42b59b68f070786955cbf9ec1";
            const PACKAGE_NAME: &'static str = "splice-amulet";
            const MODULE_NAME: &'static str = "Splice.Amulet";
            const ENTITY_NAME: &'static str = "UnclaimedActivityRecord";
        }
        impl rt::Template for UnclaimedActivityRecord {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("beneficiary", rt::ToValue::to_value(&self.beneficiary)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                    ("reason", rt::ToValue::to_value(&self.reason)),
                    ("expiresAt", rt::ToValue::to_value(&self.expires_at)),
                ])
            }
        }
        ///The `UnclaimedActivityRecord_DsoExpire` choice on [`UnclaimedActivityRecord`] (consuming).
        impl rt::Choice<UnclaimedActivityRecord>
            for crate::splice_amulet::Splice_Amulet::UnclaimedActivityRecord_DsoExpire
        {
            type Return =
                crate::splice_amulet::Splice_Amulet::UnclaimedActivityRecord_DsoExpireResult;
            const NAME: &'static str = "UnclaimedActivityRecord_DsoExpire";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`UnclaimedActivityRecord`] (consuming).
        impl rt::Choice<UnclaimedActivityRecord>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The Daml template `Splice.Amulet:UnclaimedReward`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-amulet:Splice.Amulet:UnclaimedReward`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct UnclaimedReward {
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
        }
        impl rt::ToValue for UnclaimedReward {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                ])
            }
        }
        impl rt::FromValue for UnclaimedReward {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                    amount: rt::FromValue::from_value(rt::required_field(value, 1usize, "amount")?)
                        .map_err(|e| e.at("amount"))?,
                })
            }
        }
        impl rt::Contract for UnclaimedReward {
            const PACKAGE_ID: &'static str =
                "3ca1343ab26b453d38c8adb70dca5f1ead8440c42b59b68f070786955cbf9ec1";
            const PACKAGE_NAME: &'static str = "splice-amulet";
            const MODULE_NAME: &'static str = "Splice.Amulet";
            const ENTITY_NAME: &'static str = "UnclaimedReward";
        }
        impl rt::Template for UnclaimedReward {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                ])
            }
        }
        ///The `Archive` choice on [`UnclaimedReward`] (consuming).
        impl rt::Choice<UnclaimedReward>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The Daml template `Splice.Amulet:ValidatorRewardCoupon`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-amulet:Splice.Amulet:ValidatorRewardCoupon`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `ValidatorRewardCoupon_DsoExpire` — consuming
        ///- `ValidatorRewardCoupon_ArchiveAsValidator` — consuming
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorRewardCoupon {
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            #[serde(rename = "user")]
            pub user: rt::Party,
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            #[serde(rename = "round")]
            pub round: crate::splice_amulet::Splice_Types::Round,
        }
        impl rt::ToValue for ValidatorRewardCoupon {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("user", rt::ToValue::to_value(&self.user)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                    ("round", rt::ToValue::to_value(&self.round)),
                ])
            }
        }
        impl rt::FromValue for ValidatorRewardCoupon {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                    user: rt::FromValue::from_value(rt::required_field(value, 1usize, "user")?)
                        .map_err(|e| e.at("user"))?,
                    amount: rt::FromValue::from_value(rt::required_field(value, 2usize, "amount")?)
                        .map_err(|e| e.at("amount"))?,
                    round: rt::FromValue::from_value(rt::required_field(value, 3usize, "round")?)
                        .map_err(|e| e.at("round"))?,
                })
            }
        }
        impl rt::Contract for ValidatorRewardCoupon {
            const PACKAGE_ID: &'static str =
                "3ca1343ab26b453d38c8adb70dca5f1ead8440c42b59b68f070786955cbf9ec1";
            const PACKAGE_NAME: &'static str = "splice-amulet";
            const MODULE_NAME: &'static str = "Splice.Amulet";
            const ENTITY_NAME: &'static str = "ValidatorRewardCoupon";
        }
        impl rt::Template for ValidatorRewardCoupon {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("user", rt::ToValue::to_value(&self.user)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                    ("round", rt::ToValue::to_value(&self.round)),
                ])
            }
        }
        ///The `ValidatorRewardCoupon_DsoExpire` choice on [`ValidatorRewardCoupon`] (consuming).
        impl rt::Choice<ValidatorRewardCoupon>
            for crate::splice_amulet::Splice_Amulet::ValidatorRewardCoupon_DsoExpire
        {
            type Return =
                crate::splice_amulet::Splice_Amulet::ValidatorRewardCoupon_DsoExpireResult;
            const NAME: &'static str = "ValidatorRewardCoupon_DsoExpire";
            const CONSUMING: bool = true;
        }
        ///The `ValidatorRewardCoupon_ArchiveAsValidator` choice on [`ValidatorRewardCoupon`] (consuming).
        impl rt::Choice<ValidatorRewardCoupon>
            for crate::splice_amulet::Splice_Amulet::ValidatorRewardCoupon_ArchiveAsValidator
        {
            type Return =
                crate::splice_amulet::Splice_Amulet::ValidatorRewardCoupon_ArchiveAsValidatorResult;
            const NAME: &'static str = "ValidatorRewardCoupon_ArchiveAsValidator";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`ValidatorRewardCoupon`] (consuming).
        impl rt::Choice<ValidatorRewardCoupon>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The Daml template `Splice.Amulet:ValidatorRight`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-amulet:Splice.Amulet:ValidatorRight`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `ValidatorRight_ArchiveAsValidator` — consuming
        ///- `ValidatorRight_ArchiveAsUser` — consuming
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorRight {
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            #[serde(rename = "user")]
            pub user: rt::Party,
            #[serde(rename = "validator")]
            pub validator: rt::Party,
        }
        impl rt::ToValue for ValidatorRight {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("user", rt::ToValue::to_value(&self.user)),
                    ("validator", rt::ToValue::to_value(&self.validator)),
                ])
            }
        }
        impl rt::FromValue for ValidatorRight {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                    user: rt::FromValue::from_value(rt::required_field(value, 1usize, "user")?)
                        .map_err(|e| e.at("user"))?,
                    validator: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "validator",
                    )?)
                    .map_err(|e| e.at("validator"))?,
                })
            }
        }
        impl rt::Contract for ValidatorRight {
            const PACKAGE_ID: &'static str =
                "3ca1343ab26b453d38c8adb70dca5f1ead8440c42b59b68f070786955cbf9ec1";
            const PACKAGE_NAME: &'static str = "splice-amulet";
            const MODULE_NAME: &'static str = "Splice.Amulet";
            const ENTITY_NAME: &'static str = "ValidatorRight";
        }
        impl rt::Template for ValidatorRight {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("user", rt::ToValue::to_value(&self.user)),
                    ("validator", rt::ToValue::to_value(&self.validator)),
                ])
            }
        }
        ///The `ValidatorRight_ArchiveAsValidator` choice on [`ValidatorRight`] (consuming).
        impl rt::Choice<ValidatorRight>
            for crate::splice_amulet::Splice_Amulet::ValidatorRight_ArchiveAsValidator
        {
            type Return =
                crate::splice_amulet::Splice_Amulet::ValidatorRight_ArchiveAsValidatorResult;
            const NAME: &'static str = "ValidatorRight_ArchiveAsValidator";
            const CONSUMING: bool = true;
        }
        ///The `ValidatorRight_ArchiveAsUser` choice on [`ValidatorRight`] (consuming).
        impl rt::Choice<ValidatorRight>
            for crate::splice_amulet::Splice_Amulet::ValidatorRight_ArchiveAsUser
        {
            type Return = crate::splice_amulet::Splice_Amulet::ValidatorRight_ArchiveAsUserResult;
            const NAME: &'static str = "ValidatorRight_ArchiveAsUser";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`ValidatorRight`] (consuming).
        impl rt::Choice<ValidatorRight>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
    }
    pub mod Splice_Amulet_TokenApiUtils {
        use canton_daml as rt;
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize,
        )]
        #[serde(crate = "rt::serde")]
        pub enum TxKind {
            #[serde(rename = "TxKind_Transfer")]
            TxKind_Transfer,
            #[serde(rename = "TxKind_Unlock")]
            TxKind_Unlock,
            #[serde(rename = "TxKind_MergeSplit")]
            TxKind_MergeSplit,
            #[serde(rename = "TxKind_Burn")]
            TxKind_Burn,
            #[serde(rename = "TxKind_Mint")]
            TxKind_Mint,
            #[serde(rename = "TxKind_ExpireDust")]
            TxKind_ExpireDust,
        }
        impl rt::ToValue for TxKind {
            fn to_value(&self) -> rt::Value {
                rt::enum_value(match self {
                    TxKind::TxKind_Transfer => "TxKind_Transfer",
                    TxKind::TxKind_Unlock => "TxKind_Unlock",
                    TxKind::TxKind_MergeSplit => "TxKind_MergeSplit",
                    TxKind::TxKind_Burn => "TxKind_Burn",
                    TxKind::TxKind_Mint => "TxKind_Mint",
                    TxKind::TxKind_ExpireDust => "TxKind_ExpireDust",
                })
            }
        }
        impl rt::FromValue for TxKind {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                match rt::enum_constructor(value)? {
                    "TxKind_Transfer" => ::core::result::Result::Ok(TxKind::TxKind_Transfer),
                    "TxKind_Unlock" => ::core::result::Result::Ok(TxKind::TxKind_Unlock),
                    "TxKind_MergeSplit" => ::core::result::Result::Ok(TxKind::TxKind_MergeSplit),
                    "TxKind_Burn" => ::core::result::Result::Ok(TxKind::TxKind_Burn),
                    "TxKind_Mint" => ::core::result::Result::Ok(TxKind::TxKind_Mint),
                    "TxKind_ExpireDust" => ::core::result::Result::Ok(TxKind::TxKind_ExpireDust),
                    other => {
                        ::core::result::Result::Err(rt::unexpected_constructor("TxKind", other))
                    }
                }
            }
        }
    }
    pub mod Splice_AmuletTransferInstruction {
        use canton_daml as rt;
        ///The Daml template `Splice.AmuletTransferInstruction:AmuletTransferInstruction`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-amulet:Splice.AmuletTransferInstruction:AmuletTransferInstruction`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletTransferInstruction {
            ///Daml field `lockedAmulet`.
            #[serde(rename = "lockedAmulet")]
            pub locked_amulet: rt::ContractId<
                crate::splice_amulet::Splice_Amulet::LockedAmulet,
            >,
            #[serde(rename = "transfer")]
            pub transfer: ::canton_splice_api_token_transfer_instruction_v1::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::Transfer,
        }
        impl rt::ToValue for AmuletTransferInstruction {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("lockedAmulet", rt::ToValue::to_value(&self.locked_amulet)),
                    ("transfer", rt::ToValue::to_value(&self.transfer)),
                ])
            }
        }
        impl rt::FromValue for AmuletTransferInstruction {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    locked_amulet: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "lockedAmulet",
                    )?)
                    .map_err(|e| e.at("lockedAmulet"))?,
                    transfer: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "transfer",
                    )?)
                    .map_err(|e| e.at("transfer"))?,
                })
            }
        }
        impl rt::Contract for AmuletTransferInstruction {
            const PACKAGE_ID: &'static str =
                "3ca1343ab26b453d38c8adb70dca5f1ead8440c42b59b68f070786955cbf9ec1";
            const PACKAGE_NAME: &'static str = "splice-amulet";
            const MODULE_NAME: &'static str = "Splice.AmuletTransferInstruction";
            const ENTITY_NAME: &'static str = "AmuletTransferInstruction";
        }
        impl rt::Template for AmuletTransferInstruction {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("lockedAmulet", rt::ToValue::to_value(&self.locked_amulet)),
                    ("transfer", rt::ToValue::to_value(&self.transfer)),
                ])
            }
        }
        ///The `Archive` choice on [`AmuletTransferInstruction`] (consuming).
        impl rt::Choice<AmuletTransferInstruction>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
    }
    pub mod Splice_AmuletConfig {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct PackageConfig {
            #[serde(rename = "amulet")]
            pub amulet: ::std::string::String,
            ///Daml field `amuletNameService`.
            #[serde(rename = "amuletNameService")]
            pub amulet_name_service: ::std::string::String,
            ///Daml field `dsoGovernance`.
            #[serde(rename = "dsoGovernance")]
            pub dso_governance: ::std::string::String,
            ///Daml field `validatorLifecycle`.
            #[serde(rename = "validatorLifecycle")]
            pub validator_lifecycle: ::std::string::String,
            #[serde(rename = "wallet")]
            pub wallet: ::std::string::String,
            ///Daml field `walletPayments`.
            #[serde(rename = "walletPayments")]
            pub wallet_payments: ::std::string::String,
        }
        impl rt::ToValue for PackageConfig {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("amulet", rt::ToValue::to_value(&self.amulet)),
                    (
                        "amuletNameService",
                        rt::ToValue::to_value(&self.amulet_name_service)
                    ),
                    ("dsoGovernance", rt::ToValue::to_value(&self.dso_governance)),
                    (
                        "validatorLifecycle",
                        rt::ToValue::to_value(&self.validator_lifecycle)
                    ),
                    ("wallet", rt::ToValue::to_value(&self.wallet)),
                    (
                        "walletPayments",
                        rt::ToValue::to_value(&self.wallet_payments)
                    ),
                ])
            }
        }
        impl rt::FromValue for PackageConfig {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    amulet: rt::FromValue::from_value(rt::required_field(value, 0usize, "amulet")?)
                        .map_err(|e| e.at("amulet"))?,
                    amulet_name_service: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "amuletNameService",
                    )?)
                    .map_err(|e| e.at("amuletNameService"))?,
                    dso_governance: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "dsoGovernance",
                    )?)
                    .map_err(|e| e.at("dsoGovernance"))?,
                    validator_lifecycle: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "validatorLifecycle",
                    )?)
                    .map_err(|e| e.at("validatorLifecycle"))?,
                    wallet: rt::FromValue::from_value(rt::required_field(value, 4usize, "wallet")?)
                        .map_err(|e| e.at("wallet"))?,
                    wallet_payments: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "walletPayments",
                    )?)
                    .map_err(|e| e.at("walletPayments"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletConfig<Unit> {
            ///Daml field `transferConfig`.
            #[serde(rename = "transferConfig")]
            pub transfer_config: crate::splice_amulet::Splice_AmuletConfig::TransferConfig<
                Unit,
            >,
            ///Daml field `issuanceCurve`.
            #[serde(rename = "issuanceCurve")]
            pub issuance_curve: crate::splice_amulet::Splice_Schedule::Schedule<
                ::canton_daml_stdlib::daml_stdlib_DA_Time_Types::DA_Time_Types::RelTime,
                crate::splice_amulet::Splice_Issuance::IssuanceConfig,
            >,
            ///Daml field `decentralizedSynchronizer`.
            #[serde(rename = "decentralizedSynchronizer")]
            pub decentralized_synchronizer: crate::splice_amulet::Splice_DecentralizedSynchronizer::AmuletDecentralizedSynchronizerConfig,
            ///Daml field `tickDuration`.
            #[serde(rename = "tickDuration")]
            pub tick_duration: ::canton_daml_stdlib::daml_stdlib_DA_Time_Types::DA_Time_Types::RelTime,
            ///Daml field `packageConfig`.
            #[serde(rename = "packageConfig")]
            pub package_config: crate::splice_amulet::Splice_AmuletConfig::PackageConfig,
            ///Daml field `transferPreapprovalFee`.
            #[serde(rename = "transferPreapprovalFee")]
            pub transfer_preapproval_fee: ::core::option::Option<rt::Numeric>,
            ///Daml field `featuredAppActivityMarkerAmount`.
            #[serde(rename = "featuredAppActivityMarkerAmount")]
            pub featured_app_activity_marker_amount: ::core::option::Option<rt::Numeric>,
        }
        impl<Unit> rt::ToValue for AmuletConfig<Unit>
        where
            Unit: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "transferConfig",
                        rt::ToValue::to_value(&self.transfer_config)
                    ),
                    ("issuanceCurve", rt::ToValue::to_value(&self.issuance_curve)),
                    (
                        "decentralizedSynchronizer",
                        rt::ToValue::to_value(&self.decentralized_synchronizer)
                    ),
                    ("tickDuration", rt::ToValue::to_value(&self.tick_duration)),
                    ("packageConfig", rt::ToValue::to_value(&self.package_config)),
                    (
                        "transferPreapprovalFee",
                        rt::ToValue::to_value(&self.transfer_preapproval_fee)
                    ),
                    (
                        "featuredAppActivityMarkerAmount",
                        rt::ToValue::to_value(&self.featured_app_activity_marker_amount)
                    ),
                ])
            }
        }
        impl<Unit> rt::FromValue for AmuletConfig<Unit>
        where
            Unit: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    transfer_config: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "transferConfig",
                    )?)
                    .map_err(|e| e.at("transferConfig"))?,
                    issuance_curve: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "issuanceCurve",
                    )?)
                    .map_err(|e| e.at("issuanceCurve"))?,
                    decentralized_synchronizer: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "decentralizedSynchronizer",
                    )?)
                    .map_err(|e| e.at("decentralizedSynchronizer"))?,
                    tick_duration: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "tickDuration",
                    )?)
                    .map_err(|e| e.at("tickDuration"))?,
                    package_config: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "packageConfig",
                    )?)
                    .map_err(|e| e.at("packageConfig"))?,
                    transfer_preapproval_fee: rt::optional_field(
                        value,
                        5usize,
                        "transferPreapprovalFee",
                    )
                    .map_err(|e| e.at("transferPreapprovalFee"))?,
                    featured_app_activity_marker_amount: rt::optional_field(
                        value,
                        6usize,
                        "featuredAppActivityMarkerAmount",
                    )
                    .map_err(|e| e.at("featuredAppActivityMarkerAmount"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferConfig<Unit> {
            ///Daml field `createFee`.
            #[serde(rename = "createFee")]
            pub create_fee: crate::splice_amulet::Splice_Fees::FixedFee,
            ///Daml field `holdingFee`.
            #[serde(rename = "holdingFee")]
            pub holding_fee: crate::splice_amulet::Splice_Fees::RatePerRound,
            ///Daml field `transferFee`.
            #[serde(rename = "transferFee")]
            pub transfer_fee: crate::splice_amulet::Splice_Fees::SteppedRate,
            ///Daml field `lockHolderFee`.
            #[serde(rename = "lockHolderFee")]
            pub lock_holder_fee: crate::splice_amulet::Splice_Fees::FixedFee,
            ///Daml field `extraFeaturedAppRewardAmount`.
            #[serde(rename = "extraFeaturedAppRewardAmount")]
            pub extra_featured_app_reward_amount: rt::Numeric,
            ///Daml field `maxNumInputs`.
            #[serde(rename = "maxNumInputs")]
            pub max_num_inputs: rt::Int64,
            ///Daml field `maxNumOutputs`.
            #[serde(rename = "maxNumOutputs")]
            pub max_num_outputs: rt::Int64,
            ///Daml field `maxNumLockHolders`.
            #[serde(rename = "maxNumLockHolders")]
            pub max_num_lock_holders: rt::Int64,
            #[doc(hidden)]
            #[serde(skip)]
            pub _phantom: ::core::marker::PhantomData<(Unit,)>,
        }
        impl<Unit> rt::ToValue for TransferConfig<Unit> {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("createFee", rt::ToValue::to_value(&self.create_fee)),
                    ("holdingFee", rt::ToValue::to_value(&self.holding_fee)),
                    ("transferFee", rt::ToValue::to_value(&self.transfer_fee)),
                    (
                        "lockHolderFee",
                        rt::ToValue::to_value(&self.lock_holder_fee)
                    ),
                    (
                        "extraFeaturedAppRewardAmount",
                        rt::ToValue::to_value(&self.extra_featured_app_reward_amount)
                    ),
                    ("maxNumInputs", rt::ToValue::to_value(&self.max_num_inputs)),
                    (
                        "maxNumOutputs",
                        rt::ToValue::to_value(&self.max_num_outputs)
                    ),
                    (
                        "maxNumLockHolders",
                        rt::ToValue::to_value(&self.max_num_lock_holders)
                    ),
                ])
            }
        }
        impl<Unit> rt::FromValue for TransferConfig<Unit> {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    create_fee: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "createFee",
                    )?)
                    .map_err(|e| e.at("createFee"))?,
                    holding_fee: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "holdingFee",
                    )?)
                    .map_err(|e| e.at("holdingFee"))?,
                    transfer_fee: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "transferFee",
                    )?)
                    .map_err(|e| e.at("transferFee"))?,
                    lock_holder_fee: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "lockHolderFee",
                    )?)
                    .map_err(|e| e.at("lockHolderFee"))?,
                    extra_featured_app_reward_amount: rt::FromValue::from_value(
                        rt::required_field(value, 4usize, "extraFeaturedAppRewardAmount")?,
                    )
                    .map_err(|e| e.at("extraFeaturedAppRewardAmount"))?,
                    max_num_inputs: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "maxNumInputs",
                    )?)
                    .map_err(|e| e.at("maxNumInputs"))?,
                    max_num_outputs: rt::FromValue::from_value(rt::required_field(
                        value,
                        6usize,
                        "maxNumOutputs",
                    )?)
                    .map_err(|e| e.at("maxNumOutputs"))?,
                    max_num_lock_holders: rt::FromValue::from_value(rt::required_field(
                        value,
                        7usize,
                        "maxNumLockHolders",
                    )?)
                    .map_err(|e| e.at("maxNumLockHolders"))?,
                    _phantom: ::core::marker::PhantomData,
                })
            }
        }
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize,
        )]
        #[serde(crate = "rt::serde")]
        pub enum USD {
            #[serde(rename = "USD")]
            USD,
        }
        impl rt::ToValue for USD {
            fn to_value(&self) -> rt::Value {
                rt::enum_value(match self {
                    USD::USD => "USD",
                })
            }
        }
        impl rt::FromValue for USD {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                match rt::enum_constructor(value)? {
                    "USD" => ::core::result::Result::Ok(USD::USD),
                    other => ::core::result::Result::Err(rt::unexpected_constructor("USD", other)),
                }
            }
        }
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize,
        )]
        #[serde(crate = "rt::serde")]
        pub enum Amulet {
            #[serde(rename = "Amulet")]
            Amulet,
        }
        impl rt::ToValue for Amulet {
            fn to_value(&self) -> rt::Value {
                rt::enum_value(match self {
                    Amulet::Amulet => "Amulet",
                })
            }
        }
        impl rt::FromValue for Amulet {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                match rt::enum_constructor(value)? {
                    "Amulet" => ::core::result::Result::Ok(Amulet::Amulet),
                    other => {
                        ::core::result::Result::Err(rt::unexpected_constructor("Amulet", other))
                    }
                }
            }
        }
    }
    pub mod Splice_ExternalPartyAmuletRules {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferCommand_ExpireResult {
            #[serde(rename = "sender")]
            pub sender: rt::Party,
            #[serde(rename = "nonce")]
            pub nonce: rt::Int64,
        }
        impl rt::ToValue for TransferCommand_ExpireResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("sender", rt::ToValue::to_value(&self.sender)),
                    ("nonce", rt::ToValue::to_value(&self.nonce)),
                ])
            }
        }
        impl rt::FromValue for TransferCommand_ExpireResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    sender: rt::FromValue::from_value(rt::required_field(value, 0usize, "sender")?)
                        .map_err(|e| e.at("sender"))?,
                    nonce: rt::FromValue::from_value(rt::required_field(value, 1usize, "nonce")?)
                        .map_err(|e| e.at("nonce"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferCommand_WithdrawResult {
            #[serde(rename = "sender")]
            pub sender: rt::Party,
            #[serde(rename = "nonce")]
            pub nonce: rt::Int64,
        }
        impl rt::ToValue for TransferCommand_WithdrawResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("sender", rt::ToValue::to_value(&self.sender)),
                    ("nonce", rt::ToValue::to_value(&self.nonce)),
                ])
            }
        }
        impl rt::FromValue for TransferCommand_WithdrawResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    sender: rt::FromValue::from_value(rt::required_field(value, 0usize, "sender")?)
                        .map_err(|e| e.at("sender"))?,
                    nonce: rt::FromValue::from_value(rt::required_field(value, 1usize, "nonce")?)
                        .map_err(|e| e.at("nonce"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum TransferCommandResult {
            #[serde(rename = "TransferCommandResultFailure")]
            TransferCommandResultFailure(
                crate::splice_amulet::Splice_ExternalPartyAmuletRules::TransferCommandResult_TransferCommandResultFailure,
            ),
            #[serde(rename = "TransferCommandResultSuccess")]
            TransferCommandResultSuccess(
                crate::splice_amulet::Splice_ExternalPartyAmuletRules::TransferCommandResult_TransferCommandResultSuccess,
            ),
        }
        impl rt::ToValue for TransferCommandResult {
            fn to_value(&self) -> rt::Value {
                match self {
                    TransferCommandResult::TransferCommandResultFailure(inner) => {
                        rt::variant_value(
                            "TransferCommandResultFailure",
                            rt::ToValue::to_value(inner),
                        )
                    }
                    TransferCommandResult::TransferCommandResultSuccess(inner) => {
                        rt::variant_value(
                            "TransferCommandResultSuccess",
                            rt::ToValue::to_value(inner),
                        )
                    }
                }
            }
        }
        impl rt::FromValue for TransferCommandResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                let (constructor, payload) = rt::variant_parts(value)?;
                match constructor {
                    "TransferCommandResultFailure" => ::core::result::Result::Ok(
                        TransferCommandResult::TransferCommandResultFailure(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("TransferCommandResultFailure"))?,
                        ),
                    ),
                    "TransferCommandResultSuccess" => ::core::result::Result::Ok(
                        TransferCommandResult::TransferCommandResultSuccess(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("TransferCommandResultSuccess"))?,
                        ),
                    ),
                    other => ::core::result::Result::Err(rt::unexpected_constructor(
                        "TransferCommandResult",
                        other,
                    )),
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferCommandResult_TransferCommandResultFailure {
            #[serde(rename = "reason")]
            pub reason: crate::splice_amulet::Splice_AmuletRules::InvalidTransferReason,
        }
        impl rt::ToValue for TransferCommandResult_TransferCommandResultFailure {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("reason", rt::ToValue::to_value(&self.reason)),])
            }
        }
        impl rt::FromValue for TransferCommandResult_TransferCommandResultFailure {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    reason: rt::FromValue::from_value(rt::required_field(value, 0usize, "reason")?)
                        .map_err(|e| e.at("reason"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferCommandResult_TransferCommandResultSuccess {
            #[serde(rename = "result")]
            pub result: crate::splice_amulet::Splice_AmuletRules::TransferResult,
        }
        impl rt::ToValue for TransferCommandResult_TransferCommandResultSuccess {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("result", rt::ToValue::to_value(&self.result)),])
            }
        }
        impl rt::FromValue for TransferCommandResult_TransferCommandResultSuccess {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    result: rt::FromValue::from_value(rt::required_field(value, 0usize, "result")?)
                        .map_err(|e| e.at("result"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferCommand_SendResult {
            #[serde(rename = "result")]
            pub result:
                crate::splice_amulet::Splice_ExternalPartyAmuletRules::TransferCommandResult,
            #[serde(rename = "sender")]
            pub sender: rt::Party,
            #[serde(rename = "nonce")]
            pub nonce: rt::Int64,
        }
        impl rt::ToValue for TransferCommand_SendResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("result", rt::ToValue::to_value(&self.result)),
                    ("sender", rt::ToValue::to_value(&self.sender)),
                    ("nonce", rt::ToValue::to_value(&self.nonce)),
                ])
            }
        }
        impl rt::FromValue for TransferCommand_SendResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    result: rt::FromValue::from_value(rt::required_field(value, 0usize, "result")?)
                        .map_err(|e| e.at("result"))?,
                    sender: rt::FromValue::from_value(rt::required_field(value, 1usize, "sender")?)
                        .map_err(|e| e.at("sender"))?,
                    nonce: rt::FromValue::from_value(rt::required_field(value, 2usize, "nonce")?)
                        .map_err(|e| e.at("nonce"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferCommand_Expire {
            #[serde(rename = "p")]
            pub p: rt::Party,
        }
        impl rt::ToValue for TransferCommand_Expire {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("p", rt::ToValue::to_value(&self.p)),])
            }
        }
        impl rt::FromValue for TransferCommand_Expire {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    p: rt::FromValue::from_value(rt::required_field(value, 0usize, "p")?)
                        .map_err(|e| e.at("p"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferCommand_Withdraw {}
        impl rt::ToValue for TransferCommand_Withdraw {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for TransferCommand_Withdraw {
            fn from_value(_value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferCommand_Send {
            #[serde(rename = "context")]
            pub context: crate::splice_amulet::Splice_AmuletRules::PaymentTransferContext,
            #[serde(rename = "inputs")]
            pub inputs: ::std::vec::Vec<crate::splice_amulet::Splice_AmuletRules::TransferInput>,
            ///Daml field `transferPreapprovalCidO`.
            #[serde(rename = "transferPreapprovalCidO")]
            pub transfer_preapproval_cid_o: ::core::option::Option<
                rt::ContractId<crate::splice_amulet::Splice_AmuletRules::TransferPreapproval>,
            >,
            ///Daml field `transferCounterCid`.
            #[serde(rename = "transferCounterCid")]
            pub transfer_counter_cid: rt::ContractId<
                crate::splice_amulet::Splice_ExternalPartyAmuletRules::TransferCommandCounter,
            >,
        }
        impl rt::ToValue for TransferCommand_Send {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("context", rt::ToValue::to_value(&self.context)),
                    ("inputs", rt::ToValue::to_value(&self.inputs)),
                    (
                        "transferPreapprovalCidO",
                        rt::ToValue::to_value(&self.transfer_preapproval_cid_o)
                    ),
                    (
                        "transferCounterCid",
                        rt::ToValue::to_value(&self.transfer_counter_cid)
                    ),
                ])
            }
        }
        impl rt::FromValue for TransferCommand_Send {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    context: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "context",
                    )?)
                    .map_err(|e| e.at("context"))?,
                    inputs: rt::FromValue::from_value(rt::required_field(value, 1usize, "inputs")?)
                        .map_err(|e| e.at("inputs"))?,
                    transfer_preapproval_cid_o: rt::optional_field(
                        value,
                        2usize,
                        "transferPreapprovalCidO",
                    )
                    .map_err(|e| e.at("transferPreapprovalCidO"))?,
                    transfer_counter_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "transferCounterCid",
                    )?)
                    .map_err(|e| e.at("transferCounterCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ExternalPartyAmuletRules_CreateTransferCommandResult {
            ///Daml field `transferCommandCid`.
            #[serde(rename = "transferCommandCid")]
            pub transfer_command_cid: rt::ContractId<
                crate::splice_amulet::Splice_ExternalPartyAmuletRules::TransferCommand,
            >,
        }
        impl rt::ToValue for ExternalPartyAmuletRules_CreateTransferCommandResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "transferCommandCid",
                    rt::ToValue::to_value(&self.transfer_command_cid)
                ),])
            }
        }
        impl rt::FromValue for ExternalPartyAmuletRules_CreateTransferCommandResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    transfer_command_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "transferCommandCid",
                    )?)
                    .map_err(|e| e.at("transferCommandCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ExternalPartyAmuletRules_CreateTransferCommand {
            #[serde(rename = "sender")]
            pub sender: rt::Party,
            #[serde(rename = "receiver")]
            pub receiver: rt::Party,
            #[serde(rename = "delegate")]
            pub delegate: rt::Party,
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            ///Daml field `expiresAt`.
            #[serde(rename = "expiresAt")]
            pub expires_at: rt::Timestamp,
            #[serde(rename = "nonce")]
            pub nonce: rt::Int64,
            #[serde(rename = "description")]
            pub description: ::core::option::Option<::std::string::String>,
            ///Daml field `expectedDso`.
            #[serde(rename = "expectedDso")]
            pub expected_dso: ::core::option::Option<rt::Party>,
        }
        impl rt::ToValue for ExternalPartyAmuletRules_CreateTransferCommand {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("sender", rt::ToValue::to_value(&self.sender)),
                    ("receiver", rt::ToValue::to_value(&self.receiver)),
                    ("delegate", rt::ToValue::to_value(&self.delegate)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                    ("expiresAt", rt::ToValue::to_value(&self.expires_at)),
                    ("nonce", rt::ToValue::to_value(&self.nonce)),
                    ("description", rt::ToValue::to_value(&self.description)),
                    ("expectedDso", rt::ToValue::to_value(&self.expected_dso)),
                ])
            }
        }
        impl rt::FromValue for ExternalPartyAmuletRules_CreateTransferCommand {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    sender: rt::FromValue::from_value(rt::required_field(value, 0usize, "sender")?)
                        .map_err(|e| e.at("sender"))?,
                    receiver: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "receiver",
                    )?)
                    .map_err(|e| e.at("receiver"))?,
                    delegate: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "delegate",
                    )?)
                    .map_err(|e| e.at("delegate"))?,
                    amount: rt::FromValue::from_value(rt::required_field(value, 3usize, "amount")?)
                        .map_err(|e| e.at("amount"))?,
                    expires_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "expiresAt",
                    )?)
                    .map_err(|e| e.at("expiresAt"))?,
                    nonce: rt::FromValue::from_value(rt::required_field(value, 5usize, "nonce")?)
                        .map_err(|e| e.at("nonce"))?,
                    description: rt::optional_field(value, 6usize, "description")
                        .map_err(|e| e.at("description"))?,
                    expected_dso: rt::optional_field(value, 7usize, "expectedDso")
                        .map_err(|e| e.at("expectedDso"))?,
                })
            }
        }
        ///The Daml template `Splice.ExternalPartyAmuletRules:ExternalPartyAmuletRules`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-amulet:Splice.ExternalPartyAmuletRules:ExternalPartyAmuletRules`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `ExternalPartyAmuletRules_CreateTransferCommand` — non-consuming
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ExternalPartyAmuletRules {
            #[serde(rename = "dso")]
            pub dso: rt::Party,
        }
        impl rt::ToValue for ExternalPartyAmuletRules {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("dso", rt::ToValue::to_value(&self.dso)),])
            }
        }
        impl rt::FromValue for ExternalPartyAmuletRules {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                })
            }
        }
        impl rt::Contract for ExternalPartyAmuletRules {
            const PACKAGE_ID: &'static str =
                "3ca1343ab26b453d38c8adb70dca5f1ead8440c42b59b68f070786955cbf9ec1";
            const PACKAGE_NAME: &'static str = "splice-amulet";
            const MODULE_NAME: &'static str = "Splice.ExternalPartyAmuletRules";
            const ENTITY_NAME: &'static str = "ExternalPartyAmuletRules";
        }
        impl rt::Template for ExternalPartyAmuletRules {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![("dso", rt::ToValue::to_value(&self.dso)),])
            }
        }
        ///The `ExternalPartyAmuletRules_CreateTransferCommand` choice on [`ExternalPartyAmuletRules`] (non-consuming).
        impl rt::Choice<ExternalPartyAmuletRules>
        for crate::splice_amulet::Splice_ExternalPartyAmuletRules::ExternalPartyAmuletRules_CreateTransferCommand {
            type Return = crate::splice_amulet::Splice_ExternalPartyAmuletRules::ExternalPartyAmuletRules_CreateTransferCommandResult;
            const NAME: &'static str = "ExternalPartyAmuletRules_CreateTransferCommand";
            const CONSUMING: bool = false;
        }
        ///The `Archive` choice on [`ExternalPartyAmuletRules`] (consuming).
        impl rt::Choice<ExternalPartyAmuletRules>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The Daml template `Splice.ExternalPartyAmuletRules:TransferCommand`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-amulet:Splice.ExternalPartyAmuletRules:TransferCommand`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `TransferCommand_Expire` — consuming
        ///- `TransferCommand_Send` — consuming
        ///- `Archive` — consuming
        ///- `TransferCommand_Withdraw` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferCommand {
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            #[serde(rename = "sender")]
            pub sender: rt::Party,
            #[serde(rename = "receiver")]
            pub receiver: rt::Party,
            #[serde(rename = "delegate")]
            pub delegate: rt::Party,
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            ///Daml field `expiresAt`.
            #[serde(rename = "expiresAt")]
            pub expires_at: rt::Timestamp,
            #[serde(rename = "nonce")]
            pub nonce: rt::Int64,
            #[serde(rename = "description")]
            pub description: ::core::option::Option<::std::string::String>,
        }
        impl rt::ToValue for TransferCommand {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("sender", rt::ToValue::to_value(&self.sender)),
                    ("receiver", rt::ToValue::to_value(&self.receiver)),
                    ("delegate", rt::ToValue::to_value(&self.delegate)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                    ("expiresAt", rt::ToValue::to_value(&self.expires_at)),
                    ("nonce", rt::ToValue::to_value(&self.nonce)),
                    ("description", rt::ToValue::to_value(&self.description)),
                ])
            }
        }
        impl rt::FromValue for TransferCommand {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                    sender: rt::FromValue::from_value(rt::required_field(value, 1usize, "sender")?)
                        .map_err(|e| e.at("sender"))?,
                    receiver: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "receiver",
                    )?)
                    .map_err(|e| e.at("receiver"))?,
                    delegate: rt::FromValue::from_value(rt::required_field(
                        value, 3usize, "delegate",
                    )?)
                    .map_err(|e| e.at("delegate"))?,
                    amount: rt::FromValue::from_value(rt::required_field(value, 4usize, "amount")?)
                        .map_err(|e| e.at("amount"))?,
                    expires_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "expiresAt",
                    )?)
                    .map_err(|e| e.at("expiresAt"))?,
                    nonce: rt::FromValue::from_value(rt::required_field(value, 6usize, "nonce")?)
                        .map_err(|e| e.at("nonce"))?,
                    description: rt::optional_field(value, 7usize, "description")
                        .map_err(|e| e.at("description"))?,
                })
            }
        }
        impl rt::Contract for TransferCommand {
            const PACKAGE_ID: &'static str =
                "3ca1343ab26b453d38c8adb70dca5f1ead8440c42b59b68f070786955cbf9ec1";
            const PACKAGE_NAME: &'static str = "splice-amulet";
            const MODULE_NAME: &'static str = "Splice.ExternalPartyAmuletRules";
            const ENTITY_NAME: &'static str = "TransferCommand";
        }
        impl rt::Template for TransferCommand {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("sender", rt::ToValue::to_value(&self.sender)),
                    ("receiver", rt::ToValue::to_value(&self.receiver)),
                    ("delegate", rt::ToValue::to_value(&self.delegate)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                    ("expiresAt", rt::ToValue::to_value(&self.expires_at)),
                    ("nonce", rt::ToValue::to_value(&self.nonce)),
                    ("description", rt::ToValue::to_value(&self.description)),
                ])
            }
        }
        ///The `TransferCommand_Expire` choice on [`TransferCommand`] (consuming).
        impl rt::Choice<TransferCommand>
            for crate::splice_amulet::Splice_ExternalPartyAmuletRules::TransferCommand_Expire
        {
            type Return =
                crate::splice_amulet::Splice_ExternalPartyAmuletRules::TransferCommand_ExpireResult;
            const NAME: &'static str = "TransferCommand_Expire";
            const CONSUMING: bool = true;
        }
        ///The `TransferCommand_Send` choice on [`TransferCommand`] (consuming).
        impl rt::Choice<TransferCommand>
            for crate::splice_amulet::Splice_ExternalPartyAmuletRules::TransferCommand_Send
        {
            type Return =
                crate::splice_amulet::Splice_ExternalPartyAmuletRules::TransferCommand_SendResult;
            const NAME: &'static str = "TransferCommand_Send";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`TransferCommand`] (consuming).
        impl rt::Choice<TransferCommand>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `TransferCommand_Withdraw` choice on [`TransferCommand`] (consuming).
        impl rt::Choice<TransferCommand>
            for crate::splice_amulet::Splice_ExternalPartyAmuletRules::TransferCommand_Withdraw
        {
            type Return = crate::splice_amulet::Splice_ExternalPartyAmuletRules::TransferCommand_WithdrawResult;
            const NAME: &'static str = "TransferCommand_Withdraw";
            const CONSUMING: bool = true;
        }
        ///The Daml template `Splice.ExternalPartyAmuletRules:TransferCommandCounter`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-amulet:Splice.ExternalPartyAmuletRules:TransferCommandCounter`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferCommandCounter {
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            #[serde(rename = "sender")]
            pub sender: rt::Party,
            ///Daml field `nextNonce`.
            #[serde(rename = "nextNonce")]
            pub next_nonce: rt::Int64,
        }
        impl rt::ToValue for TransferCommandCounter {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("sender", rt::ToValue::to_value(&self.sender)),
                    ("nextNonce", rt::ToValue::to_value(&self.next_nonce)),
                ])
            }
        }
        impl rt::FromValue for TransferCommandCounter {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                    sender: rt::FromValue::from_value(rt::required_field(value, 1usize, "sender")?)
                        .map_err(|e| e.at("sender"))?,
                    next_nonce: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "nextNonce",
                    )?)
                    .map_err(|e| e.at("nextNonce"))?,
                })
            }
        }
        impl rt::Contract for TransferCommandCounter {
            const PACKAGE_ID: &'static str =
                "3ca1343ab26b453d38c8adb70dca5f1ead8440c42b59b68f070786955cbf9ec1";
            const PACKAGE_NAME: &'static str = "splice-amulet";
            const MODULE_NAME: &'static str = "Splice.ExternalPartyAmuletRules";
            const ENTITY_NAME: &'static str = "TransferCommandCounter";
        }
        impl rt::Template for TransferCommandCounter {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("sender", rt::ToValue::to_value(&self.sender)),
                    ("nextNonce", rt::ToValue::to_value(&self.next_nonce)),
                ])
            }
        }
        ///The `Archive` choice on [`TransferCommandCounter`] (consuming).
        impl rt::Choice<TransferCommandCounter>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
    }
    pub mod Splice_Schedule {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Schedule<T, A> {
            ///Daml field `initialValue`.
            #[serde(rename = "initialValue")]
            pub initial_value: A,
            ///Daml field `futureValues`.
            #[serde(rename = "futureValues")]
            pub future_values:
                ::std::vec::Vec<::canton_daml_stdlib::daml_prim_DA_Types::DA_Types::Tuple2<T, A>>,
        }
        impl<T, A> rt::ToValue for Schedule<T, A>
        where
            T: rt::ToValue,
            A: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("initialValue", rt::ToValue::to_value(&self.initial_value)),
                    ("futureValues", rt::ToValue::to_value(&self.future_values)),
                ])
            }
        }
        impl<T, A> rt::FromValue for Schedule<T, A>
        where
            T: rt::FromValue,
            A: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    initial_value: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "initialValue",
                    )?)
                    .map_err(|e| e.at("initialValue"))?,
                    future_values: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "futureValues",
                    )?)
                    .map_err(|e| e.at("futureValues"))?,
                })
            }
        }
    }
    pub mod Splice_Fees {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct RatePerDay {
            #[serde(rename = "rate")]
            pub rate: rt::Numeric,
        }
        impl rt::ToValue for RatePerDay {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("rate", rt::ToValue::to_value(&self.rate)),])
            }
        }
        impl rt::FromValue for RatePerDay {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    rate: rt::FromValue::from_value(rt::required_field(value, 0usize, "rate")?)
                        .map_err(|e| e.at("rate"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ExpiringAmount {
            ///Daml field `initialAmount`.
            #[serde(rename = "initialAmount")]
            pub initial_amount: rt::Numeric,
            ///Daml field `createdAt`.
            #[serde(rename = "createdAt")]
            pub created_at: crate::splice_amulet::Splice_Types::Round,
            ///Daml field `ratePerRound`.
            #[serde(rename = "ratePerRound")]
            pub rate_per_round: crate::splice_amulet::Splice_Fees::RatePerRound,
        }
        impl rt::ToValue for ExpiringAmount {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("initialAmount", rt::ToValue::to_value(&self.initial_amount)),
                    ("createdAt", rt::ToValue::to_value(&self.created_at)),
                    ("ratePerRound", rt::ToValue::to_value(&self.rate_per_round)),
                ])
            }
        }
        impl rt::FromValue for ExpiringAmount {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    initial_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "initialAmount",
                    )?)
                    .map_err(|e| e.at("initialAmount"))?,
                    created_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "createdAt",
                    )?)
                    .map_err(|e| e.at("createdAt"))?,
                    rate_per_round: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "ratePerRound",
                    )?)
                    .map_err(|e| e.at("ratePerRound"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SteppedRate {
            ///Daml field `initialRate`.
            #[serde(rename = "initialRate")]
            pub initial_rate: rt::Numeric,
            #[serde(rename = "steps")]
            pub steps: ::std::vec::Vec<
                ::canton_daml_stdlib::daml_prim_DA_Types::DA_Types::Tuple2<
                    rt::Numeric,
                    rt::Numeric,
                >,
            >,
        }
        impl rt::ToValue for SteppedRate {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("initialRate", rt::ToValue::to_value(&self.initial_rate)),
                    ("steps", rt::ToValue::to_value(&self.steps)),
                ])
            }
        }
        impl rt::FromValue for SteppedRate {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    initial_rate: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "initialRate",
                    )?)
                    .map_err(|e| e.at("initialRate"))?,
                    steps: rt::FromValue::from_value(rt::required_field(value, 1usize, "steps")?)
                        .map_err(|e| e.at("steps"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct FixedFee {
            #[serde(rename = "fee")]
            pub fee: rt::Numeric,
        }
        impl rt::ToValue for FixedFee {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("fee", rt::ToValue::to_value(&self.fee)),])
            }
        }
        impl rt::FromValue for FixedFee {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    fee: rt::FromValue::from_value(rt::required_field(value, 0usize, "fee")?)
                        .map_err(|e| e.at("fee"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct RatePerRound {
            #[serde(rename = "rate")]
            pub rate: rt::Numeric,
        }
        impl rt::ToValue for RatePerRound {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("rate", rt::ToValue::to_value(&self.rate)),])
            }
        }
        impl rt::FromValue for RatePerRound {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    rate: rt::FromValue::from_value(rt::required_field(value, 0usize, "rate")?)
                        .map_err(|e| e.at("rate"))?,
                })
            }
        }
    }
    pub mod Splice_Expiry {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TimeLock {
            #[serde(rename = "holders")]
            pub holders: ::std::vec::Vec<rt::Party>,
            ///Daml field `expiresAt`.
            #[serde(rename = "expiresAt")]
            pub expires_at: rt::Timestamp,
            ///Daml field `optContext`.
            #[serde(rename = "optContext")]
            pub opt_context: ::core::option::Option<::std::string::String>,
        }
        impl rt::ToValue for TimeLock {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("holders", rt::ToValue::to_value(&self.holders)),
                    ("expiresAt", rt::ToValue::to_value(&self.expires_at)),
                    ("optContext", rt::ToValue::to_value(&self.opt_context)),
                ])
            }
        }
        impl rt::FromValue for TimeLock {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    holders: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "holders",
                    )?)
                    .map_err(|e| e.at("holders"))?,
                    expires_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "expiresAt",
                    )?)
                    .map_err(|e| e.at("expiresAt"))?,
                    opt_context: rt::optional_field(value, 2usize, "optContext")
                        .map_err(|e| e.at("optContext"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum BoundedSet<A> {
            #[serde(rename = "Singleton")]
            Singleton(A),
            #[serde(rename = "AfterMaxBound")]
            AfterMaxBound(rt::Unit),
        }
        impl<A> rt::ToValue for BoundedSet<A>
        where
            A: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                match self {
                    BoundedSet::Singleton(inner) => {
                        rt::variant_value("Singleton", rt::ToValue::to_value(inner))
                    }
                    BoundedSet::AfterMaxBound(inner) => {
                        rt::variant_value("AfterMaxBound", rt::ToValue::to_value(inner))
                    }
                }
            }
        }
        impl<A> rt::FromValue for BoundedSet<A>
        where
            A: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                let (constructor, payload) = rt::variant_parts(value)?;
                match constructor {
                    "Singleton" => ::core::result::Result::Ok(BoundedSet::Singleton(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("Singleton"))?,
                    )),
                    "AfterMaxBound" => ::core::result::Result::Ok(BoundedSet::AfterMaxBound(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("AfterMaxBound"))?,
                    )),
                    other => {
                        ::core::result::Result::Err(rt::unexpected_constructor("BoundedSet", other))
                    }
                }
            }
        }
    }
    pub mod Splice_AmuletAllocation {
        use canton_daml as rt;
        ///The Daml template `Splice.AmuletAllocation:AmuletAllocation`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-amulet:Splice.AmuletAllocation:AmuletAllocation`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletAllocation {
            ///Daml field `lockedAmulet`.
            #[serde(rename = "lockedAmulet")]
            pub locked_amulet: rt::ContractId<
                crate::splice_amulet::Splice_Amulet::LockedAmulet,
            >,
            #[serde(rename = "allocation")]
            pub allocation: ::canton_splice_api_token_allocation_v1::splice_api_token_allocation_v1::Splice_Api_Token_AllocationV1::AllocationSpecification,
        }
        impl rt::ToValue for AmuletAllocation {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("lockedAmulet", rt::ToValue::to_value(&self.locked_amulet)),
                    ("allocation", rt::ToValue::to_value(&self.allocation)),
                ])
            }
        }
        impl rt::FromValue for AmuletAllocation {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    locked_amulet: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "lockedAmulet",
                    )?)
                    .map_err(|e| e.at("lockedAmulet"))?,
                    allocation: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "allocation",
                    )?)
                    .map_err(|e| e.at("allocation"))?,
                })
            }
        }
        impl rt::Contract for AmuletAllocation {
            const PACKAGE_ID: &'static str =
                "3ca1343ab26b453d38c8adb70dca5f1ead8440c42b59b68f070786955cbf9ec1";
            const PACKAGE_NAME: &'static str = "splice-amulet";
            const MODULE_NAME: &'static str = "Splice.AmuletAllocation";
            const ENTITY_NAME: &'static str = "AmuletAllocation";
        }
        impl rt::Template for AmuletAllocation {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("lockedAmulet", rt::ToValue::to_value(&self.locked_amulet)),
                    ("allocation", rt::ToValue::to_value(&self.allocation)),
                ])
            }
        }
        ///The `Archive` choice on [`AmuletAllocation`] (consuming).
        impl rt::Choice<AmuletAllocation>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
    }
    pub mod Splice_Issuance {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct IssuanceTranche {
            ///Daml field `rewardsToIssue`.
            #[serde(rename = "rewardsToIssue")]
            pub rewards_to_issue: rt::Numeric,
            ///Daml field `issuancePerCoupon`.
            #[serde(rename = "issuancePerCoupon")]
            pub issuance_per_coupon: rt::Numeric,
            ///Daml field `unclaimedRewards`.
            #[serde(rename = "unclaimedRewards")]
            pub unclaimed_rewards: rt::Numeric,
        }
        impl rt::ToValue for IssuanceTranche {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "rewardsToIssue",
                        rt::ToValue::to_value(&self.rewards_to_issue)
                    ),
                    (
                        "issuancePerCoupon",
                        rt::ToValue::to_value(&self.issuance_per_coupon)
                    ),
                    (
                        "unclaimedRewards",
                        rt::ToValue::to_value(&self.unclaimed_rewards)
                    ),
                ])
            }
        }
        impl rt::FromValue for IssuanceTranche {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    rewards_to_issue: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "rewardsToIssue",
                    )?)
                    .map_err(|e| e.at("rewardsToIssue"))?,
                    issuance_per_coupon: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "issuancePerCoupon",
                    )?)
                    .map_err(|e| e.at("issuancePerCoupon"))?,
                    unclaimed_rewards: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "unclaimedRewards",
                    )?)
                    .map_err(|e| e.at("unclaimedRewards"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct IssuingRoundParameters {
            ///Daml field `issuancePerValidatorRewardCoupon`.
            #[serde(rename = "issuancePerValidatorRewardCoupon")]
            pub issuance_per_validator_reward_coupon: rt::Numeric,
            ///Daml field `issuancePerFeaturedAppRewardCoupon`.
            #[serde(rename = "issuancePerFeaturedAppRewardCoupon")]
            pub issuance_per_featured_app_reward_coupon: rt::Numeric,
            ///Daml field `issuancePerUnfeaturedAppRewardCoupon`.
            #[serde(rename = "issuancePerUnfeaturedAppRewardCoupon")]
            pub issuance_per_unfeatured_app_reward_coupon: rt::Numeric,
            ///Daml field `issuancePerSvRewardCoupon`.
            #[serde(rename = "issuancePerSvRewardCoupon")]
            pub issuance_per_sv_reward_coupon: rt::Numeric,
            ///Daml field `unclaimedAppRewards`.
            #[serde(rename = "unclaimedAppRewards")]
            pub unclaimed_app_rewards: rt::Numeric,
            ///Daml field `unclaimedValidatorRewards`.
            #[serde(rename = "unclaimedValidatorRewards")]
            pub unclaimed_validator_rewards: rt::Numeric,
            ///Daml field `unclaimedSvRewards`.
            #[serde(rename = "unclaimedSvRewards")]
            pub unclaimed_sv_rewards: rt::Numeric,
            ///Daml field `issuancePerValidatorFaucetCoupon`.
            #[serde(rename = "issuancePerValidatorFaucetCoupon")]
            pub issuance_per_validator_faucet_coupon: rt::Numeric,
        }
        impl rt::ToValue for IssuingRoundParameters {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "issuancePerValidatorRewardCoupon",
                        rt::ToValue::to_value(&self.issuance_per_validator_reward_coupon)
                    ),
                    (
                        "issuancePerFeaturedAppRewardCoupon",
                        rt::ToValue::to_value(&self.issuance_per_featured_app_reward_coupon)
                    ),
                    (
                        "issuancePerUnfeaturedAppRewardCoupon",
                        rt::ToValue::to_value(&self.issuance_per_unfeatured_app_reward_coupon)
                    ),
                    (
                        "issuancePerSvRewardCoupon",
                        rt::ToValue::to_value(&self.issuance_per_sv_reward_coupon)
                    ),
                    (
                        "unclaimedAppRewards",
                        rt::ToValue::to_value(&self.unclaimed_app_rewards)
                    ),
                    (
                        "unclaimedValidatorRewards",
                        rt::ToValue::to_value(&self.unclaimed_validator_rewards)
                    ),
                    (
                        "unclaimedSvRewards",
                        rt::ToValue::to_value(&self.unclaimed_sv_rewards)
                    ),
                    (
                        "issuancePerValidatorFaucetCoupon",
                        rt::ToValue::to_value(&self.issuance_per_validator_faucet_coupon)
                    ),
                ])
            }
        }
        impl rt::FromValue for IssuingRoundParameters {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    issuance_per_validator_reward_coupon: rt::FromValue::from_value(
                        rt::required_field(value, 0usize, "issuancePerValidatorRewardCoupon")?,
                    )
                    .map_err(|e| e.at("issuancePerValidatorRewardCoupon"))?,
                    issuance_per_featured_app_reward_coupon: rt::FromValue::from_value(
                        rt::required_field(value, 1usize, "issuancePerFeaturedAppRewardCoupon")?,
                    )
                    .map_err(|e| e.at("issuancePerFeaturedAppRewardCoupon"))?,
                    issuance_per_unfeatured_app_reward_coupon: rt::FromValue::from_value(
                        rt::required_field(value, 2usize, "issuancePerUnfeaturedAppRewardCoupon")?,
                    )
                    .map_err(|e| e.at("issuancePerUnfeaturedAppRewardCoupon"))?,
                    issuance_per_sv_reward_coupon: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "issuancePerSvRewardCoupon",
                    )?)
                    .map_err(|e| e.at("issuancePerSvRewardCoupon"))?,
                    unclaimed_app_rewards: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "unclaimedAppRewards",
                    )?)
                    .map_err(|e| e.at("unclaimedAppRewards"))?,
                    unclaimed_validator_rewards: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "unclaimedValidatorRewards",
                    )?)
                    .map_err(|e| e.at("unclaimedValidatorRewards"))?,
                    unclaimed_sv_rewards: rt::FromValue::from_value(rt::required_field(
                        value,
                        6usize,
                        "unclaimedSvRewards",
                    )?)
                    .map_err(|e| e.at("unclaimedSvRewards"))?,
                    issuance_per_validator_faucet_coupon: rt::FromValue::from_value(
                        rt::required_field(value, 7usize, "issuancePerValidatorFaucetCoupon")?,
                    )
                    .map_err(|e| e.at("issuancePerValidatorFaucetCoupon"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct OpenMiningRoundSummary {
            ///Daml field `totalValidatorRewardCoupons`.
            #[serde(rename = "totalValidatorRewardCoupons")]
            pub total_validator_reward_coupons: rt::Numeric,
            ///Daml field `totalFeaturedAppRewardCoupons`.
            #[serde(rename = "totalFeaturedAppRewardCoupons")]
            pub total_featured_app_reward_coupons: rt::Numeric,
            ///Daml field `totalUnfeaturedAppRewardCoupons`.
            #[serde(rename = "totalUnfeaturedAppRewardCoupons")]
            pub total_unfeatured_app_reward_coupons: rt::Numeric,
            ///Daml field `totalSvRewardWeight`.
            #[serde(rename = "totalSvRewardWeight")]
            pub total_sv_reward_weight: rt::Int64,
            ///Daml field `optTotalValidatorFaucetCoupons`.
            #[serde(rename = "optTotalValidatorFaucetCoupons")]
            pub opt_total_validator_faucet_coupons: ::core::option::Option<rt::Int64>,
        }
        impl rt::ToValue for OpenMiningRoundSummary {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "totalValidatorRewardCoupons",
                        rt::ToValue::to_value(&self.total_validator_reward_coupons)
                    ),
                    (
                        "totalFeaturedAppRewardCoupons",
                        rt::ToValue::to_value(&self.total_featured_app_reward_coupons)
                    ),
                    (
                        "totalUnfeaturedAppRewardCoupons",
                        rt::ToValue::to_value(&self.total_unfeatured_app_reward_coupons)
                    ),
                    (
                        "totalSvRewardWeight",
                        rt::ToValue::to_value(&self.total_sv_reward_weight)
                    ),
                    (
                        "optTotalValidatorFaucetCoupons",
                        rt::ToValue::to_value(&self.opt_total_validator_faucet_coupons)
                    ),
                ])
            }
        }
        impl rt::FromValue for OpenMiningRoundSummary {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    total_validator_reward_coupons: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "totalValidatorRewardCoupons",
                    )?)
                    .map_err(|e| e.at("totalValidatorRewardCoupons"))?,
                    total_featured_app_reward_coupons: rt::FromValue::from_value(
                        rt::required_field(value, 1usize, "totalFeaturedAppRewardCoupons")?,
                    )
                    .map_err(|e| e.at("totalFeaturedAppRewardCoupons"))?,
                    total_unfeatured_app_reward_coupons: rt::FromValue::from_value(
                        rt::required_field(value, 2usize, "totalUnfeaturedAppRewardCoupons")?,
                    )
                    .map_err(|e| e.at("totalUnfeaturedAppRewardCoupons"))?,
                    total_sv_reward_weight: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "totalSvRewardWeight",
                    )?)
                    .map_err(|e| e.at("totalSvRewardWeight"))?,
                    opt_total_validator_faucet_coupons: rt::optional_field(
                        value,
                        4usize,
                        "optTotalValidatorFaucetCoupons",
                    )
                    .map_err(|e| e.at("optTotalValidatorFaucetCoupons"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct IssuanceConfig {
            ///Daml field `amuletToIssuePerYear`.
            #[serde(rename = "amuletToIssuePerYear")]
            pub amulet_to_issue_per_year: rt::Numeric,
            ///Daml field `validatorRewardPercentage`.
            #[serde(rename = "validatorRewardPercentage")]
            pub validator_reward_percentage: rt::Numeric,
            ///Daml field `appRewardPercentage`.
            #[serde(rename = "appRewardPercentage")]
            pub app_reward_percentage: rt::Numeric,
            ///Daml field `validatorRewardCap`.
            #[serde(rename = "validatorRewardCap")]
            pub validator_reward_cap: rt::Numeric,
            ///Daml field `featuredAppRewardCap`.
            #[serde(rename = "featuredAppRewardCap")]
            pub featured_app_reward_cap: rt::Numeric,
            ///Daml field `unfeaturedAppRewardCap`.
            #[serde(rename = "unfeaturedAppRewardCap")]
            pub unfeatured_app_reward_cap: rt::Numeric,
            ///Daml field `optValidatorFaucetCap`.
            #[serde(rename = "optValidatorFaucetCap")]
            pub opt_validator_faucet_cap: ::core::option::Option<rt::Numeric>,
        }
        impl rt::ToValue for IssuanceConfig {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "amuletToIssuePerYear",
                        rt::ToValue::to_value(&self.amulet_to_issue_per_year)
                    ),
                    (
                        "validatorRewardPercentage",
                        rt::ToValue::to_value(&self.validator_reward_percentage)
                    ),
                    (
                        "appRewardPercentage",
                        rt::ToValue::to_value(&self.app_reward_percentage)
                    ),
                    (
                        "validatorRewardCap",
                        rt::ToValue::to_value(&self.validator_reward_cap)
                    ),
                    (
                        "featuredAppRewardCap",
                        rt::ToValue::to_value(&self.featured_app_reward_cap)
                    ),
                    (
                        "unfeaturedAppRewardCap",
                        rt::ToValue::to_value(&self.unfeatured_app_reward_cap)
                    ),
                    (
                        "optValidatorFaucetCap",
                        rt::ToValue::to_value(&self.opt_validator_faucet_cap)
                    ),
                ])
            }
        }
        impl rt::FromValue for IssuanceConfig {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    amulet_to_issue_per_year: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "amuletToIssuePerYear",
                    )?)
                    .map_err(|e| e.at("amuletToIssuePerYear"))?,
                    validator_reward_percentage: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "validatorRewardPercentage",
                    )?)
                    .map_err(|e| e.at("validatorRewardPercentage"))?,
                    app_reward_percentage: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "appRewardPercentage",
                    )?)
                    .map_err(|e| e.at("appRewardPercentage"))?,
                    validator_reward_cap: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "validatorRewardCap",
                    )?)
                    .map_err(|e| e.at("validatorRewardCap"))?,
                    featured_app_reward_cap: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "featuredAppRewardCap",
                    )?)
                    .map_err(|e| e.at("featuredAppRewardCap"))?,
                    unfeatured_app_reward_cap: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "unfeaturedAppRewardCap",
                    )?)
                    .map_err(|e| e.at("unfeaturedAppRewardCap"))?,
                    opt_validator_faucet_cap: rt::optional_field(
                        value,
                        6usize,
                        "optValidatorFaucetCap",
                    )
                    .map_err(|e| e.at("optValidatorFaucetCap"))?,
                })
            }
        }
    }
    pub mod Splice_ValidatorLicense {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorLivenessActivityRecord_DsoExpire {
            ///Daml field `closedRoundCid`.
            #[serde(rename = "closedRoundCid")]
            pub closed_round_cid:
                rt::ContractId<crate::splice_amulet::Splice_Round::ClosedMiningRound>,
        }
        impl rt::ToValue for ValidatorLivenessActivityRecord_DsoExpire {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "closedRoundCid",
                    rt::ToValue::to_value(&self.closed_round_cid)
                ),])
            }
        }
        impl rt::FromValue for ValidatorLivenessActivityRecord_DsoExpire {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    closed_round_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "closedRoundCid",
                    )?)
                    .map_err(|e| e.at("closedRoundCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorFaucetCoupon_DsoExpire {
            ///Daml field `closedRoundCid`.
            #[serde(rename = "closedRoundCid")]
            pub closed_round_cid:
                rt::ContractId<crate::splice_amulet::Splice_Round::ClosedMiningRound>,
        }
        impl rt::ToValue for ValidatorFaucetCoupon_DsoExpire {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "closedRoundCid",
                    rt::ToValue::to_value(&self.closed_round_cid)
                ),])
            }
        }
        impl rt::FromValue for ValidatorFaucetCoupon_DsoExpire {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    closed_round_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "closedRoundCid",
                    )?)
                    .map_err(|e| e.at("closedRoundCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorLicense_ReportActive {}
        impl rt::ToValue for ValidatorLicense_ReportActive {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for ValidatorLicense_ReportActive {
            fn from_value(_value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorLicense_UpdateMetadata {
            #[serde(rename = "version")]
            pub version: ::std::string::String,
            ///Daml field `contactPoint`.
            #[serde(rename = "contactPoint")]
            pub contact_point: ::std::string::String,
        }
        impl rt::ToValue for ValidatorLicense_UpdateMetadata {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("version", rt::ToValue::to_value(&self.version)),
                    ("contactPoint", rt::ToValue::to_value(&self.contact_point)),
                ])
            }
        }
        impl rt::FromValue for ValidatorLicense_UpdateMetadata {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    version: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "version",
                    )?)
                    .map_err(|e| e.at("version"))?,
                    contact_point: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "contactPoint",
                    )?)
                    .map_err(|e| e.at("contactPoint"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorLicense_Cancel {
            #[serde(rename = "reason")]
            pub reason: ::std::string::String,
        }
        impl rt::ToValue for ValidatorLicense_Cancel {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("reason", rt::ToValue::to_value(&self.reason)),])
            }
        }
        impl rt::FromValue for ValidatorLicense_Cancel {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    reason: rt::FromValue::from_value(rt::required_field(value, 0usize, "reason")?)
                        .map_err(|e| e.at("reason"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorLicense_Withdraw {
            #[serde(rename = "reason")]
            pub reason: ::std::string::String,
        }
        impl rt::ToValue for ValidatorLicense_Withdraw {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("reason", rt::ToValue::to_value(&self.reason)),])
            }
        }
        impl rt::FromValue for ValidatorLicense_Withdraw {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    reason: rt::FromValue::from_value(rt::required_field(value, 0usize, "reason")?)
                        .map_err(|e| e.at("reason"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorLicense_RecordValidatorLivenessActivity {
            ///Daml field `openRoundCid`.
            #[serde(rename = "openRoundCid")]
            pub open_round_cid: rt::ContractId<crate::splice_amulet::Splice_Round::OpenMiningRound>,
        }
        impl rt::ToValue for ValidatorLicense_RecordValidatorLivenessActivity {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "openRoundCid",
                    rt::ToValue::to_value(&self.open_round_cid)
                ),])
            }
        }
        impl rt::FromValue for ValidatorLicense_RecordValidatorLivenessActivity {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    open_round_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "openRoundCid",
                    )?)
                    .map_err(|e| e.at("openRoundCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorLicense_ReceiveFaucetCoupon {
            ///Daml field `openRoundCid`.
            #[serde(rename = "openRoundCid")]
            pub open_round_cid: rt::ContractId<crate::splice_amulet::Splice_Round::OpenMiningRound>,
        }
        impl rt::ToValue for ValidatorLicense_ReceiveFaucetCoupon {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "openRoundCid",
                    rt::ToValue::to_value(&self.open_round_cid)
                ),])
            }
        }
        impl rt::FromValue for ValidatorLicense_ReceiveFaucetCoupon {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    open_round_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "openRoundCid",
                    )?)
                    .map_err(|e| e.at("openRoundCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorLicenseMetadata {
            ///Daml field `lastUpdatedAt`.
            #[serde(rename = "lastUpdatedAt")]
            pub last_updated_at: rt::Timestamp,
            #[serde(rename = "version")]
            pub version: ::std::string::String,
            ///Daml field `contactPoint`.
            #[serde(rename = "contactPoint")]
            pub contact_point: ::std::string::String,
        }
        impl rt::ToValue for ValidatorLicenseMetadata {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "lastUpdatedAt",
                        rt::ToValue::to_value(&self.last_updated_at)
                    ),
                    ("version", rt::ToValue::to_value(&self.version)),
                    ("contactPoint", rt::ToValue::to_value(&self.contact_point)),
                ])
            }
        }
        impl rt::FromValue for ValidatorLicenseMetadata {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    last_updated_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "lastUpdatedAt",
                    )?)
                    .map_err(|e| e.at("lastUpdatedAt"))?,
                    version: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "version",
                    )?)
                    .map_err(|e| e.at("version"))?,
                    contact_point: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "contactPoint",
                    )?)
                    .map_err(|e| e.at("contactPoint"))?,
                })
            }
        }
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize,
        )]
        #[serde(crate = "rt::serde")]
        pub enum ValidatorLivenessActivityRecord_DsoExpireResult {
            #[serde(rename = "ValidatorLivenessActivityRecord_DsoExpireResult")]
            ValidatorLivenessActivityRecord_DsoExpireResult,
        }
        impl rt::ToValue for ValidatorLivenessActivityRecord_DsoExpireResult {
            fn to_value(&self) -> rt::Value {
                rt::enum_value(
                    match self {
                        ValidatorLivenessActivityRecord_DsoExpireResult::ValidatorLivenessActivityRecord_DsoExpireResult => {
                            "ValidatorLivenessActivityRecord_DsoExpireResult"
                        }
                    },
                )
            }
        }
        impl rt::FromValue for ValidatorLivenessActivityRecord_DsoExpireResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                match rt::enum_constructor(value)? {
                    "ValidatorLivenessActivityRecord_DsoExpireResult" => {
                        ::core::result::Result::Ok(
                            ValidatorLivenessActivityRecord_DsoExpireResult::ValidatorLivenessActivityRecord_DsoExpireResult,
                        )
                    }
                    other => {
                        ::core::result::Result::Err(
                            rt::unexpected_constructor(
                                "ValidatorLivenessActivityRecord_DsoExpireResult",
                                other,
                            ),
                        )
                    }
                }
            }
        }
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize,
        )]
        #[serde(crate = "rt::serde")]
        pub enum ValidatorFaucetCoupon_DsoExpireResult {
            #[serde(rename = "ValidatorFaucetCoupon_DsoExpireResult")]
            ValidatorFaucetCoupon_DsoExpireResult,
        }
        impl rt::ToValue for ValidatorFaucetCoupon_DsoExpireResult {
            fn to_value(&self) -> rt::Value {
                rt::enum_value(
                    match self {
                        ValidatorFaucetCoupon_DsoExpireResult::ValidatorFaucetCoupon_DsoExpireResult => {
                            "ValidatorFaucetCoupon_DsoExpireResult"
                        }
                    },
                )
            }
        }
        impl rt::FromValue for ValidatorFaucetCoupon_DsoExpireResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                match rt::enum_constructor(value)? {
                    "ValidatorFaucetCoupon_DsoExpireResult" => {
                        ::core::result::Result::Ok(
                            ValidatorFaucetCoupon_DsoExpireResult::ValidatorFaucetCoupon_DsoExpireResult,
                        )
                    }
                    other => {
                        ::core::result::Result::Err(
                            rt::unexpected_constructor(
                                "ValidatorFaucetCoupon_DsoExpireResult",
                                other,
                            ),
                        )
                    }
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorLicense_ReportActiveResult {
            ///Daml field `licenseCid`.
            #[serde(rename = "licenseCid")]
            pub license_cid:
                rt::ContractId<crate::splice_amulet::Splice_ValidatorLicense::ValidatorLicense>,
        }
        impl rt::ToValue for ValidatorLicense_ReportActiveResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "licenseCid",
                    rt::ToValue::to_value(&self.license_cid)
                ),])
            }
        }
        impl rt::FromValue for ValidatorLicense_ReportActiveResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    license_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "licenseCid",
                    )?)
                    .map_err(|e| e.at("licenseCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorLicense_UpdateMetadataResult {
            ///Daml field `licenseCid`.
            #[serde(rename = "licenseCid")]
            pub license_cid:
                rt::ContractId<crate::splice_amulet::Splice_ValidatorLicense::ValidatorLicense>,
        }
        impl rt::ToValue for ValidatorLicense_UpdateMetadataResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "licenseCid",
                    rt::ToValue::to_value(&self.license_cid)
                ),])
            }
        }
        impl rt::FromValue for ValidatorLicense_UpdateMetadataResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    license_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "licenseCid",
                    )?)
                    .map_err(|e| e.at("licenseCid"))?,
                })
            }
        }
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize,
        )]
        #[serde(crate = "rt::serde")]
        pub enum ValidatorLicense_CancelResult {
            #[serde(rename = "ValidatorLicense_CancelResult")]
            ValidatorLicense_CancelResult,
        }
        impl rt::ToValue for ValidatorLicense_CancelResult {
            fn to_value(&self) -> rt::Value {
                rt::enum_value(match self {
                    ValidatorLicense_CancelResult::ValidatorLicense_CancelResult => {
                        "ValidatorLicense_CancelResult"
                    }
                })
            }
        }
        impl rt::FromValue for ValidatorLicense_CancelResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                match rt::enum_constructor(value)? {
                    "ValidatorLicense_CancelResult" => ::core::result::Result::Ok(
                        ValidatorLicense_CancelResult::ValidatorLicense_CancelResult,
                    ),
                    other => ::core::result::Result::Err(rt::unexpected_constructor(
                        "ValidatorLicense_CancelResult",
                        other,
                    )),
                }
            }
        }
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize,
        )]
        #[serde(crate = "rt::serde")]
        pub enum ValidatorLicense_WithdrawResult {
            #[serde(rename = "ValidatorLicense_WithdrawResult")]
            ValidatorLicense_WithdrawResult,
        }
        impl rt::ToValue for ValidatorLicense_WithdrawResult {
            fn to_value(&self) -> rt::Value {
                rt::enum_value(match self {
                    ValidatorLicense_WithdrawResult::ValidatorLicense_WithdrawResult => {
                        "ValidatorLicense_WithdrawResult"
                    }
                })
            }
        }
        impl rt::FromValue for ValidatorLicense_WithdrawResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                match rt::enum_constructor(value)? {
                    "ValidatorLicense_WithdrawResult" => ::core::result::Result::Ok(
                        ValidatorLicense_WithdrawResult::ValidatorLicense_WithdrawResult,
                    ),
                    other => ::core::result::Result::Err(rt::unexpected_constructor(
                        "ValidatorLicense_WithdrawResult",
                        other,
                    )),
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorLicense_RecordValidatorLivenessActivityResult {
            ///Daml field `licenseCid`.
            #[serde(rename = "licenseCid")]
            pub license_cid:
                rt::ContractId<crate::splice_amulet::Splice_ValidatorLicense::ValidatorLicense>,
            ///Daml field `couponCid`.
            #[serde(rename = "couponCid")]
            pub coupon_cid: rt::ContractId<
                crate::splice_amulet::Splice_ValidatorLicense::ValidatorLivenessActivityRecord,
            >,
        }
        impl rt::ToValue for ValidatorLicense_RecordValidatorLivenessActivityResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("licenseCid", rt::ToValue::to_value(&self.license_cid)),
                    ("couponCid", rt::ToValue::to_value(&self.coupon_cid)),
                ])
            }
        }
        impl rt::FromValue for ValidatorLicense_RecordValidatorLivenessActivityResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    license_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "licenseCid",
                    )?)
                    .map_err(|e| e.at("licenseCid"))?,
                    coupon_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "couponCid",
                    )?)
                    .map_err(|e| e.at("couponCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorLicense_ReceiveFaucetCouponResult {
            ///Daml field `licenseCid`.
            #[serde(rename = "licenseCid")]
            pub license_cid:
                rt::ContractId<crate::splice_amulet::Splice_ValidatorLicense::ValidatorLicense>,
            ///Daml field `couponCid`.
            #[serde(rename = "couponCid")]
            pub coupon_cid: rt::ContractId<
                crate::splice_amulet::Splice_ValidatorLicense::ValidatorFaucetCoupon,
            >,
        }
        impl rt::ToValue for ValidatorLicense_ReceiveFaucetCouponResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("licenseCid", rt::ToValue::to_value(&self.license_cid)),
                    ("couponCid", rt::ToValue::to_value(&self.coupon_cid)),
                ])
            }
        }
        impl rt::FromValue for ValidatorLicense_ReceiveFaucetCouponResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    license_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "licenseCid",
                    )?)
                    .map_err(|e| e.at("licenseCid"))?,
                    coupon_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "couponCid",
                    )?)
                    .map_err(|e| e.at("couponCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct FaucetState {
            ///Daml field `firstReceivedFor`.
            #[serde(rename = "firstReceivedFor")]
            pub first_received_for: crate::splice_amulet::Splice_Types::Round,
            ///Daml field `lastReceivedFor`.
            #[serde(rename = "lastReceivedFor")]
            pub last_received_for: crate::splice_amulet::Splice_Types::Round,
            ///Daml field `numCouponsMissed`.
            #[serde(rename = "numCouponsMissed")]
            pub num_coupons_missed: rt::Int64,
        }
        impl rt::ToValue for FaucetState {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "firstReceivedFor",
                        rt::ToValue::to_value(&self.first_received_for)
                    ),
                    (
                        "lastReceivedFor",
                        rt::ToValue::to_value(&self.last_received_for)
                    ),
                    (
                        "numCouponsMissed",
                        rt::ToValue::to_value(&self.num_coupons_missed)
                    ),
                ])
            }
        }
        impl rt::FromValue for FaucetState {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    first_received_for: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "firstReceivedFor",
                    )?)
                    .map_err(|e| e.at("firstReceivedFor"))?,
                    last_received_for: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "lastReceivedFor",
                    )?)
                    .map_err(|e| e.at("lastReceivedFor"))?,
                    num_coupons_missed: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "numCouponsMissed",
                    )?)
                    .map_err(|e| e.at("numCouponsMissed"))?,
                })
            }
        }
        ///The Daml template `Splice.ValidatorLicense:ValidatorFaucetCoupon`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-amulet:Splice.ValidatorLicense:ValidatorFaucetCoupon`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `ValidatorFaucetCoupon_DsoExpire` — consuming
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorFaucetCoupon {
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            #[serde(rename = "validator")]
            pub validator: rt::Party,
            #[serde(rename = "round")]
            pub round: crate::splice_amulet::Splice_Types::Round,
        }
        impl rt::ToValue for ValidatorFaucetCoupon {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("validator", rt::ToValue::to_value(&self.validator)),
                    ("round", rt::ToValue::to_value(&self.round)),
                ])
            }
        }
        impl rt::FromValue for ValidatorFaucetCoupon {
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
                    round: rt::FromValue::from_value(rt::required_field(value, 2usize, "round")?)
                        .map_err(|e| e.at("round"))?,
                })
            }
        }
        impl rt::Contract for ValidatorFaucetCoupon {
            const PACKAGE_ID: &'static str =
                "3ca1343ab26b453d38c8adb70dca5f1ead8440c42b59b68f070786955cbf9ec1";
            const PACKAGE_NAME: &'static str = "splice-amulet";
            const MODULE_NAME: &'static str = "Splice.ValidatorLicense";
            const ENTITY_NAME: &'static str = "ValidatorFaucetCoupon";
        }
        impl rt::Template for ValidatorFaucetCoupon {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("validator", rt::ToValue::to_value(&self.validator)),
                    ("round", rt::ToValue::to_value(&self.round)),
                ])
            }
        }
        ///The `ValidatorFaucetCoupon_DsoExpire` choice on [`ValidatorFaucetCoupon`] (consuming).
        impl rt::Choice<ValidatorFaucetCoupon>
            for crate::splice_amulet::Splice_ValidatorLicense::ValidatorFaucetCoupon_DsoExpire
        {
            type Return = crate::splice_amulet::Splice_ValidatorLicense::ValidatorFaucetCoupon_DsoExpireResult;
            const NAME: &'static str = "ValidatorFaucetCoupon_DsoExpire";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`ValidatorFaucetCoupon`] (consuming).
        impl rt::Choice<ValidatorFaucetCoupon>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The Daml template `Splice.ValidatorLicense:ValidatorLicense`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-amulet:Splice.ValidatorLicense:ValidatorLicense`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `ValidatorLicense_ReceiveFaucetCoupon` — consuming
        ///- `ValidatorLicense_RecordValidatorLivenessActivity` — consuming
        ///- `ValidatorLicense_Withdraw` — consuming
        ///- `ValidatorLicense_Cancel` — consuming
        ///- `ValidatorLicense_UpdateMetadata` — consuming
        ///- `ValidatorLicense_ReportActive` — consuming
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorLicense {
            #[serde(rename = "validator")]
            pub validator: rt::Party,
            #[serde(rename = "sponsor")]
            pub sponsor: rt::Party,
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///Daml field `faucetState`.
            #[serde(rename = "faucetState")]
            pub faucet_state:
                ::core::option::Option<crate::splice_amulet::Splice_ValidatorLicense::FaucetState>,
            #[serde(rename = "metadata")]
            pub metadata: ::core::option::Option<
                crate::splice_amulet::Splice_ValidatorLicense::ValidatorLicenseMetadata,
            >,
            ///Daml field `lastActiveAt`.
            #[serde(rename = "lastActiveAt")]
            pub last_active_at: ::core::option::Option<rt::Timestamp>,
        }
        impl rt::ToValue for ValidatorLicense {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("validator", rt::ToValue::to_value(&self.validator)),
                    ("sponsor", rt::ToValue::to_value(&self.sponsor)),
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("faucetState", rt::ToValue::to_value(&self.faucet_state)),
                    ("metadata", rt::ToValue::to_value(&self.metadata)),
                    ("lastActiveAt", rt::ToValue::to_value(&self.last_active_at)),
                ])
            }
        }
        impl rt::FromValue for ValidatorLicense {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    validator: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "validator",
                    )?)
                    .map_err(|e| e.at("validator"))?,
                    sponsor: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "sponsor",
                    )?)
                    .map_err(|e| e.at("sponsor"))?,
                    dso: rt::FromValue::from_value(rt::required_field(value, 2usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                    faucet_state: rt::optional_field(value, 3usize, "faucetState")
                        .map_err(|e| e.at("faucetState"))?,
                    metadata: rt::optional_field(value, 4usize, "metadata")
                        .map_err(|e| e.at("metadata"))?,
                    last_active_at: rt::optional_field(value, 5usize, "lastActiveAt")
                        .map_err(|e| e.at("lastActiveAt"))?,
                })
            }
        }
        impl rt::Contract for ValidatorLicense {
            const PACKAGE_ID: &'static str =
                "3ca1343ab26b453d38c8adb70dca5f1ead8440c42b59b68f070786955cbf9ec1";
            const PACKAGE_NAME: &'static str = "splice-amulet";
            const MODULE_NAME: &'static str = "Splice.ValidatorLicense";
            const ENTITY_NAME: &'static str = "ValidatorLicense";
        }
        impl rt::Template for ValidatorLicense {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("validator", rt::ToValue::to_value(&self.validator)),
                    ("sponsor", rt::ToValue::to_value(&self.sponsor)),
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("faucetState", rt::ToValue::to_value(&self.faucet_state)),
                    ("metadata", rt::ToValue::to_value(&self.metadata)),
                    ("lastActiveAt", rt::ToValue::to_value(&self.last_active_at)),
                ])
            }
        }
        ///The `ValidatorLicense_ReceiveFaucetCoupon` choice on [`ValidatorLicense`] (consuming).
        impl rt::Choice<ValidatorLicense>
            for crate::splice_amulet::Splice_ValidatorLicense::ValidatorLicense_ReceiveFaucetCoupon
        {
            type Return = crate::splice_amulet::Splice_ValidatorLicense::ValidatorLicense_ReceiveFaucetCouponResult;
            const NAME: &'static str = "ValidatorLicense_ReceiveFaucetCoupon";
            const CONSUMING: bool = true;
        }
        ///The `ValidatorLicense_RecordValidatorLivenessActivity` choice on [`ValidatorLicense`] (consuming).
        impl rt::Choice<ValidatorLicense>
        for crate::splice_amulet::Splice_ValidatorLicense::ValidatorLicense_RecordValidatorLivenessActivity {
            type Return = crate::splice_amulet::Splice_ValidatorLicense::ValidatorLicense_RecordValidatorLivenessActivityResult;
            const NAME: &'static str = "ValidatorLicense_RecordValidatorLivenessActivity";
            const CONSUMING: bool = true;
        }
        ///The `ValidatorLicense_Withdraw` choice on [`ValidatorLicense`] (consuming).
        impl rt::Choice<ValidatorLicense>
            for crate::splice_amulet::Splice_ValidatorLicense::ValidatorLicense_Withdraw
        {
            type Return =
                crate::splice_amulet::Splice_ValidatorLicense::ValidatorLicense_WithdrawResult;
            const NAME: &'static str = "ValidatorLicense_Withdraw";
            const CONSUMING: bool = true;
        }
        ///The `ValidatorLicense_Cancel` choice on [`ValidatorLicense`] (consuming).
        impl rt::Choice<ValidatorLicense>
            for crate::splice_amulet::Splice_ValidatorLicense::ValidatorLicense_Cancel
        {
            type Return =
                crate::splice_amulet::Splice_ValidatorLicense::ValidatorLicense_CancelResult;
            const NAME: &'static str = "ValidatorLicense_Cancel";
            const CONSUMING: bool = true;
        }
        ///The `ValidatorLicense_UpdateMetadata` choice on [`ValidatorLicense`] (consuming).
        impl rt::Choice<ValidatorLicense>
            for crate::splice_amulet::Splice_ValidatorLicense::ValidatorLicense_UpdateMetadata
        {
            type Return = crate::splice_amulet::Splice_ValidatorLicense::ValidatorLicense_UpdateMetadataResult;
            const NAME: &'static str = "ValidatorLicense_UpdateMetadata";
            const CONSUMING: bool = true;
        }
        ///The `ValidatorLicense_ReportActive` choice on [`ValidatorLicense`] (consuming).
        impl rt::Choice<ValidatorLicense>
            for crate::splice_amulet::Splice_ValidatorLicense::ValidatorLicense_ReportActive
        {
            type Return =
                crate::splice_amulet::Splice_ValidatorLicense::ValidatorLicense_ReportActiveResult;
            const NAME: &'static str = "ValidatorLicense_ReportActive";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`ValidatorLicense`] (consuming).
        impl rt::Choice<ValidatorLicense>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The Daml template `Splice.ValidatorLicense:ValidatorLivenessActivityRecord`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-amulet:Splice.ValidatorLicense:ValidatorLivenessActivityRecord`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `Archive` — consuming
        ///- `ValidatorLivenessActivityRecord_DsoExpire` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorLivenessActivityRecord {
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            #[serde(rename = "validator")]
            pub validator: rt::Party,
            #[serde(rename = "round")]
            pub round: crate::splice_amulet::Splice_Types::Round,
        }
        impl rt::ToValue for ValidatorLivenessActivityRecord {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("validator", rt::ToValue::to_value(&self.validator)),
                    ("round", rt::ToValue::to_value(&self.round)),
                ])
            }
        }
        impl rt::FromValue for ValidatorLivenessActivityRecord {
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
                    round: rt::FromValue::from_value(rt::required_field(value, 2usize, "round")?)
                        .map_err(|e| e.at("round"))?,
                })
            }
        }
        impl rt::Contract for ValidatorLivenessActivityRecord {
            const PACKAGE_ID: &'static str =
                "3ca1343ab26b453d38c8adb70dca5f1ead8440c42b59b68f070786955cbf9ec1";
            const PACKAGE_NAME: &'static str = "splice-amulet";
            const MODULE_NAME: &'static str = "Splice.ValidatorLicense";
            const ENTITY_NAME: &'static str = "ValidatorLivenessActivityRecord";
        }
        impl rt::Template for ValidatorLivenessActivityRecord {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("validator", rt::ToValue::to_value(&self.validator)),
                    ("round", rt::ToValue::to_value(&self.round)),
                ])
            }
        }
        ///The `Archive` choice on [`ValidatorLivenessActivityRecord`] (consuming).
        impl rt::Choice<ValidatorLivenessActivityRecord>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `ValidatorLivenessActivityRecord_DsoExpire` choice on [`ValidatorLivenessActivityRecord`] (consuming).
        impl rt::Choice<ValidatorLivenessActivityRecord>
        for crate::splice_amulet::Splice_ValidatorLicense::ValidatorLivenessActivityRecord_DsoExpire {
            type Return = crate::splice_amulet::Splice_ValidatorLicense::ValidatorLivenessActivityRecord_DsoExpireResult;
            const NAME: &'static str = "ValidatorLivenessActivityRecord_DsoExpire";
            const CONSUMING: bool = true;
        }
    }
    pub mod Splice_DecentralizedSynchronizer {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ForMemberTraffic {
            #[serde(rename = "dso")]
            pub dso: rt::Party,
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
        impl rt::ToValue for ForMemberTraffic {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("memberId", rt::ToValue::to_value(&self.member_id)),
                    (
                        "synchronizerId",
                        rt::ToValue::to_value(&self.synchronizer_id)
                    ),
                    ("migrationId", rt::ToValue::to_value(&self.migration_id)),
                ])
            }
        }
        impl rt::FromValue for ForMemberTraffic {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
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
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SynchronizerFeesConfig {
            ///Daml field `baseRateTrafficLimits`.
            #[serde(rename = "baseRateTrafficLimits")]
            pub base_rate_traffic_limits:
                crate::splice_amulet::Splice_DecentralizedSynchronizer::BaseRateTrafficLimits,
            ///Daml field `extraTrafficPrice`.
            #[serde(rename = "extraTrafficPrice")]
            pub extra_traffic_price: rt::Numeric,
            ///Daml field `readVsWriteScalingFactor`.
            #[serde(rename = "readVsWriteScalingFactor")]
            pub read_vs_write_scaling_factor: rt::Int64,
            ///Daml field `minTopupAmount`.
            #[serde(rename = "minTopupAmount")]
            pub min_topup_amount: rt::Int64,
        }
        impl rt::ToValue for SynchronizerFeesConfig {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "baseRateTrafficLimits",
                        rt::ToValue::to_value(&self.base_rate_traffic_limits)
                    ),
                    (
                        "extraTrafficPrice",
                        rt::ToValue::to_value(&self.extra_traffic_price)
                    ),
                    (
                        "readVsWriteScalingFactor",
                        rt::ToValue::to_value(&self.read_vs_write_scaling_factor)
                    ),
                    (
                        "minTopupAmount",
                        rt::ToValue::to_value(&self.min_topup_amount)
                    ),
                ])
            }
        }
        impl rt::FromValue for SynchronizerFeesConfig {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    base_rate_traffic_limits: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "baseRateTrafficLimits",
                    )?)
                    .map_err(|e| e.at("baseRateTrafficLimits"))?,
                    extra_traffic_price: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "extraTrafficPrice",
                    )?)
                    .map_err(|e| e.at("extraTrafficPrice"))?,
                    read_vs_write_scaling_factor: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "readVsWriteScalingFactor",
                    )?)
                    .map_err(|e| e.at("readVsWriteScalingFactor"))?,
                    min_topup_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "minTopupAmount",
                    )?)
                    .map_err(|e| e.at("minTopupAmount"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct BaseRateTrafficLimits {
            ///Daml field `burstAmount`.
            #[serde(rename = "burstAmount")]
            pub burst_amount: rt::Int64,
            ///Daml field `burstWindow`.
            #[serde(rename = "burstWindow")]
            pub burst_window:
                ::canton_daml_stdlib::daml_stdlib_DA_Time_Types::DA_Time_Types::RelTime,
        }
        impl rt::ToValue for BaseRateTrafficLimits {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("burstAmount", rt::ToValue::to_value(&self.burst_amount)),
                    ("burstWindow", rt::ToValue::to_value(&self.burst_window)),
                ])
            }
        }
        impl rt::FromValue for BaseRateTrafficLimits {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    burst_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "burstAmount",
                    )?)
                    .map_err(|e| e.at("burstAmount"))?,
                    burst_window: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "burstWindow",
                    )?)
                    .map_err(|e| e.at("burstWindow"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletDecentralizedSynchronizerConfig {
            ///Daml field `requiredSynchronizers`.
            #[serde(rename = "requiredSynchronizers")]
            pub required_synchronizers:
                ::canton_daml_stdlib::daml_stdlib_DA_Set_Types::DA_Set_Types::Set<
                    ::std::string::String,
                >,
            ///Daml field `activeSynchronizer`.
            #[serde(rename = "activeSynchronizer")]
            pub active_synchronizer: ::std::string::String,
            #[serde(rename = "fees")]
            pub fees:
                crate::splice_amulet::Splice_DecentralizedSynchronizer::SynchronizerFeesConfig,
        }
        impl rt::ToValue for AmuletDecentralizedSynchronizerConfig {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "requiredSynchronizers",
                        rt::ToValue::to_value(&self.required_synchronizers)
                    ),
                    (
                        "activeSynchronizer",
                        rt::ToValue::to_value(&self.active_synchronizer)
                    ),
                    ("fees", rt::ToValue::to_value(&self.fees)),
                ])
            }
        }
        impl rt::FromValue for AmuletDecentralizedSynchronizerConfig {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    required_synchronizers: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "requiredSynchronizers",
                    )?)
                    .map_err(|e| e.at("requiredSynchronizers"))?,
                    active_synchronizer: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "activeSynchronizer",
                    )?)
                    .map_err(|e| e.at("activeSynchronizer"))?,
                    fees: rt::FromValue::from_value(rt::required_field(value, 2usize, "fees")?)
                        .map_err(|e| e.at("fees"))?,
                })
            }
        }
        ///The Daml template `Splice.DecentralizedSynchronizer:MemberTraffic`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-amulet:Splice.DecentralizedSynchronizer:MemberTraffic`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct MemberTraffic {
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///Daml field `memberId`.
            #[serde(rename = "memberId")]
            pub member_id: ::std::string::String,
            ///Daml field `synchronizerId`.
            #[serde(rename = "synchronizerId")]
            pub synchronizer_id: ::std::string::String,
            ///Daml field `migrationId`.
            #[serde(rename = "migrationId")]
            pub migration_id: rt::Int64,
            ///Daml field `totalPurchased`.
            #[serde(rename = "totalPurchased")]
            pub total_purchased: rt::Int64,
            ///Daml field `numPurchases`.
            #[serde(rename = "numPurchases")]
            pub num_purchases: rt::Int64,
            ///Daml field `amuletSpent`.
            #[serde(rename = "amuletSpent")]
            pub amulet_spent: rt::Numeric,
            ///Daml field `usdSpent`.
            #[serde(rename = "usdSpent")]
            pub usd_spent: rt::Numeric,
        }
        impl rt::ToValue for MemberTraffic {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("memberId", rt::ToValue::to_value(&self.member_id)),
                    (
                        "synchronizerId",
                        rt::ToValue::to_value(&self.synchronizer_id)
                    ),
                    ("migrationId", rt::ToValue::to_value(&self.migration_id)),
                    (
                        "totalPurchased",
                        rt::ToValue::to_value(&self.total_purchased)
                    ),
                    ("numPurchases", rt::ToValue::to_value(&self.num_purchases)),
                    ("amuletSpent", rt::ToValue::to_value(&self.amulet_spent)),
                    ("usdSpent", rt::ToValue::to_value(&self.usd_spent)),
                ])
            }
        }
        impl rt::FromValue for MemberTraffic {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
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
                    total_purchased: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "totalPurchased",
                    )?)
                    .map_err(|e| e.at("totalPurchased"))?,
                    num_purchases: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "numPurchases",
                    )?)
                    .map_err(|e| e.at("numPurchases"))?,
                    amulet_spent: rt::FromValue::from_value(rt::required_field(
                        value,
                        6usize,
                        "amuletSpent",
                    )?)
                    .map_err(|e| e.at("amuletSpent"))?,
                    usd_spent: rt::FromValue::from_value(rt::required_field(
                        value, 7usize, "usdSpent",
                    )?)
                    .map_err(|e| e.at("usdSpent"))?,
                })
            }
        }
        impl rt::Contract for MemberTraffic {
            const PACKAGE_ID: &'static str =
                "3ca1343ab26b453d38c8adb70dca5f1ead8440c42b59b68f070786955cbf9ec1";
            const PACKAGE_NAME: &'static str = "splice-amulet";
            const MODULE_NAME: &'static str = "Splice.DecentralizedSynchronizer";
            const ENTITY_NAME: &'static str = "MemberTraffic";
        }
        impl rt::Template for MemberTraffic {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("memberId", rt::ToValue::to_value(&self.member_id)),
                    (
                        "synchronizerId",
                        rt::ToValue::to_value(&self.synchronizer_id)
                    ),
                    ("migrationId", rt::ToValue::to_value(&self.migration_id)),
                    (
                        "totalPurchased",
                        rt::ToValue::to_value(&self.total_purchased)
                    ),
                    ("numPurchases", rt::ToValue::to_value(&self.num_purchases)),
                    ("amuletSpent", rt::ToValue::to_value(&self.amulet_spent)),
                    ("usdSpent", rt::ToValue::to_value(&self.usd_spent)),
                ])
            }
        }
        ///The `Archive` choice on [`MemberTraffic`] (consuming).
        impl rt::Choice<MemberTraffic>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
    }
    pub mod Splice_RelRound {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct RelRound {
            #[serde(rename = "diff")]
            pub diff: rt::Int64,
        }
        impl rt::ToValue for RelRound {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("diff", rt::ToValue::to_value(&self.diff)),])
            }
        }
        impl rt::FromValue for RelRound {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    diff: rt::FromValue::from_value(rt::required_field(value, 0usize, "diff")?)
                        .map_err(|e| e.at("diff"))?,
                })
            }
        }
    }
    pub mod Splice_Round {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct OpenMiningRound_Fetch {
            #[serde(rename = "p")]
            pub p: rt::Party,
        }
        impl rt::ToValue for OpenMiningRound_Fetch {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("p", rt::ToValue::to_value(&self.p)),])
            }
        }
        impl rt::FromValue for OpenMiningRound_Fetch {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    p: rt::FromValue::from_value(rt::required_field(value, 0usize, "p")?)
                        .map_err(|e| e.at("p"))?,
                })
            }
        }
        ///The Daml template `Splice.Round:ClosedMiningRound`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-amulet:Splice.Round:ClosedMiningRound`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ClosedMiningRound {
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            #[serde(rename = "round")]
            pub round: crate::splice_amulet::Splice_Types::Round,
            ///Daml field `issuancePerValidatorRewardCoupon`.
            #[serde(rename = "issuancePerValidatorRewardCoupon")]
            pub issuance_per_validator_reward_coupon: rt::Numeric,
            ///Daml field `issuancePerFeaturedAppRewardCoupon`.
            #[serde(rename = "issuancePerFeaturedAppRewardCoupon")]
            pub issuance_per_featured_app_reward_coupon: rt::Numeric,
            ///Daml field `issuancePerUnfeaturedAppRewardCoupon`.
            #[serde(rename = "issuancePerUnfeaturedAppRewardCoupon")]
            pub issuance_per_unfeatured_app_reward_coupon: rt::Numeric,
            ///Daml field `issuancePerSvRewardCoupon`.
            #[serde(rename = "issuancePerSvRewardCoupon")]
            pub issuance_per_sv_reward_coupon: rt::Numeric,
            ///Daml field `optIssuancePerValidatorFaucetCoupon`.
            #[serde(rename = "optIssuancePerValidatorFaucetCoupon")]
            pub opt_issuance_per_validator_faucet_coupon: ::core::option::Option<rt::Numeric>,
        }
        impl rt::ToValue for ClosedMiningRound {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("round", rt::ToValue::to_value(&self.round)),
                    (
                        "issuancePerValidatorRewardCoupon",
                        rt::ToValue::to_value(&self.issuance_per_validator_reward_coupon)
                    ),
                    (
                        "issuancePerFeaturedAppRewardCoupon",
                        rt::ToValue::to_value(&self.issuance_per_featured_app_reward_coupon)
                    ),
                    (
                        "issuancePerUnfeaturedAppRewardCoupon",
                        rt::ToValue::to_value(&self.issuance_per_unfeatured_app_reward_coupon)
                    ),
                    (
                        "issuancePerSvRewardCoupon",
                        rt::ToValue::to_value(&self.issuance_per_sv_reward_coupon)
                    ),
                    (
                        "optIssuancePerValidatorFaucetCoupon",
                        rt::ToValue::to_value(&self.opt_issuance_per_validator_faucet_coupon)
                    ),
                ])
            }
        }
        impl rt::FromValue for ClosedMiningRound {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                    round: rt::FromValue::from_value(rt::required_field(value, 1usize, "round")?)
                        .map_err(|e| e.at("round"))?,
                    issuance_per_validator_reward_coupon: rt::FromValue::from_value(
                        rt::required_field(value, 2usize, "issuancePerValidatorRewardCoupon")?,
                    )
                    .map_err(|e| e.at("issuancePerValidatorRewardCoupon"))?,
                    issuance_per_featured_app_reward_coupon: rt::FromValue::from_value(
                        rt::required_field(value, 3usize, "issuancePerFeaturedAppRewardCoupon")?,
                    )
                    .map_err(|e| e.at("issuancePerFeaturedAppRewardCoupon"))?,
                    issuance_per_unfeatured_app_reward_coupon: rt::FromValue::from_value(
                        rt::required_field(value, 4usize, "issuancePerUnfeaturedAppRewardCoupon")?,
                    )
                    .map_err(|e| e.at("issuancePerUnfeaturedAppRewardCoupon"))?,
                    issuance_per_sv_reward_coupon: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "issuancePerSvRewardCoupon",
                    )?)
                    .map_err(|e| e.at("issuancePerSvRewardCoupon"))?,
                    opt_issuance_per_validator_faucet_coupon: rt::optional_field(
                        value,
                        6usize,
                        "optIssuancePerValidatorFaucetCoupon",
                    )
                    .map_err(|e| e.at("optIssuancePerValidatorFaucetCoupon"))?,
                })
            }
        }
        impl rt::Contract for ClosedMiningRound {
            const PACKAGE_ID: &'static str =
                "3ca1343ab26b453d38c8adb70dca5f1ead8440c42b59b68f070786955cbf9ec1";
            const PACKAGE_NAME: &'static str = "splice-amulet";
            const MODULE_NAME: &'static str = "Splice.Round";
            const ENTITY_NAME: &'static str = "ClosedMiningRound";
        }
        impl rt::Template for ClosedMiningRound {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("round", rt::ToValue::to_value(&self.round)),
                    (
                        "issuancePerValidatorRewardCoupon",
                        rt::ToValue::to_value(&self.issuance_per_validator_reward_coupon)
                    ),
                    (
                        "issuancePerFeaturedAppRewardCoupon",
                        rt::ToValue::to_value(&self.issuance_per_featured_app_reward_coupon)
                    ),
                    (
                        "issuancePerUnfeaturedAppRewardCoupon",
                        rt::ToValue::to_value(&self.issuance_per_unfeatured_app_reward_coupon)
                    ),
                    (
                        "issuancePerSvRewardCoupon",
                        rt::ToValue::to_value(&self.issuance_per_sv_reward_coupon)
                    ),
                    (
                        "optIssuancePerValidatorFaucetCoupon",
                        rt::ToValue::to_value(&self.opt_issuance_per_validator_faucet_coupon)
                    ),
                ])
            }
        }
        ///The `Archive` choice on [`ClosedMiningRound`] (consuming).
        impl rt::Choice<ClosedMiningRound>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The Daml template `Splice.Round:IssuingMiningRound`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-amulet:Splice.Round:IssuingMiningRound`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct IssuingMiningRound {
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            #[serde(rename = "round")]
            pub round: crate::splice_amulet::Splice_Types::Round,
            ///Daml field `issuancePerValidatorRewardCoupon`.
            #[serde(rename = "issuancePerValidatorRewardCoupon")]
            pub issuance_per_validator_reward_coupon: rt::Numeric,
            ///Daml field `issuancePerFeaturedAppRewardCoupon`.
            #[serde(rename = "issuancePerFeaturedAppRewardCoupon")]
            pub issuance_per_featured_app_reward_coupon: rt::Numeric,
            ///Daml field `issuancePerUnfeaturedAppRewardCoupon`.
            #[serde(rename = "issuancePerUnfeaturedAppRewardCoupon")]
            pub issuance_per_unfeatured_app_reward_coupon: rt::Numeric,
            ///Daml field `issuancePerSvRewardCoupon`.
            #[serde(rename = "issuancePerSvRewardCoupon")]
            pub issuance_per_sv_reward_coupon: rt::Numeric,
            ///Daml field `opensAt`.
            #[serde(rename = "opensAt")]
            pub opens_at: rt::Timestamp,
            ///Daml field `targetClosesAt`.
            #[serde(rename = "targetClosesAt")]
            pub target_closes_at: rt::Timestamp,
            ///Daml field `optIssuancePerValidatorFaucetCoupon`.
            #[serde(rename = "optIssuancePerValidatorFaucetCoupon")]
            pub opt_issuance_per_validator_faucet_coupon: ::core::option::Option<rt::Numeric>,
        }
        impl rt::ToValue for IssuingMiningRound {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("round", rt::ToValue::to_value(&self.round)),
                    (
                        "issuancePerValidatorRewardCoupon",
                        rt::ToValue::to_value(&self.issuance_per_validator_reward_coupon)
                    ),
                    (
                        "issuancePerFeaturedAppRewardCoupon",
                        rt::ToValue::to_value(&self.issuance_per_featured_app_reward_coupon)
                    ),
                    (
                        "issuancePerUnfeaturedAppRewardCoupon",
                        rt::ToValue::to_value(&self.issuance_per_unfeatured_app_reward_coupon)
                    ),
                    (
                        "issuancePerSvRewardCoupon",
                        rt::ToValue::to_value(&self.issuance_per_sv_reward_coupon)
                    ),
                    ("opensAt", rt::ToValue::to_value(&self.opens_at)),
                    (
                        "targetClosesAt",
                        rt::ToValue::to_value(&self.target_closes_at)
                    ),
                    (
                        "optIssuancePerValidatorFaucetCoupon",
                        rt::ToValue::to_value(&self.opt_issuance_per_validator_faucet_coupon)
                    ),
                ])
            }
        }
        impl rt::FromValue for IssuingMiningRound {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                    round: rt::FromValue::from_value(rt::required_field(value, 1usize, "round")?)
                        .map_err(|e| e.at("round"))?,
                    issuance_per_validator_reward_coupon: rt::FromValue::from_value(
                        rt::required_field(value, 2usize, "issuancePerValidatorRewardCoupon")?,
                    )
                    .map_err(|e| e.at("issuancePerValidatorRewardCoupon"))?,
                    issuance_per_featured_app_reward_coupon: rt::FromValue::from_value(
                        rt::required_field(value, 3usize, "issuancePerFeaturedAppRewardCoupon")?,
                    )
                    .map_err(|e| e.at("issuancePerFeaturedAppRewardCoupon"))?,
                    issuance_per_unfeatured_app_reward_coupon: rt::FromValue::from_value(
                        rt::required_field(value, 4usize, "issuancePerUnfeaturedAppRewardCoupon")?,
                    )
                    .map_err(|e| e.at("issuancePerUnfeaturedAppRewardCoupon"))?,
                    issuance_per_sv_reward_coupon: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "issuancePerSvRewardCoupon",
                    )?)
                    .map_err(|e| e.at("issuancePerSvRewardCoupon"))?,
                    opens_at: rt::FromValue::from_value(rt::required_field(
                        value, 6usize, "opensAt",
                    )?)
                    .map_err(|e| e.at("opensAt"))?,
                    target_closes_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        7usize,
                        "targetClosesAt",
                    )?)
                    .map_err(|e| e.at("targetClosesAt"))?,
                    opt_issuance_per_validator_faucet_coupon: rt::optional_field(
                        value,
                        8usize,
                        "optIssuancePerValidatorFaucetCoupon",
                    )
                    .map_err(|e| e.at("optIssuancePerValidatorFaucetCoupon"))?,
                })
            }
        }
        impl rt::Contract for IssuingMiningRound {
            const PACKAGE_ID: &'static str =
                "3ca1343ab26b453d38c8adb70dca5f1ead8440c42b59b68f070786955cbf9ec1";
            const PACKAGE_NAME: &'static str = "splice-amulet";
            const MODULE_NAME: &'static str = "Splice.Round";
            const ENTITY_NAME: &'static str = "IssuingMiningRound";
        }
        impl rt::Template for IssuingMiningRound {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("round", rt::ToValue::to_value(&self.round)),
                    (
                        "issuancePerValidatorRewardCoupon",
                        rt::ToValue::to_value(&self.issuance_per_validator_reward_coupon)
                    ),
                    (
                        "issuancePerFeaturedAppRewardCoupon",
                        rt::ToValue::to_value(&self.issuance_per_featured_app_reward_coupon)
                    ),
                    (
                        "issuancePerUnfeaturedAppRewardCoupon",
                        rt::ToValue::to_value(&self.issuance_per_unfeatured_app_reward_coupon)
                    ),
                    (
                        "issuancePerSvRewardCoupon",
                        rt::ToValue::to_value(&self.issuance_per_sv_reward_coupon)
                    ),
                    ("opensAt", rt::ToValue::to_value(&self.opens_at)),
                    (
                        "targetClosesAt",
                        rt::ToValue::to_value(&self.target_closes_at)
                    ),
                    (
                        "optIssuancePerValidatorFaucetCoupon",
                        rt::ToValue::to_value(&self.opt_issuance_per_validator_faucet_coupon)
                    ),
                ])
            }
        }
        ///The `Archive` choice on [`IssuingMiningRound`] (consuming).
        impl rt::Choice<IssuingMiningRound>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The Daml template `Splice.Round:OpenMiningRound`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-amulet:Splice.Round:OpenMiningRound`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `Archive` — consuming
        ///- `OpenMiningRound_Fetch` — non-consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct OpenMiningRound {
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            #[serde(rename = "round")]
            pub round: crate::splice_amulet::Splice_Types::Round,
            ///Daml field `amuletPrice`.
            #[serde(rename = "amuletPrice")]
            pub amulet_price: rt::Numeric,
            ///Daml field `opensAt`.
            #[serde(rename = "opensAt")]
            pub opens_at: rt::Timestamp,
            ///Daml field `targetClosesAt`.
            #[serde(rename = "targetClosesAt")]
            pub target_closes_at: rt::Timestamp,
            ///Daml field `issuingFor`.
            #[serde(rename = "issuingFor")]
            pub issuing_for:
                ::canton_daml_stdlib::daml_stdlib_DA_Time_Types::DA_Time_Types::RelTime,
            ///Daml field `transferConfigUsd`.
            #[serde(rename = "transferConfigUsd")]
            pub transfer_config_usd: crate::splice_amulet::Splice_AmuletConfig::TransferConfig<
                crate::splice_amulet::Splice_AmuletConfig::USD,
            >,
            ///Daml field `issuanceConfig`.
            #[serde(rename = "issuanceConfig")]
            pub issuance_config: crate::splice_amulet::Splice_Issuance::IssuanceConfig,
            ///Daml field `tickDuration`.
            #[serde(rename = "tickDuration")]
            pub tick_duration:
                ::canton_daml_stdlib::daml_stdlib_DA_Time_Types::DA_Time_Types::RelTime,
        }
        impl rt::ToValue for OpenMiningRound {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("round", rt::ToValue::to_value(&self.round)),
                    ("amuletPrice", rt::ToValue::to_value(&self.amulet_price)),
                    ("opensAt", rt::ToValue::to_value(&self.opens_at)),
                    (
                        "targetClosesAt",
                        rt::ToValue::to_value(&self.target_closes_at)
                    ),
                    ("issuingFor", rt::ToValue::to_value(&self.issuing_for)),
                    (
                        "transferConfigUsd",
                        rt::ToValue::to_value(&self.transfer_config_usd)
                    ),
                    (
                        "issuanceConfig",
                        rt::ToValue::to_value(&self.issuance_config)
                    ),
                    ("tickDuration", rt::ToValue::to_value(&self.tick_duration)),
                ])
            }
        }
        impl rt::FromValue for OpenMiningRound {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                    round: rt::FromValue::from_value(rt::required_field(value, 1usize, "round")?)
                        .map_err(|e| e.at("round"))?,
                    amulet_price: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "amuletPrice",
                    )?)
                    .map_err(|e| e.at("amuletPrice"))?,
                    opens_at: rt::FromValue::from_value(rt::required_field(
                        value, 3usize, "opensAt",
                    )?)
                    .map_err(|e| e.at("opensAt"))?,
                    target_closes_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "targetClosesAt",
                    )?)
                    .map_err(|e| e.at("targetClosesAt"))?,
                    issuing_for: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "issuingFor",
                    )?)
                    .map_err(|e| e.at("issuingFor"))?,
                    transfer_config_usd: rt::FromValue::from_value(rt::required_field(
                        value,
                        6usize,
                        "transferConfigUsd",
                    )?)
                    .map_err(|e| e.at("transferConfigUsd"))?,
                    issuance_config: rt::FromValue::from_value(rt::required_field(
                        value,
                        7usize,
                        "issuanceConfig",
                    )?)
                    .map_err(|e| e.at("issuanceConfig"))?,
                    tick_duration: rt::FromValue::from_value(rt::required_field(
                        value,
                        8usize,
                        "tickDuration",
                    )?)
                    .map_err(|e| e.at("tickDuration"))?,
                })
            }
        }
        impl rt::Contract for OpenMiningRound {
            const PACKAGE_ID: &'static str =
                "3ca1343ab26b453d38c8adb70dca5f1ead8440c42b59b68f070786955cbf9ec1";
            const PACKAGE_NAME: &'static str = "splice-amulet";
            const MODULE_NAME: &'static str = "Splice.Round";
            const ENTITY_NAME: &'static str = "OpenMiningRound";
        }
        impl rt::Template for OpenMiningRound {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("round", rt::ToValue::to_value(&self.round)),
                    ("amuletPrice", rt::ToValue::to_value(&self.amulet_price)),
                    ("opensAt", rt::ToValue::to_value(&self.opens_at)),
                    (
                        "targetClosesAt",
                        rt::ToValue::to_value(&self.target_closes_at)
                    ),
                    ("issuingFor", rt::ToValue::to_value(&self.issuing_for)),
                    (
                        "transferConfigUsd",
                        rt::ToValue::to_value(&self.transfer_config_usd)
                    ),
                    (
                        "issuanceConfig",
                        rt::ToValue::to_value(&self.issuance_config)
                    ),
                    ("tickDuration", rt::ToValue::to_value(&self.tick_duration)),
                ])
            }
        }
        ///The `Archive` choice on [`OpenMiningRound`] (consuming).
        impl rt::Choice<OpenMiningRound>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `OpenMiningRound_Fetch` choice on [`OpenMiningRound`] (non-consuming).
        impl rt::Choice<OpenMiningRound> for crate::splice_amulet::Splice_Round::OpenMiningRound_Fetch {
            type Return = crate::splice_amulet::Splice_Round::OpenMiningRound;
            const NAME: &'static str = "OpenMiningRound_Fetch";
            const CONSUMING: bool = false;
        }
        ///The Daml template `Splice.Round:SummarizingMiningRound`.
        ///
        ///Submit with `rt::create_command`; its on-ledger id is `#splice-amulet:Splice.Round:SummarizingMiningRound`.
        ///
        ///# Choices
        ///
        ///Exercise with `rt::exercise_command`:
        ///
        ///- `Archive` — consuming
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SummarizingMiningRound {
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            #[serde(rename = "round")]
            pub round: crate::splice_amulet::Splice_Types::Round,
            ///Daml field `amuletPrice`.
            #[serde(rename = "amuletPrice")]
            pub amulet_price: rt::Numeric,
            ///Daml field `issuanceConfig`.
            #[serde(rename = "issuanceConfig")]
            pub issuance_config: crate::splice_amulet::Splice_Issuance::IssuanceConfig,
            ///Daml field `tickDuration`.
            #[serde(rename = "tickDuration")]
            pub tick_duration:
                ::canton_daml_stdlib::daml_stdlib_DA_Time_Types::DA_Time_Types::RelTime,
        }
        impl rt::ToValue for SummarizingMiningRound {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("round", rt::ToValue::to_value(&self.round)),
                    ("amuletPrice", rt::ToValue::to_value(&self.amulet_price)),
                    (
                        "issuanceConfig",
                        rt::ToValue::to_value(&self.issuance_config)
                    ),
                    ("tickDuration", rt::ToValue::to_value(&self.tick_duration)),
                ])
            }
        }
        impl rt::FromValue for SummarizingMiningRound {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                    round: rt::FromValue::from_value(rt::required_field(value, 1usize, "round")?)
                        .map_err(|e| e.at("round"))?,
                    amulet_price: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "amuletPrice",
                    )?)
                    .map_err(|e| e.at("amuletPrice"))?,
                    issuance_config: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "issuanceConfig",
                    )?)
                    .map_err(|e| e.at("issuanceConfig"))?,
                    tick_duration: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "tickDuration",
                    )?)
                    .map_err(|e| e.at("tickDuration"))?,
                })
            }
        }
        impl rt::Contract for SummarizingMiningRound {
            const PACKAGE_ID: &'static str =
                "3ca1343ab26b453d38c8adb70dca5f1ead8440c42b59b68f070786955cbf9ec1";
            const PACKAGE_NAME: &'static str = "splice-amulet";
            const MODULE_NAME: &'static str = "Splice.Round";
            const ENTITY_NAME: &'static str = "SummarizingMiningRound";
        }
        impl rt::Template for SummarizingMiningRound {
            fn to_record(&self) -> rt::Record {
                rt::record_fields(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("round", rt::ToValue::to_value(&self.round)),
                    ("amuletPrice", rt::ToValue::to_value(&self.amulet_price)),
                    (
                        "issuanceConfig",
                        rt::ToValue::to_value(&self.issuance_config)
                    ),
                    ("tickDuration", rt::ToValue::to_value(&self.tick_duration)),
                ])
            }
        }
        ///The `Archive` choice on [`SummarizingMiningRound`] (consuming).
        impl rt::Choice<SummarizingMiningRound>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
    }
    pub mod Splice_Amulet_TwoStepTransfer {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TwoStepTransfer {
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            #[serde(rename = "sender")]
            pub sender: rt::Party,
            #[serde(rename = "receiver")]
            pub receiver: rt::Party,
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            ///Daml field `lockContext`.
            #[serde(rename = "lockContext")]
            pub lock_context: ::std::string::String,
            ///Daml field `transferBefore`.
            #[serde(rename = "transferBefore")]
            pub transfer_before: rt::Timestamp,
            ///Daml field `transferBeforeDeadline`.
            #[serde(rename = "transferBeforeDeadline")]
            pub transfer_before_deadline: ::std::string::String,
            #[serde(rename = "provider")]
            pub provider: rt::Party,
            ///Daml field `allowFeaturing`.
            #[serde(rename = "allowFeaturing")]
            pub allow_featuring: bool,
        }
        impl rt::ToValue for TwoStepTransfer {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("dso", rt::ToValue::to_value(&self.dso)),
                    ("sender", rt::ToValue::to_value(&self.sender)),
                    ("receiver", rt::ToValue::to_value(&self.receiver)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                    ("lockContext", rt::ToValue::to_value(&self.lock_context)),
                    (
                        "transferBefore",
                        rt::ToValue::to_value(&self.transfer_before)
                    ),
                    (
                        "transferBeforeDeadline",
                        rt::ToValue::to_value(&self.transfer_before_deadline)
                    ),
                    ("provider", rt::ToValue::to_value(&self.provider)),
                    (
                        "allowFeaturing",
                        rt::ToValue::to_value(&self.allow_featuring)
                    ),
                ])
            }
        }
        impl rt::FromValue for TwoStepTransfer {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)
                        .map_err(|e| e.at("dso"))?,
                    sender: rt::FromValue::from_value(rt::required_field(value, 1usize, "sender")?)
                        .map_err(|e| e.at("sender"))?,
                    receiver: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "receiver",
                    )?)
                    .map_err(|e| e.at("receiver"))?,
                    amount: rt::FromValue::from_value(rt::required_field(value, 3usize, "amount")?)
                        .map_err(|e| e.at("amount"))?,
                    lock_context: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "lockContext",
                    )?)
                    .map_err(|e| e.at("lockContext"))?,
                    transfer_before: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "transferBefore",
                    )?)
                    .map_err(|e| e.at("transferBefore"))?,
                    transfer_before_deadline: rt::FromValue::from_value(rt::required_field(
                        value,
                        6usize,
                        "transferBeforeDeadline",
                    )?)
                    .map_err(|e| e.at("transferBeforeDeadline"))?,
                    provider: rt::FromValue::from_value(rt::required_field(
                        value, 7usize, "provider",
                    )?)
                    .map_err(|e| e.at("provider"))?,
                    allow_featuring: rt::FromValue::from_value(rt::required_field(
                        value,
                        8usize,
                        "allowFeaturing",
                    )?)
                    .map_err(|e| e.at("allowFeaturing"))?,
                })
            }
        }
    }
}
