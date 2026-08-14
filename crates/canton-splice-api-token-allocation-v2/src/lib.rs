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
//! - [`splice_api_token_allocation_v2`] — 1 Daml module

pub mod splice_api_token_allocation_v2 {
    pub mod Splice_Api_Token_AllocationV2 {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SettlementFactory_SettleBatchResult {
            ///Daml field `allocationSettleResults`.
            #[serde(rename = "allocationSettleResults")]
            pub allocation_settle_results: ::std::vec::Vec<
                crate::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::AllocationResult,
            >,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for SettlementFactory_SettleBatchResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "allocationSettleResults",
                        rt::ToValue::to_value(&self.allocation_settle_results)
                    ),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for SettlementFactory_SettleBatchResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    allocation_settle_results: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "allocationSettleResults",
                    )?)
                    .map_err(|e| e.at("allocationSettleResults"))?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 1usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SettlementFactory_PublicFetch {
            #[serde(rename = "actors")]
            pub actors: ::std::vec::Vec<rt::Party>,
        }
        impl rt::ToValue for SettlementFactory_PublicFetch {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("actors", rt::ToValue::to_value(&self.actors)),])
            }
        }
        impl rt::FromValue for SettlementFactory_PublicFetch {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    actors: rt::FromValue::from_value(rt::required_field(value, 0usize, "actors")?)
                        .map_err(|e| e.at("actors"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SettlementFactory_SettleBatch {
            #[serde(rename = "settlement")]
            pub settlement: crate::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::SettlementInfo,
            ///Daml field `transferLegs`.
            #[serde(rename = "transferLegs")]
            pub transfer_legs: ::std::vec::Vec<
                crate::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::TransferLeg,
            >,
            #[serde(rename = "allocations")]
            pub allocations: ::std::vec::Vec<
                crate::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::FinalizedAllocation,
            >,
            #[serde(rename = "actors")]
            pub actors: ::std::vec::Vec<rt::Party>,
            ///Daml field `extraArgs`.
            #[serde(rename = "extraArgs")]
            pub extra_args: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for SettlementFactory_SettleBatch {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("settlement", rt::ToValue::to_value(&self.settlement)),
                    ("transferLegs", rt::ToValue::to_value(&self.transfer_legs)),
                    ("allocations", rt::ToValue::to_value(&self.allocations)),
                    ("actors", rt::ToValue::to_value(&self.actors)),
                    ("extraArgs", rt::ToValue::to_value(&self.extra_args)),
                ])
            }
        }
        impl rt::FromValue for SettlementFactory_SettleBatch {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    settlement: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "settlement",
                    )?)
                    .map_err(|e| e.at("settlement"))?,
                    transfer_legs: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "transferLegs",
                    )?)
                    .map_err(|e| e.at("transferLegs"))?,
                    allocations: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "allocations",
                    )?)
                    .map_err(|e| e.at("allocations"))?,
                    actors: rt::FromValue::from_value(rt::required_field(value, 3usize, "actors")?)
                        .map_err(|e| e.at("actors"))?,
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
        pub struct SettlementFactoryView {
            #[serde(rename = "admin")]
            pub admin: rt::Party,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for SettlementFactoryView {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("admin", rt::ToValue::to_value(&self.admin)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for SettlementFactoryView {
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
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum AllocationResult_Output {
            #[serde(rename = "AllocationResult_Pending")]
            AllocationResult_Pending(
                crate::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::AllocationResult_Output_AllocationResult_Pending,
            ),
            #[serde(rename = "AllocationResult_Settled")]
            AllocationResult_Settled(
                crate::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::AllocationResult_Output_AllocationResult_Settled,
            ),
            #[serde(rename = "AllocationResult_Cancelled")]
            AllocationResult_Cancelled(rt::Unit),
            #[serde(rename = "AllocationResult_Withdrawn")]
            AllocationResult_Withdrawn(rt::Unit),
        }
        impl rt::ToValue for AllocationResult_Output {
            fn to_value(&self) -> rt::Value {
                match self {
                    AllocationResult_Output::AllocationResult_Pending(inner) => {
                        rt::variant_value("AllocationResult_Pending", rt::ToValue::to_value(inner))
                    }
                    AllocationResult_Output::AllocationResult_Settled(inner) => {
                        rt::variant_value("AllocationResult_Settled", rt::ToValue::to_value(inner))
                    }
                    AllocationResult_Output::AllocationResult_Cancelled(inner) => {
                        rt::variant_value(
                            "AllocationResult_Cancelled",
                            rt::ToValue::to_value(inner),
                        )
                    }
                    AllocationResult_Output::AllocationResult_Withdrawn(inner) => {
                        rt::variant_value(
                            "AllocationResult_Withdrawn",
                            rt::ToValue::to_value(inner),
                        )
                    }
                }
            }
        }
        impl rt::FromValue for AllocationResult_Output {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                let (constructor, payload) = rt::variant_parts(value)?;
                match constructor {
                    "AllocationResult_Pending" => ::core::result::Result::Ok(
                        AllocationResult_Output::AllocationResult_Pending(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("AllocationResult_Pending"))?,
                        ),
                    ),
                    "AllocationResult_Settled" => ::core::result::Result::Ok(
                        AllocationResult_Output::AllocationResult_Settled(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("AllocationResult_Settled"))?,
                        ),
                    ),
                    "AllocationResult_Cancelled" => ::core::result::Result::Ok(
                        AllocationResult_Output::AllocationResult_Cancelled(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("AllocationResult_Cancelled"))?,
                        ),
                    ),
                    "AllocationResult_Withdrawn" => ::core::result::Result::Ok(
                        AllocationResult_Output::AllocationResult_Withdrawn(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("AllocationResult_Withdrawn"))?,
                        ),
                    ),
                    other => ::core::result::Result::Err(rt::unexpected_constructor(
                        "AllocationResult_Output",
                        other,
                    )),
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationResult_Output_AllocationResult_Pending {
            ///Daml field `allocationCid`.
            #[serde(rename = "allocationCid")]
            pub allocation_cid: rt::ContractId<
                crate::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::Allocation,
            >,
        }
        impl rt::ToValue for AllocationResult_Output_AllocationResult_Pending {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "allocationCid",
                    rt::ToValue::to_value(&self.allocation_cid)
                ),])
            }
        }
        impl rt::FromValue for AllocationResult_Output_AllocationResult_Pending {
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
        pub struct AllocationResult_Output_AllocationResult_Settled {
            ///Daml field `nextIterationAllocationCid`.
            #[serde(rename = "nextIterationAllocationCid")]
            pub next_iteration_allocation_cid: ::core::option::Option<
                rt::ContractId<
                    crate::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::Allocation,
                >,
            >,
        }
        impl rt::ToValue for AllocationResult_Output_AllocationResult_Settled {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "nextIterationAllocationCid",
                    rt::ToValue::to_value(&self.next_iteration_allocation_cid)
                ),])
            }
        }
        impl rt::FromValue for AllocationResult_Output_AllocationResult_Settled {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    next_iteration_allocation_cid: rt::optional_field(
                        value,
                        0usize,
                        "nextIterationAllocationCid",
                    )
                    .map_err(|e| e.at("nextIterationAllocationCid"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationResult {
            #[serde(rename = "output")]
            pub output: crate::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::AllocationResult_Output,
            ///Daml field `authorizerHoldingCids`.
            #[serde(rename = "authorizerHoldingCids")]
            pub authorizer_holding_cids: rt::TextMap<
                ::std::vec::Vec<
                    rt::ContractId<
                        ::canton_splice_api_token_holding_v2::splice_api_token_holding_v2::Splice_Api_Token_HoldingV2::Holding,
                    >,
                >,
            >,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for AllocationResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("output", rt::ToValue::to_value(&self.output)),
                    (
                        "authorizerHoldingCids",
                        rt::ToValue::to_value(&self.authorizer_holding_cids)
                    ),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for AllocationResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    output: rt::FromValue::from_value(rt::required_field(value, 0usize, "output")?)
                        .map_err(|e| e.at("output"))?,
                    authorizer_holding_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "authorizerHoldingCids",
                    )?)
                    .map_err(|e| e.at("authorizerHoldingCids"))?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 2usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Allocation_Withdraw {
            #[serde(rename = "actors")]
            pub actors: ::std::vec::Vec<rt::Party>,
            ///Daml field `extraArgs`.
            #[serde(rename = "extraArgs")]
            pub extra_args: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for Allocation_Withdraw {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("actors", rt::ToValue::to_value(&self.actors)),
                    ("extraArgs", rt::ToValue::to_value(&self.extra_args)),
                ])
            }
        }
        impl rt::FromValue for Allocation_Withdraw {
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
        pub struct Allocation_Cancel {
            #[serde(rename = "actors")]
            pub actors: ::std::vec::Vec<rt::Party>,
            ///Daml field `extraArgs`.
            #[serde(rename = "extraArgs")]
            pub extra_args: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for Allocation_Cancel {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("actors", rt::ToValue::to_value(&self.actors)),
                    ("extraArgs", rt::ToValue::to_value(&self.extra_args)),
                ])
            }
        }
        impl rt::FromValue for Allocation_Cancel {
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
        pub struct Allocation_Settle {
            #[serde(rename = "actors")]
            pub actors: ::std::vec::Vec<rt::Party>,
            ///Daml field `extraTransferLegSides`.
            #[serde(rename = "extraTransferLegSides")]
            pub extra_transfer_leg_sides: ::std::vec::Vec<
                crate::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::TransferLegSide,
            >,
            ///Daml field `nextIterationFunding`.
            #[serde(rename = "nextIterationFunding")]
            pub next_iteration_funding: ::core::option::Option<rt::TextMap<rt::Numeric>>,
            ///Daml field `extraArgs`.
            #[serde(rename = "extraArgs")]
            pub extra_args: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for Allocation_Settle {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("actors", rt::ToValue::to_value(&self.actors)),
                    (
                        "extraTransferLegSides",
                        rt::ToValue::to_value(&self.extra_transfer_leg_sides)
                    ),
                    (
                        "nextIterationFunding",
                        rt::ToValue::to_value(&self.next_iteration_funding)
                    ),
                    ("extraArgs", rt::ToValue::to_value(&self.extra_args)),
                ])
            }
        }
        impl rt::FromValue for Allocation_Settle {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    actors: rt::FromValue::from_value(rt::required_field(value, 0usize, "actors")?)
                        .map_err(|e| e.at("actors"))?,
                    extra_transfer_leg_sides: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "extraTransferLegSides",
                    )?)
                    .map_err(|e| e.at("extraTransferLegSides"))?,
                    next_iteration_funding: rt::optional_field(
                        value,
                        2usize,
                        "nextIterationFunding",
                    )
                    .map_err(|e| e.at("nextIterationFunding"))?,
                    extra_args: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "extraArgs",
                    )?)
                    .map_err(|e| e.at("extraArgs"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct FinalizedAllocation {
            ///Daml field `allocationCid`.
            #[serde(rename = "allocationCid")]
            pub allocation_cid: rt::ContractId<
                crate::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::Allocation,
            >,
            ///Daml field `extraTransferLegSides`.
            #[serde(rename = "extraTransferLegSides")]
            pub extra_transfer_leg_sides: ::std::vec::Vec<
                crate::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::TransferLegSide,
            >,
            ///Daml field `nextIterationFunding`.
            #[serde(rename = "nextIterationFunding")]
            pub next_iteration_funding: ::core::option::Option<rt::TextMap<rt::Numeric>>,
        }
        impl rt::ToValue for FinalizedAllocation {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("allocationCid", rt::ToValue::to_value(&self.allocation_cid)),
                    (
                        "extraTransferLegSides",
                        rt::ToValue::to_value(&self.extra_transfer_leg_sides)
                    ),
                    (
                        "nextIterationFunding",
                        rt::ToValue::to_value(&self.next_iteration_funding)
                    ),
                ])
            }
        }
        impl rt::FromValue for FinalizedAllocation {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    allocation_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "allocationCid",
                    )?)
                    .map_err(|e| e.at("allocationCid"))?,
                    extra_transfer_leg_sides: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "extraTransferLegSides",
                    )?)
                    .map_err(|e| e.at("extraTransferLegSides"))?,
                    next_iteration_funding: rt::optional_field(
                        value,
                        2usize,
                        "nextIterationFunding",
                    )
                    .map_err(|e| e.at("nextIterationFunding"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationView {
            ///Daml field `originalAllocationCid`.
            #[serde(rename = "originalAllocationCid")]
            pub original_allocation_cid: ::core::option::Option<
                rt::ContractId<
                    crate::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::Allocation,
                >,
            >,
            #[serde(rename = "settlement")]
            pub settlement: crate::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::SettlementInfo,
            #[serde(rename = "allocation")]
            pub allocation: crate::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::AllocationSpecification,
            ///Daml field `holdingCids`.
            #[serde(rename = "holdingCids")]
            pub holding_cids: ::std::vec::Vec<
                rt::ContractId<
                    ::canton_splice_api_token_holding_v2::splice_api_token_holding_v2::Splice_Api_Token_HoldingV2::Holding,
                >,
            >,
            ///Daml field `createdAt`.
            #[serde(rename = "createdAt")]
            pub created_at: rt::Timestamp,
            ///Daml field `numIterations`.
            #[serde(rename = "numIterations")]
            pub num_iterations: rt::Int64,
            ///Daml field `expiresAt`.
            #[serde(rename = "expiresAt")]
            pub expires_at: ::core::option::Option<rt::Timestamp>,
            ///Daml field `availableActions`.
            #[serde(rename = "availableActions")]
            pub available_actions: rt::GenMap<
                crate::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::AllocationAction,
                ::std::vec::Vec<::std::vec::Vec<rt::Party>>,
            >,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for AllocationView {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "originalAllocationCid",
                        rt::ToValue::to_value(&self.original_allocation_cid)
                    ),
                    ("settlement", rt::ToValue::to_value(&self.settlement)),
                    ("allocation", rt::ToValue::to_value(&self.allocation)),
                    ("holdingCids", rt::ToValue::to_value(&self.holding_cids)),
                    ("createdAt", rt::ToValue::to_value(&self.created_at)),
                    ("numIterations", rt::ToValue::to_value(&self.num_iterations)),
                    ("expiresAt", rt::ToValue::to_value(&self.expires_at)),
                    (
                        "availableActions",
                        rt::ToValue::to_value(&self.available_actions)
                    ),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for AllocationView {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    original_allocation_cid: rt::optional_field(
                        value,
                        0usize,
                        "originalAllocationCid",
                    )
                    .map_err(|e| e.at("originalAllocationCid"))?,
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
                    holding_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "holdingCids",
                    )?)
                    .map_err(|e| e.at("holdingCids"))?,
                    created_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "createdAt",
                    )?)
                    .map_err(|e| e.at("createdAt"))?,
                    num_iterations: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "numIterations",
                    )?)
                    .map_err(|e| e.at("numIterations"))?,
                    expires_at: rt::optional_field(value, 6usize, "expiresAt")
                        .map_err(|e| e.at("expiresAt"))?,
                    available_actions: rt::FromValue::from_value(rt::required_field(
                        value,
                        7usize,
                        "availableActions",
                    )?)
                    .map_err(|e| e.at("availableActions"))?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 8usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum AllocationAction {
            #[serde(rename = "AA_Settle")]
            AA_Settle(rt::Unit),
            #[serde(rename = "AA_Cancel")]
            AA_Cancel(rt::Unit),
            #[serde(rename = "AA_Withdraw")]
            AA_Withdraw(rt::Unit),
            #[serde(rename = "AA_Custom")]
            AA_Custom(
                crate::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::AllocationAction_AA_Custom,
            ),
        }
        impl rt::ToValue for AllocationAction {
            fn to_value(&self) -> rt::Value {
                match self {
                    AllocationAction::AA_Settle(inner) => {
                        rt::variant_value("AA_Settle", rt::ToValue::to_value(inner))
                    }
                    AllocationAction::AA_Cancel(inner) => {
                        rt::variant_value("AA_Cancel", rt::ToValue::to_value(inner))
                    }
                    AllocationAction::AA_Withdraw(inner) => {
                        rt::variant_value("AA_Withdraw", rt::ToValue::to_value(inner))
                    }
                    AllocationAction::AA_Custom(inner) => {
                        rt::variant_value("AA_Custom", rt::ToValue::to_value(inner))
                    }
                }
            }
        }
        impl rt::FromValue for AllocationAction {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                let (constructor, payload) = rt::variant_parts(value)?;
                match constructor {
                    "AA_Settle" => ::core::result::Result::Ok(AllocationAction::AA_Settle(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("AA_Settle"))?,
                    )),
                    "AA_Cancel" => ::core::result::Result::Ok(AllocationAction::AA_Cancel(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("AA_Cancel"))?,
                    )),
                    "AA_Withdraw" => ::core::result::Result::Ok(AllocationAction::AA_Withdraw(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("AA_Withdraw"))?,
                    )),
                    "AA_Custom" => ::core::result::Result::Ok(AllocationAction::AA_Custom(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("AA_Custom"))?,
                    )),
                    other => ::core::result::Result::Err(rt::unexpected_constructor(
                        "AllocationAction",
                        other,
                    )),
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationAction_AA_Custom {
            #[serde(rename = "id")]
            pub id: ::std::string::String,
        }
        impl rt::ToValue for AllocationAction_AA_Custom {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("id", rt::ToValue::to_value(&self.id)),])
            }
        }
        impl rt::FromValue for AllocationAction_AA_Custom {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    id: rt::FromValue::from_value(rt::required_field(value, 0usize, "id")?)
                        .map_err(|e| e.at("id"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationSpecification {
            #[serde(rename = "admin")]
            pub admin: rt::Party,
            #[serde(rename = "authorizer")]
            pub authorizer: ::canton_splice_api_token_holding_v2::splice_api_token_holding_v2::Splice_Api_Token_HoldingV2::Account,
            ///Daml field `transferLegSides`.
            #[serde(rename = "transferLegSides")]
            pub transfer_leg_sides: ::std::vec::Vec<
                crate::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::TransferLegSide,
            >,
            ///Daml field `settlementDeadline`.
            #[serde(rename = "settlementDeadline")]
            pub settlement_deadline: ::core::option::Option<rt::Timestamp>,
            ///Daml field `nextIterationFunding`.
            #[serde(rename = "nextIterationFunding")]
            pub next_iteration_funding: ::core::option::Option<rt::TextMap<rt::Numeric>>,
            #[serde(rename = "committed")]
            pub committed: bool,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for AllocationSpecification {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("admin", rt::ToValue::to_value(&self.admin)),
                    ("authorizer", rt::ToValue::to_value(&self.authorizer)),
                    (
                        "transferLegSides",
                        rt::ToValue::to_value(&self.transfer_leg_sides)
                    ),
                    (
                        "settlementDeadline",
                        rt::ToValue::to_value(&self.settlement_deadline)
                    ),
                    (
                        "nextIterationFunding",
                        rt::ToValue::to_value(&self.next_iteration_funding)
                    ),
                    ("committed", rt::ToValue::to_value(&self.committed)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for AllocationSpecification {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    admin: rt::FromValue::from_value(rt::required_field(value, 0usize, "admin")?)
                        .map_err(|e| e.at("admin"))?,
                    authorizer: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "authorizer",
                    )?)
                    .map_err(|e| e.at("authorizer"))?,
                    transfer_leg_sides: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "transferLegSides",
                    )?)
                    .map_err(|e| e.at("transferLegSides"))?,
                    settlement_deadline: rt::optional_field(value, 3usize, "settlementDeadline")
                        .map_err(|e| e.at("settlementDeadline"))?,
                    next_iteration_funding: rt::optional_field(
                        value,
                        4usize,
                        "nextIterationFunding",
                    )
                    .map_err(|e| e.at("nextIterationFunding"))?,
                    committed: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "committed",
                    )?)
                    .map_err(|e| e.at("committed"))?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 6usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferLegSide {
            ///Daml field `transferLegId`.
            #[serde(rename = "transferLegId")]
            pub transfer_leg_id: ::std::string::String,
            #[serde(rename = "side")]
            pub side: crate::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::TransferSide,
            #[serde(rename = "otherside")]
            pub otherside: ::canton_splice_api_token_holding_v2::splice_api_token_holding_v2::Splice_Api_Token_HoldingV2::Account,
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            ///Daml field `instrumentId`.
            #[serde(rename = "instrumentId")]
            pub instrument_id: ::std::string::String,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for TransferLegSide {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "transferLegId",
                        rt::ToValue::to_value(&self.transfer_leg_id)
                    ),
                    ("side", rt::ToValue::to_value(&self.side)),
                    ("otherside", rt::ToValue::to_value(&self.otherside)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                    ("instrumentId", rt::ToValue::to_value(&self.instrument_id)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for TransferLegSide {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    transfer_leg_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "transferLegId",
                    )?)
                    .map_err(|e| e.at("transferLegId"))?,
                    side: rt::FromValue::from_value(rt::required_field(value, 1usize, "side")?)
                        .map_err(|e| e.at("side"))?,
                    otherside: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "otherside",
                    )?)
                    .map_err(|e| e.at("otherside"))?,
                    amount: rt::FromValue::from_value(rt::required_field(value, 3usize, "amount")?)
                        .map_err(|e| e.at("amount"))?,
                    instrument_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "instrumentId",
                    )?)
                    .map_err(|e| e.at("instrumentId"))?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 5usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize,
        )]
        #[serde(crate = "rt::serde")]
        pub enum TransferSide {
            #[serde(rename = "SenderSide")]
            SenderSide,
            #[serde(rename = "ReceiverSide")]
            ReceiverSide,
        }
        impl rt::ToValue for TransferSide {
            fn to_value(&self) -> rt::Value {
                rt::enum_value(match self {
                    TransferSide::SenderSide => "SenderSide",
                    TransferSide::ReceiverSide => "ReceiverSide",
                })
            }
        }
        impl rt::FromValue for TransferSide {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                match rt::enum_constructor(value)? {
                    "SenderSide" => ::core::result::Result::Ok(TransferSide::SenderSide),
                    "ReceiverSide" => ::core::result::Result::Ok(TransferSide::ReceiverSide),
                    other => ::core::result::Result::Err(rt::unexpected_constructor(
                        "TransferSide",
                        other,
                    )),
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferLeg {
            ///Daml field `transferLegId`.
            #[serde(rename = "transferLegId")]
            pub transfer_leg_id: ::std::string::String,
            #[serde(rename = "sender")]
            pub sender: ::canton_splice_api_token_holding_v2::splice_api_token_holding_v2::Splice_Api_Token_HoldingV2::Account,
            #[serde(rename = "receiver")]
            pub receiver: ::canton_splice_api_token_holding_v2::splice_api_token_holding_v2::Splice_Api_Token_HoldingV2::Account,
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            ///Daml field `instrumentId`.
            #[serde(rename = "instrumentId")]
            pub instrument_id: ::std::string::String,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for TransferLeg {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "transferLegId",
                        rt::ToValue::to_value(&self.transfer_leg_id)
                    ),
                    ("sender", rt::ToValue::to_value(&self.sender)),
                    ("receiver", rt::ToValue::to_value(&self.receiver)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                    ("instrumentId", rt::ToValue::to_value(&self.instrument_id)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for TransferLeg {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    transfer_leg_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "transferLegId",
                    )?)
                    .map_err(|e| e.at("transferLegId"))?,
                    sender: rt::FromValue::from_value(rt::required_field(value, 1usize, "sender")?)
                        .map_err(|e| e.at("sender"))?,
                    receiver: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "receiver",
                    )?)
                    .map_err(|e| e.at("receiver"))?,
                    amount: rt::FromValue::from_value(rt::required_field(value, 3usize, "amount")?)
                        .map_err(|e| e.at("amount"))?,
                    instrument_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "instrumentId",
                    )?)
                    .map_err(|e| e.at("instrumentId"))?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 5usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SettlementInfo {
            #[serde(rename = "executors")]
            pub executors: ::std::vec::Vec<rt::Party>,
            #[serde(rename = "id")]
            pub id: ::std::string::String,
            #[serde(rename = "cid")]
            pub cid: ::core::option::Option<
                rt::ContractId<
                    ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::AnyContract,
                >,
            >,
            #[serde(rename = "meta")]
            pub meta: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for SettlementInfo {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("executors", rt::ToValue::to_value(&self.executors)),
                    ("id", rt::ToValue::to_value(&self.id)),
                    ("cid", rt::ToValue::to_value(&self.cid)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for SettlementInfo {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    executors: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "executors",
                    )?)
                    .map_err(|e| e.at("executors"))?,
                    id: rt::FromValue::from_value(rt::required_field(value, 1usize, "id")?)
                        .map_err(|e| e.at("id"))?,
                    cid: rt::optional_field(value, 2usize, "cid").map_err(|e| e.at("cid"))?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 3usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        ///Marker for the Daml interface `Allocation` (held via `ContractId`).
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct Allocation;
        ///Marker for the Daml interface `SettlementFactory` (held via `ContractId`).
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct SettlementFactory;
        impl rt::Contract for Allocation {
            const PACKAGE_ID: &'static str =
                "051a3b0563a6fa4df4cb34448081e48b061e555aa1a265abf6ae8f3f4cafe439";
            const PACKAGE_NAME: &'static str = "splice-api-token-allocation-v2";
            const MODULE_NAME: &'static str = "Splice.Api.Token.AllocationV2";
            const ENTITY_NAME: &'static str = "Allocation";
        }
        impl rt::Interface for Allocation {
            type View = crate::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::AllocationView;
        }
        ///The `Archive` choice on [`Allocation`] (consuming).
        impl rt::Choice<Allocation>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `Allocation_Settle` choice on [`Allocation`] (non-consuming).
        impl rt::Choice<Allocation>
        for crate::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::Allocation_Settle {
            type Return = crate::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::AllocationResult;
            const NAME: &'static str = "Allocation_Settle";
            const CONSUMING: bool = false;
        }
        ///The `Allocation_Cancel` choice on [`Allocation`] (non-consuming).
        impl rt::Choice<Allocation>
        for crate::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::Allocation_Cancel {
            type Return = crate::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::AllocationResult;
            const NAME: &'static str = "Allocation_Cancel";
            const CONSUMING: bool = false;
        }
        ///The `Allocation_Withdraw` choice on [`Allocation`] (non-consuming).
        impl rt::Choice<Allocation>
        for crate::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::Allocation_Withdraw {
            type Return = crate::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::AllocationResult;
            const NAME: &'static str = "Allocation_Withdraw";
            const CONSUMING: bool = false;
        }
        impl rt::Contract for SettlementFactory {
            const PACKAGE_ID: &'static str =
                "051a3b0563a6fa4df4cb34448081e48b061e555aa1a265abf6ae8f3f4cafe439";
            const PACKAGE_NAME: &'static str = "splice-api-token-allocation-v2";
            const MODULE_NAME: &'static str = "Splice.Api.Token.AllocationV2";
            const ENTITY_NAME: &'static str = "SettlementFactory";
        }
        impl rt::Interface for SettlementFactory {
            type View = crate::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::SettlementFactoryView;
        }
        ///The `Archive` choice on [`SettlementFactory`] (consuming).
        impl rt::Choice<SettlementFactory>
            for ::canton_daml_stdlib::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `SettlementFactory_PublicFetch` choice on [`SettlementFactory`] (non-consuming).
        impl rt::Choice<SettlementFactory>
        for crate::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::SettlementFactory_PublicFetch {
            type Return = crate::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::SettlementFactoryView;
            const NAME: &'static str = "SettlementFactory_PublicFetch";
            const CONSUMING: bool = false;
        }
        ///The `SettlementFactory_SettleBatch` choice on [`SettlementFactory`] (non-consuming).
        impl rt::Choice<SettlementFactory>
        for crate::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::SettlementFactory_SettleBatch {
            type Return = crate::splice_api_token_allocation_v2::Splice_Api_Token_AllocationV2::SettlementFactory_SettleBatchResult;
            const NAME: &'static str = "SettlementFactory_SettleBatch";
            const CONSUMING: bool = false;
        }
    }
}
