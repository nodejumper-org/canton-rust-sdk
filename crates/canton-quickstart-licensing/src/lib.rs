#![allow(
    non_camel_case_types,
    non_snake_case,
    unused_imports,
    dead_code,
    unused_variables,
    clippy::all
)]
//! Generated Daml bindings — do not edit by hand.

pub mod daml_prim_DA_Exception_ArithmeticError_1_0_0 {
    pub mod DA_Exception_ArithmeticError {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ArithmeticError {
            ///The Daml `message` field.
            #[serde(rename = "message")]
            pub message: String,
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
                    message: rt::FromValue::from_value(rt::record_field(value, "message")?)?,
                })
            }
        }
    }
}
pub mod daml_prim_DA_Exception_AssertionFailed_1_0_0 {
    pub mod DA_Exception_AssertionFailed {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AssertionFailed {
            ///The Daml `message` field.
            #[serde(rename = "message")]
            pub message: String,
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
                    message: rt::FromValue::from_value(rt::record_field(value, "message")?)?,
                })
            }
        }
    }
}
pub mod daml_prim_DA_Exception_GeneralError_1_0_0 {
    pub mod DA_Exception_GeneralError {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct GeneralError {
            ///The Daml `message` field.
            #[serde(rename = "message")]
            pub message: String,
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
                    message: rt::FromValue::from_value(rt::record_field(value, "message")?)?,
                })
            }
        }
    }
}
pub mod daml_prim_DA_Exception_PreconditionFailed_1_0_0 {
    pub mod DA_Exception_PreconditionFailed {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct PreconditionFailed {
            ///The Daml `message` field.
            #[serde(rename = "message")]
            pub message: String,
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
                    message: rt::FromValue::from_value(rt::record_field(value, "message")?)?,
                })
            }
        }
    }
}
pub mod daml_prim_DA_Types_1_0_0 {
    pub mod DA_Types {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum Either<A, B> {
            ///The Daml `Left` constructor.
            #[serde(rename = "Left")]
            Left(A),
            ///The Daml `Right` constructor.
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
                    "Left" => ::core::result::Result::Ok(Either::Left(rt::FromValue::from_value(
                        payload,
                    )?)),
                    "Right" => ::core::result::Result::Ok(Either::Right(
                        rt::FromValue::from_value(payload)?,
                    )),
                    other => {
                        ::core::result::Result::Err(rt::unexpected_constructor("Either", other))
                    }
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple2<T1, T2> {
            ///The Daml `_1` field.
            #[serde(rename = "_1")]
            pub _1: T1,
            ///The Daml `_2` field.
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
                    _1: rt::FromValue::from_value(rt::record_field(value, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::record_field(value, "_2")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple3<T1, T2, T3> {
            ///The Daml `_1` field.
            #[serde(rename = "_1")]
            pub _1: T1,
            ///The Daml `_2` field.
            #[serde(rename = "_2")]
            pub _2: T2,
            ///The Daml `_3` field.
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
                    _1: rt::FromValue::from_value(rt::record_field(value, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::record_field(value, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::record_field(value, "_3")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple4<T1, T2, T3, T4> {
            ///The Daml `_1` field.
            #[serde(rename = "_1")]
            pub _1: T1,
            ///The Daml `_2` field.
            #[serde(rename = "_2")]
            pub _2: T2,
            ///The Daml `_3` field.
            #[serde(rename = "_3")]
            pub _3: T3,
            ///The Daml `_4` field.
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
                    _1: rt::FromValue::from_value(rt::record_field(value, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::record_field(value, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::record_field(value, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::record_field(value, "_4")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple5<T1, T2, T3, T4, T5> {
            ///The Daml `_1` field.
            #[serde(rename = "_1")]
            pub _1: T1,
            ///The Daml `_2` field.
            #[serde(rename = "_2")]
            pub _2: T2,
            ///The Daml `_3` field.
            #[serde(rename = "_3")]
            pub _3: T3,
            ///The Daml `_4` field.
            #[serde(rename = "_4")]
            pub _4: T4,
            ///The Daml `_5` field.
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
                    _1: rt::FromValue::from_value(rt::record_field(value, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::record_field(value, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::record_field(value, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::record_field(value, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::record_field(value, "_5")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple6<T1, T2, T3, T4, T5, T6> {
            ///The Daml `_1` field.
            #[serde(rename = "_1")]
            pub _1: T1,
            ///The Daml `_2` field.
            #[serde(rename = "_2")]
            pub _2: T2,
            ///The Daml `_3` field.
            #[serde(rename = "_3")]
            pub _3: T3,
            ///The Daml `_4` field.
            #[serde(rename = "_4")]
            pub _4: T4,
            ///The Daml `_5` field.
            #[serde(rename = "_5")]
            pub _5: T5,
            ///The Daml `_6` field.
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
                    _1: rt::FromValue::from_value(rt::record_field(value, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::record_field(value, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::record_field(value, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::record_field(value, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::record_field(value, "_5")?)?,
                    _6: rt::FromValue::from_value(rt::record_field(value, "_6")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple7<T1, T2, T3, T4, T5, T6, T7> {
            ///The Daml `_1` field.
            #[serde(rename = "_1")]
            pub _1: T1,
            ///The Daml `_2` field.
            #[serde(rename = "_2")]
            pub _2: T2,
            ///The Daml `_3` field.
            #[serde(rename = "_3")]
            pub _3: T3,
            ///The Daml `_4` field.
            #[serde(rename = "_4")]
            pub _4: T4,
            ///The Daml `_5` field.
            #[serde(rename = "_5")]
            pub _5: T5,
            ///The Daml `_6` field.
            #[serde(rename = "_6")]
            pub _6: T6,
            ///The Daml `_7` field.
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
                    _1: rt::FromValue::from_value(rt::record_field(value, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::record_field(value, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::record_field(value, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::record_field(value, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::record_field(value, "_5")?)?,
                    _6: rt::FromValue::from_value(rt::record_field(value, "_6")?)?,
                    _7: rt::FromValue::from_value(rt::record_field(value, "_7")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple8<T1, T2, T3, T4, T5, T6, T7, T8> {
            ///The Daml `_1` field.
            #[serde(rename = "_1")]
            pub _1: T1,
            ///The Daml `_2` field.
            #[serde(rename = "_2")]
            pub _2: T2,
            ///The Daml `_3` field.
            #[serde(rename = "_3")]
            pub _3: T3,
            ///The Daml `_4` field.
            #[serde(rename = "_4")]
            pub _4: T4,
            ///The Daml `_5` field.
            #[serde(rename = "_5")]
            pub _5: T5,
            ///The Daml `_6` field.
            #[serde(rename = "_6")]
            pub _6: T6,
            ///The Daml `_7` field.
            #[serde(rename = "_7")]
            pub _7: T7,
            ///The Daml `_8` field.
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
                    _1: rt::FromValue::from_value(rt::record_field(value, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::record_field(value, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::record_field(value, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::record_field(value, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::record_field(value, "_5")?)?,
                    _6: rt::FromValue::from_value(rt::record_field(value, "_6")?)?,
                    _7: rt::FromValue::from_value(rt::record_field(value, "_7")?)?,
                    _8: rt::FromValue::from_value(rt::record_field(value, "_8")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple9<T1, T2, T3, T4, T5, T6, T7, T8, T9> {
            ///The Daml `_1` field.
            #[serde(rename = "_1")]
            pub _1: T1,
            ///The Daml `_2` field.
            #[serde(rename = "_2")]
            pub _2: T2,
            ///The Daml `_3` field.
            #[serde(rename = "_3")]
            pub _3: T3,
            ///The Daml `_4` field.
            #[serde(rename = "_4")]
            pub _4: T4,
            ///The Daml `_5` field.
            #[serde(rename = "_5")]
            pub _5: T5,
            ///The Daml `_6` field.
            #[serde(rename = "_6")]
            pub _6: T6,
            ///The Daml `_7` field.
            #[serde(rename = "_7")]
            pub _7: T7,
            ///The Daml `_8` field.
            #[serde(rename = "_8")]
            pub _8: T8,
            ///The Daml `_9` field.
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
                    _1: rt::FromValue::from_value(rt::record_field(value, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::record_field(value, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::record_field(value, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::record_field(value, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::record_field(value, "_5")?)?,
                    _6: rt::FromValue::from_value(rt::record_field(value, "_6")?)?,
                    _7: rt::FromValue::from_value(rt::record_field(value, "_7")?)?,
                    _8: rt::FromValue::from_value(rt::record_field(value, "_8")?)?,
                    _9: rt::FromValue::from_value(rt::record_field(value, "_9")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple10<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10> {
            ///The Daml `_1` field.
            #[serde(rename = "_1")]
            pub _1: T1,
            ///The Daml `_2` field.
            #[serde(rename = "_2")]
            pub _2: T2,
            ///The Daml `_3` field.
            #[serde(rename = "_3")]
            pub _3: T3,
            ///The Daml `_4` field.
            #[serde(rename = "_4")]
            pub _4: T4,
            ///The Daml `_5` field.
            #[serde(rename = "_5")]
            pub _5: T5,
            ///The Daml `_6` field.
            #[serde(rename = "_6")]
            pub _6: T6,
            ///The Daml `_7` field.
            #[serde(rename = "_7")]
            pub _7: T7,
            ///The Daml `_8` field.
            #[serde(rename = "_8")]
            pub _8: T8,
            ///The Daml `_9` field.
            #[serde(rename = "_9")]
            pub _9: T9,
            ///The Daml `_10` field.
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
                    _1: rt::FromValue::from_value(rt::record_field(value, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::record_field(value, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::record_field(value, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::record_field(value, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::record_field(value, "_5")?)?,
                    _6: rt::FromValue::from_value(rt::record_field(value, "_6")?)?,
                    _7: rt::FromValue::from_value(rt::record_field(value, "_7")?)?,
                    _8: rt::FromValue::from_value(rt::record_field(value, "_8")?)?,
                    _9: rt::FromValue::from_value(rt::record_field(value, "_9")?)?,
                    _10: rt::FromValue::from_value(rt::record_field(value, "_10")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple11<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11> {
            ///The Daml `_1` field.
            #[serde(rename = "_1")]
            pub _1: T1,
            ///The Daml `_2` field.
            #[serde(rename = "_2")]
            pub _2: T2,
            ///The Daml `_3` field.
            #[serde(rename = "_3")]
            pub _3: T3,
            ///The Daml `_4` field.
            #[serde(rename = "_4")]
            pub _4: T4,
            ///The Daml `_5` field.
            #[serde(rename = "_5")]
            pub _5: T5,
            ///The Daml `_6` field.
            #[serde(rename = "_6")]
            pub _6: T6,
            ///The Daml `_7` field.
            #[serde(rename = "_7")]
            pub _7: T7,
            ///The Daml `_8` field.
            #[serde(rename = "_8")]
            pub _8: T8,
            ///The Daml `_9` field.
            #[serde(rename = "_9")]
            pub _9: T9,
            ///The Daml `_10` field.
            #[serde(rename = "_10")]
            pub _10: T10,
            ///The Daml `_11` field.
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
                    _1: rt::FromValue::from_value(rt::record_field(value, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::record_field(value, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::record_field(value, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::record_field(value, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::record_field(value, "_5")?)?,
                    _6: rt::FromValue::from_value(rt::record_field(value, "_6")?)?,
                    _7: rt::FromValue::from_value(rt::record_field(value, "_7")?)?,
                    _8: rt::FromValue::from_value(rt::record_field(value, "_8")?)?,
                    _9: rt::FromValue::from_value(rt::record_field(value, "_9")?)?,
                    _10: rt::FromValue::from_value(rt::record_field(value, "_10")?)?,
                    _11: rt::FromValue::from_value(rt::record_field(value, "_11")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple12<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12> {
            ///The Daml `_1` field.
            #[serde(rename = "_1")]
            pub _1: T1,
            ///The Daml `_2` field.
            #[serde(rename = "_2")]
            pub _2: T2,
            ///The Daml `_3` field.
            #[serde(rename = "_3")]
            pub _3: T3,
            ///The Daml `_4` field.
            #[serde(rename = "_4")]
            pub _4: T4,
            ///The Daml `_5` field.
            #[serde(rename = "_5")]
            pub _5: T5,
            ///The Daml `_6` field.
            #[serde(rename = "_6")]
            pub _6: T6,
            ///The Daml `_7` field.
            #[serde(rename = "_7")]
            pub _7: T7,
            ///The Daml `_8` field.
            #[serde(rename = "_8")]
            pub _8: T8,
            ///The Daml `_9` field.
            #[serde(rename = "_9")]
            pub _9: T9,
            ///The Daml `_10` field.
            #[serde(rename = "_10")]
            pub _10: T10,
            ///The Daml `_11` field.
            #[serde(rename = "_11")]
            pub _11: T11,
            ///The Daml `_12` field.
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
                    _1: rt::FromValue::from_value(rt::record_field(value, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::record_field(value, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::record_field(value, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::record_field(value, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::record_field(value, "_5")?)?,
                    _6: rt::FromValue::from_value(rt::record_field(value, "_6")?)?,
                    _7: rt::FromValue::from_value(rt::record_field(value, "_7")?)?,
                    _8: rt::FromValue::from_value(rt::record_field(value, "_8")?)?,
                    _9: rt::FromValue::from_value(rt::record_field(value, "_9")?)?,
                    _10: rt::FromValue::from_value(rt::record_field(value, "_10")?)?,
                    _11: rt::FromValue::from_value(rt::record_field(value, "_11")?)?,
                    _12: rt::FromValue::from_value(rt::record_field(value, "_12")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple13<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13> {
            ///The Daml `_1` field.
            #[serde(rename = "_1")]
            pub _1: T1,
            ///The Daml `_2` field.
            #[serde(rename = "_2")]
            pub _2: T2,
            ///The Daml `_3` field.
            #[serde(rename = "_3")]
            pub _3: T3,
            ///The Daml `_4` field.
            #[serde(rename = "_4")]
            pub _4: T4,
            ///The Daml `_5` field.
            #[serde(rename = "_5")]
            pub _5: T5,
            ///The Daml `_6` field.
            #[serde(rename = "_6")]
            pub _6: T6,
            ///The Daml `_7` field.
            #[serde(rename = "_7")]
            pub _7: T7,
            ///The Daml `_8` field.
            #[serde(rename = "_8")]
            pub _8: T8,
            ///The Daml `_9` field.
            #[serde(rename = "_9")]
            pub _9: T9,
            ///The Daml `_10` field.
            #[serde(rename = "_10")]
            pub _10: T10,
            ///The Daml `_11` field.
            #[serde(rename = "_11")]
            pub _11: T11,
            ///The Daml `_12` field.
            #[serde(rename = "_12")]
            pub _12: T12,
            ///The Daml `_13` field.
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
                    _1: rt::FromValue::from_value(rt::record_field(value, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::record_field(value, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::record_field(value, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::record_field(value, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::record_field(value, "_5")?)?,
                    _6: rt::FromValue::from_value(rt::record_field(value, "_6")?)?,
                    _7: rt::FromValue::from_value(rt::record_field(value, "_7")?)?,
                    _8: rt::FromValue::from_value(rt::record_field(value, "_8")?)?,
                    _9: rt::FromValue::from_value(rt::record_field(value, "_9")?)?,
                    _10: rt::FromValue::from_value(rt::record_field(value, "_10")?)?,
                    _11: rt::FromValue::from_value(rt::record_field(value, "_11")?)?,
                    _12: rt::FromValue::from_value(rt::record_field(value, "_12")?)?,
                    _13: rt::FromValue::from_value(rt::record_field(value, "_13")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple14<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14> {
            ///The Daml `_1` field.
            #[serde(rename = "_1")]
            pub _1: T1,
            ///The Daml `_2` field.
            #[serde(rename = "_2")]
            pub _2: T2,
            ///The Daml `_3` field.
            #[serde(rename = "_3")]
            pub _3: T3,
            ///The Daml `_4` field.
            #[serde(rename = "_4")]
            pub _4: T4,
            ///The Daml `_5` field.
            #[serde(rename = "_5")]
            pub _5: T5,
            ///The Daml `_6` field.
            #[serde(rename = "_6")]
            pub _6: T6,
            ///The Daml `_7` field.
            #[serde(rename = "_7")]
            pub _7: T7,
            ///The Daml `_8` field.
            #[serde(rename = "_8")]
            pub _8: T8,
            ///The Daml `_9` field.
            #[serde(rename = "_9")]
            pub _9: T9,
            ///The Daml `_10` field.
            #[serde(rename = "_10")]
            pub _10: T10,
            ///The Daml `_11` field.
            #[serde(rename = "_11")]
            pub _11: T11,
            ///The Daml `_12` field.
            #[serde(rename = "_12")]
            pub _12: T12,
            ///The Daml `_13` field.
            #[serde(rename = "_13")]
            pub _13: T13,
            ///The Daml `_14` field.
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
                    _1: rt::FromValue::from_value(rt::record_field(value, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::record_field(value, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::record_field(value, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::record_field(value, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::record_field(value, "_5")?)?,
                    _6: rt::FromValue::from_value(rt::record_field(value, "_6")?)?,
                    _7: rt::FromValue::from_value(rt::record_field(value, "_7")?)?,
                    _8: rt::FromValue::from_value(rt::record_field(value, "_8")?)?,
                    _9: rt::FromValue::from_value(rt::record_field(value, "_9")?)?,
                    _10: rt::FromValue::from_value(rt::record_field(value, "_10")?)?,
                    _11: rt::FromValue::from_value(rt::record_field(value, "_11")?)?,
                    _12: rt::FromValue::from_value(rt::record_field(value, "_12")?)?,
                    _13: rt::FromValue::from_value(rt::record_field(value, "_13")?)?,
                    _14: rt::FromValue::from_value(rt::record_field(value, "_14")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple15<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15> {
            ///The Daml `_1` field.
            #[serde(rename = "_1")]
            pub _1: T1,
            ///The Daml `_2` field.
            #[serde(rename = "_2")]
            pub _2: T2,
            ///The Daml `_3` field.
            #[serde(rename = "_3")]
            pub _3: T3,
            ///The Daml `_4` field.
            #[serde(rename = "_4")]
            pub _4: T4,
            ///The Daml `_5` field.
            #[serde(rename = "_5")]
            pub _5: T5,
            ///The Daml `_6` field.
            #[serde(rename = "_6")]
            pub _6: T6,
            ///The Daml `_7` field.
            #[serde(rename = "_7")]
            pub _7: T7,
            ///The Daml `_8` field.
            #[serde(rename = "_8")]
            pub _8: T8,
            ///The Daml `_9` field.
            #[serde(rename = "_9")]
            pub _9: T9,
            ///The Daml `_10` field.
            #[serde(rename = "_10")]
            pub _10: T10,
            ///The Daml `_11` field.
            #[serde(rename = "_11")]
            pub _11: T11,
            ///The Daml `_12` field.
            #[serde(rename = "_12")]
            pub _12: T12,
            ///The Daml `_13` field.
            #[serde(rename = "_13")]
            pub _13: T13,
            ///The Daml `_14` field.
            #[serde(rename = "_14")]
            pub _14: T14,
            ///The Daml `_15` field.
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
                    _1: rt::FromValue::from_value(rt::record_field(value, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::record_field(value, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::record_field(value, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::record_field(value, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::record_field(value, "_5")?)?,
                    _6: rt::FromValue::from_value(rt::record_field(value, "_6")?)?,
                    _7: rt::FromValue::from_value(rt::record_field(value, "_7")?)?,
                    _8: rt::FromValue::from_value(rt::record_field(value, "_8")?)?,
                    _9: rt::FromValue::from_value(rt::record_field(value, "_9")?)?,
                    _10: rt::FromValue::from_value(rt::record_field(value, "_10")?)?,
                    _11: rt::FromValue::from_value(rt::record_field(value, "_11")?)?,
                    _12: rt::FromValue::from_value(rt::record_field(value, "_12")?)?,
                    _13: rt::FromValue::from_value(rt::record_field(value, "_13")?)?,
                    _14: rt::FromValue::from_value(rt::record_field(value, "_14")?)?,
                    _15: rt::FromValue::from_value(rt::record_field(value, "_15")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Tuple16<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16> {
            ///The Daml `_1` field.
            #[serde(rename = "_1")]
            pub _1: T1,
            ///The Daml `_2` field.
            #[serde(rename = "_2")]
            pub _2: T2,
            ///The Daml `_3` field.
            #[serde(rename = "_3")]
            pub _3: T3,
            ///The Daml `_4` field.
            #[serde(rename = "_4")]
            pub _4: T4,
            ///The Daml `_5` field.
            #[serde(rename = "_5")]
            pub _5: T5,
            ///The Daml `_6` field.
            #[serde(rename = "_6")]
            pub _6: T6,
            ///The Daml `_7` field.
            #[serde(rename = "_7")]
            pub _7: T7,
            ///The Daml `_8` field.
            #[serde(rename = "_8")]
            pub _8: T8,
            ///The Daml `_9` field.
            #[serde(rename = "_9")]
            pub _9: T9,
            ///The Daml `_10` field.
            #[serde(rename = "_10")]
            pub _10: T10,
            ///The Daml `_11` field.
            #[serde(rename = "_11")]
            pub _11: T11,
            ///The Daml `_12` field.
            #[serde(rename = "_12")]
            pub _12: T12,
            ///The Daml `_13` field.
            #[serde(rename = "_13")]
            pub _13: T13,
            ///The Daml `_14` field.
            #[serde(rename = "_14")]
            pub _14: T14,
            ///The Daml `_15` field.
            #[serde(rename = "_15")]
            pub _15: T15,
            ///The Daml `_16` field.
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
                    _1: rt::FromValue::from_value(rt::record_field(value, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::record_field(value, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::record_field(value, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::record_field(value, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::record_field(value, "_5")?)?,
                    _6: rt::FromValue::from_value(rt::record_field(value, "_6")?)?,
                    _7: rt::FromValue::from_value(rt::record_field(value, "_7")?)?,
                    _8: rt::FromValue::from_value(rt::record_field(value, "_8")?)?,
                    _9: rt::FromValue::from_value(rt::record_field(value, "_9")?)?,
                    _10: rt::FromValue::from_value(rt::record_field(value, "_10")?)?,
                    _11: rt::FromValue::from_value(rt::record_field(value, "_11")?)?,
                    _12: rt::FromValue::from_value(rt::record_field(value, "_12")?)?,
                    _13: rt::FromValue::from_value(rt::record_field(value, "_13")?)?,
                    _14: rt::FromValue::from_value(rt::record_field(value, "_14")?)?,
                    _15: rt::FromValue::from_value(rt::record_field(value, "_15")?)?,
                    _16: rt::FromValue::from_value(rt::record_field(value, "_16")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
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
            ///The Daml `_1` field.
            #[serde(rename = "_1")]
            pub _1: T1,
            ///The Daml `_2` field.
            #[serde(rename = "_2")]
            pub _2: T2,
            ///The Daml `_3` field.
            #[serde(rename = "_3")]
            pub _3: T3,
            ///The Daml `_4` field.
            #[serde(rename = "_4")]
            pub _4: T4,
            ///The Daml `_5` field.
            #[serde(rename = "_5")]
            pub _5: T5,
            ///The Daml `_6` field.
            #[serde(rename = "_6")]
            pub _6: T6,
            ///The Daml `_7` field.
            #[serde(rename = "_7")]
            pub _7: T7,
            ///The Daml `_8` field.
            #[serde(rename = "_8")]
            pub _8: T8,
            ///The Daml `_9` field.
            #[serde(rename = "_9")]
            pub _9: T9,
            ///The Daml `_10` field.
            #[serde(rename = "_10")]
            pub _10: T10,
            ///The Daml `_11` field.
            #[serde(rename = "_11")]
            pub _11: T11,
            ///The Daml `_12` field.
            #[serde(rename = "_12")]
            pub _12: T12,
            ///The Daml `_13` field.
            #[serde(rename = "_13")]
            pub _13: T13,
            ///The Daml `_14` field.
            #[serde(rename = "_14")]
            pub _14: T14,
            ///The Daml `_15` field.
            #[serde(rename = "_15")]
            pub _15: T15,
            ///The Daml `_16` field.
            #[serde(rename = "_16")]
            pub _16: T16,
            ///The Daml `_17` field.
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
                    _1: rt::FromValue::from_value(rt::record_field(value, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::record_field(value, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::record_field(value, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::record_field(value, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::record_field(value, "_5")?)?,
                    _6: rt::FromValue::from_value(rt::record_field(value, "_6")?)?,
                    _7: rt::FromValue::from_value(rt::record_field(value, "_7")?)?,
                    _8: rt::FromValue::from_value(rt::record_field(value, "_8")?)?,
                    _9: rt::FromValue::from_value(rt::record_field(value, "_9")?)?,
                    _10: rt::FromValue::from_value(rt::record_field(value, "_10")?)?,
                    _11: rt::FromValue::from_value(rt::record_field(value, "_11")?)?,
                    _12: rt::FromValue::from_value(rt::record_field(value, "_12")?)?,
                    _13: rt::FromValue::from_value(rt::record_field(value, "_13")?)?,
                    _14: rt::FromValue::from_value(rt::record_field(value, "_14")?)?,
                    _15: rt::FromValue::from_value(rt::record_field(value, "_15")?)?,
                    _16: rt::FromValue::from_value(rt::record_field(value, "_16")?)?,
                    _17: rt::FromValue::from_value(rt::record_field(value, "_17")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
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
            ///The Daml `_1` field.
            #[serde(rename = "_1")]
            pub _1: T1,
            ///The Daml `_2` field.
            #[serde(rename = "_2")]
            pub _2: T2,
            ///The Daml `_3` field.
            #[serde(rename = "_3")]
            pub _3: T3,
            ///The Daml `_4` field.
            #[serde(rename = "_4")]
            pub _4: T4,
            ///The Daml `_5` field.
            #[serde(rename = "_5")]
            pub _5: T5,
            ///The Daml `_6` field.
            #[serde(rename = "_6")]
            pub _6: T6,
            ///The Daml `_7` field.
            #[serde(rename = "_7")]
            pub _7: T7,
            ///The Daml `_8` field.
            #[serde(rename = "_8")]
            pub _8: T8,
            ///The Daml `_9` field.
            #[serde(rename = "_9")]
            pub _9: T9,
            ///The Daml `_10` field.
            #[serde(rename = "_10")]
            pub _10: T10,
            ///The Daml `_11` field.
            #[serde(rename = "_11")]
            pub _11: T11,
            ///The Daml `_12` field.
            #[serde(rename = "_12")]
            pub _12: T12,
            ///The Daml `_13` field.
            #[serde(rename = "_13")]
            pub _13: T13,
            ///The Daml `_14` field.
            #[serde(rename = "_14")]
            pub _14: T14,
            ///The Daml `_15` field.
            #[serde(rename = "_15")]
            pub _15: T15,
            ///The Daml `_16` field.
            #[serde(rename = "_16")]
            pub _16: T16,
            ///The Daml `_17` field.
            #[serde(rename = "_17")]
            pub _17: T17,
            ///The Daml `_18` field.
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
                    _1: rt::FromValue::from_value(rt::record_field(value, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::record_field(value, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::record_field(value, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::record_field(value, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::record_field(value, "_5")?)?,
                    _6: rt::FromValue::from_value(rt::record_field(value, "_6")?)?,
                    _7: rt::FromValue::from_value(rt::record_field(value, "_7")?)?,
                    _8: rt::FromValue::from_value(rt::record_field(value, "_8")?)?,
                    _9: rt::FromValue::from_value(rt::record_field(value, "_9")?)?,
                    _10: rt::FromValue::from_value(rt::record_field(value, "_10")?)?,
                    _11: rt::FromValue::from_value(rt::record_field(value, "_11")?)?,
                    _12: rt::FromValue::from_value(rt::record_field(value, "_12")?)?,
                    _13: rt::FromValue::from_value(rt::record_field(value, "_13")?)?,
                    _14: rt::FromValue::from_value(rt::record_field(value, "_14")?)?,
                    _15: rt::FromValue::from_value(rt::record_field(value, "_15")?)?,
                    _16: rt::FromValue::from_value(rt::record_field(value, "_16")?)?,
                    _17: rt::FromValue::from_value(rt::record_field(value, "_17")?)?,
                    _18: rt::FromValue::from_value(rt::record_field(value, "_18")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
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
            ///The Daml `_1` field.
            #[serde(rename = "_1")]
            pub _1: T1,
            ///The Daml `_2` field.
            #[serde(rename = "_2")]
            pub _2: T2,
            ///The Daml `_3` field.
            #[serde(rename = "_3")]
            pub _3: T3,
            ///The Daml `_4` field.
            #[serde(rename = "_4")]
            pub _4: T4,
            ///The Daml `_5` field.
            #[serde(rename = "_5")]
            pub _5: T5,
            ///The Daml `_6` field.
            #[serde(rename = "_6")]
            pub _6: T6,
            ///The Daml `_7` field.
            #[serde(rename = "_7")]
            pub _7: T7,
            ///The Daml `_8` field.
            #[serde(rename = "_8")]
            pub _8: T8,
            ///The Daml `_9` field.
            #[serde(rename = "_9")]
            pub _9: T9,
            ///The Daml `_10` field.
            #[serde(rename = "_10")]
            pub _10: T10,
            ///The Daml `_11` field.
            #[serde(rename = "_11")]
            pub _11: T11,
            ///The Daml `_12` field.
            #[serde(rename = "_12")]
            pub _12: T12,
            ///The Daml `_13` field.
            #[serde(rename = "_13")]
            pub _13: T13,
            ///The Daml `_14` field.
            #[serde(rename = "_14")]
            pub _14: T14,
            ///The Daml `_15` field.
            #[serde(rename = "_15")]
            pub _15: T15,
            ///The Daml `_16` field.
            #[serde(rename = "_16")]
            pub _16: T16,
            ///The Daml `_17` field.
            #[serde(rename = "_17")]
            pub _17: T17,
            ///The Daml `_18` field.
            #[serde(rename = "_18")]
            pub _18: T18,
            ///The Daml `_19` field.
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
                    _1: rt::FromValue::from_value(rt::record_field(value, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::record_field(value, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::record_field(value, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::record_field(value, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::record_field(value, "_5")?)?,
                    _6: rt::FromValue::from_value(rt::record_field(value, "_6")?)?,
                    _7: rt::FromValue::from_value(rt::record_field(value, "_7")?)?,
                    _8: rt::FromValue::from_value(rt::record_field(value, "_8")?)?,
                    _9: rt::FromValue::from_value(rt::record_field(value, "_9")?)?,
                    _10: rt::FromValue::from_value(rt::record_field(value, "_10")?)?,
                    _11: rt::FromValue::from_value(rt::record_field(value, "_11")?)?,
                    _12: rt::FromValue::from_value(rt::record_field(value, "_12")?)?,
                    _13: rt::FromValue::from_value(rt::record_field(value, "_13")?)?,
                    _14: rt::FromValue::from_value(rt::record_field(value, "_14")?)?,
                    _15: rt::FromValue::from_value(rt::record_field(value, "_15")?)?,
                    _16: rt::FromValue::from_value(rt::record_field(value, "_16")?)?,
                    _17: rt::FromValue::from_value(rt::record_field(value, "_17")?)?,
                    _18: rt::FromValue::from_value(rt::record_field(value, "_18")?)?,
                    _19: rt::FromValue::from_value(rt::record_field(value, "_19")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
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
            ///The Daml `_1` field.
            #[serde(rename = "_1")]
            pub _1: T1,
            ///The Daml `_2` field.
            #[serde(rename = "_2")]
            pub _2: T2,
            ///The Daml `_3` field.
            #[serde(rename = "_3")]
            pub _3: T3,
            ///The Daml `_4` field.
            #[serde(rename = "_4")]
            pub _4: T4,
            ///The Daml `_5` field.
            #[serde(rename = "_5")]
            pub _5: T5,
            ///The Daml `_6` field.
            #[serde(rename = "_6")]
            pub _6: T6,
            ///The Daml `_7` field.
            #[serde(rename = "_7")]
            pub _7: T7,
            ///The Daml `_8` field.
            #[serde(rename = "_8")]
            pub _8: T8,
            ///The Daml `_9` field.
            #[serde(rename = "_9")]
            pub _9: T9,
            ///The Daml `_10` field.
            #[serde(rename = "_10")]
            pub _10: T10,
            ///The Daml `_11` field.
            #[serde(rename = "_11")]
            pub _11: T11,
            ///The Daml `_12` field.
            #[serde(rename = "_12")]
            pub _12: T12,
            ///The Daml `_13` field.
            #[serde(rename = "_13")]
            pub _13: T13,
            ///The Daml `_14` field.
            #[serde(rename = "_14")]
            pub _14: T14,
            ///The Daml `_15` field.
            #[serde(rename = "_15")]
            pub _15: T15,
            ///The Daml `_16` field.
            #[serde(rename = "_16")]
            pub _16: T16,
            ///The Daml `_17` field.
            #[serde(rename = "_17")]
            pub _17: T17,
            ///The Daml `_18` field.
            #[serde(rename = "_18")]
            pub _18: T18,
            ///The Daml `_19` field.
            #[serde(rename = "_19")]
            pub _19: T19,
            ///The Daml `_20` field.
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
                    _1: rt::FromValue::from_value(rt::record_field(value, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::record_field(value, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::record_field(value, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::record_field(value, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::record_field(value, "_5")?)?,
                    _6: rt::FromValue::from_value(rt::record_field(value, "_6")?)?,
                    _7: rt::FromValue::from_value(rt::record_field(value, "_7")?)?,
                    _8: rt::FromValue::from_value(rt::record_field(value, "_8")?)?,
                    _9: rt::FromValue::from_value(rt::record_field(value, "_9")?)?,
                    _10: rt::FromValue::from_value(rt::record_field(value, "_10")?)?,
                    _11: rt::FromValue::from_value(rt::record_field(value, "_11")?)?,
                    _12: rt::FromValue::from_value(rt::record_field(value, "_12")?)?,
                    _13: rt::FromValue::from_value(rt::record_field(value, "_13")?)?,
                    _14: rt::FromValue::from_value(rt::record_field(value, "_14")?)?,
                    _15: rt::FromValue::from_value(rt::record_field(value, "_15")?)?,
                    _16: rt::FromValue::from_value(rt::record_field(value, "_16")?)?,
                    _17: rt::FromValue::from_value(rt::record_field(value, "_17")?)?,
                    _18: rt::FromValue::from_value(rt::record_field(value, "_18")?)?,
                    _19: rt::FromValue::from_value(rt::record_field(value, "_19")?)?,
                    _20: rt::FromValue::from_value(rt::record_field(value, "_20")?)?,
                })
            }
        }
    }
}
pub mod daml_prim_GHC_Tuple_1_0_0 {
    pub mod GHC_Tuple {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Unit<A> {
            ///The Daml `_1` field.
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
                    _1: rt::FromValue::from_value(rt::record_field(value, "_1")?)?,
                })
            }
        }
    }
}
pub mod daml_prim_GHC_Types_1_0_0 {
    pub mod GHC_Types {
        use canton_daml as rt;
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize,
        )]
        #[serde(crate = "rt::serde")]
        pub enum Ordering {
            ///The Daml `LT` value.
            #[serde(rename = "LT")]
            LT,
            ///The Daml `EQ` value.
            #[serde(rename = "EQ")]
            EQ,
            ///The Daml `GT` value.
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
pub mod daml_stdlib_DA_Date_Types_1_0_0 {
    pub mod DA_Date_Types {
        use canton_daml as rt;
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize,
        )]
        #[serde(crate = "rt::serde")]
        pub enum DayOfWeek {
            ///The Daml `Monday` value.
            #[serde(rename = "Monday")]
            Monday,
            ///The Daml `Tuesday` value.
            #[serde(rename = "Tuesday")]
            Tuesday,
            ///The Daml `Wednesday` value.
            #[serde(rename = "Wednesday")]
            Wednesday,
            ///The Daml `Thursday` value.
            #[serde(rename = "Thursday")]
            Thursday,
            ///The Daml `Friday` value.
            #[serde(rename = "Friday")]
            Friday,
            ///The Daml `Saturday` value.
            #[serde(rename = "Saturday")]
            Saturday,
            ///The Daml `Sunday` value.
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
            ///The Daml `Jan` value.
            #[serde(rename = "Jan")]
            Jan,
            ///The Daml `Feb` value.
            #[serde(rename = "Feb")]
            Feb,
            ///The Daml `Mar` value.
            #[serde(rename = "Mar")]
            Mar,
            ///The Daml `Apr` value.
            #[serde(rename = "Apr")]
            Apr,
            ///The Daml `May` value.
            #[serde(rename = "May")]
            May,
            ///The Daml `Jun` value.
            #[serde(rename = "Jun")]
            Jun,
            ///The Daml `Jul` value.
            #[serde(rename = "Jul")]
            Jul,
            ///The Daml `Aug` value.
            #[serde(rename = "Aug")]
            Aug,
            ///The Daml `Sep` value.
            #[serde(rename = "Sep")]
            Sep,
            ///The Daml `Oct` value.
            #[serde(rename = "Oct")]
            Oct,
            ///The Daml `Nov` value.
            #[serde(rename = "Nov")]
            Nov,
            ///The Daml `Dec` value.
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
pub mod daml_stdlib_DA_Internal_Down_1_0_0 {
    pub mod DA_Internal_Down {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Down<A> {
            ///The Daml `unpack` field.
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
                    unpack: rt::FromValue::from_value(rt::record_field(value, "unpack")?)?,
                })
            }
        }
    }
}
pub mod ghc_stdlib_DA_Internal_Template_1_0_0 {
    pub mod DA_Internal_Template {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Archive {}
        impl rt::ToValue for Archive {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for Archive {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
    }
}
pub mod daml_stdlib_DA_Logic_Types_1_0_0 {
    pub mod DA_Logic_Types {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum Formula<A> {
            ///The Daml `Proposition` constructor.
            #[serde(rename = "Proposition")]
            Proposition(A),
            ///The Daml `Negation` constructor.
            #[serde(rename = "Negation")]
            Negation(Box<crate::daml_stdlib_DA_Logic_Types_1_0_0::DA_Logic_Types::Formula<A>>),
            ///The Daml `Conjunction` constructor.
            #[serde(rename = "Conjunction")]
            Conjunction(Vec<crate::daml_stdlib_DA_Logic_Types_1_0_0::DA_Logic_Types::Formula<A>>),
            ///The Daml `Disjunction` constructor.
            #[serde(rename = "Disjunction")]
            Disjunction(Vec<crate::daml_stdlib_DA_Logic_Types_1_0_0::DA_Logic_Types::Formula<A>>),
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
                        rt::FromValue::from_value(payload)?,
                    )),
                    "Negation" => ::core::result::Result::Ok(Formula::Negation(
                        rt::FromValue::from_value(payload)?,
                    )),
                    "Conjunction" => ::core::result::Result::Ok(Formula::Conjunction(
                        rt::FromValue::from_value(payload)?,
                    )),
                    "Disjunction" => ::core::result::Result::Ok(Formula::Disjunction(
                        rt::FromValue::from_value(payload)?,
                    )),
                    other => {
                        ::core::result::Result::Err(rt::unexpected_constructor("Formula", other))
                    }
                }
            }
        }
    }
}
pub mod daml_stdlib_DA_Monoid_Types_1_0_0 {
    pub mod DA_Monoid_Types {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct All {
            ///The Daml `getAll` field.
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
                    get_all: rt::FromValue::from_value(rt::record_field(value, "getAll")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Any {
            ///The Daml `getAny` field.
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
                    get_any: rt::FromValue::from_value(rt::record_field(value, "getAny")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Sum<A> {
            ///The Daml `unpack` field.
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
                    unpack: rt::FromValue::from_value(rt::record_field(value, "unpack")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Product<A> {
            ///The Daml `unpack` field.
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
                    unpack: rt::FromValue::from_value(rt::record_field(value, "unpack")?)?,
                })
            }
        }
    }
}
pub mod daml_stdlib_DA_NonEmpty_Types_1_0_0 {
    pub mod DA_NonEmpty_Types {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct NonEmpty<A> {
            ///The Daml `hd` field.
            #[serde(rename = "hd")]
            pub hd: A,
            ///The Daml `tl` field.
            #[serde(rename = "tl")]
            pub tl: Vec<A>,
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
                    hd: rt::FromValue::from_value(rt::record_field(value, "hd")?)?,
                    tl: rt::FromValue::from_value(rt::record_field(value, "tl")?)?,
                })
            }
        }
    }
}
pub mod daml_stdlib_DA_Random_Types_1_0_0 {
    pub mod DA_Random_Types {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum Minstd {
            ///The Daml `Minstd` constructor.
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
                        rt::FromValue::from_value(payload)?,
                    )),
                    other => {
                        ::core::result::Result::Err(rt::unexpected_constructor("Minstd", other))
                    }
                }
            }
        }
    }
}
pub mod daml_stdlib_DA_Semigroup_Types_1_0_0 {
    pub mod DA_Semigroup_Types {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Min<A> {
            ///The Daml `unpack` field.
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
                    unpack: rt::FromValue::from_value(rt::record_field(value, "unpack")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Max<A> {
            ///The Daml `unpack` field.
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
                    unpack: rt::FromValue::from_value(rt::record_field(value, "unpack")?)?,
                })
            }
        }
    }
}
pub mod daml_stdlib_DA_Set_Types_1_0_0 {
    pub mod DA_Set_Types {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Set<K> {
            ///The Daml `map` field.
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
                    map: rt::FromValue::from_value(rt::record_field(value, "map")?)?,
                })
            }
        }
    }
}
pub mod daml_stdlib_DA_Stack_Types_1_0_0 {
    pub mod DA_Stack_Types {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SrcLoc {
            ///The Daml `srcLocPackage` field.
            #[serde(rename = "srcLocPackage")]
            pub src_loc_package: String,
            ///The Daml `srcLocModule` field.
            #[serde(rename = "srcLocModule")]
            pub src_loc_module: String,
            ///The Daml `srcLocFile` field.
            #[serde(rename = "srcLocFile")]
            pub src_loc_file: String,
            ///The Daml `srcLocStartLine` field.
            #[serde(rename = "srcLocStartLine")]
            pub src_loc_start_line: rt::Int64,
            ///The Daml `srcLocStartCol` field.
            #[serde(rename = "srcLocStartCol")]
            pub src_loc_start_col: rt::Int64,
            ///The Daml `srcLocEndLine` field.
            #[serde(rename = "srcLocEndLine")]
            pub src_loc_end_line: rt::Int64,
            ///The Daml `srcLocEndCol` field.
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
                    src_loc_package: rt::FromValue::from_value(rt::record_field(
                        value,
                        "srcLocPackage",
                    )?)?,
                    src_loc_module: rt::FromValue::from_value(rt::record_field(
                        value,
                        "srcLocModule",
                    )?)?,
                    src_loc_file: rt::FromValue::from_value(rt::record_field(
                        value,
                        "srcLocFile",
                    )?)?,
                    src_loc_start_line: rt::FromValue::from_value(rt::record_field(
                        value,
                        "srcLocStartLine",
                    )?)?,
                    src_loc_start_col: rt::FromValue::from_value(rt::record_field(
                        value,
                        "srcLocStartCol",
                    )?)?,
                    src_loc_end_line: rt::FromValue::from_value(rt::record_field(
                        value,
                        "srcLocEndLine",
                    )?)?,
                    src_loc_end_col: rt::FromValue::from_value(rt::record_field(
                        value,
                        "srcLocEndCol",
                    )?)?,
                })
            }
        }
    }
}
pub mod daml_stdlib_DA_Time_Types_1_0_0 {
    pub mod DA_Time_Types {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct RelTime {
            ///The Daml `microseconds` field.
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
                    microseconds: rt::FromValue::from_value(rt::record_field(
                        value,
                        "microseconds",
                    )?)?,
                })
            }
        }
    }
}
pub mod daml_stdlib_DA_Validation_Types_1_0_0 {
    pub mod DA_Validation_Types {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum Validation<Errs, A> {
            ///The Daml `Errors` constructor.
            #[serde(rename = "Errors")]
            Errors(crate::daml_stdlib_DA_NonEmpty_Types_1_0_0::DA_NonEmpty_Types::NonEmpty<Errs>),
            ///The Daml `Success` constructor.
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
                        rt::FromValue::from_value(payload)?,
                    )),
                    "Success" => ::core::result::Result::Ok(Validation::Success(
                        rt::FromValue::from_value(payload)?,
                    )),
                    other => {
                        ::core::result::Result::Err(rt::unexpected_constructor("Validation", other))
                    }
                }
            }
        }
    }
}
pub mod quickstart_licensing_0_0_1 {
    pub mod Licensing_License {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct LicenseRenewalRequest_CompleteRenewal {
            ///The Daml `allocationCid` field.
            #[serde(rename = "allocationCid")]
            pub allocation_cid: rt::ContractId<
                crate::splice_api_token_allocation_v1_1_0_0::Splice_Api_Token_AllocationV1::Allocation,
            >,
            ///The Daml `licenseCid` field.
            #[serde(rename = "licenseCid")]
            pub license_cid: rt::ContractId<
                crate::quickstart_licensing_0_0_1::Licensing_License::License,
            >,
            ///The Daml `extraArgs` field.
            #[serde(rename = "extraArgs")]
            pub extra_args: crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::ExtraArgs,
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
                    allocation_cid: rt::FromValue::from_value(rt::record_field(
                        value,
                        "allocationCid",
                    )?)?,
                    license_cid: rt::FromValue::from_value(rt::record_field(value, "licenseCid")?)?,
                    extra_args: rt::FromValue::from_value(rt::record_field(value, "extraArgs")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct License_Expire {
            ///The Daml `actor` field.
            #[serde(rename = "actor")]
            pub actor: rt::Party,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta:
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
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
                    actor: rt::FromValue::from_value(rt::record_field(value, "actor")?)?,
                    meta: rt::FromValue::from_value(rt::record_field(value, "meta")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct License_Renew {
            ///The Daml `requestId` field.
            #[serde(rename = "requestId")]
            pub request_id: String,
            ///The Daml `licenseFeeInstrumentId` field.
            #[serde(rename = "licenseFeeInstrumentId")]
            pub license_fee_instrument_id:
                crate::splice_api_token_holding_v1_1_0_0::Splice_Api_Token_HoldingV1::InstrumentId,
            ///The Daml `licenseFeeAmount` field.
            #[serde(rename = "licenseFeeAmount")]
            pub license_fee_amount: rt::Numeric,
            ///The Daml `licenseExtensionDuration` field.
            #[serde(rename = "licenseExtensionDuration")]
            pub license_extension_duration:
                crate::daml_stdlib_DA_Time_Types_1_0_0::DA_Time_Types::RelTime,
            ///The Daml `requestedAt` field.
            #[serde(rename = "requestedAt")]
            pub requested_at: rt::Timestamp,
            ///The Daml `prepareUntil` field.
            #[serde(rename = "prepareUntil")]
            pub prepare_until: rt::Timestamp,
            ///The Daml `settleBefore` field.
            #[serde(rename = "settleBefore")]
            pub settle_before: rt::Timestamp,
            ///The Daml `description` field.
            #[serde(rename = "description")]
            pub description: String,
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
                    request_id: rt::FromValue::from_value(rt::record_field(value, "requestId")?)?,
                    license_fee_instrument_id: rt::FromValue::from_value(rt::record_field(
                        value,
                        "licenseFeeInstrumentId",
                    )?)?,
                    license_fee_amount: rt::FromValue::from_value(rt::record_field(
                        value,
                        "licenseFeeAmount",
                    )?)?,
                    license_extension_duration: rt::FromValue::from_value(rt::record_field(
                        value,
                        "licenseExtensionDuration",
                    )?)?,
                    requested_at: rt::FromValue::from_value(rt::record_field(
                        value,
                        "requestedAt",
                    )?)?,
                    prepare_until: rt::FromValue::from_value(rt::record_field(
                        value,
                        "prepareUntil",
                    )?)?,
                    settle_before: rt::FromValue::from_value(rt::record_field(
                        value,
                        "settleBefore",
                    )?)?,
                    description: rt::FromValue::from_value(rt::record_field(
                        value,
                        "description",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct LicenseParams {
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta:
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for LicenseParams {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("meta", rt::ToValue::to_value(&self.meta)),])
            }
        }
        impl rt::FromValue for LicenseParams {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    meta: rt::FromValue::from_value(rt::record_field(value, "meta")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct License {
            ///The Daml `provider` field.
            #[serde(rename = "provider")]
            pub provider: rt::Party,
            ///The Daml `user` field.
            #[serde(rename = "user")]
            pub user: rt::Party,
            ///The Daml `expiresAt` field.
            #[serde(rename = "expiresAt")]
            pub expires_at: rt::Timestamp,
            ///The Daml `licenseNum` field.
            #[serde(rename = "licenseNum")]
            pub license_num: rt::Int64,
            ///The Daml `params` field.
            #[serde(rename = "params")]
            pub params: crate::quickstart_licensing_0_0_1::Licensing_License::LicenseParams,
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
                    provider: rt::FromValue::from_value(rt::record_field(value, "provider")?)?,
                    user: rt::FromValue::from_value(rt::record_field(value, "user")?)?,
                    expires_at: rt::FromValue::from_value(rt::record_field(value, "expiresAt")?)?,
                    license_num: rt::FromValue::from_value(rt::record_field(value, "licenseNum")?)?,
                    params: rt::FromValue::from_value(rt::record_field(value, "params")?)?,
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
        impl rt::Template for License {}
        ///The `Archive` choice on [`License`] (consuming).
        impl rt::Choice<License>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `License_Renew` choice on [`License`] (non-consuming).
        impl rt::Choice<License> for crate::quickstart_licensing_0_0_1::Licensing_License::License_Renew {
            type Return = rt::ContractId<
                crate::quickstart_licensing_0_0_1::Licensing_License::LicenseRenewalRequest,
            >;
            const NAME: &'static str = "License_Renew";
            const CONSUMING: bool = false;
        }
        ///The `License_Expire` choice on [`License`] (consuming).
        impl rt::Choice<License> for crate::quickstart_licensing_0_0_1::Licensing_License::License_Expire {
            type Return = rt::Unit;
            const NAME: &'static str = "License_Expire";
            const CONSUMING: bool = true;
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct LicenseRenewalRequest {
            ///The Daml `requestId` field.
            #[serde(rename = "requestId")]
            pub request_id: String,
            ///The Daml `provider` field.
            #[serde(rename = "provider")]
            pub provider: rt::Party,
            ///The Daml `user` field.
            #[serde(rename = "user")]
            pub user: rt::Party,
            ///The Daml `licenseNum` field.
            #[serde(rename = "licenseNum")]
            pub license_num: rt::Int64,
            ///The Daml `licenseFeeAmount` field.
            #[serde(rename = "licenseFeeAmount")]
            pub license_fee_amount: rt::Numeric,
            ///The Daml `licenseFeeInstrumentId` field.
            #[serde(rename = "licenseFeeInstrumentId")]
            pub license_fee_instrument_id:
                crate::splice_api_token_holding_v1_1_0_0::Splice_Api_Token_HoldingV1::InstrumentId,
            ///The Daml `licenseExtensionDuration` field.
            #[serde(rename = "licenseExtensionDuration")]
            pub license_extension_duration:
                crate::daml_stdlib_DA_Time_Types_1_0_0::DA_Time_Types::RelTime,
            ///The Daml `prepareUntil` field.
            #[serde(rename = "prepareUntil")]
            pub prepare_until: rt::Timestamp,
            ///The Daml `settleBefore` field.
            #[serde(rename = "settleBefore")]
            pub settle_before: rt::Timestamp,
            ///The Daml `requestedAt` field.
            #[serde(rename = "requestedAt")]
            pub requested_at: rt::Timestamp,
            ///The Daml `description` field.
            #[serde(rename = "description")]
            pub description: String,
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
                    request_id: rt::FromValue::from_value(rt::record_field(value, "requestId")?)?,
                    provider: rt::FromValue::from_value(rt::record_field(value, "provider")?)?,
                    user: rt::FromValue::from_value(rt::record_field(value, "user")?)?,
                    license_num: rt::FromValue::from_value(rt::record_field(value, "licenseNum")?)?,
                    license_fee_amount: rt::FromValue::from_value(rt::record_field(
                        value,
                        "licenseFeeAmount",
                    )?)?,
                    license_fee_instrument_id: rt::FromValue::from_value(rt::record_field(
                        value,
                        "licenseFeeInstrumentId",
                    )?)?,
                    license_extension_duration: rt::FromValue::from_value(rt::record_field(
                        value,
                        "licenseExtensionDuration",
                    )?)?,
                    prepare_until: rt::FromValue::from_value(rt::record_field(
                        value,
                        "prepareUntil",
                    )?)?,
                    settle_before: rt::FromValue::from_value(rt::record_field(
                        value,
                        "settleBefore",
                    )?)?,
                    requested_at: rt::FromValue::from_value(rt::record_field(
                        value,
                        "requestedAt",
                    )?)?,
                    description: rt::FromValue::from_value(rt::record_field(
                        value,
                        "description",
                    )?)?,
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
        impl rt::Template for LicenseRenewalRequest {}
        ///The `LicenseRenewalRequest_CompleteRenewal` choice on [`LicenseRenewalRequest`] (non-consuming).
        impl rt::Choice<LicenseRenewalRequest>
        for crate::quickstart_licensing_0_0_1::Licensing_License::LicenseRenewalRequest_CompleteRenewal {
            type Return = rt::ContractId<
                crate::quickstart_licensing_0_0_1::Licensing_License::License,
            >;
            const NAME: &'static str = "LicenseRenewalRequest_CompleteRenewal";
            const CONSUMING: bool = false;
        }
        ///The `Archive` choice on [`LicenseRenewalRequest`] (consuming).
        impl rt::Choice<LicenseRenewalRequest>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
    }
    pub mod Licensing_AppInstall {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AppInstallRequest_Reject {
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta:
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for AppInstallRequest_Reject {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("meta", rt::ToValue::to_value(&self.meta)),])
            }
        }
        impl rt::FromValue for AppInstallRequest_Reject {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    meta: rt::FromValue::from_value(rt::record_field(value, "meta")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AppInstallRequest_Accept {
            ///The Daml `installMeta` field.
            #[serde(rename = "installMeta")]
            pub install_meta:
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta:
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
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
                    install_meta: rt::FromValue::from_value(rt::record_field(
                        value,
                        "installMeta",
                    )?)?,
                    meta: rt::FromValue::from_value(rt::record_field(value, "meta")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AppInstall_Cancel {
            ///The Daml `actor` field.
            #[serde(rename = "actor")]
            pub actor: rt::Party,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta:
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
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
                    actor: rt::FromValue::from_value(rt::record_field(value, "actor")?)?,
                    meta: rt::FromValue::from_value(rt::record_field(value, "meta")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AppInstall_CreateLicense {
            ///The Daml `params` field.
            #[serde(rename = "params")]
            pub params: crate::quickstart_licensing_0_0_1::Licensing_License::LicenseParams,
        }
        impl rt::ToValue for AppInstall_CreateLicense {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("params", rt::ToValue::to_value(&self.params)),])
            }
        }
        impl rt::FromValue for AppInstall_CreateLicense {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    params: rt::FromValue::from_value(rt::record_field(value, "params")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AppInstall_CreateLicense_Result {
            ///The Daml `installId` field.
            #[serde(rename = "installId")]
            pub install_id:
                rt::ContractId<crate::quickstart_licensing_0_0_1::Licensing_AppInstall::AppInstall>,
            ///The Daml `licenseId` field.
            #[serde(rename = "licenseId")]
            pub license_id:
                rt::ContractId<crate::quickstart_licensing_0_0_1::Licensing_License::License>,
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
                    install_id: rt::FromValue::from_value(rt::record_field(value, "installId")?)?,
                    license_id: rt::FromValue::from_value(rt::record_field(value, "licenseId")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AppInstall {
            ///The Daml `provider` field.
            #[serde(rename = "provider")]
            pub provider: rt::Party,
            ///The Daml `user` field.
            #[serde(rename = "user")]
            pub user: rt::Party,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta:
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
            ///The Daml `numLicensesCreated` field.
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
                    provider: rt::FromValue::from_value(rt::record_field(value, "provider")?)?,
                    user: rt::FromValue::from_value(rt::record_field(value, "user")?)?,
                    meta: rt::FromValue::from_value(rt::record_field(value, "meta")?)?,
                    num_licenses_created: rt::FromValue::from_value(rt::record_field(
                        value,
                        "numLicensesCreated",
                    )?)?,
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
        impl rt::Template for AppInstall {}
        ///The `AppInstall_CreateLicense` choice on [`AppInstall`] (consuming).
        impl rt::Choice<AppInstall>
            for crate::quickstart_licensing_0_0_1::Licensing_AppInstall::AppInstall_CreateLicense
        {
            type Return = crate::quickstart_licensing_0_0_1::Licensing_AppInstall::AppInstall_CreateLicense_Result;
            const NAME: &'static str = "AppInstall_CreateLicense";
            const CONSUMING: bool = true;
        }
        ///The `AppInstall_Cancel` choice on [`AppInstall`] (consuming).
        impl rt::Choice<AppInstall>
            for crate::quickstart_licensing_0_0_1::Licensing_AppInstall::AppInstall_Cancel
        {
            type Return = rt::Unit;
            const NAME: &'static str = "AppInstall_Cancel";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`AppInstall`] (consuming).
        impl rt::Choice<AppInstall>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AppInstallRequest {
            ///The Daml `provider` field.
            #[serde(rename = "provider")]
            pub provider: rt::Party,
            ///The Daml `user` field.
            #[serde(rename = "user")]
            pub user: rt::Party,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta:
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
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
                    provider: rt::FromValue::from_value(rt::record_field(value, "provider")?)?,
                    user: rt::FromValue::from_value(rt::record_field(value, "user")?)?,
                    meta: rt::FromValue::from_value(rt::record_field(value, "meta")?)?,
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
        impl rt::Template for AppInstallRequest {}
        ///The `AppInstallRequest_Accept` choice on [`AppInstallRequest`] (consuming).
        impl rt::Choice<AppInstallRequest>
            for crate::quickstart_licensing_0_0_1::Licensing_AppInstall::AppInstallRequest_Accept
        {
            type Return =
                rt::ContractId<crate::quickstart_licensing_0_0_1::Licensing_AppInstall::AppInstall>;
            const NAME: &'static str = "AppInstallRequest_Accept";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`AppInstallRequest`] (consuming).
        impl rt::Choice<AppInstallRequest>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `AppInstallRequest_Reject` choice on [`AppInstallRequest`] (consuming).
        impl rt::Choice<AppInstallRequest>
            for crate::quickstart_licensing_0_0_1::Licensing_AppInstall::AppInstallRequest_Reject
        {
            type Return = rt::Unit;
            const NAME: &'static str = "AppInstallRequest_Reject";
            const CONSUMING: bool = true;
        }
    }
}
pub mod splice_api_token_allocation_request_v1_1_0_0 {
    pub mod Splice_Api_Token_AllocationRequestV1 {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationRequestView {
            ///The Daml `settlement` field.
            #[serde(rename = "settlement")]
            pub settlement: crate::splice_api_token_allocation_v1_1_0_0::Splice_Api_Token_AllocationV1::SettlementInfo,
            ///The Daml `transferLegs` field.
            #[serde(rename = "transferLegs")]
            pub transfer_legs: rt::TextMap<
                crate::splice_api_token_allocation_v1_1_0_0::Splice_Api_Token_AllocationV1::TransferLeg,
            >,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta: crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for AllocationRequestView {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("settlement", rt::ToValue::to_value(&self.settlement)),
                    ("transferLegs", rt::ToValue::to_value(&self.transfer_legs)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for AllocationRequestView {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    settlement: rt::FromValue::from_value(rt::record_field(value, "settlement")?)?,
                    transfer_legs: rt::FromValue::from_value(rt::record_field(
                        value,
                        "transferLegs",
                    )?)?,
                    meta: rt::FromValue::from_value(rt::record_field(value, "meta")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationRequest_Withdraw {
            ///The Daml `extraArgs` field.
            #[serde(rename = "extraArgs")]
            pub extra_args:
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for AllocationRequest_Withdraw {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "extraArgs",
                    rt::ToValue::to_value(&self.extra_args)
                ),])
            }
        }
        impl rt::FromValue for AllocationRequest_Withdraw {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    extra_args: rt::FromValue::from_value(rt::record_field(value, "extraArgs")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationRequest_Reject {
            ///The Daml `actor` field.
            #[serde(rename = "actor")]
            pub actor: rt::Party,
            ///The Daml `extraArgs` field.
            #[serde(rename = "extraArgs")]
            pub extra_args:
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for AllocationRequest_Reject {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("actor", rt::ToValue::to_value(&self.actor)),
                    ("extraArgs", rt::ToValue::to_value(&self.extra_args)),
                ])
            }
        }
        impl rt::FromValue for AllocationRequest_Reject {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    actor: rt::FromValue::from_value(rt::record_field(value, "actor")?)?,
                    extra_args: rt::FromValue::from_value(rt::record_field(value, "extraArgs")?)?,
                })
            }
        }
        ///Marker for the Daml interface `AllocationRequest` (held via `ContractId`).
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct AllocationRequest;
        impl rt::Contract for AllocationRequest {
            const PACKAGE_ID: &'static str =
                "6fe848530b2404017c4a12874c956ad7d5c8a419ee9b040f96b5c13172d2e193";
            const PACKAGE_NAME: &'static str = "splice-api-token-allocation-request-v1";
            const MODULE_NAME: &'static str = "Splice.Api.Token.AllocationRequestV1";
            const ENTITY_NAME: &'static str = "AllocationRequest";
        }
        impl rt::Interface for AllocationRequest {
            type View = crate::splice_api_token_allocation_request_v1_1_0_0::Splice_Api_Token_AllocationRequestV1::AllocationRequestView;
        }
        ///The `Archive` choice on [`AllocationRequest`] (consuming).
        impl rt::Choice<AllocationRequest>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `AllocationRequest_Reject` choice on [`AllocationRequest`] (consuming).
        impl rt::Choice<AllocationRequest>
        for crate::splice_api_token_allocation_request_v1_1_0_0::Splice_Api_Token_AllocationRequestV1::AllocationRequest_Reject {
            type Return = crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::ChoiceExecutionMetadata;
            const NAME: &'static str = "AllocationRequest_Reject";
            const CONSUMING: bool = true;
        }
        ///The `AllocationRequest_Withdraw` choice on [`AllocationRequest`] (consuming).
        impl rt::Choice<AllocationRequest>
        for crate::splice_api_token_allocation_request_v1_1_0_0::Splice_Api_Token_AllocationRequestV1::AllocationRequest_Withdraw {
            type Return = crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::ChoiceExecutionMetadata;
            const NAME: &'static str = "AllocationRequest_Withdraw";
            const CONSUMING: bool = true;
        }
    }
}
pub mod splice_api_token_allocation_v1_1_0_0 {
    pub mod Splice_Api_Token_AllocationV1 {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Allocation_WithdrawResult {
            ///The Daml `senderHoldingCids` field.
            #[serde(rename = "senderHoldingCids")]
            pub sender_holding_cids: Vec<
                rt::ContractId<
                    crate::splice_api_token_holding_v1_1_0_0::Splice_Api_Token_HoldingV1::Holding,
                >,
            >,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta:
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for Allocation_WithdrawResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "senderHoldingCids",
                        rt::ToValue::to_value(&self.sender_holding_cids)
                    ),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for Allocation_WithdrawResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    sender_holding_cids: rt::FromValue::from_value(rt::record_field(
                        value,
                        "senderHoldingCids",
                    )?)?,
                    meta: rt::FromValue::from_value(rt::record_field(value, "meta")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Allocation_CancelResult {
            ///The Daml `senderHoldingCids` field.
            #[serde(rename = "senderHoldingCids")]
            pub sender_holding_cids: Vec<
                rt::ContractId<
                    crate::splice_api_token_holding_v1_1_0_0::Splice_Api_Token_HoldingV1::Holding,
                >,
            >,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta:
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for Allocation_CancelResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "senderHoldingCids",
                        rt::ToValue::to_value(&self.sender_holding_cids)
                    ),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for Allocation_CancelResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    sender_holding_cids: rt::FromValue::from_value(rt::record_field(
                        value,
                        "senderHoldingCids",
                    )?)?,
                    meta: rt::FromValue::from_value(rt::record_field(value, "meta")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Allocation_ExecuteTransferResult {
            ///The Daml `senderHoldingCids` field.
            #[serde(rename = "senderHoldingCids")]
            pub sender_holding_cids: Vec<
                rt::ContractId<
                    crate::splice_api_token_holding_v1_1_0_0::Splice_Api_Token_HoldingV1::Holding,
                >,
            >,
            ///The Daml `receiverHoldingCids` field.
            #[serde(rename = "receiverHoldingCids")]
            pub receiver_holding_cids: Vec<
                rt::ContractId<
                    crate::splice_api_token_holding_v1_1_0_0::Splice_Api_Token_HoldingV1::Holding,
                >,
            >,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta:
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for Allocation_ExecuteTransferResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "senderHoldingCids",
                        rt::ToValue::to_value(&self.sender_holding_cids)
                    ),
                    (
                        "receiverHoldingCids",
                        rt::ToValue::to_value(&self.receiver_holding_cids)
                    ),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for Allocation_ExecuteTransferResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    sender_holding_cids: rt::FromValue::from_value(rt::record_field(
                        value,
                        "senderHoldingCids",
                    )?)?,
                    receiver_holding_cids: rt::FromValue::from_value(rt::record_field(
                        value,
                        "receiverHoldingCids",
                    )?)?,
                    meta: rt::FromValue::from_value(rt::record_field(value, "meta")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Allocation_Withdraw {
            ///The Daml `extraArgs` field.
            #[serde(rename = "extraArgs")]
            pub extra_args:
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for Allocation_Withdraw {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "extraArgs",
                    rt::ToValue::to_value(&self.extra_args)
                ),])
            }
        }
        impl rt::FromValue for Allocation_Withdraw {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    extra_args: rt::FromValue::from_value(rt::record_field(value, "extraArgs")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Allocation_Cancel {
            ///The Daml `extraArgs` field.
            #[serde(rename = "extraArgs")]
            pub extra_args:
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for Allocation_Cancel {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "extraArgs",
                    rt::ToValue::to_value(&self.extra_args)
                ),])
            }
        }
        impl rt::FromValue for Allocation_Cancel {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    extra_args: rt::FromValue::from_value(rt::record_field(value, "extraArgs")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Allocation_ExecuteTransfer {
            ///The Daml `extraArgs` field.
            #[serde(rename = "extraArgs")]
            pub extra_args:
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for Allocation_ExecuteTransfer {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "extraArgs",
                    rt::ToValue::to_value(&self.extra_args)
                ),])
            }
        }
        impl rt::FromValue for Allocation_ExecuteTransfer {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    extra_args: rt::FromValue::from_value(rt::record_field(value, "extraArgs")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationView {
            ///The Daml `allocation` field.
            #[serde(rename = "allocation")]
            pub allocation: crate::splice_api_token_allocation_v1_1_0_0::Splice_Api_Token_AllocationV1::AllocationSpecification,
            ///The Daml `holdingCids` field.
            #[serde(rename = "holdingCids")]
            pub holding_cids: Vec<
                rt::ContractId<
                    crate::splice_api_token_holding_v1_1_0_0::Splice_Api_Token_HoldingV1::Holding,
                >,
            >,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta: crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for AllocationView {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("allocation", rt::ToValue::to_value(&self.allocation)),
                    ("holdingCids", rt::ToValue::to_value(&self.holding_cids)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for AllocationView {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    allocation: rt::FromValue::from_value(rt::record_field(value, "allocation")?)?,
                    holding_cids: rt::FromValue::from_value(rt::record_field(
                        value,
                        "holdingCids",
                    )?)?,
                    meta: rt::FromValue::from_value(rt::record_field(value, "meta")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationSpecification {
            ///The Daml `settlement` field.
            #[serde(rename = "settlement")]
            pub settlement: crate::splice_api_token_allocation_v1_1_0_0::Splice_Api_Token_AllocationV1::SettlementInfo,
            ///The Daml `transferLegId` field.
            #[serde(rename = "transferLegId")]
            pub transfer_leg_id: String,
            ///The Daml `transferLeg` field.
            #[serde(rename = "transferLeg")]
            pub transfer_leg: crate::splice_api_token_allocation_v1_1_0_0::Splice_Api_Token_AllocationV1::TransferLeg,
        }
        impl rt::ToValue for AllocationSpecification {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("settlement", rt::ToValue::to_value(&self.settlement)),
                    (
                        "transferLegId",
                        rt::ToValue::to_value(&self.transfer_leg_id)
                    ),
                    ("transferLeg", rt::ToValue::to_value(&self.transfer_leg)),
                ])
            }
        }
        impl rt::FromValue for AllocationSpecification {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    settlement: rt::FromValue::from_value(rt::record_field(value, "settlement")?)?,
                    transfer_leg_id: rt::FromValue::from_value(rt::record_field(
                        value,
                        "transferLegId",
                    )?)?,
                    transfer_leg: rt::FromValue::from_value(rt::record_field(
                        value,
                        "transferLeg",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferLeg {
            ///The Daml `sender` field.
            #[serde(rename = "sender")]
            pub sender: rt::Party,
            ///The Daml `receiver` field.
            #[serde(rename = "receiver")]
            pub receiver: rt::Party,
            ///The Daml `amount` field.
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            ///The Daml `instrumentId` field.
            #[serde(rename = "instrumentId")]
            pub instrument_id:
                crate::splice_api_token_holding_v1_1_0_0::Splice_Api_Token_HoldingV1::InstrumentId,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta:
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for TransferLeg {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
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
                    sender: rt::FromValue::from_value(rt::record_field(value, "sender")?)?,
                    receiver: rt::FromValue::from_value(rt::record_field(value, "receiver")?)?,
                    amount: rt::FromValue::from_value(rt::record_field(value, "amount")?)?,
                    instrument_id: rt::FromValue::from_value(rt::record_field(
                        value,
                        "instrumentId",
                    )?)?,
                    meta: rt::FromValue::from_value(rt::record_field(value, "meta")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SettlementInfo {
            ///The Daml `executor` field.
            #[serde(rename = "executor")]
            pub executor: rt::Party,
            ///The Daml `settlementRef` field.
            #[serde(rename = "settlementRef")]
            pub settlement_ref: crate::splice_api_token_allocation_v1_1_0_0::Splice_Api_Token_AllocationV1::Reference,
            ///The Daml `requestedAt` field.
            #[serde(rename = "requestedAt")]
            pub requested_at: rt::Timestamp,
            ///The Daml `allocateBefore` field.
            #[serde(rename = "allocateBefore")]
            pub allocate_before: rt::Timestamp,
            ///The Daml `settleBefore` field.
            #[serde(rename = "settleBefore")]
            pub settle_before: rt::Timestamp,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta: crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for SettlementInfo {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("executor", rt::ToValue::to_value(&self.executor)),
                    ("settlementRef", rt::ToValue::to_value(&self.settlement_ref)),
                    ("requestedAt", rt::ToValue::to_value(&self.requested_at)),
                    (
                        "allocateBefore",
                        rt::ToValue::to_value(&self.allocate_before)
                    ),
                    ("settleBefore", rt::ToValue::to_value(&self.settle_before)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for SettlementInfo {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    executor: rt::FromValue::from_value(rt::record_field(value, "executor")?)?,
                    settlement_ref: rt::FromValue::from_value(rt::record_field(
                        value,
                        "settlementRef",
                    )?)?,
                    requested_at: rt::FromValue::from_value(rt::record_field(
                        value,
                        "requestedAt",
                    )?)?,
                    allocate_before: rt::FromValue::from_value(rt::record_field(
                        value,
                        "allocateBefore",
                    )?)?,
                    settle_before: rt::FromValue::from_value(rt::record_field(
                        value,
                        "settleBefore",
                    )?)?,
                    meta: rt::FromValue::from_value(rt::record_field(value, "meta")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Reference {
            ///The Daml `id` field.
            #[serde(rename = "id")]
            pub id: String,
            ///The Daml `cid` field.
            #[serde(rename = "cid")]
            pub cid: Option<
                rt::ContractId<
                    crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::AnyContract,
                >,
            >,
        }
        impl rt::ToValue for Reference {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("id", rt::ToValue::to_value(&self.id)),
                    ("cid", rt::ToValue::to_value(&self.cid)),
                ])
            }
        }
        impl rt::FromValue for Reference {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    id: rt::FromValue::from_value(rt::record_field(value, "id")?)?,
                    cid: rt::FromValue::from_value(rt::record_field(value, "cid")?)?,
                })
            }
        }
        ///Marker for the Daml interface `Allocation` (held via `ContractId`).
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct Allocation;
        impl rt::Contract for Allocation {
            const PACKAGE_ID: &'static str =
                "93c942ae2b4c2ba674fb152fe38473c507bda4e82b4e4c5da55a552a9d8cce1d";
            const PACKAGE_NAME: &'static str = "splice-api-token-allocation-v1";
            const MODULE_NAME: &'static str = "Splice.Api.Token.AllocationV1";
            const ENTITY_NAME: &'static str = "Allocation";
        }
        impl rt::Interface for Allocation {
            type View = crate::splice_api_token_allocation_v1_1_0_0::Splice_Api_Token_AllocationV1::AllocationView;
        }
        ///The `Archive` choice on [`Allocation`] (consuming).
        impl rt::Choice<Allocation>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `Allocation_Withdraw` choice on [`Allocation`] (consuming).
        impl rt::Choice<Allocation>
        for crate::splice_api_token_allocation_v1_1_0_0::Splice_Api_Token_AllocationV1::Allocation_Withdraw {
            type Return = crate::splice_api_token_allocation_v1_1_0_0::Splice_Api_Token_AllocationV1::Allocation_WithdrawResult;
            const NAME: &'static str = "Allocation_Withdraw";
            const CONSUMING: bool = true;
        }
        ///The `Allocation_Cancel` choice on [`Allocation`] (consuming).
        impl rt::Choice<Allocation>
        for crate::splice_api_token_allocation_v1_1_0_0::Splice_Api_Token_AllocationV1::Allocation_Cancel {
            type Return = crate::splice_api_token_allocation_v1_1_0_0::Splice_Api_Token_AllocationV1::Allocation_CancelResult;
            const NAME: &'static str = "Allocation_Cancel";
            const CONSUMING: bool = true;
        }
        ///The `Allocation_ExecuteTransfer` choice on [`Allocation`] (consuming).
        impl rt::Choice<Allocation>
        for crate::splice_api_token_allocation_v1_1_0_0::Splice_Api_Token_AllocationV1::Allocation_ExecuteTransfer {
            type Return = crate::splice_api_token_allocation_v1_1_0_0::Splice_Api_Token_AllocationV1::Allocation_ExecuteTransferResult;
            const NAME: &'static str = "Allocation_ExecuteTransfer";
            const CONSUMING: bool = true;
        }
    }
}
pub mod splice_api_token_holding_v1_1_0_0 {
    pub mod Splice_Api_Token_HoldingV1 {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct HoldingView {
            ///The Daml `owner` field.
            #[serde(rename = "owner")]
            pub owner: rt::Party,
            ///The Daml `instrumentId` field.
            #[serde(rename = "instrumentId")]
            pub instrument_id:
                crate::splice_api_token_holding_v1_1_0_0::Splice_Api_Token_HoldingV1::InstrumentId,
            ///The Daml `amount` field.
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            ///The Daml `lock` field.
            #[serde(rename = "lock")]
            pub lock:
                Option<crate::splice_api_token_holding_v1_1_0_0::Splice_Api_Token_HoldingV1::Lock>,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta:
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for HoldingView {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("owner", rt::ToValue::to_value(&self.owner)),
                    ("instrumentId", rt::ToValue::to_value(&self.instrument_id)),
                    ("amount", rt::ToValue::to_value(&self.amount)),
                    ("lock", rt::ToValue::to_value(&self.lock)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for HoldingView {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    owner: rt::FromValue::from_value(rt::record_field(value, "owner")?)?,
                    instrument_id: rt::FromValue::from_value(rt::record_field(
                        value,
                        "instrumentId",
                    )?)?,
                    amount: rt::FromValue::from_value(rt::record_field(value, "amount")?)?,
                    lock: rt::FromValue::from_value(rt::record_field(value, "lock")?)?,
                    meta: rt::FromValue::from_value(rt::record_field(value, "meta")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Lock {
            ///The Daml `holders` field.
            #[serde(rename = "holders")]
            pub holders: Vec<rt::Party>,
            ///The Daml `expiresAt` field.
            #[serde(rename = "expiresAt")]
            pub expires_at: Option<rt::Timestamp>,
            ///The Daml `expiresAfter` field.
            #[serde(rename = "expiresAfter")]
            pub expires_after:
                Option<crate::daml_stdlib_DA_Time_Types_1_0_0::DA_Time_Types::RelTime>,
            ///The Daml `context` field.
            #[serde(rename = "context")]
            pub context: Option<String>,
        }
        impl rt::ToValue for Lock {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("holders", rt::ToValue::to_value(&self.holders)),
                    ("expiresAt", rt::ToValue::to_value(&self.expires_at)),
                    ("expiresAfter", rt::ToValue::to_value(&self.expires_after)),
                    ("context", rt::ToValue::to_value(&self.context)),
                ])
            }
        }
        impl rt::FromValue for Lock {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    holders: rt::FromValue::from_value(rt::record_field(value, "holders")?)?,
                    expires_at: rt::FromValue::from_value(rt::record_field(value, "expiresAt")?)?,
                    expires_after: rt::FromValue::from_value(rt::record_field(
                        value,
                        "expiresAfter",
                    )?)?,
                    context: rt::FromValue::from_value(rt::record_field(value, "context")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct InstrumentId {
            ///The Daml `admin` field.
            #[serde(rename = "admin")]
            pub admin: rt::Party,
            ///The Daml `id` field.
            #[serde(rename = "id")]
            pub id: String,
        }
        impl rt::ToValue for InstrumentId {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("admin", rt::ToValue::to_value(&self.admin)),
                    ("id", rt::ToValue::to_value(&self.id)),
                ])
            }
        }
        impl rt::FromValue for InstrumentId {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    admin: rt::FromValue::from_value(rt::record_field(value, "admin")?)?,
                    id: rt::FromValue::from_value(rt::record_field(value, "id")?)?,
                })
            }
        }
        ///Marker for the Daml interface `Holding` (held via `ContractId`).
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct Holding;
        impl rt::Contract for Holding {
            const PACKAGE_ID: &'static str =
                "718a0f77e505a8de22f188bd4c87fe74101274e9d4cb1bfac7d09aec7158d35b";
            const PACKAGE_NAME: &'static str = "splice-api-token-holding-v1";
            const MODULE_NAME: &'static str = "Splice.Api.Token.HoldingV1";
            const ENTITY_NAME: &'static str = "Holding";
        }
        impl rt::Interface for Holding {
            type View =
                crate::splice_api_token_holding_v1_1_0_0::Splice_Api_Token_HoldingV1::HoldingView;
        }
        ///The `Archive` choice on [`Holding`] (consuming).
        impl rt::Choice<Holding>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
    }
}
pub mod splice_api_token_metadata_v1_1_0_0 {
    pub mod Splice_Api_Token_MetadataV1 {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ChoiceExecutionMetadata {
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta:
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for ChoiceExecutionMetadata {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("meta", rt::ToValue::to_value(&self.meta)),])
            }
        }
        impl rt::FromValue for ChoiceExecutionMetadata {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    meta: rt::FromValue::from_value(rt::record_field(value, "meta")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ExtraArgs {
            ///The Daml `context` field.
            #[serde(rename = "context")]
            pub context: crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::ChoiceContext,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta: crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
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
                    context: rt::FromValue::from_value(rt::record_field(value, "context")?)?,
                    meta: rt::FromValue::from_value(rt::record_field(value, "meta")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Metadata {
            ///The Daml `values` field.
            #[serde(rename = "values")]
            pub values: rt::TextMap<String>,
        }
        impl rt::ToValue for Metadata {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("values", rt::ToValue::to_value(&self.values)),])
            }
        }
        impl rt::FromValue for Metadata {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    values: rt::FromValue::from_value(rt::record_field(value, "values")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ChoiceContext {
            ///The Daml `values` field.
            #[serde(rename = "values")]
            pub values: rt::TextMap<
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::AnyValue,
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
                    values: rt::FromValue::from_value(rt::record_field(value, "values")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AnyContractView {}
        impl rt::ToValue for AnyContractView {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for AnyContractView {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum AnyValue {
            ///The Daml `AV_Text` constructor.
            #[serde(rename = "AV_Text")]
            AV_Text(String),
            ///The Daml `AV_Int` constructor.
            #[serde(rename = "AV_Int")]
            AV_Int(rt::Int64),
            ///The Daml `AV_Decimal` constructor.
            #[serde(rename = "AV_Decimal")]
            AV_Decimal(rt::Numeric),
            ///The Daml `AV_Bool` constructor.
            #[serde(rename = "AV_Bool")]
            AV_Bool(bool),
            ///The Daml `AV_Date` constructor.
            #[serde(rename = "AV_Date")]
            AV_Date(rt::Date),
            ///The Daml `AV_Time` constructor.
            #[serde(rename = "AV_Time")]
            AV_Time(rt::Timestamp),
            ///The Daml `AV_RelTime` constructor.
            #[serde(rename = "AV_RelTime")]
            AV_RelTime(crate::daml_stdlib_DA_Time_Types_1_0_0::DA_Time_Types::RelTime),
            ///The Daml `AV_Party` constructor.
            #[serde(rename = "AV_Party")]
            AV_Party(rt::Party),
            ///The Daml `AV_ContractId` constructor.
            #[serde(rename = "AV_ContractId")]
            AV_ContractId(
                rt::ContractId<
                    crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::AnyContract,
                >,
            ),
            ///The Daml `AV_List` constructor.
            #[serde(rename = "AV_List")]
            AV_List(
                Vec<
                    crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::AnyValue,
                >,
            ),
            ///The Daml `AV_Map` constructor.
            #[serde(rename = "AV_Map")]
            AV_Map(
                rt::TextMap<
                    crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::AnyValue,
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
                        rt::FromValue::from_value(payload)?,
                    )),
                    "AV_Int" => ::core::result::Result::Ok(AnyValue::AV_Int(
                        rt::FromValue::from_value(payload)?,
                    )),
                    "AV_Decimal" => ::core::result::Result::Ok(AnyValue::AV_Decimal(
                        rt::FromValue::from_value(payload)?,
                    )),
                    "AV_Bool" => ::core::result::Result::Ok(AnyValue::AV_Bool(
                        rt::FromValue::from_value(payload)?,
                    )),
                    "AV_Date" => ::core::result::Result::Ok(AnyValue::AV_Date(
                        rt::FromValue::from_value(payload)?,
                    )),
                    "AV_Time" => ::core::result::Result::Ok(AnyValue::AV_Time(
                        rt::FromValue::from_value(payload)?,
                    )),
                    "AV_RelTime" => ::core::result::Result::Ok(AnyValue::AV_RelTime(
                        rt::FromValue::from_value(payload)?,
                    )),
                    "AV_Party" => ::core::result::Result::Ok(AnyValue::AV_Party(
                        rt::FromValue::from_value(payload)?,
                    )),
                    "AV_ContractId" => ::core::result::Result::Ok(AnyValue::AV_ContractId(
                        rt::FromValue::from_value(payload)?,
                    )),
                    "AV_List" => ::core::result::Result::Ok(AnyValue::AV_List(
                        rt::FromValue::from_value(payload)?,
                    )),
                    "AV_Map" => ::core::result::Result::Ok(AnyValue::AV_Map(
                        rt::FromValue::from_value(payload)?,
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
            type View = crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::AnyContractView;
        }
        ///The `Archive` choice on [`AnyContract`] (consuming).
        impl rt::Choice<AnyContract>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
    }
}
