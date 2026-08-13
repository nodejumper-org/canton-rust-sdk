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
//! - [`daml_prim_DA_Exception_ArithmeticError`] — 1 Daml module
//! - [`daml_prim_DA_Exception_AssertionFailed`] — 1 Daml module
//! - [`daml_prim_DA_Exception_GeneralError`] — 1 Daml module
//! - [`daml_prim_DA_Exception_PreconditionFailed`] — 1 Daml module
//! - [`daml_prim_DA_Types`] — 1 Daml module
//! - [`daml_prim_GHC_Tuple`] — 1 Daml module
//! - [`daml_prim_GHC_Types`] — 1 Daml module
//! - [`daml_stdlib_DA_Date_Types`] — 1 Daml module
//! - [`daml_stdlib_DA_Internal_Down`] — 1 Daml module
//! - [`ghc_stdlib_DA_Internal_Template`] — 1 Daml module
//! - [`daml_stdlib_DA_Logic_Types`] — 1 Daml module
//! - [`daml_stdlib_DA_Monoid_Types`] — 1 Daml module
//! - [`daml_stdlib_DA_NonEmpty_Types`] — 1 Daml module
//! - [`daml_stdlib_DA_Random_Types`] — 1 Daml module
//! - [`daml_stdlib_DA_Semigroup_Types`] — 1 Daml module
//! - [`daml_stdlib_DA_Set_Types`] — 1 Daml module
//! - [`daml_stdlib_DA_Stack_Types`] — 1 Daml module
//! - [`daml_stdlib_DA_Time_Types`] — 1 Daml module
//! - [`daml_stdlib_DA_Validation_Types`] — 1 Daml module
//! - [`splice_api_token_transfer_instruction_v1`] — 1 Daml module

pub mod daml_prim_DA_Exception_ArithmeticError {
    pub mod DA_Exception_ArithmeticError {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ArithmeticError {
            #[serde(rename = "message")]
            pub message: ::std::string::String,
        }
        impl rt::ToValue for ArithmeticError {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "message",
                    rt::ToValue::to_value(&self.message)
                ),])
            }
        }
        impl rt::FromValue for ArithmeticError {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    message: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "message",
                    )?)
                    .map_err(|e| e.at("message"))?,
                })
            }
        }
    }
}
pub mod daml_prim_DA_Exception_AssertionFailed {
    pub mod DA_Exception_AssertionFailed {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AssertionFailed {
            #[serde(rename = "message")]
            pub message: ::std::string::String,
        }
        impl rt::ToValue for AssertionFailed {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "message",
                    rt::ToValue::to_value(&self.message)
                ),])
            }
        }
        impl rt::FromValue for AssertionFailed {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    message: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "message",
                    )?)
                    .map_err(|e| e.at("message"))?,
                })
            }
        }
    }
}
pub mod daml_prim_DA_Exception_GeneralError {
    pub mod DA_Exception_GeneralError {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct GeneralError {
            #[serde(rename = "message")]
            pub message: ::std::string::String,
        }
        impl rt::ToValue for GeneralError {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "message",
                    rt::ToValue::to_value(&self.message)
                ),])
            }
        }
        impl rt::FromValue for GeneralError {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    message: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "message",
                    )?)
                    .map_err(|e| e.at("message"))?,
                })
            }
        }
    }
}
pub mod daml_prim_DA_Exception_PreconditionFailed {
    pub mod DA_Exception_PreconditionFailed {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct PreconditionFailed {
            #[serde(rename = "message")]
            pub message: ::std::string::String,
        }
        impl rt::ToValue for PreconditionFailed {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "message",
                    rt::ToValue::to_value(&self.message)
                ),])
            }
        }
        impl rt::FromValue for PreconditionFailed {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    message: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "message",
                    )?)
                    .map_err(|e| e.at("message"))?,
                })
            }
        }
    }
}
pub mod daml_prim_DA_Types {
    pub mod DA_Types {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum Either<A, B> {
            #[serde(rename = "Left")]
            Left(A),
            #[serde(rename = "Right")]
            Right(B),
        }
        impl<A, B> rt::ToValue for Either<A, B>
        where
            A: rt::ToValue,
            B: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                match self {
                    Either::Left(inner) => rt::variant_value("Left", rt::ToValue::to_value(inner)),
                    Either::Right(inner) => {
                        rt::variant_value("Right", rt::ToValue::to_value(inner))
                    }
                }
            }
        }
        impl<A, B> rt::FromValue for Either<A, B>
        where
            A: rt::FromValue,
            B: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                let (constructor, payload) = rt::variant_parts(value)?;
                match constructor {
                    "Left" => ::core::result::Result::Ok(Either::Left(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("Left"))?,
                    )),
                    "Right" => ::core::result::Result::Ok(Either::Right(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("Right"))?,
                    )),
                    other => {
                        ::core::result::Result::Err(rt::unexpected_constructor("Either", other))
                    }
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple2<T1, T2> {
            #[serde(rename = "_1")]
            pub _1: T1,
            #[serde(rename = "_2")]
            pub _2: T2,
        }
        impl<T1, T2> rt::ToValue for Tuple2<T1, T2>
        where
            T1: rt::ToValue,
            T2: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("_1", rt::ToValue::to_value(&self._1)),
                    ("_2", rt::ToValue::to_value(&self._2)),
                ])
            }
        }
        impl<T1, T2> rt::FromValue for Tuple2<T1, T2>
        where
            T1: rt::FromValue,
            T2: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)
                        .map_err(|e| e.at("_1"))?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)
                        .map_err(|e| e.at("_2"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple3<T1, T2, T3> {
            #[serde(rename = "_1")]
            pub _1: T1,
            #[serde(rename = "_2")]
            pub _2: T2,
            #[serde(rename = "_3")]
            pub _3: T3,
        }
        impl<T1, T2, T3> rt::ToValue for Tuple3<T1, T2, T3>
        where
            T1: rt::ToValue,
            T2: rt::ToValue,
            T3: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("_1", rt::ToValue::to_value(&self._1)),
                    ("_2", rt::ToValue::to_value(&self._2)),
                    ("_3", rt::ToValue::to_value(&self._3)),
                ])
            }
        }
        impl<T1, T2, T3> rt::FromValue for Tuple3<T1, T2, T3>
        where
            T1: rt::FromValue,
            T2: rt::FromValue,
            T3: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)
                        .map_err(|e| e.at("_1"))?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)
                        .map_err(|e| e.at("_2"))?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)
                        .map_err(|e| e.at("_3"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple4<T1, T2, T3, T4> {
            #[serde(rename = "_1")]
            pub _1: T1,
            #[serde(rename = "_2")]
            pub _2: T2,
            #[serde(rename = "_3")]
            pub _3: T3,
            #[serde(rename = "_4")]
            pub _4: T4,
        }
        impl<T1, T2, T3, T4> rt::ToValue for Tuple4<T1, T2, T3, T4>
        where
            T1: rt::ToValue,
            T2: rt::ToValue,
            T3: rt::ToValue,
            T4: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("_1", rt::ToValue::to_value(&self._1)),
                    ("_2", rt::ToValue::to_value(&self._2)),
                    ("_3", rt::ToValue::to_value(&self._3)),
                    ("_4", rt::ToValue::to_value(&self._4)),
                ])
            }
        }
        impl<T1, T2, T3, T4> rt::FromValue for Tuple4<T1, T2, T3, T4>
        where
            T1: rt::FromValue,
            T2: rt::FromValue,
            T3: rt::FromValue,
            T4: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)
                        .map_err(|e| e.at("_1"))?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)
                        .map_err(|e| e.at("_2"))?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)
                        .map_err(|e| e.at("_3"))?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)
                        .map_err(|e| e.at("_4"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple5<T1, T2, T3, T4, T5> {
            #[serde(rename = "_1")]
            pub _1: T1,
            #[serde(rename = "_2")]
            pub _2: T2,
            #[serde(rename = "_3")]
            pub _3: T3,
            #[serde(rename = "_4")]
            pub _4: T4,
            #[serde(rename = "_5")]
            pub _5: T5,
        }
        impl<T1, T2, T3, T4, T5> rt::ToValue for Tuple5<T1, T2, T3, T4, T5>
        where
            T1: rt::ToValue,
            T2: rt::ToValue,
            T3: rt::ToValue,
            T4: rt::ToValue,
            T5: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("_1", rt::ToValue::to_value(&self._1)),
                    ("_2", rt::ToValue::to_value(&self._2)),
                    ("_3", rt::ToValue::to_value(&self._3)),
                    ("_4", rt::ToValue::to_value(&self._4)),
                    ("_5", rt::ToValue::to_value(&self._5)),
                ])
            }
        }
        impl<T1, T2, T3, T4, T5> rt::FromValue for Tuple5<T1, T2, T3, T4, T5>
        where
            T1: rt::FromValue,
            T2: rt::FromValue,
            T3: rt::FromValue,
            T4: rt::FromValue,
            T5: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)
                        .map_err(|e| e.at("_1"))?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)
                        .map_err(|e| e.at("_2"))?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)
                        .map_err(|e| e.at("_3"))?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)
                        .map_err(|e| e.at("_4"))?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)
                        .map_err(|e| e.at("_5"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple6<T1, T2, T3, T4, T5, T6> {
            #[serde(rename = "_1")]
            pub _1: T1,
            #[serde(rename = "_2")]
            pub _2: T2,
            #[serde(rename = "_3")]
            pub _3: T3,
            #[serde(rename = "_4")]
            pub _4: T4,
            #[serde(rename = "_5")]
            pub _5: T5,
            #[serde(rename = "_6")]
            pub _6: T6,
        }
        impl<T1, T2, T3, T4, T5, T6> rt::ToValue for Tuple6<T1, T2, T3, T4, T5, T6>
        where
            T1: rt::ToValue,
            T2: rt::ToValue,
            T3: rt::ToValue,
            T4: rt::ToValue,
            T5: rt::ToValue,
            T6: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("_1", rt::ToValue::to_value(&self._1)),
                    ("_2", rt::ToValue::to_value(&self._2)),
                    ("_3", rt::ToValue::to_value(&self._3)),
                    ("_4", rt::ToValue::to_value(&self._4)),
                    ("_5", rt::ToValue::to_value(&self._5)),
                    ("_6", rt::ToValue::to_value(&self._6)),
                ])
            }
        }
        impl<T1, T2, T3, T4, T5, T6> rt::FromValue for Tuple6<T1, T2, T3, T4, T5, T6>
        where
            T1: rt::FromValue,
            T2: rt::FromValue,
            T3: rt::FromValue,
            T4: rt::FromValue,
            T5: rt::FromValue,
            T6: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)
                        .map_err(|e| e.at("_1"))?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)
                        .map_err(|e| e.at("_2"))?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)
                        .map_err(|e| e.at("_3"))?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)
                        .map_err(|e| e.at("_4"))?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)
                        .map_err(|e| e.at("_5"))?,
                    _6: rt::FromValue::from_value(rt::required_field(value, 5usize, "_6")?)
                        .map_err(|e| e.at("_6"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple7<T1, T2, T3, T4, T5, T6, T7> {
            #[serde(rename = "_1")]
            pub _1: T1,
            #[serde(rename = "_2")]
            pub _2: T2,
            #[serde(rename = "_3")]
            pub _3: T3,
            #[serde(rename = "_4")]
            pub _4: T4,
            #[serde(rename = "_5")]
            pub _5: T5,
            #[serde(rename = "_6")]
            pub _6: T6,
            #[serde(rename = "_7")]
            pub _7: T7,
        }
        impl<T1, T2, T3, T4, T5, T6, T7> rt::ToValue for Tuple7<T1, T2, T3, T4, T5, T6, T7>
        where
            T1: rt::ToValue,
            T2: rt::ToValue,
            T3: rt::ToValue,
            T4: rt::ToValue,
            T5: rt::ToValue,
            T6: rt::ToValue,
            T7: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("_1", rt::ToValue::to_value(&self._1)),
                    ("_2", rt::ToValue::to_value(&self._2)),
                    ("_3", rt::ToValue::to_value(&self._3)),
                    ("_4", rt::ToValue::to_value(&self._4)),
                    ("_5", rt::ToValue::to_value(&self._5)),
                    ("_6", rt::ToValue::to_value(&self._6)),
                    ("_7", rt::ToValue::to_value(&self._7)),
                ])
            }
        }
        impl<T1, T2, T3, T4, T5, T6, T7> rt::FromValue for Tuple7<T1, T2, T3, T4, T5, T6, T7>
        where
            T1: rt::FromValue,
            T2: rt::FromValue,
            T3: rt::FromValue,
            T4: rt::FromValue,
            T5: rt::FromValue,
            T6: rt::FromValue,
            T7: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)
                        .map_err(|e| e.at("_1"))?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)
                        .map_err(|e| e.at("_2"))?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)
                        .map_err(|e| e.at("_3"))?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)
                        .map_err(|e| e.at("_4"))?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)
                        .map_err(|e| e.at("_5"))?,
                    _6: rt::FromValue::from_value(rt::required_field(value, 5usize, "_6")?)
                        .map_err(|e| e.at("_6"))?,
                    _7: rt::FromValue::from_value(rt::required_field(value, 6usize, "_7")?)
                        .map_err(|e| e.at("_7"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple8<T1, T2, T3, T4, T5, T6, T7, T8> {
            #[serde(rename = "_1")]
            pub _1: T1,
            #[serde(rename = "_2")]
            pub _2: T2,
            #[serde(rename = "_3")]
            pub _3: T3,
            #[serde(rename = "_4")]
            pub _4: T4,
            #[serde(rename = "_5")]
            pub _5: T5,
            #[serde(rename = "_6")]
            pub _6: T6,
            #[serde(rename = "_7")]
            pub _7: T7,
            #[serde(rename = "_8")]
            pub _8: T8,
        }
        impl<T1, T2, T3, T4, T5, T6, T7, T8> rt::ToValue for Tuple8<T1, T2, T3, T4, T5, T6, T7, T8>
        where
            T1: rt::ToValue,
            T2: rt::ToValue,
            T3: rt::ToValue,
            T4: rt::ToValue,
            T5: rt::ToValue,
            T6: rt::ToValue,
            T7: rt::ToValue,
            T8: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("_1", rt::ToValue::to_value(&self._1)),
                    ("_2", rt::ToValue::to_value(&self._2)),
                    ("_3", rt::ToValue::to_value(&self._3)),
                    ("_4", rt::ToValue::to_value(&self._4)),
                    ("_5", rt::ToValue::to_value(&self._5)),
                    ("_6", rt::ToValue::to_value(&self._6)),
                    ("_7", rt::ToValue::to_value(&self._7)),
                    ("_8", rt::ToValue::to_value(&self._8)),
                ])
            }
        }
        impl<T1, T2, T3, T4, T5, T6, T7, T8> rt::FromValue for Tuple8<T1, T2, T3, T4, T5, T6, T7, T8>
        where
            T1: rt::FromValue,
            T2: rt::FromValue,
            T3: rt::FromValue,
            T4: rt::FromValue,
            T5: rt::FromValue,
            T6: rt::FromValue,
            T7: rt::FromValue,
            T8: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)
                        .map_err(|e| e.at("_1"))?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)
                        .map_err(|e| e.at("_2"))?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)
                        .map_err(|e| e.at("_3"))?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)
                        .map_err(|e| e.at("_4"))?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)
                        .map_err(|e| e.at("_5"))?,
                    _6: rt::FromValue::from_value(rt::required_field(value, 5usize, "_6")?)
                        .map_err(|e| e.at("_6"))?,
                    _7: rt::FromValue::from_value(rt::required_field(value, 6usize, "_7")?)
                        .map_err(|e| e.at("_7"))?,
                    _8: rt::FromValue::from_value(rt::required_field(value, 7usize, "_8")?)
                        .map_err(|e| e.at("_8"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple9<T1, T2, T3, T4, T5, T6, T7, T8, T9> {
            #[serde(rename = "_1")]
            pub _1: T1,
            #[serde(rename = "_2")]
            pub _2: T2,
            #[serde(rename = "_3")]
            pub _3: T3,
            #[serde(rename = "_4")]
            pub _4: T4,
            #[serde(rename = "_5")]
            pub _5: T5,
            #[serde(rename = "_6")]
            pub _6: T6,
            #[serde(rename = "_7")]
            pub _7: T7,
            #[serde(rename = "_8")]
            pub _8: T8,
            #[serde(rename = "_9")]
            pub _9: T9,
        }
        impl<T1, T2, T3, T4, T5, T6, T7, T8, T9> rt::ToValue for Tuple9<T1, T2, T3, T4, T5, T6, T7, T8, T9>
        where
            T1: rt::ToValue,
            T2: rt::ToValue,
            T3: rt::ToValue,
            T4: rt::ToValue,
            T5: rt::ToValue,
            T6: rt::ToValue,
            T7: rt::ToValue,
            T8: rt::ToValue,
            T9: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("_1", rt::ToValue::to_value(&self._1)),
                    ("_2", rt::ToValue::to_value(&self._2)),
                    ("_3", rt::ToValue::to_value(&self._3)),
                    ("_4", rt::ToValue::to_value(&self._4)),
                    ("_5", rt::ToValue::to_value(&self._5)),
                    ("_6", rt::ToValue::to_value(&self._6)),
                    ("_7", rt::ToValue::to_value(&self._7)),
                    ("_8", rt::ToValue::to_value(&self._8)),
                    ("_9", rt::ToValue::to_value(&self._9)),
                ])
            }
        }
        impl<T1, T2, T3, T4, T5, T6, T7, T8, T9> rt::FromValue
            for Tuple9<T1, T2, T3, T4, T5, T6, T7, T8, T9>
        where
            T1: rt::FromValue,
            T2: rt::FromValue,
            T3: rt::FromValue,
            T4: rt::FromValue,
            T5: rt::FromValue,
            T6: rt::FromValue,
            T7: rt::FromValue,
            T8: rt::FromValue,
            T9: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)
                        .map_err(|e| e.at("_1"))?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)
                        .map_err(|e| e.at("_2"))?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)
                        .map_err(|e| e.at("_3"))?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)
                        .map_err(|e| e.at("_4"))?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)
                        .map_err(|e| e.at("_5"))?,
                    _6: rt::FromValue::from_value(rt::required_field(value, 5usize, "_6")?)
                        .map_err(|e| e.at("_6"))?,
                    _7: rt::FromValue::from_value(rt::required_field(value, 6usize, "_7")?)
                        .map_err(|e| e.at("_7"))?,
                    _8: rt::FromValue::from_value(rt::required_field(value, 7usize, "_8")?)
                        .map_err(|e| e.at("_8"))?,
                    _9: rt::FromValue::from_value(rt::required_field(value, 8usize, "_9")?)
                        .map_err(|e| e.at("_9"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple10<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
            #[serde(rename = "_1")]
            pub _1: T1,
            #[serde(rename = "_2")]
            pub _2: T2,
            #[serde(rename = "_3")]
            pub _3: T3,
            #[serde(rename = "_4")]
            pub _4: T4,
            #[serde(rename = "_5")]
            pub _5: T5,
            #[serde(rename = "_6")]
            pub _6: T6,
            #[serde(rename = "_7")]
            pub _7: T7,
            #[serde(rename = "_8")]
            pub _8: T8,
            #[serde(rename = "_9")]
            pub _9: T9,
            #[serde(rename = "_10")]
            pub _10: T10,
        }
        impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> rt::ToValue
            for Tuple10<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>
        where
            T1: rt::ToValue,
            T2: rt::ToValue,
            T3: rt::ToValue,
            T4: rt::ToValue,
            T5: rt::ToValue,
            T6: rt::ToValue,
            T7: rt::ToValue,
            T8: rt::ToValue,
            T9: rt::ToValue,
            T10: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("_1", rt::ToValue::to_value(&self._1)),
                    ("_2", rt::ToValue::to_value(&self._2)),
                    ("_3", rt::ToValue::to_value(&self._3)),
                    ("_4", rt::ToValue::to_value(&self._4)),
                    ("_5", rt::ToValue::to_value(&self._5)),
                    ("_6", rt::ToValue::to_value(&self._6)),
                    ("_7", rt::ToValue::to_value(&self._7)),
                    ("_8", rt::ToValue::to_value(&self._8)),
                    ("_9", rt::ToValue::to_value(&self._9)),
                    ("_10", rt::ToValue::to_value(&self._10)),
                ])
            }
        }
        impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> rt::FromValue
            for Tuple10<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>
        where
            T1: rt::FromValue,
            T2: rt::FromValue,
            T3: rt::FromValue,
            T4: rt::FromValue,
            T5: rt::FromValue,
            T6: rt::FromValue,
            T7: rt::FromValue,
            T8: rt::FromValue,
            T9: rt::FromValue,
            T10: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)
                        .map_err(|e| e.at("_1"))?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)
                        .map_err(|e| e.at("_2"))?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)
                        .map_err(|e| e.at("_3"))?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)
                        .map_err(|e| e.at("_4"))?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)
                        .map_err(|e| e.at("_5"))?,
                    _6: rt::FromValue::from_value(rt::required_field(value, 5usize, "_6")?)
                        .map_err(|e| e.at("_6"))?,
                    _7: rt::FromValue::from_value(rt::required_field(value, 6usize, "_7")?)
                        .map_err(|e| e.at("_7"))?,
                    _8: rt::FromValue::from_value(rt::required_field(value, 7usize, "_8")?)
                        .map_err(|e| e.at("_8"))?,
                    _9: rt::FromValue::from_value(rt::required_field(value, 8usize, "_9")?)
                        .map_err(|e| e.at("_9"))?,
                    _10: rt::FromValue::from_value(rt::required_field(value, 9usize, "_10")?)
                        .map_err(|e| e.at("_10"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple11<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
            #[serde(rename = "_1")]
            pub _1: T1,
            #[serde(rename = "_2")]
            pub _2: T2,
            #[serde(rename = "_3")]
            pub _3: T3,
            #[serde(rename = "_4")]
            pub _4: T4,
            #[serde(rename = "_5")]
            pub _5: T5,
            #[serde(rename = "_6")]
            pub _6: T6,
            #[serde(rename = "_7")]
            pub _7: T7,
            #[serde(rename = "_8")]
            pub _8: T8,
            #[serde(rename = "_9")]
            pub _9: T9,
            #[serde(rename = "_10")]
            pub _10: T10,
            #[serde(rename = "_11")]
            pub _11: T11,
        }
        impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> rt::ToValue
            for Tuple11<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>
        where
            T1: rt::ToValue,
            T2: rt::ToValue,
            T3: rt::ToValue,
            T4: rt::ToValue,
            T5: rt::ToValue,
            T6: rt::ToValue,
            T7: rt::ToValue,
            T8: rt::ToValue,
            T9: rt::ToValue,
            T10: rt::ToValue,
            T11: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("_1", rt::ToValue::to_value(&self._1)),
                    ("_2", rt::ToValue::to_value(&self._2)),
                    ("_3", rt::ToValue::to_value(&self._3)),
                    ("_4", rt::ToValue::to_value(&self._4)),
                    ("_5", rt::ToValue::to_value(&self._5)),
                    ("_6", rt::ToValue::to_value(&self._6)),
                    ("_7", rt::ToValue::to_value(&self._7)),
                    ("_8", rt::ToValue::to_value(&self._8)),
                    ("_9", rt::ToValue::to_value(&self._9)),
                    ("_10", rt::ToValue::to_value(&self._10)),
                    ("_11", rt::ToValue::to_value(&self._11)),
                ])
            }
        }
        impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> rt::FromValue
            for Tuple11<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11>
        where
            T1: rt::FromValue,
            T2: rt::FromValue,
            T3: rt::FromValue,
            T4: rt::FromValue,
            T5: rt::FromValue,
            T6: rt::FromValue,
            T7: rt::FromValue,
            T8: rt::FromValue,
            T9: rt::FromValue,
            T10: rt::FromValue,
            T11: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)
                        .map_err(|e| e.at("_1"))?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)
                        .map_err(|e| e.at("_2"))?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)
                        .map_err(|e| e.at("_3"))?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)
                        .map_err(|e| e.at("_4"))?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)
                        .map_err(|e| e.at("_5"))?,
                    _6: rt::FromValue::from_value(rt::required_field(value, 5usize, "_6")?)
                        .map_err(|e| e.at("_6"))?,
                    _7: rt::FromValue::from_value(rt::required_field(value, 6usize, "_7")?)
                        .map_err(|e| e.at("_7"))?,
                    _8: rt::FromValue::from_value(rt::required_field(value, 7usize, "_8")?)
                        .map_err(|e| e.at("_8"))?,
                    _9: rt::FromValue::from_value(rt::required_field(value, 8usize, "_9")?)
                        .map_err(|e| e.at("_9"))?,
                    _10: rt::FromValue::from_value(rt::required_field(value, 9usize, "_10")?)
                        .map_err(|e| e.at("_10"))?,
                    _11: rt::FromValue::from_value(rt::required_field(value, 10usize, "_11")?)
                        .map_err(|e| e.at("_11"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple12<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
            #[serde(rename = "_1")]
            pub _1: T1,
            #[serde(rename = "_2")]
            pub _2: T2,
            #[serde(rename = "_3")]
            pub _3: T3,
            #[serde(rename = "_4")]
            pub _4: T4,
            #[serde(rename = "_5")]
            pub _5: T5,
            #[serde(rename = "_6")]
            pub _6: T6,
            #[serde(rename = "_7")]
            pub _7: T7,
            #[serde(rename = "_8")]
            pub _8: T8,
            #[serde(rename = "_9")]
            pub _9: T9,
            #[serde(rename = "_10")]
            pub _10: T10,
            #[serde(rename = "_11")]
            pub _11: T11,
            #[serde(rename = "_12")]
            pub _12: T12,
        }
        impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> rt::ToValue
            for Tuple12<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>
        where
            T1: rt::ToValue,
            T2: rt::ToValue,
            T3: rt::ToValue,
            T4: rt::ToValue,
            T5: rt::ToValue,
            T6: rt::ToValue,
            T7: rt::ToValue,
            T8: rt::ToValue,
            T9: rt::ToValue,
            T10: rt::ToValue,
            T11: rt::ToValue,
            T12: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("_1", rt::ToValue::to_value(&self._1)),
                    ("_2", rt::ToValue::to_value(&self._2)),
                    ("_3", rt::ToValue::to_value(&self._3)),
                    ("_4", rt::ToValue::to_value(&self._4)),
                    ("_5", rt::ToValue::to_value(&self._5)),
                    ("_6", rt::ToValue::to_value(&self._6)),
                    ("_7", rt::ToValue::to_value(&self._7)),
                    ("_8", rt::ToValue::to_value(&self._8)),
                    ("_9", rt::ToValue::to_value(&self._9)),
                    ("_10", rt::ToValue::to_value(&self._10)),
                    ("_11", rt::ToValue::to_value(&self._11)),
                    ("_12", rt::ToValue::to_value(&self._12)),
                ])
            }
        }
        impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> rt::FromValue
            for Tuple12<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12>
        where
            T1: rt::FromValue,
            T2: rt::FromValue,
            T3: rt::FromValue,
            T4: rt::FromValue,
            T5: rt::FromValue,
            T6: rt::FromValue,
            T7: rt::FromValue,
            T8: rt::FromValue,
            T9: rt::FromValue,
            T10: rt::FromValue,
            T11: rt::FromValue,
            T12: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)
                        .map_err(|e| e.at("_1"))?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)
                        .map_err(|e| e.at("_2"))?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)
                        .map_err(|e| e.at("_3"))?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)
                        .map_err(|e| e.at("_4"))?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)
                        .map_err(|e| e.at("_5"))?,
                    _6: rt::FromValue::from_value(rt::required_field(value, 5usize, "_6")?)
                        .map_err(|e| e.at("_6"))?,
                    _7: rt::FromValue::from_value(rt::required_field(value, 6usize, "_7")?)
                        .map_err(|e| e.at("_7"))?,
                    _8: rt::FromValue::from_value(rt::required_field(value, 7usize, "_8")?)
                        .map_err(|e| e.at("_8"))?,
                    _9: rt::FromValue::from_value(rt::required_field(value, 8usize, "_9")?)
                        .map_err(|e| e.at("_9"))?,
                    _10: rt::FromValue::from_value(rt::required_field(value, 9usize, "_10")?)
                        .map_err(|e| e.at("_10"))?,
                    _11: rt::FromValue::from_value(rt::required_field(value, 10usize, "_11")?)
                        .map_err(|e| e.at("_11"))?,
                    _12: rt::FromValue::from_value(rt::required_field(value, 11usize, "_12")?)
                        .map_err(|e| e.at("_12"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple13<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
            #[serde(rename = "_1")]
            pub _1: T1,
            #[serde(rename = "_2")]
            pub _2: T2,
            #[serde(rename = "_3")]
            pub _3: T3,
            #[serde(rename = "_4")]
            pub _4: T4,
            #[serde(rename = "_5")]
            pub _5: T5,
            #[serde(rename = "_6")]
            pub _6: T6,
            #[serde(rename = "_7")]
            pub _7: T7,
            #[serde(rename = "_8")]
            pub _8: T8,
            #[serde(rename = "_9")]
            pub _9: T9,
            #[serde(rename = "_10")]
            pub _10: T10,
            #[serde(rename = "_11")]
            pub _11: T11,
            #[serde(rename = "_12")]
            pub _12: T12,
            #[serde(rename = "_13")]
            pub _13: T13,
        }
        impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> rt::ToValue
            for Tuple13<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>
        where
            T1: rt::ToValue,
            T2: rt::ToValue,
            T3: rt::ToValue,
            T4: rt::ToValue,
            T5: rt::ToValue,
            T6: rt::ToValue,
            T7: rt::ToValue,
            T8: rt::ToValue,
            T9: rt::ToValue,
            T10: rt::ToValue,
            T11: rt::ToValue,
            T12: rt::ToValue,
            T13: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("_1", rt::ToValue::to_value(&self._1)),
                    ("_2", rt::ToValue::to_value(&self._2)),
                    ("_3", rt::ToValue::to_value(&self._3)),
                    ("_4", rt::ToValue::to_value(&self._4)),
                    ("_5", rt::ToValue::to_value(&self._5)),
                    ("_6", rt::ToValue::to_value(&self._6)),
                    ("_7", rt::ToValue::to_value(&self._7)),
                    ("_8", rt::ToValue::to_value(&self._8)),
                    ("_9", rt::ToValue::to_value(&self._9)),
                    ("_10", rt::ToValue::to_value(&self._10)),
                    ("_11", rt::ToValue::to_value(&self._11)),
                    ("_12", rt::ToValue::to_value(&self._12)),
                    ("_13", rt::ToValue::to_value(&self._13)),
                ])
            }
        }
        impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> rt::FromValue
            for Tuple13<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>
        where
            T1: rt::FromValue,
            T2: rt::FromValue,
            T3: rt::FromValue,
            T4: rt::FromValue,
            T5: rt::FromValue,
            T6: rt::FromValue,
            T7: rt::FromValue,
            T8: rt::FromValue,
            T9: rt::FromValue,
            T10: rt::FromValue,
            T11: rt::FromValue,
            T12: rt::FromValue,
            T13: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)
                        .map_err(|e| e.at("_1"))?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)
                        .map_err(|e| e.at("_2"))?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)
                        .map_err(|e| e.at("_3"))?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)
                        .map_err(|e| e.at("_4"))?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)
                        .map_err(|e| e.at("_5"))?,
                    _6: rt::FromValue::from_value(rt::required_field(value, 5usize, "_6")?)
                        .map_err(|e| e.at("_6"))?,
                    _7: rt::FromValue::from_value(rt::required_field(value, 6usize, "_7")?)
                        .map_err(|e| e.at("_7"))?,
                    _8: rt::FromValue::from_value(rt::required_field(value, 7usize, "_8")?)
                        .map_err(|e| e.at("_8"))?,
                    _9: rt::FromValue::from_value(rt::required_field(value, 8usize, "_9")?)
                        .map_err(|e| e.at("_9"))?,
                    _10: rt::FromValue::from_value(rt::required_field(value, 9usize, "_10")?)
                        .map_err(|e| e.at("_10"))?,
                    _11: rt::FromValue::from_value(rt::required_field(value, 10usize, "_11")?)
                        .map_err(|e| e.at("_11"))?,
                    _12: rt::FromValue::from_value(rt::required_field(value, 11usize, "_12")?)
                        .map_err(|e| e.at("_12"))?,
                    _13: rt::FromValue::from_value(rt::required_field(value, 12usize, "_13")?)
                        .map_err(|e| e.at("_13"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple14<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
            #[serde(rename = "_1")]
            pub _1: T1,
            #[serde(rename = "_2")]
            pub _2: T2,
            #[serde(rename = "_3")]
            pub _3: T3,
            #[serde(rename = "_4")]
            pub _4: T4,
            #[serde(rename = "_5")]
            pub _5: T5,
            #[serde(rename = "_6")]
            pub _6: T6,
            #[serde(rename = "_7")]
            pub _7: T7,
            #[serde(rename = "_8")]
            pub _8: T8,
            #[serde(rename = "_9")]
            pub _9: T9,
            #[serde(rename = "_10")]
            pub _10: T10,
            #[serde(rename = "_11")]
            pub _11: T11,
            #[serde(rename = "_12")]
            pub _12: T12,
            #[serde(rename = "_13")]
            pub _13: T13,
            #[serde(rename = "_14")]
            pub _14: T14,
        }
        impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> rt::ToValue
            for Tuple14<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14>
        where
            T1: rt::ToValue,
            T2: rt::ToValue,
            T3: rt::ToValue,
            T4: rt::ToValue,
            T5: rt::ToValue,
            T6: rt::ToValue,
            T7: rt::ToValue,
            T8: rt::ToValue,
            T9: rt::ToValue,
            T10: rt::ToValue,
            T11: rt::ToValue,
            T12: rt::ToValue,
            T13: rt::ToValue,
            T14: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("_1", rt::ToValue::to_value(&self._1)),
                    ("_2", rt::ToValue::to_value(&self._2)),
                    ("_3", rt::ToValue::to_value(&self._3)),
                    ("_4", rt::ToValue::to_value(&self._4)),
                    ("_5", rt::ToValue::to_value(&self._5)),
                    ("_6", rt::ToValue::to_value(&self._6)),
                    ("_7", rt::ToValue::to_value(&self._7)),
                    ("_8", rt::ToValue::to_value(&self._8)),
                    ("_9", rt::ToValue::to_value(&self._9)),
                    ("_10", rt::ToValue::to_value(&self._10)),
                    ("_11", rt::ToValue::to_value(&self._11)),
                    ("_12", rt::ToValue::to_value(&self._12)),
                    ("_13", rt::ToValue::to_value(&self._13)),
                    ("_14", rt::ToValue::to_value(&self._14)),
                ])
            }
        }
        impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> rt::FromValue
            for Tuple14<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14>
        where
            T1: rt::FromValue,
            T2: rt::FromValue,
            T3: rt::FromValue,
            T4: rt::FromValue,
            T5: rt::FromValue,
            T6: rt::FromValue,
            T7: rt::FromValue,
            T8: rt::FromValue,
            T9: rt::FromValue,
            T10: rt::FromValue,
            T11: rt::FromValue,
            T12: rt::FromValue,
            T13: rt::FromValue,
            T14: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)
                        .map_err(|e| e.at("_1"))?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)
                        .map_err(|e| e.at("_2"))?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)
                        .map_err(|e| e.at("_3"))?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)
                        .map_err(|e| e.at("_4"))?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)
                        .map_err(|e| e.at("_5"))?,
                    _6: rt::FromValue::from_value(rt::required_field(value, 5usize, "_6")?)
                        .map_err(|e| e.at("_6"))?,
                    _7: rt::FromValue::from_value(rt::required_field(value, 6usize, "_7")?)
                        .map_err(|e| e.at("_7"))?,
                    _8: rt::FromValue::from_value(rt::required_field(value, 7usize, "_8")?)
                        .map_err(|e| e.at("_8"))?,
                    _9: rt::FromValue::from_value(rt::required_field(value, 8usize, "_9")?)
                        .map_err(|e| e.at("_9"))?,
                    _10: rt::FromValue::from_value(rt::required_field(value, 9usize, "_10")?)
                        .map_err(|e| e.at("_10"))?,
                    _11: rt::FromValue::from_value(rt::required_field(value, 10usize, "_11")?)
                        .map_err(|e| e.at("_11"))?,
                    _12: rt::FromValue::from_value(rt::required_field(value, 11usize, "_12")?)
                        .map_err(|e| e.at("_12"))?,
                    _13: rt::FromValue::from_value(rt::required_field(value, 12usize, "_13")?)
                        .map_err(|e| e.at("_13"))?,
                    _14: rt::FromValue::from_value(rt::required_field(value, 13usize, "_14")?)
                        .map_err(|e| e.at("_14"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple15<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
            #[serde(rename = "_1")]
            pub _1: T1,
            #[serde(rename = "_2")]
            pub _2: T2,
            #[serde(rename = "_3")]
            pub _3: T3,
            #[serde(rename = "_4")]
            pub _4: T4,
            #[serde(rename = "_5")]
            pub _5: T5,
            #[serde(rename = "_6")]
            pub _6: T6,
            #[serde(rename = "_7")]
            pub _7: T7,
            #[serde(rename = "_8")]
            pub _8: T8,
            #[serde(rename = "_9")]
            pub _9: T9,
            #[serde(rename = "_10")]
            pub _10: T10,
            #[serde(rename = "_11")]
            pub _11: T11,
            #[serde(rename = "_12")]
            pub _12: T12,
            #[serde(rename = "_13")]
            pub _13: T13,
            #[serde(rename = "_14")]
            pub _14: T14,
            #[serde(rename = "_15")]
            pub _15: T15,
        }
        impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> rt::ToValue
            for Tuple15<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15>
        where
            T1: rt::ToValue,
            T2: rt::ToValue,
            T3: rt::ToValue,
            T4: rt::ToValue,
            T5: rt::ToValue,
            T6: rt::ToValue,
            T7: rt::ToValue,
            T8: rt::ToValue,
            T9: rt::ToValue,
            T10: rt::ToValue,
            T11: rt::ToValue,
            T12: rt::ToValue,
            T13: rt::ToValue,
            T14: rt::ToValue,
            T15: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("_1", rt::ToValue::to_value(&self._1)),
                    ("_2", rt::ToValue::to_value(&self._2)),
                    ("_3", rt::ToValue::to_value(&self._3)),
                    ("_4", rt::ToValue::to_value(&self._4)),
                    ("_5", rt::ToValue::to_value(&self._5)),
                    ("_6", rt::ToValue::to_value(&self._6)),
                    ("_7", rt::ToValue::to_value(&self._7)),
                    ("_8", rt::ToValue::to_value(&self._8)),
                    ("_9", rt::ToValue::to_value(&self._9)),
                    ("_10", rt::ToValue::to_value(&self._10)),
                    ("_11", rt::ToValue::to_value(&self._11)),
                    ("_12", rt::ToValue::to_value(&self._12)),
                    ("_13", rt::ToValue::to_value(&self._13)),
                    ("_14", rt::ToValue::to_value(&self._14)),
                    ("_15", rt::ToValue::to_value(&self._15)),
                ])
            }
        }
        impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> rt::FromValue
            for Tuple15<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15>
        where
            T1: rt::FromValue,
            T2: rt::FromValue,
            T3: rt::FromValue,
            T4: rt::FromValue,
            T5: rt::FromValue,
            T6: rt::FromValue,
            T7: rt::FromValue,
            T8: rt::FromValue,
            T9: rt::FromValue,
            T10: rt::FromValue,
            T11: rt::FromValue,
            T12: rt::FromValue,
            T13: rt::FromValue,
            T14: rt::FromValue,
            T15: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)
                        .map_err(|e| e.at("_1"))?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)
                        .map_err(|e| e.at("_2"))?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)
                        .map_err(|e| e.at("_3"))?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)
                        .map_err(|e| e.at("_4"))?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)
                        .map_err(|e| e.at("_5"))?,
                    _6: rt::FromValue::from_value(rt::required_field(value, 5usize, "_6")?)
                        .map_err(|e| e.at("_6"))?,
                    _7: rt::FromValue::from_value(rt::required_field(value, 6usize, "_7")?)
                        .map_err(|e| e.at("_7"))?,
                    _8: rt::FromValue::from_value(rt::required_field(value, 7usize, "_8")?)
                        .map_err(|e| e.at("_8"))?,
                    _9: rt::FromValue::from_value(rt::required_field(value, 8usize, "_9")?)
                        .map_err(|e| e.at("_9"))?,
                    _10: rt::FromValue::from_value(rt::required_field(value, 9usize, "_10")?)
                        .map_err(|e| e.at("_10"))?,
                    _11: rt::FromValue::from_value(rt::required_field(value, 10usize, "_11")?)
                        .map_err(|e| e.at("_11"))?,
                    _12: rt::FromValue::from_value(rt::required_field(value, 11usize, "_12")?)
                        .map_err(|e| e.at("_12"))?,
                    _13: rt::FromValue::from_value(rt::required_field(value, 12usize, "_13")?)
                        .map_err(|e| e.at("_13"))?,
                    _14: rt::FromValue::from_value(rt::required_field(value, 13usize, "_14")?)
                        .map_err(|e| e.at("_14"))?,
                    _15: rt::FromValue::from_value(rt::required_field(value, 14usize, "_15")?)
                        .map_err(|e| e.at("_15"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple16<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> {
            #[serde(rename = "_1")]
            pub _1: T1,
            #[serde(rename = "_2")]
            pub _2: T2,
            #[serde(rename = "_3")]
            pub _3: T3,
            #[serde(rename = "_4")]
            pub _4: T4,
            #[serde(rename = "_5")]
            pub _5: T5,
            #[serde(rename = "_6")]
            pub _6: T6,
            #[serde(rename = "_7")]
            pub _7: T7,
            #[serde(rename = "_8")]
            pub _8: T8,
            #[serde(rename = "_9")]
            pub _9: T9,
            #[serde(rename = "_10")]
            pub _10: T10,
            #[serde(rename = "_11")]
            pub _11: T11,
            #[serde(rename = "_12")]
            pub _12: T12,
            #[serde(rename = "_13")]
            pub _13: T13,
            #[serde(rename = "_14")]
            pub _14: T14,
            #[serde(rename = "_15")]
            pub _15: T15,
            #[serde(rename = "_16")]
            pub _16: T16,
        }
        impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> rt::ToValue
            for Tuple16<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16>
        where
            T1: rt::ToValue,
            T2: rt::ToValue,
            T3: rt::ToValue,
            T4: rt::ToValue,
            T5: rt::ToValue,
            T6: rt::ToValue,
            T7: rt::ToValue,
            T8: rt::ToValue,
            T9: rt::ToValue,
            T10: rt::ToValue,
            T11: rt::ToValue,
            T12: rt::ToValue,
            T13: rt::ToValue,
            T14: rt::ToValue,
            T15: rt::ToValue,
            T16: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("_1", rt::ToValue::to_value(&self._1)),
                    ("_2", rt::ToValue::to_value(&self._2)),
                    ("_3", rt::ToValue::to_value(&self._3)),
                    ("_4", rt::ToValue::to_value(&self._4)),
                    ("_5", rt::ToValue::to_value(&self._5)),
                    ("_6", rt::ToValue::to_value(&self._6)),
                    ("_7", rt::ToValue::to_value(&self._7)),
                    ("_8", rt::ToValue::to_value(&self._8)),
                    ("_9", rt::ToValue::to_value(&self._9)),
                    ("_10", rt::ToValue::to_value(&self._10)),
                    ("_11", rt::ToValue::to_value(&self._11)),
                    ("_12", rt::ToValue::to_value(&self._12)),
                    ("_13", rt::ToValue::to_value(&self._13)),
                    ("_14", rt::ToValue::to_value(&self._14)),
                    ("_15", rt::ToValue::to_value(&self._15)),
                    ("_16", rt::ToValue::to_value(&self._16)),
                ])
            }
        }
        impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> rt::FromValue
            for Tuple16<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16>
        where
            T1: rt::FromValue,
            T2: rt::FromValue,
            T3: rt::FromValue,
            T4: rt::FromValue,
            T5: rt::FromValue,
            T6: rt::FromValue,
            T7: rt::FromValue,
            T8: rt::FromValue,
            T9: rt::FromValue,
            T10: rt::FromValue,
            T11: rt::FromValue,
            T12: rt::FromValue,
            T13: rt::FromValue,
            T14: rt::FromValue,
            T15: rt::FromValue,
            T16: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)
                        .map_err(|e| e.at("_1"))?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)
                        .map_err(|e| e.at("_2"))?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)
                        .map_err(|e| e.at("_3"))?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)
                        .map_err(|e| e.at("_4"))?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)
                        .map_err(|e| e.at("_5"))?,
                    _6: rt::FromValue::from_value(rt::required_field(value, 5usize, "_6")?)
                        .map_err(|e| e.at("_6"))?,
                    _7: rt::FromValue::from_value(rt::required_field(value, 6usize, "_7")?)
                        .map_err(|e| e.at("_7"))?,
                    _8: rt::FromValue::from_value(rt::required_field(value, 7usize, "_8")?)
                        .map_err(|e| e.at("_8"))?,
                    _9: rt::FromValue::from_value(rt::required_field(value, 8usize, "_9")?)
                        .map_err(|e| e.at("_9"))?,
                    _10: rt::FromValue::from_value(rt::required_field(value, 9usize, "_10")?)
                        .map_err(|e| e.at("_10"))?,
                    _11: rt::FromValue::from_value(rt::required_field(value, 10usize, "_11")?)
                        .map_err(|e| e.at("_11"))?,
                    _12: rt::FromValue::from_value(rt::required_field(value, 11usize, "_12")?)
                        .map_err(|e| e.at("_12"))?,
                    _13: rt::FromValue::from_value(rt::required_field(value, 12usize, "_13")?)
                        .map_err(|e| e.at("_13"))?,
                    _14: rt::FromValue::from_value(rt::required_field(value, 13usize, "_14")?)
                        .map_err(|e| e.at("_14"))?,
                    _15: rt::FromValue::from_value(rt::required_field(value, 14usize, "_15")?)
                        .map_err(|e| e.at("_15"))?,
                    _16: rt::FromValue::from_value(rt::required_field(value, 15usize, "_16")?)
                        .map_err(|e| e.at("_16"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple17<
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            T15,
            T16,
            T17,
        > {
            #[serde(rename = "_1")]
            pub _1: T1,
            #[serde(rename = "_2")]
            pub _2: T2,
            #[serde(rename = "_3")]
            pub _3: T3,
            #[serde(rename = "_4")]
            pub _4: T4,
            #[serde(rename = "_5")]
            pub _5: T5,
            #[serde(rename = "_6")]
            pub _6: T6,
            #[serde(rename = "_7")]
            pub _7: T7,
            #[serde(rename = "_8")]
            pub _8: T8,
            #[serde(rename = "_9")]
            pub _9: T9,
            #[serde(rename = "_10")]
            pub _10: T10,
            #[serde(rename = "_11")]
            pub _11: T11,
            #[serde(rename = "_12")]
            pub _12: T12,
            #[serde(rename = "_13")]
            pub _13: T13,
            #[serde(rename = "_14")]
            pub _14: T14,
            #[serde(rename = "_15")]
            pub _15: T15,
            #[serde(rename = "_16")]
            pub _16: T16,
            #[serde(rename = "_17")]
            pub _17: T17,
        }
        impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17> rt::ToValue
            for Tuple17<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17>
        where
            T1: rt::ToValue,
            T2: rt::ToValue,
            T3: rt::ToValue,
            T4: rt::ToValue,
            T5: rt::ToValue,
            T6: rt::ToValue,
            T7: rt::ToValue,
            T8: rt::ToValue,
            T9: rt::ToValue,
            T10: rt::ToValue,
            T11: rt::ToValue,
            T12: rt::ToValue,
            T13: rt::ToValue,
            T14: rt::ToValue,
            T15: rt::ToValue,
            T16: rt::ToValue,
            T17: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("_1", rt::ToValue::to_value(&self._1)),
                    ("_2", rt::ToValue::to_value(&self._2)),
                    ("_3", rt::ToValue::to_value(&self._3)),
                    ("_4", rt::ToValue::to_value(&self._4)),
                    ("_5", rt::ToValue::to_value(&self._5)),
                    ("_6", rt::ToValue::to_value(&self._6)),
                    ("_7", rt::ToValue::to_value(&self._7)),
                    ("_8", rt::ToValue::to_value(&self._8)),
                    ("_9", rt::ToValue::to_value(&self._9)),
                    ("_10", rt::ToValue::to_value(&self._10)),
                    ("_11", rt::ToValue::to_value(&self._11)),
                    ("_12", rt::ToValue::to_value(&self._12)),
                    ("_13", rt::ToValue::to_value(&self._13)),
                    ("_14", rt::ToValue::to_value(&self._14)),
                    ("_15", rt::ToValue::to_value(&self._15)),
                    ("_16", rt::ToValue::to_value(&self._16)),
                    ("_17", rt::ToValue::to_value(&self._17)),
                ])
            }
        }
        impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17>
            rt::FromValue
            for Tuple17<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17>
        where
            T1: rt::FromValue,
            T2: rt::FromValue,
            T3: rt::FromValue,
            T4: rt::FromValue,
            T5: rt::FromValue,
            T6: rt::FromValue,
            T7: rt::FromValue,
            T8: rt::FromValue,
            T9: rt::FromValue,
            T10: rt::FromValue,
            T11: rt::FromValue,
            T12: rt::FromValue,
            T13: rt::FromValue,
            T14: rt::FromValue,
            T15: rt::FromValue,
            T16: rt::FromValue,
            T17: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)
                        .map_err(|e| e.at("_1"))?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)
                        .map_err(|e| e.at("_2"))?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)
                        .map_err(|e| e.at("_3"))?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)
                        .map_err(|e| e.at("_4"))?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)
                        .map_err(|e| e.at("_5"))?,
                    _6: rt::FromValue::from_value(rt::required_field(value, 5usize, "_6")?)
                        .map_err(|e| e.at("_6"))?,
                    _7: rt::FromValue::from_value(rt::required_field(value, 6usize, "_7")?)
                        .map_err(|e| e.at("_7"))?,
                    _8: rt::FromValue::from_value(rt::required_field(value, 7usize, "_8")?)
                        .map_err(|e| e.at("_8"))?,
                    _9: rt::FromValue::from_value(rt::required_field(value, 8usize, "_9")?)
                        .map_err(|e| e.at("_9"))?,
                    _10: rt::FromValue::from_value(rt::required_field(value, 9usize, "_10")?)
                        .map_err(|e| e.at("_10"))?,
                    _11: rt::FromValue::from_value(rt::required_field(value, 10usize, "_11")?)
                        .map_err(|e| e.at("_11"))?,
                    _12: rt::FromValue::from_value(rt::required_field(value, 11usize, "_12")?)
                        .map_err(|e| e.at("_12"))?,
                    _13: rt::FromValue::from_value(rt::required_field(value, 12usize, "_13")?)
                        .map_err(|e| e.at("_13"))?,
                    _14: rt::FromValue::from_value(rt::required_field(value, 13usize, "_14")?)
                        .map_err(|e| e.at("_14"))?,
                    _15: rt::FromValue::from_value(rt::required_field(value, 14usize, "_15")?)
                        .map_err(|e| e.at("_15"))?,
                    _16: rt::FromValue::from_value(rt::required_field(value, 15usize, "_16")?)
                        .map_err(|e| e.at("_16"))?,
                    _17: rt::FromValue::from_value(rt::required_field(value, 16usize, "_17")?)
                        .map_err(|e| e.at("_17"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple18<
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            T15,
            T16,
            T17,
            T18,
        > {
            #[serde(rename = "_1")]
            pub _1: T1,
            #[serde(rename = "_2")]
            pub _2: T2,
            #[serde(rename = "_3")]
            pub _3: T3,
            #[serde(rename = "_4")]
            pub _4: T4,
            #[serde(rename = "_5")]
            pub _5: T5,
            #[serde(rename = "_6")]
            pub _6: T6,
            #[serde(rename = "_7")]
            pub _7: T7,
            #[serde(rename = "_8")]
            pub _8: T8,
            #[serde(rename = "_9")]
            pub _9: T9,
            #[serde(rename = "_10")]
            pub _10: T10,
            #[serde(rename = "_11")]
            pub _11: T11,
            #[serde(rename = "_12")]
            pub _12: T12,
            #[serde(rename = "_13")]
            pub _13: T13,
            #[serde(rename = "_14")]
            pub _14: T14,
            #[serde(rename = "_15")]
            pub _15: T15,
            #[serde(rename = "_16")]
            pub _16: T16,
            #[serde(rename = "_17")]
            pub _17: T17,
            #[serde(rename = "_18")]
            pub _18: T18,
        }
        impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18>
            rt::ToValue
            for Tuple18<
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
            >
        where
            T1: rt::ToValue,
            T2: rt::ToValue,
            T3: rt::ToValue,
            T4: rt::ToValue,
            T5: rt::ToValue,
            T6: rt::ToValue,
            T7: rt::ToValue,
            T8: rt::ToValue,
            T9: rt::ToValue,
            T10: rt::ToValue,
            T11: rt::ToValue,
            T12: rt::ToValue,
            T13: rt::ToValue,
            T14: rt::ToValue,
            T15: rt::ToValue,
            T16: rt::ToValue,
            T17: rt::ToValue,
            T18: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("_1", rt::ToValue::to_value(&self._1)),
                    ("_2", rt::ToValue::to_value(&self._2)),
                    ("_3", rt::ToValue::to_value(&self._3)),
                    ("_4", rt::ToValue::to_value(&self._4)),
                    ("_5", rt::ToValue::to_value(&self._5)),
                    ("_6", rt::ToValue::to_value(&self._6)),
                    ("_7", rt::ToValue::to_value(&self._7)),
                    ("_8", rt::ToValue::to_value(&self._8)),
                    ("_9", rt::ToValue::to_value(&self._9)),
                    ("_10", rt::ToValue::to_value(&self._10)),
                    ("_11", rt::ToValue::to_value(&self._11)),
                    ("_12", rt::ToValue::to_value(&self._12)),
                    ("_13", rt::ToValue::to_value(&self._13)),
                    ("_14", rt::ToValue::to_value(&self._14)),
                    ("_15", rt::ToValue::to_value(&self._15)),
                    ("_16", rt::ToValue::to_value(&self._16)),
                    ("_17", rt::ToValue::to_value(&self._17)),
                    ("_18", rt::ToValue::to_value(&self._18)),
                ])
            }
        }
        impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18>
            rt::FromValue
            for Tuple18<
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
            >
        where
            T1: rt::FromValue,
            T2: rt::FromValue,
            T3: rt::FromValue,
            T4: rt::FromValue,
            T5: rt::FromValue,
            T6: rt::FromValue,
            T7: rt::FromValue,
            T8: rt::FromValue,
            T9: rt::FromValue,
            T10: rt::FromValue,
            T11: rt::FromValue,
            T12: rt::FromValue,
            T13: rt::FromValue,
            T14: rt::FromValue,
            T15: rt::FromValue,
            T16: rt::FromValue,
            T17: rt::FromValue,
            T18: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)
                        .map_err(|e| e.at("_1"))?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)
                        .map_err(|e| e.at("_2"))?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)
                        .map_err(|e| e.at("_3"))?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)
                        .map_err(|e| e.at("_4"))?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)
                        .map_err(|e| e.at("_5"))?,
                    _6: rt::FromValue::from_value(rt::required_field(value, 5usize, "_6")?)
                        .map_err(|e| e.at("_6"))?,
                    _7: rt::FromValue::from_value(rt::required_field(value, 6usize, "_7")?)
                        .map_err(|e| e.at("_7"))?,
                    _8: rt::FromValue::from_value(rt::required_field(value, 7usize, "_8")?)
                        .map_err(|e| e.at("_8"))?,
                    _9: rt::FromValue::from_value(rt::required_field(value, 8usize, "_9")?)
                        .map_err(|e| e.at("_9"))?,
                    _10: rt::FromValue::from_value(rt::required_field(value, 9usize, "_10")?)
                        .map_err(|e| e.at("_10"))?,
                    _11: rt::FromValue::from_value(rt::required_field(value, 10usize, "_11")?)
                        .map_err(|e| e.at("_11"))?,
                    _12: rt::FromValue::from_value(rt::required_field(value, 11usize, "_12")?)
                        .map_err(|e| e.at("_12"))?,
                    _13: rt::FromValue::from_value(rt::required_field(value, 12usize, "_13")?)
                        .map_err(|e| e.at("_13"))?,
                    _14: rt::FromValue::from_value(rt::required_field(value, 13usize, "_14")?)
                        .map_err(|e| e.at("_14"))?,
                    _15: rt::FromValue::from_value(rt::required_field(value, 14usize, "_15")?)
                        .map_err(|e| e.at("_15"))?,
                    _16: rt::FromValue::from_value(rt::required_field(value, 15usize, "_16")?)
                        .map_err(|e| e.at("_16"))?,
                    _17: rt::FromValue::from_value(rt::required_field(value, 16usize, "_17")?)
                        .map_err(|e| e.at("_17"))?,
                    _18: rt::FromValue::from_value(rt::required_field(value, 17usize, "_18")?)
                        .map_err(|e| e.at("_18"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple19<
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            T15,
            T16,
            T17,
            T18,
            T19,
        > {
            #[serde(rename = "_1")]
            pub _1: T1,
            #[serde(rename = "_2")]
            pub _2: T2,
            #[serde(rename = "_3")]
            pub _3: T3,
            #[serde(rename = "_4")]
            pub _4: T4,
            #[serde(rename = "_5")]
            pub _5: T5,
            #[serde(rename = "_6")]
            pub _6: T6,
            #[serde(rename = "_7")]
            pub _7: T7,
            #[serde(rename = "_8")]
            pub _8: T8,
            #[serde(rename = "_9")]
            pub _9: T9,
            #[serde(rename = "_10")]
            pub _10: T10,
            #[serde(rename = "_11")]
            pub _11: T11,
            #[serde(rename = "_12")]
            pub _12: T12,
            #[serde(rename = "_13")]
            pub _13: T13,
            #[serde(rename = "_14")]
            pub _14: T14,
            #[serde(rename = "_15")]
            pub _15: T15,
            #[serde(rename = "_16")]
            pub _16: T16,
            #[serde(rename = "_17")]
            pub _17: T17,
            #[serde(rename = "_18")]
            pub _18: T18,
            #[serde(rename = "_19")]
            pub _19: T19,
        }
        impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19>
            rt::ToValue
            for Tuple19<
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
            >
        where
            T1: rt::ToValue,
            T2: rt::ToValue,
            T3: rt::ToValue,
            T4: rt::ToValue,
            T5: rt::ToValue,
            T6: rt::ToValue,
            T7: rt::ToValue,
            T8: rt::ToValue,
            T9: rt::ToValue,
            T10: rt::ToValue,
            T11: rt::ToValue,
            T12: rt::ToValue,
            T13: rt::ToValue,
            T14: rt::ToValue,
            T15: rt::ToValue,
            T16: rt::ToValue,
            T17: rt::ToValue,
            T18: rt::ToValue,
            T19: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("_1", rt::ToValue::to_value(&self._1)),
                    ("_2", rt::ToValue::to_value(&self._2)),
                    ("_3", rt::ToValue::to_value(&self._3)),
                    ("_4", rt::ToValue::to_value(&self._4)),
                    ("_5", rt::ToValue::to_value(&self._5)),
                    ("_6", rt::ToValue::to_value(&self._6)),
                    ("_7", rt::ToValue::to_value(&self._7)),
                    ("_8", rt::ToValue::to_value(&self._8)),
                    ("_9", rt::ToValue::to_value(&self._9)),
                    ("_10", rt::ToValue::to_value(&self._10)),
                    ("_11", rt::ToValue::to_value(&self._11)),
                    ("_12", rt::ToValue::to_value(&self._12)),
                    ("_13", rt::ToValue::to_value(&self._13)),
                    ("_14", rt::ToValue::to_value(&self._14)),
                    ("_15", rt::ToValue::to_value(&self._15)),
                    ("_16", rt::ToValue::to_value(&self._16)),
                    ("_17", rt::ToValue::to_value(&self._17)),
                    ("_18", rt::ToValue::to_value(&self._18)),
                    ("_19", rt::ToValue::to_value(&self._19)),
                ])
            }
        }
        impl<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19>
            rt::FromValue
            for Tuple19<
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
            >
        where
            T1: rt::FromValue,
            T2: rt::FromValue,
            T3: rt::FromValue,
            T4: rt::FromValue,
            T5: rt::FromValue,
            T6: rt::FromValue,
            T7: rt::FromValue,
            T8: rt::FromValue,
            T9: rt::FromValue,
            T10: rt::FromValue,
            T11: rt::FromValue,
            T12: rt::FromValue,
            T13: rt::FromValue,
            T14: rt::FromValue,
            T15: rt::FromValue,
            T16: rt::FromValue,
            T17: rt::FromValue,
            T18: rt::FromValue,
            T19: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)
                        .map_err(|e| e.at("_1"))?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)
                        .map_err(|e| e.at("_2"))?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)
                        .map_err(|e| e.at("_3"))?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)
                        .map_err(|e| e.at("_4"))?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)
                        .map_err(|e| e.at("_5"))?,
                    _6: rt::FromValue::from_value(rt::required_field(value, 5usize, "_6")?)
                        .map_err(|e| e.at("_6"))?,
                    _7: rt::FromValue::from_value(rt::required_field(value, 6usize, "_7")?)
                        .map_err(|e| e.at("_7"))?,
                    _8: rt::FromValue::from_value(rt::required_field(value, 7usize, "_8")?)
                        .map_err(|e| e.at("_8"))?,
                    _9: rt::FromValue::from_value(rt::required_field(value, 8usize, "_9")?)
                        .map_err(|e| e.at("_9"))?,
                    _10: rt::FromValue::from_value(rt::required_field(value, 9usize, "_10")?)
                        .map_err(|e| e.at("_10"))?,
                    _11: rt::FromValue::from_value(rt::required_field(value, 10usize, "_11")?)
                        .map_err(|e| e.at("_11"))?,
                    _12: rt::FromValue::from_value(rt::required_field(value, 11usize, "_12")?)
                        .map_err(|e| e.at("_12"))?,
                    _13: rt::FromValue::from_value(rt::required_field(value, 12usize, "_13")?)
                        .map_err(|e| e.at("_13"))?,
                    _14: rt::FromValue::from_value(rt::required_field(value, 13usize, "_14")?)
                        .map_err(|e| e.at("_14"))?,
                    _15: rt::FromValue::from_value(rt::required_field(value, 14usize, "_15")?)
                        .map_err(|e| e.at("_15"))?,
                    _16: rt::FromValue::from_value(rt::required_field(value, 15usize, "_16")?)
                        .map_err(|e| e.at("_16"))?,
                    _17: rt::FromValue::from_value(rt::required_field(value, 16usize, "_17")?)
                        .map_err(|e| e.at("_17"))?,
                    _18: rt::FromValue::from_value(rt::required_field(value, 17usize, "_18")?)
                        .map_err(|e| e.at("_18"))?,
                    _19: rt::FromValue::from_value(rt::required_field(value, 18usize, "_19")?)
                        .map_err(|e| e.at("_19"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple20<
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            T15,
            T16,
            T17,
            T18,
            T19,
            T20,
        > {
            #[serde(rename = "_1")]
            pub _1: T1,
            #[serde(rename = "_2")]
            pub _2: T2,
            #[serde(rename = "_3")]
            pub _3: T3,
            #[serde(rename = "_4")]
            pub _4: T4,
            #[serde(rename = "_5")]
            pub _5: T5,
            #[serde(rename = "_6")]
            pub _6: T6,
            #[serde(rename = "_7")]
            pub _7: T7,
            #[serde(rename = "_8")]
            pub _8: T8,
            #[serde(rename = "_9")]
            pub _9: T9,
            #[serde(rename = "_10")]
            pub _10: T10,
            #[serde(rename = "_11")]
            pub _11: T11,
            #[serde(rename = "_12")]
            pub _12: T12,
            #[serde(rename = "_13")]
            pub _13: T13,
            #[serde(rename = "_14")]
            pub _14: T14,
            #[serde(rename = "_15")]
            pub _15: T15,
            #[serde(rename = "_16")]
            pub _16: T16,
            #[serde(rename = "_17")]
            pub _17: T17,
            #[serde(rename = "_18")]
            pub _18: T18,
            #[serde(rename = "_19")]
            pub _19: T19,
            #[serde(rename = "_20")]
            pub _20: T20,
        }
        impl<
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            T15,
            T16,
            T17,
            T18,
            T19,
            T20,
        > rt::ToValue
            for Tuple20<
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
            >
        where
            T1: rt::ToValue,
            T2: rt::ToValue,
            T3: rt::ToValue,
            T4: rt::ToValue,
            T5: rt::ToValue,
            T6: rt::ToValue,
            T7: rt::ToValue,
            T8: rt::ToValue,
            T9: rt::ToValue,
            T10: rt::ToValue,
            T11: rt::ToValue,
            T12: rt::ToValue,
            T13: rt::ToValue,
            T14: rt::ToValue,
            T15: rt::ToValue,
            T16: rt::ToValue,
            T17: rt::ToValue,
            T18: rt::ToValue,
            T19: rt::ToValue,
            T20: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("_1", rt::ToValue::to_value(&self._1)),
                    ("_2", rt::ToValue::to_value(&self._2)),
                    ("_3", rt::ToValue::to_value(&self._3)),
                    ("_4", rt::ToValue::to_value(&self._4)),
                    ("_5", rt::ToValue::to_value(&self._5)),
                    ("_6", rt::ToValue::to_value(&self._6)),
                    ("_7", rt::ToValue::to_value(&self._7)),
                    ("_8", rt::ToValue::to_value(&self._8)),
                    ("_9", rt::ToValue::to_value(&self._9)),
                    ("_10", rt::ToValue::to_value(&self._10)),
                    ("_11", rt::ToValue::to_value(&self._11)),
                    ("_12", rt::ToValue::to_value(&self._12)),
                    ("_13", rt::ToValue::to_value(&self._13)),
                    ("_14", rt::ToValue::to_value(&self._14)),
                    ("_15", rt::ToValue::to_value(&self._15)),
                    ("_16", rt::ToValue::to_value(&self._16)),
                    ("_17", rt::ToValue::to_value(&self._17)),
                    ("_18", rt::ToValue::to_value(&self._18)),
                    ("_19", rt::ToValue::to_value(&self._19)),
                    ("_20", rt::ToValue::to_value(&self._20)),
                ])
            }
        }
        impl<
            T1,
            T2,
            T3,
            T4,
            T5,
            T6,
            T7,
            T8,
            T9,
            T10,
            T11,
            T12,
            T13,
            T14,
            T15,
            T16,
            T17,
            T18,
            T19,
            T20,
        > rt::FromValue
            for Tuple20<
                T1,
                T2,
                T3,
                T4,
                T5,
                T6,
                T7,
                T8,
                T9,
                T10,
                T11,
                T12,
                T13,
                T14,
                T15,
                T16,
                T17,
                T18,
                T19,
                T20,
            >
        where
            T1: rt::FromValue,
            T2: rt::FromValue,
            T3: rt::FromValue,
            T4: rt::FromValue,
            T5: rt::FromValue,
            T6: rt::FromValue,
            T7: rt::FromValue,
            T8: rt::FromValue,
            T9: rt::FromValue,
            T10: rt::FromValue,
            T11: rt::FromValue,
            T12: rt::FromValue,
            T13: rt::FromValue,
            T14: rt::FromValue,
            T15: rt::FromValue,
            T16: rt::FromValue,
            T17: rt::FromValue,
            T18: rt::FromValue,
            T19: rt::FromValue,
            T20: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)
                        .map_err(|e| e.at("_1"))?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)
                        .map_err(|e| e.at("_2"))?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)
                        .map_err(|e| e.at("_3"))?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)
                        .map_err(|e| e.at("_4"))?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)
                        .map_err(|e| e.at("_5"))?,
                    _6: rt::FromValue::from_value(rt::required_field(value, 5usize, "_6")?)
                        .map_err(|e| e.at("_6"))?,
                    _7: rt::FromValue::from_value(rt::required_field(value, 6usize, "_7")?)
                        .map_err(|e| e.at("_7"))?,
                    _8: rt::FromValue::from_value(rt::required_field(value, 7usize, "_8")?)
                        .map_err(|e| e.at("_8"))?,
                    _9: rt::FromValue::from_value(rt::required_field(value, 8usize, "_9")?)
                        .map_err(|e| e.at("_9"))?,
                    _10: rt::FromValue::from_value(rt::required_field(value, 9usize, "_10")?)
                        .map_err(|e| e.at("_10"))?,
                    _11: rt::FromValue::from_value(rt::required_field(value, 10usize, "_11")?)
                        .map_err(|e| e.at("_11"))?,
                    _12: rt::FromValue::from_value(rt::required_field(value, 11usize, "_12")?)
                        .map_err(|e| e.at("_12"))?,
                    _13: rt::FromValue::from_value(rt::required_field(value, 12usize, "_13")?)
                        .map_err(|e| e.at("_13"))?,
                    _14: rt::FromValue::from_value(rt::required_field(value, 13usize, "_14")?)
                        .map_err(|e| e.at("_14"))?,
                    _15: rt::FromValue::from_value(rt::required_field(value, 14usize, "_15")?)
                        .map_err(|e| e.at("_15"))?,
                    _16: rt::FromValue::from_value(rt::required_field(value, 15usize, "_16")?)
                        .map_err(|e| e.at("_16"))?,
                    _17: rt::FromValue::from_value(rt::required_field(value, 16usize, "_17")?)
                        .map_err(|e| e.at("_17"))?,
                    _18: rt::FromValue::from_value(rt::required_field(value, 17usize, "_18")?)
                        .map_err(|e| e.at("_18"))?,
                    _19: rt::FromValue::from_value(rt::required_field(value, 18usize, "_19")?)
                        .map_err(|e| e.at("_19"))?,
                    _20: rt::FromValue::from_value(rt::required_field(value, 19usize, "_20")?)
                        .map_err(|e| e.at("_20"))?,
                })
            }
        }
    }
}
pub mod daml_prim_GHC_Tuple {
    pub mod GHC_Tuple {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Unit<A> {
            #[serde(rename = "_1")]
            pub _1: A,
        }
        impl<A> rt::ToValue for Unit<A>
        where
            A: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("_1", rt::ToValue::to_value(&self._1)),])
            }
        }
        impl<A> rt::FromValue for Unit<A>
        where
            A: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)
                        .map_err(|e| e.at("_1"))?,
                })
            }
        }
    }
}
pub mod daml_prim_GHC_Types {
    pub mod GHC_Types {
        use canton_daml as rt;
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize,
        )]
        #[serde(crate = "rt::serde")]
        pub enum Ordering {
            #[serde(rename = "LT")]
            LT,
            #[serde(rename = "EQ")]
            EQ,
            #[serde(rename = "GT")]
            GT,
        }
        impl rt::ToValue for Ordering {
            fn to_value(&self) -> rt::Value {
                rt::enum_value(match self {
                    Ordering::LT => "LT",
                    Ordering::EQ => "EQ",
                    Ordering::GT => "GT",
                })
            }
        }
        impl rt::FromValue for Ordering {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                match rt::enum_constructor(value)? {
                    "LT" => ::core::result::Result::Ok(Ordering::LT),
                    "EQ" => ::core::result::Result::Ok(Ordering::EQ),
                    "GT" => ::core::result::Result::Ok(Ordering::GT),
                    other => {
                        ::core::result::Result::Err(rt::unexpected_constructor("Ordering", other))
                    }
                }
            }
        }
    }
}
pub mod daml_stdlib_DA_Date_Types {
    pub mod DA_Date_Types {
        use canton_daml as rt;
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize,
        )]
        #[serde(crate = "rt::serde")]
        pub enum DayOfWeek {
            #[serde(rename = "Monday")]
            Monday,
            #[serde(rename = "Tuesday")]
            Tuesday,
            #[serde(rename = "Wednesday")]
            Wednesday,
            #[serde(rename = "Thursday")]
            Thursday,
            #[serde(rename = "Friday")]
            Friday,
            #[serde(rename = "Saturday")]
            Saturday,
            #[serde(rename = "Sunday")]
            Sunday,
        }
        impl rt::ToValue for DayOfWeek {
            fn to_value(&self) -> rt::Value {
                rt::enum_value(match self {
                    DayOfWeek::Monday => "Monday",
                    DayOfWeek::Tuesday => "Tuesday",
                    DayOfWeek::Wednesday => "Wednesday",
                    DayOfWeek::Thursday => "Thursday",
                    DayOfWeek::Friday => "Friday",
                    DayOfWeek::Saturday => "Saturday",
                    DayOfWeek::Sunday => "Sunday",
                })
            }
        }
        impl rt::FromValue for DayOfWeek {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                match rt::enum_constructor(value)? {
                    "Monday" => ::core::result::Result::Ok(DayOfWeek::Monday),
                    "Tuesday" => ::core::result::Result::Ok(DayOfWeek::Tuesday),
                    "Wednesday" => ::core::result::Result::Ok(DayOfWeek::Wednesday),
                    "Thursday" => ::core::result::Result::Ok(DayOfWeek::Thursday),
                    "Friday" => ::core::result::Result::Ok(DayOfWeek::Friday),
                    "Saturday" => ::core::result::Result::Ok(DayOfWeek::Saturday),
                    "Sunday" => ::core::result::Result::Ok(DayOfWeek::Sunday),
                    other => {
                        ::core::result::Result::Err(rt::unexpected_constructor("DayOfWeek", other))
                    }
                }
            }
        }
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize,
        )]
        #[serde(crate = "rt::serde")]
        pub enum Month {
            #[serde(rename = "Jan")]
            Jan,
            #[serde(rename = "Feb")]
            Feb,
            #[serde(rename = "Mar")]
            Mar,
            #[serde(rename = "Apr")]
            Apr,
            #[serde(rename = "May")]
            May,
            #[serde(rename = "Jun")]
            Jun,
            #[serde(rename = "Jul")]
            Jul,
            #[serde(rename = "Aug")]
            Aug,
            #[serde(rename = "Sep")]
            Sep,
            #[serde(rename = "Oct")]
            Oct,
            #[serde(rename = "Nov")]
            Nov,
            #[serde(rename = "Dec")]
            Dec,
        }
        impl rt::ToValue for Month {
            fn to_value(&self) -> rt::Value {
                rt::enum_value(match self {
                    Month::Jan => "Jan",
                    Month::Feb => "Feb",
                    Month::Mar => "Mar",
                    Month::Apr => "Apr",
                    Month::May => "May",
                    Month::Jun => "Jun",
                    Month::Jul => "Jul",
                    Month::Aug => "Aug",
                    Month::Sep => "Sep",
                    Month::Oct => "Oct",
                    Month::Nov => "Nov",
                    Month::Dec => "Dec",
                })
            }
        }
        impl rt::FromValue for Month {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                match rt::enum_constructor(value)? {
                    "Jan" => ::core::result::Result::Ok(Month::Jan),
                    "Feb" => ::core::result::Result::Ok(Month::Feb),
                    "Mar" => ::core::result::Result::Ok(Month::Mar),
                    "Apr" => ::core::result::Result::Ok(Month::Apr),
                    "May" => ::core::result::Result::Ok(Month::May),
                    "Jun" => ::core::result::Result::Ok(Month::Jun),
                    "Jul" => ::core::result::Result::Ok(Month::Jul),
                    "Aug" => ::core::result::Result::Ok(Month::Aug),
                    "Sep" => ::core::result::Result::Ok(Month::Sep),
                    "Oct" => ::core::result::Result::Ok(Month::Oct),
                    "Nov" => ::core::result::Result::Ok(Month::Nov),
                    "Dec" => ::core::result::Result::Ok(Month::Dec),
                    other => {
                        ::core::result::Result::Err(rt::unexpected_constructor("Month", other))
                    }
                }
            }
        }
    }
}
pub mod daml_stdlib_DA_Internal_Down {
    pub mod DA_Internal_Down {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Down<A> {
            #[serde(rename = "unpack")]
            pub unpack: A,
        }
        impl<A> rt::ToValue for Down<A>
        where
            A: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("unpack", rt::ToValue::to_value(&self.unpack)),])
            }
        }
        impl<A> rt::FromValue for Down<A>
        where
            A: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    unpack: rt::FromValue::from_value(rt::required_field(value, 0usize, "unpack")?)
                        .map_err(|e| e.at("unpack"))?,
                })
            }
        }
    }
}
pub mod ghc_stdlib_DA_Internal_Template {
    pub mod DA_Internal_Template {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Archive {}
        impl rt::ToValue for Archive {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for Archive {
            fn from_value(_value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
    }
}
pub mod daml_stdlib_DA_Logic_Types {
    pub mod DA_Logic_Types {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum Formula<A> {
            #[serde(rename = "Proposition")]
            Proposition(A),
            #[serde(rename = "Negation")]
            Negation(
                ::std::boxed::Box<crate::daml_stdlib_DA_Logic_Types::DA_Logic_Types::Formula<A>>,
            ),
            #[serde(rename = "Conjunction")]
            Conjunction(
                ::std::vec::Vec<crate::daml_stdlib_DA_Logic_Types::DA_Logic_Types::Formula<A>>,
            ),
            #[serde(rename = "Disjunction")]
            Disjunction(
                ::std::vec::Vec<crate::daml_stdlib_DA_Logic_Types::DA_Logic_Types::Formula<A>>,
            ),
        }
        impl<A> rt::ToValue for Formula<A>
        where
            A: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                match self {
                    Formula::Proposition(inner) => {
                        rt::variant_value("Proposition", rt::ToValue::to_value(inner))
                    }
                    Formula::Negation(inner) => {
                        rt::variant_value("Negation", rt::ToValue::to_value(inner))
                    }
                    Formula::Conjunction(inner) => {
                        rt::variant_value("Conjunction", rt::ToValue::to_value(inner))
                    }
                    Formula::Disjunction(inner) => {
                        rt::variant_value("Disjunction", rt::ToValue::to_value(inner))
                    }
                }
            }
        }
        impl<A> rt::FromValue for Formula<A>
        where
            A: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                let (constructor, payload) = rt::variant_parts(value)?;
                match constructor {
                    "Proposition" => ::core::result::Result::Ok(Formula::Proposition(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("Proposition"))?,
                    )),
                    "Negation" => ::core::result::Result::Ok(Formula::Negation(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("Negation"))?,
                    )),
                    "Conjunction" => ::core::result::Result::Ok(Formula::Conjunction(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("Conjunction"))?,
                    )),
                    "Disjunction" => ::core::result::Result::Ok(Formula::Disjunction(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("Disjunction"))?,
                    )),
                    other => {
                        ::core::result::Result::Err(rt::unexpected_constructor("Formula", other))
                    }
                }
            }
        }
    }
}
pub mod daml_stdlib_DA_Monoid_Types {
    pub mod DA_Monoid_Types {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct All {
            ///Daml field `getAll`.
            #[serde(rename = "getAll")]
            pub get_all: bool,
        }
        impl rt::ToValue for All {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "getAll",
                    rt::ToValue::to_value(&self.get_all)
                ),])
            }
        }
        impl rt::FromValue for All {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    get_all: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "getAll",
                    )?)
                    .map_err(|e| e.at("getAll"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Any {
            ///Daml field `getAny`.
            #[serde(rename = "getAny")]
            pub get_any: bool,
        }
        impl rt::ToValue for Any {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "getAny",
                    rt::ToValue::to_value(&self.get_any)
                ),])
            }
        }
        impl rt::FromValue for Any {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    get_any: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "getAny",
                    )?)
                    .map_err(|e| e.at("getAny"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Sum<A> {
            #[serde(rename = "unpack")]
            pub unpack: A,
        }
        impl<A> rt::ToValue for Sum<A>
        where
            A: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("unpack", rt::ToValue::to_value(&self.unpack)),])
            }
        }
        impl<A> rt::FromValue for Sum<A>
        where
            A: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    unpack: rt::FromValue::from_value(rt::required_field(value, 0usize, "unpack")?)
                        .map_err(|e| e.at("unpack"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Product<A> {
            #[serde(rename = "unpack")]
            pub unpack: A,
        }
        impl<A> rt::ToValue for Product<A>
        where
            A: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("unpack", rt::ToValue::to_value(&self.unpack)),])
            }
        }
        impl<A> rt::FromValue for Product<A>
        where
            A: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    unpack: rt::FromValue::from_value(rt::required_field(value, 0usize, "unpack")?)
                        .map_err(|e| e.at("unpack"))?,
                })
            }
        }
    }
}
pub mod daml_stdlib_DA_NonEmpty_Types {
    pub mod DA_NonEmpty_Types {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct NonEmpty<A> {
            #[serde(rename = "hd")]
            pub hd: A,
            #[serde(rename = "tl")]
            pub tl: ::std::vec::Vec<A>,
        }
        impl<A> rt::ToValue for NonEmpty<A>
        where
            A: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("hd", rt::ToValue::to_value(&self.hd)),
                    ("tl", rt::ToValue::to_value(&self.tl)),
                ])
            }
        }
        impl<A> rt::FromValue for NonEmpty<A>
        where
            A: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    hd: rt::FromValue::from_value(rt::required_field(value, 0usize, "hd")?)
                        .map_err(|e| e.at("hd"))?,
                    tl: rt::FromValue::from_value(rt::required_field(value, 1usize, "tl")?)
                        .map_err(|e| e.at("tl"))?,
                })
            }
        }
    }
}
pub mod daml_stdlib_DA_Random_Types {
    pub mod DA_Random_Types {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum Minstd {
            #[serde(rename = "Minstd")]
            Minstd(rt::Int64),
        }
        impl rt::ToValue for Minstd {
            fn to_value(&self) -> rt::Value {
                match self {
                    Minstd::Minstd(inner) => {
                        rt::variant_value("Minstd", rt::ToValue::to_value(inner))
                    }
                }
            }
        }
        impl rt::FromValue for Minstd {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                let (constructor, payload) = rt::variant_parts(value)?;
                match constructor {
                    "Minstd" => ::core::result::Result::Ok(Minstd::Minstd(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("Minstd"))?,
                    )),
                    other => {
                        ::core::result::Result::Err(rt::unexpected_constructor("Minstd", other))
                    }
                }
            }
        }
    }
}
pub mod daml_stdlib_DA_Semigroup_Types {
    pub mod DA_Semigroup_Types {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Min<A> {
            #[serde(rename = "unpack")]
            pub unpack: A,
        }
        impl<A> rt::ToValue for Min<A>
        where
            A: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("unpack", rt::ToValue::to_value(&self.unpack)),])
            }
        }
        impl<A> rt::FromValue for Min<A>
        where
            A: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    unpack: rt::FromValue::from_value(rt::required_field(value, 0usize, "unpack")?)
                        .map_err(|e| e.at("unpack"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Max<A> {
            #[serde(rename = "unpack")]
            pub unpack: A,
        }
        impl<A> rt::ToValue for Max<A>
        where
            A: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("unpack", rt::ToValue::to_value(&self.unpack)),])
            }
        }
        impl<A> rt::FromValue for Max<A>
        where
            A: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    unpack: rt::FromValue::from_value(rt::required_field(value, 0usize, "unpack")?)
                        .map_err(|e| e.at("unpack"))?,
                })
            }
        }
    }
}
pub mod daml_stdlib_DA_Set_Types {
    pub mod DA_Set_Types {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Set<K> {
            #[serde(rename = "map")]
            pub map: rt::GenMap<K, rt::Unit>,
        }
        impl<K> rt::ToValue for Set<K>
        where
            K: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("map", rt::ToValue::to_value(&self.map)),])
            }
        }
        impl<K> rt::FromValue for Set<K>
        where
            K: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    map: rt::FromValue::from_value(rt::required_field(value, 0usize, "map")?)
                        .map_err(|e| e.at("map"))?,
                })
            }
        }
    }
}
pub mod daml_stdlib_DA_Stack_Types {
    pub mod DA_Stack_Types {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SrcLoc {
            ///Daml field `srcLocPackage`.
            #[serde(rename = "srcLocPackage")]
            pub src_loc_package: ::std::string::String,
            ///Daml field `srcLocModule`.
            #[serde(rename = "srcLocModule")]
            pub src_loc_module: ::std::string::String,
            ///Daml field `srcLocFile`.
            #[serde(rename = "srcLocFile")]
            pub src_loc_file: ::std::string::String,
            ///Daml field `srcLocStartLine`.
            #[serde(rename = "srcLocStartLine")]
            pub src_loc_start_line: rt::Int64,
            ///Daml field `srcLocStartCol`.
            #[serde(rename = "srcLocStartCol")]
            pub src_loc_start_col: rt::Int64,
            ///Daml field `srcLocEndLine`.
            #[serde(rename = "srcLocEndLine")]
            pub src_loc_end_line: rt::Int64,
            ///Daml field `srcLocEndCol`.
            #[serde(rename = "srcLocEndCol")]
            pub src_loc_end_col: rt::Int64,
        }
        impl rt::ToValue for SrcLoc {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "srcLocPackage",
                        rt::ToValue::to_value(&self.src_loc_package)
                    ),
                    ("srcLocModule", rt::ToValue::to_value(&self.src_loc_module)),
                    ("srcLocFile", rt::ToValue::to_value(&self.src_loc_file)),
                    (
                        "srcLocStartLine",
                        rt::ToValue::to_value(&self.src_loc_start_line)
                    ),
                    (
                        "srcLocStartCol",
                        rt::ToValue::to_value(&self.src_loc_start_col)
                    ),
                    (
                        "srcLocEndLine",
                        rt::ToValue::to_value(&self.src_loc_end_line)
                    ),
                    ("srcLocEndCol", rt::ToValue::to_value(&self.src_loc_end_col)),
                ])
            }
        }
        impl rt::FromValue for SrcLoc {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    src_loc_package: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "srcLocPackage",
                    )?)
                    .map_err(|e| e.at("srcLocPackage"))?,
                    src_loc_module: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "srcLocModule",
                    )?)
                    .map_err(|e| e.at("srcLocModule"))?,
                    src_loc_file: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "srcLocFile",
                    )?)
                    .map_err(|e| e.at("srcLocFile"))?,
                    src_loc_start_line: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "srcLocStartLine",
                    )?)
                    .map_err(|e| e.at("srcLocStartLine"))?,
                    src_loc_start_col: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "srcLocStartCol",
                    )?)
                    .map_err(|e| e.at("srcLocStartCol"))?,
                    src_loc_end_line: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "srcLocEndLine",
                    )?)
                    .map_err(|e| e.at("srcLocEndLine"))?,
                    src_loc_end_col: rt::FromValue::from_value(rt::required_field(
                        value,
                        6usize,
                        "srcLocEndCol",
                    )?)
                    .map_err(|e| e.at("srcLocEndCol"))?,
                })
            }
        }
    }
}
pub mod daml_stdlib_DA_Time_Types {
    pub mod DA_Time_Types {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct RelTime {
            #[serde(rename = "microseconds")]
            pub microseconds: rt::Int64,
        }
        impl rt::ToValue for RelTime {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "microseconds",
                    rt::ToValue::to_value(&self.microseconds)
                ),])
            }
        }
        impl rt::FromValue for RelTime {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    microseconds: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "microseconds",
                    )?)
                    .map_err(|e| e.at("microseconds"))?,
                })
            }
        }
    }
}
pub mod daml_stdlib_DA_Validation_Types {
    pub mod DA_Validation_Types {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum Validation<Errs, A> {
            #[serde(rename = "Errors")]
            Errors(crate::daml_stdlib_DA_NonEmpty_Types::DA_NonEmpty_Types::NonEmpty<Errs>),
            #[serde(rename = "Success")]
            Success(A),
        }
        impl<Errs, A> rt::ToValue for Validation<Errs, A>
        where
            Errs: rt::ToValue,
            A: rt::ToValue,
        {
            fn to_value(&self) -> rt::Value {
                match self {
                    Validation::Errors(inner) => {
                        rt::variant_value("Errors", rt::ToValue::to_value(inner))
                    }
                    Validation::Success(inner) => {
                        rt::variant_value("Success", rt::ToValue::to_value(inner))
                    }
                }
            }
        }
        impl<Errs, A> rt::FromValue for Validation<Errs, A>
        where
            Errs: rt::FromValue,
            A: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                let (constructor, payload) = rt::variant_parts(value)?;
                match constructor {
                    "Errors" => ::core::result::Result::Ok(Validation::Errors(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("Errors"))?,
                    )),
                    "Success" => ::core::result::Result::Ok(Validation::Success(
                        rt::FromValue::from_value(payload).map_err(|e| e.at("Success"))?,
                    )),
                    other => {
                        ::core::result::Result::Err(rt::unexpected_constructor("Validation", other))
                    }
                }
            }
        }
    }
}
pub mod splice_api_token_transfer_instruction_v1 {
    pub mod Splice_Api_Token_TransferInstructionV1 {
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
            ///Daml field `expectedAdmin`.
            #[serde(rename = "expectedAdmin")]
            pub expected_admin: rt::Party,
            #[serde(rename = "actor")]
            pub actor: rt::Party,
        }
        impl rt::ToValue for TransferFactory_PublicFetch {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("expectedAdmin", rt::ToValue::to_value(&self.expected_admin)),
                    ("actor", rt::ToValue::to_value(&self.actor)),
                ])
            }
        }
        impl rt::FromValue for TransferFactory_PublicFetch {
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
        pub struct TransferFactory_Transfer {
            ///Daml field `expectedAdmin`.
            #[serde(rename = "expectedAdmin")]
            pub expected_admin: rt::Party,
            #[serde(rename = "transfer")]
            pub transfer: crate::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::Transfer,
            ///Daml field `extraArgs`.
            #[serde(rename = "extraArgs")]
            pub extra_args: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for TransferFactory_Transfer {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("expectedAdmin", rt::ToValue::to_value(&self.expected_admin)),
                    ("transfer", rt::ToValue::to_value(&self.transfer)),
                    ("extraArgs", rt::ToValue::to_value(&self.extra_args)),
                ])
            }
        }
        impl rt::FromValue for TransferFactory_Transfer {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    expected_admin: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "expectedAdmin",
                    )?)
                    .map_err(|e| e.at("expectedAdmin"))?,
                    transfer: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "transfer",
                    )?)
                    .map_err(|e| e.at("transfer"))?,
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
        pub struct TransferInstruction_Update {
            ///Daml field `extraActors`.
            #[serde(rename = "extraActors")]
            pub extra_actors: ::std::vec::Vec<rt::Party>,
            ///Daml field `extraArgs`.
            #[serde(rename = "extraArgs")]
            pub extra_args: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for TransferInstruction_Update {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("extraActors", rt::ToValue::to_value(&self.extra_actors)),
                    ("extraArgs", rt::ToValue::to_value(&self.extra_args)),
                ])
            }
        }
        impl rt::FromValue for TransferInstruction_Update {
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
        pub struct TransferInstruction_Withdraw {
            ///Daml field `extraArgs`.
            #[serde(rename = "extraArgs")]
            pub extra_args: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for TransferInstruction_Withdraw {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "extraArgs",
                    rt::ToValue::to_value(&self.extra_args)
                ),])
            }
        }
        impl rt::FromValue for TransferInstruction_Withdraw {
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
        pub struct TransferInstruction_Reject {
            ///Daml field `extraArgs`.
            #[serde(rename = "extraArgs")]
            pub extra_args: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for TransferInstruction_Reject {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "extraArgs",
                    rt::ToValue::to_value(&self.extra_args)
                ),])
            }
        }
        impl rt::FromValue for TransferInstruction_Reject {
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
        pub struct TransferInstruction_Accept {
            ///Daml field `extraArgs`.
            #[serde(rename = "extraArgs")]
            pub extra_args: ::canton_splice_api_token_metadata_v1::splice_api_token_metadata_v1::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for TransferInstruction_Accept {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "extraArgs",
                    rt::ToValue::to_value(&self.extra_args)
                ),])
            }
        }
        impl rt::FromValue for TransferInstruction_Accept {
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
        pub struct TransferInstructionView {
            ///Daml field `originalInstructionCid`.
            #[serde(rename = "originalInstructionCid")]
            pub original_instruction_cid: ::core::option::Option<
                rt::ContractId<
                    crate::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferInstruction,
                >,
            >,
            #[serde(rename = "transfer")]
            pub transfer: crate::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::Transfer,
            #[serde(rename = "status")]
            pub status: crate::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferInstructionStatus,
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
                    ("status", rt::ToValue::to_value(&self.status)),
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
                    status: rt::FromValue::from_value(rt::required_field(value, 2usize, "status")?)
                        .map_err(|e| e.at("status"))?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 3usize, "meta")?)
                        .map_err(|e| e.at("meta"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum TransferInstructionStatus {
            #[serde(rename = "TransferPendingReceiverAcceptance")]
            TransferPendingReceiverAcceptance(rt::Unit),
            #[serde(rename = "TransferPendingInternalWorkflow")]
            TransferPendingInternalWorkflow(
                crate::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferInstructionStatus_TransferPendingInternalWorkflow,
            ),
        }
        impl rt::ToValue for TransferInstructionStatus {
            fn to_value(&self) -> rt::Value {
                match self {
                    TransferInstructionStatus::TransferPendingReceiverAcceptance(inner) => {
                        rt::variant_value(
                            "TransferPendingReceiverAcceptance",
                            rt::ToValue::to_value(inner),
                        )
                    }
                    TransferInstructionStatus::TransferPendingInternalWorkflow(inner) => {
                        rt::variant_value(
                            "TransferPendingInternalWorkflow",
                            rt::ToValue::to_value(inner),
                        )
                    }
                }
            }
        }
        impl rt::FromValue for TransferInstructionStatus {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                let (constructor, payload) = rt::variant_parts(value)?;
                match constructor {
                    "TransferPendingReceiverAcceptance" => ::core::result::Result::Ok(
                        TransferInstructionStatus::TransferPendingReceiverAcceptance(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("TransferPendingReceiverAcceptance"))?,
                        ),
                    ),
                    "TransferPendingInternalWorkflow" => ::core::result::Result::Ok(
                        TransferInstructionStatus::TransferPendingInternalWorkflow(
                            rt::FromValue::from_value(payload)
                                .map_err(|e| e.at("TransferPendingInternalWorkflow"))?,
                        ),
                    ),
                    other => ::core::result::Result::Err(rt::unexpected_constructor(
                        "TransferInstructionStatus",
                        other,
                    )),
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferInstructionStatus_TransferPendingInternalWorkflow {
            ///Daml field `pendingActions`.
            #[serde(rename = "pendingActions")]
            pub pending_actions: rt::GenMap<rt::Party, ::std::string::String>,
        }
        impl rt::ToValue for TransferInstructionStatus_TransferPendingInternalWorkflow {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "pendingActions",
                    rt::ToValue::to_value(&self.pending_actions)
                ),])
            }
        }
        impl rt::FromValue for TransferInstructionStatus_TransferPendingInternalWorkflow {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    pending_actions: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "pendingActions",
                    )?)
                    .map_err(|e| e.at("pendingActions"))?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum TransferInstructionResult_Output {
            #[serde(rename = "TransferInstructionResult_Pending")]
            TransferInstructionResult_Pending(
                crate::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferInstructionResult_Output_TransferInstructionResult_Pending,
            ),
            #[serde(rename = "TransferInstructionResult_Completed")]
            TransferInstructionResult_Completed(
                crate::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferInstructionResult_Output_TransferInstructionResult_Completed,
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
                crate::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferInstruction,
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
                    ::canton_splice_api_token_holding_v1::splice_api_token_holding_v1::Splice_Api_Token_HoldingV1::Holding,
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
            pub output: crate::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferInstructionResult_Output,
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
            pub sender: rt::Party,
            #[serde(rename = "receiver")]
            pub receiver: rt::Party,
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            ///Daml field `instrumentId`.
            #[serde(rename = "instrumentId")]
            pub instrument_id: ::canton_splice_api_token_holding_v1::splice_api_token_holding_v1::Splice_Api_Token_HoldingV1::InstrumentId,
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
                    ::canton_splice_api_token_holding_v1::splice_api_token_holding_v1::Splice_Api_Token_HoldingV1::Holding,
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
                "55ba4deb0ad4662c4168b39859738a0e91388d252286480c7331b3f71a517281";
            const PACKAGE_NAME: &'static str = "splice-api-token-transfer-instruction-v1";
            const MODULE_NAME: &'static str = "Splice.Api.Token.TransferInstructionV1";
            const ENTITY_NAME: &'static str = "TransferFactory";
        }
        impl rt::Interface for TransferFactory {
            type View = crate::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferFactoryView;
        }
        ///The `Archive` choice on [`TransferFactory`] (consuming).
        impl rt::Choice<TransferFactory>
            for crate::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `TransferFactory_Transfer` choice on [`TransferFactory`] (non-consuming).
        impl rt::Choice<TransferFactory>
        for crate::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferFactory_Transfer {
            type Return = crate::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferInstructionResult;
            const NAME: &'static str = "TransferFactory_Transfer";
            const CONSUMING: bool = false;
        }
        ///The `TransferFactory_PublicFetch` choice on [`TransferFactory`] (non-consuming).
        impl rt::Choice<TransferFactory>
        for crate::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferFactory_PublicFetch {
            type Return = crate::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferFactoryView;
            const NAME: &'static str = "TransferFactory_PublicFetch";
            const CONSUMING: bool = false;
        }
        impl rt::Contract for TransferInstruction {
            const PACKAGE_ID: &'static str =
                "55ba4deb0ad4662c4168b39859738a0e91388d252286480c7331b3f71a517281";
            const PACKAGE_NAME: &'static str = "splice-api-token-transfer-instruction-v1";
            const MODULE_NAME: &'static str = "Splice.Api.Token.TransferInstructionV1";
            const ENTITY_NAME: &'static str = "TransferInstruction";
        }
        impl rt::Interface for TransferInstruction {
            type View = crate::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferInstructionView;
        }
        ///The `Archive` choice on [`TransferInstruction`] (consuming).
        impl rt::Choice<TransferInstruction>
            for crate::ghc_stdlib_DA_Internal_Template::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `TransferInstruction_Accept` choice on [`TransferInstruction`] (consuming).
        impl rt::Choice<TransferInstruction>
        for crate::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferInstruction_Accept {
            type Return = crate::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferInstructionResult;
            const NAME: &'static str = "TransferInstruction_Accept";
            const CONSUMING: bool = true;
        }
        ///The `TransferInstruction_Reject` choice on [`TransferInstruction`] (consuming).
        impl rt::Choice<TransferInstruction>
        for crate::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferInstruction_Reject {
            type Return = crate::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferInstructionResult;
            const NAME: &'static str = "TransferInstruction_Reject";
            const CONSUMING: bool = true;
        }
        ///The `TransferInstruction_Withdraw` choice on [`TransferInstruction`] (consuming).
        impl rt::Choice<TransferInstruction>
        for crate::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferInstruction_Withdraw {
            type Return = crate::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferInstructionResult;
            const NAME: &'static str = "TransferInstruction_Withdraw";
            const CONSUMING: bool = true;
        }
        ///The `TransferInstruction_Update` choice on [`TransferInstruction`] (consuming).
        impl rt::Choice<TransferInstruction>
        for crate::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferInstruction_Update {
            type Return = crate::splice_api_token_transfer_instruction_v1::Splice_Api_Token_TransferInstructionV1::TransferInstructionResult;
            const NAME: &'static str = "TransferInstruction_Update";
            const CONSUMING: bool = true;
        }
    }
}
