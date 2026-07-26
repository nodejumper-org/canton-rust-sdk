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
                    )?)?,
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
                    )?)?,
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
                    )?)?,
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
                    )?)?,
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
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)?,
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
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)?,
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
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)?,
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
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)?,
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
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)?,
                    _6: rt::FromValue::from_value(rt::required_field(value, 5usize, "_6")?)?,
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
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)?,
                    _6: rt::FromValue::from_value(rt::required_field(value, 5usize, "_6")?)?,
                    _7: rt::FromValue::from_value(rt::required_field(value, 6usize, "_7")?)?,
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
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)?,
                    _6: rt::FromValue::from_value(rt::required_field(value, 5usize, "_6")?)?,
                    _7: rt::FromValue::from_value(rt::required_field(value, 6usize, "_7")?)?,
                    _8: rt::FromValue::from_value(rt::required_field(value, 7usize, "_8")?)?,
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
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)?,
                    _6: rt::FromValue::from_value(rt::required_field(value, 5usize, "_6")?)?,
                    _7: rt::FromValue::from_value(rt::required_field(value, 6usize, "_7")?)?,
                    _8: rt::FromValue::from_value(rt::required_field(value, 7usize, "_8")?)?,
                    _9: rt::FromValue::from_value(rt::required_field(value, 8usize, "_9")?)?,
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
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)?,
                    _6: rt::FromValue::from_value(rt::required_field(value, 5usize, "_6")?)?,
                    _7: rt::FromValue::from_value(rt::required_field(value, 6usize, "_7")?)?,
                    _8: rt::FromValue::from_value(rt::required_field(value, 7usize, "_8")?)?,
                    _9: rt::FromValue::from_value(rt::required_field(value, 8usize, "_9")?)?,
                    _10: rt::FromValue::from_value(rt::required_field(value, 9usize, "_10")?)?,
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
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)?,
                    _6: rt::FromValue::from_value(rt::required_field(value, 5usize, "_6")?)?,
                    _7: rt::FromValue::from_value(rt::required_field(value, 6usize, "_7")?)?,
                    _8: rt::FromValue::from_value(rt::required_field(value, 7usize, "_8")?)?,
                    _9: rt::FromValue::from_value(rt::required_field(value, 8usize, "_9")?)?,
                    _10: rt::FromValue::from_value(rt::required_field(value, 9usize, "_10")?)?,
                    _11: rt::FromValue::from_value(rt::required_field(value, 10usize, "_11")?)?,
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
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)?,
                    _6: rt::FromValue::from_value(rt::required_field(value, 5usize, "_6")?)?,
                    _7: rt::FromValue::from_value(rt::required_field(value, 6usize, "_7")?)?,
                    _8: rt::FromValue::from_value(rt::required_field(value, 7usize, "_8")?)?,
                    _9: rt::FromValue::from_value(rt::required_field(value, 8usize, "_9")?)?,
                    _10: rt::FromValue::from_value(rt::required_field(value, 9usize, "_10")?)?,
                    _11: rt::FromValue::from_value(rt::required_field(value, 10usize, "_11")?)?,
                    _12: rt::FromValue::from_value(rt::required_field(value, 11usize, "_12")?)?,
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
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)?,
                    _6: rt::FromValue::from_value(rt::required_field(value, 5usize, "_6")?)?,
                    _7: rt::FromValue::from_value(rt::required_field(value, 6usize, "_7")?)?,
                    _8: rt::FromValue::from_value(rt::required_field(value, 7usize, "_8")?)?,
                    _9: rt::FromValue::from_value(rt::required_field(value, 8usize, "_9")?)?,
                    _10: rt::FromValue::from_value(rt::required_field(value, 9usize, "_10")?)?,
                    _11: rt::FromValue::from_value(rt::required_field(value, 10usize, "_11")?)?,
                    _12: rt::FromValue::from_value(rt::required_field(value, 11usize, "_12")?)?,
                    _13: rt::FromValue::from_value(rt::required_field(value, 12usize, "_13")?)?,
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
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)?,
                    _6: rt::FromValue::from_value(rt::required_field(value, 5usize, "_6")?)?,
                    _7: rt::FromValue::from_value(rt::required_field(value, 6usize, "_7")?)?,
                    _8: rt::FromValue::from_value(rt::required_field(value, 7usize, "_8")?)?,
                    _9: rt::FromValue::from_value(rt::required_field(value, 8usize, "_9")?)?,
                    _10: rt::FromValue::from_value(rt::required_field(value, 9usize, "_10")?)?,
                    _11: rt::FromValue::from_value(rt::required_field(value, 10usize, "_11")?)?,
                    _12: rt::FromValue::from_value(rt::required_field(value, 11usize, "_12")?)?,
                    _13: rt::FromValue::from_value(rt::required_field(value, 12usize, "_13")?)?,
                    _14: rt::FromValue::from_value(rt::required_field(value, 13usize, "_14")?)?,
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
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)?,
                    _6: rt::FromValue::from_value(rt::required_field(value, 5usize, "_6")?)?,
                    _7: rt::FromValue::from_value(rt::required_field(value, 6usize, "_7")?)?,
                    _8: rt::FromValue::from_value(rt::required_field(value, 7usize, "_8")?)?,
                    _9: rt::FromValue::from_value(rt::required_field(value, 8usize, "_9")?)?,
                    _10: rt::FromValue::from_value(rt::required_field(value, 9usize, "_10")?)?,
                    _11: rt::FromValue::from_value(rt::required_field(value, 10usize, "_11")?)?,
                    _12: rt::FromValue::from_value(rt::required_field(value, 11usize, "_12")?)?,
                    _13: rt::FromValue::from_value(rt::required_field(value, 12usize, "_13")?)?,
                    _14: rt::FromValue::from_value(rt::required_field(value, 13usize, "_14")?)?,
                    _15: rt::FromValue::from_value(rt::required_field(value, 14usize, "_15")?)?,
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
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)?,
                    _6: rt::FromValue::from_value(rt::required_field(value, 5usize, "_6")?)?,
                    _7: rt::FromValue::from_value(rt::required_field(value, 6usize, "_7")?)?,
                    _8: rt::FromValue::from_value(rt::required_field(value, 7usize, "_8")?)?,
                    _9: rt::FromValue::from_value(rt::required_field(value, 8usize, "_9")?)?,
                    _10: rt::FromValue::from_value(rt::required_field(value, 9usize, "_10")?)?,
                    _11: rt::FromValue::from_value(rt::required_field(value, 10usize, "_11")?)?,
                    _12: rt::FromValue::from_value(rt::required_field(value, 11usize, "_12")?)?,
                    _13: rt::FromValue::from_value(rt::required_field(value, 12usize, "_13")?)?,
                    _14: rt::FromValue::from_value(rt::required_field(value, 13usize, "_14")?)?,
                    _15: rt::FromValue::from_value(rt::required_field(value, 14usize, "_15")?)?,
                    _16: rt::FromValue::from_value(rt::required_field(value, 15usize, "_16")?)?,
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
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)?,
                    _6: rt::FromValue::from_value(rt::required_field(value, 5usize, "_6")?)?,
                    _7: rt::FromValue::from_value(rt::required_field(value, 6usize, "_7")?)?,
                    _8: rt::FromValue::from_value(rt::required_field(value, 7usize, "_8")?)?,
                    _9: rt::FromValue::from_value(rt::required_field(value, 8usize, "_9")?)?,
                    _10: rt::FromValue::from_value(rt::required_field(value, 9usize, "_10")?)?,
                    _11: rt::FromValue::from_value(rt::required_field(value, 10usize, "_11")?)?,
                    _12: rt::FromValue::from_value(rt::required_field(value, 11usize, "_12")?)?,
                    _13: rt::FromValue::from_value(rt::required_field(value, 12usize, "_13")?)?,
                    _14: rt::FromValue::from_value(rt::required_field(value, 13usize, "_14")?)?,
                    _15: rt::FromValue::from_value(rt::required_field(value, 14usize, "_15")?)?,
                    _16: rt::FromValue::from_value(rt::required_field(value, 15usize, "_16")?)?,
                    _17: rt::FromValue::from_value(rt::required_field(value, 16usize, "_17")?)?,
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
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)?,
                    _6: rt::FromValue::from_value(rt::required_field(value, 5usize, "_6")?)?,
                    _7: rt::FromValue::from_value(rt::required_field(value, 6usize, "_7")?)?,
                    _8: rt::FromValue::from_value(rt::required_field(value, 7usize, "_8")?)?,
                    _9: rt::FromValue::from_value(rt::required_field(value, 8usize, "_9")?)?,
                    _10: rt::FromValue::from_value(rt::required_field(value, 9usize, "_10")?)?,
                    _11: rt::FromValue::from_value(rt::required_field(value, 10usize, "_11")?)?,
                    _12: rt::FromValue::from_value(rt::required_field(value, 11usize, "_12")?)?,
                    _13: rt::FromValue::from_value(rt::required_field(value, 12usize, "_13")?)?,
                    _14: rt::FromValue::from_value(rt::required_field(value, 13usize, "_14")?)?,
                    _15: rt::FromValue::from_value(rt::required_field(value, 14usize, "_15")?)?,
                    _16: rt::FromValue::from_value(rt::required_field(value, 15usize, "_16")?)?,
                    _17: rt::FromValue::from_value(rt::required_field(value, 16usize, "_17")?)?,
                    _18: rt::FromValue::from_value(rt::required_field(value, 17usize, "_18")?)?,
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
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)?,
                    _6: rt::FromValue::from_value(rt::required_field(value, 5usize, "_6")?)?,
                    _7: rt::FromValue::from_value(rt::required_field(value, 6usize, "_7")?)?,
                    _8: rt::FromValue::from_value(rt::required_field(value, 7usize, "_8")?)?,
                    _9: rt::FromValue::from_value(rt::required_field(value, 8usize, "_9")?)?,
                    _10: rt::FromValue::from_value(rt::required_field(value, 9usize, "_10")?)?,
                    _11: rt::FromValue::from_value(rt::required_field(value, 10usize, "_11")?)?,
                    _12: rt::FromValue::from_value(rt::required_field(value, 11usize, "_12")?)?,
                    _13: rt::FromValue::from_value(rt::required_field(value, 12usize, "_13")?)?,
                    _14: rt::FromValue::from_value(rt::required_field(value, 13usize, "_14")?)?,
                    _15: rt::FromValue::from_value(rt::required_field(value, 14usize, "_15")?)?,
                    _16: rt::FromValue::from_value(rt::required_field(value, 15usize, "_16")?)?,
                    _17: rt::FromValue::from_value(rt::required_field(value, 16usize, "_17")?)?,
                    _18: rt::FromValue::from_value(rt::required_field(value, 17usize, "_18")?)?,
                    _19: rt::FromValue::from_value(rt::required_field(value, 18usize, "_19")?)?,
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
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)?,
                    _2: rt::FromValue::from_value(rt::required_field(value, 1usize, "_2")?)?,
                    _3: rt::FromValue::from_value(rt::required_field(value, 2usize, "_3")?)?,
                    _4: rt::FromValue::from_value(rt::required_field(value, 3usize, "_4")?)?,
                    _5: rt::FromValue::from_value(rt::required_field(value, 4usize, "_5")?)?,
                    _6: rt::FromValue::from_value(rt::required_field(value, 5usize, "_6")?)?,
                    _7: rt::FromValue::from_value(rt::required_field(value, 6usize, "_7")?)?,
                    _8: rt::FromValue::from_value(rt::required_field(value, 7usize, "_8")?)?,
                    _9: rt::FromValue::from_value(rt::required_field(value, 8usize, "_9")?)?,
                    _10: rt::FromValue::from_value(rt::required_field(value, 9usize, "_10")?)?,
                    _11: rt::FromValue::from_value(rt::required_field(value, 10usize, "_11")?)?,
                    _12: rt::FromValue::from_value(rt::required_field(value, 11usize, "_12")?)?,
                    _13: rt::FromValue::from_value(rt::required_field(value, 12usize, "_13")?)?,
                    _14: rt::FromValue::from_value(rt::required_field(value, 13usize, "_14")?)?,
                    _15: rt::FromValue::from_value(rt::required_field(value, 14usize, "_15")?)?,
                    _16: rt::FromValue::from_value(rt::required_field(value, 15usize, "_16")?)?,
                    _17: rt::FromValue::from_value(rt::required_field(value, 16usize, "_17")?)?,
                    _18: rt::FromValue::from_value(rt::required_field(value, 17usize, "_18")?)?,
                    _19: rt::FromValue::from_value(rt::required_field(value, 18usize, "_19")?)?,
                    _20: rt::FromValue::from_value(rt::required_field(value, 19usize, "_20")?)?,
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
                    _1: rt::FromValue::from_value(rt::required_field(value, 0usize, "_1")?)?,
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
                    unpack: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "unpack",
                    )?)?,
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
            Negation(
                ::std::boxed::Box<
                    crate::daml_stdlib_DA_Logic_Types_1_0_0::DA_Logic_Types::Formula<A>,
                >,
            ),
            ///The Daml `Conjunction` constructor.
            #[serde(rename = "Conjunction")]
            Conjunction(
                ::std::vec::Vec<
                    crate::daml_stdlib_DA_Logic_Types_1_0_0::DA_Logic_Types::Formula<A>,
                >,
            ),
            ///The Daml `Disjunction` constructor.
            #[serde(rename = "Disjunction")]
            Disjunction(
                ::std::vec::Vec<
                    crate::daml_stdlib_DA_Logic_Types_1_0_0::DA_Logic_Types::Formula<A>,
                >,
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
                    get_all: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "getAll",
                    )?)?,
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
                    get_any: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "getAny",
                    )?)?,
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
                    unpack: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "unpack",
                    )?)?,
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
                    unpack: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "unpack",
                    )?)?,
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
                    hd: rt::FromValue::from_value(rt::required_field(value, 0usize, "hd")?)?,
                    tl: rt::FromValue::from_value(rt::required_field(value, 1usize, "tl")?)?,
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
                    unpack: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "unpack",
                    )?)?,
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
                    unpack: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "unpack",
                    )?)?,
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
                    map: rt::FromValue::from_value(rt::required_field(value, 0usize, "map")?)?,
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
            pub src_loc_package: ::std::string::String,
            ///The Daml `srcLocModule` field.
            #[serde(rename = "srcLocModule")]
            pub src_loc_module: ::std::string::String,
            ///The Daml `srcLocFile` field.
            #[serde(rename = "srcLocFile")]
            pub src_loc_file: ::std::string::String,
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
                    src_loc_package: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "srcLocPackage",
                    )?)?,
                    src_loc_module: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "srcLocModule",
                    )?)?,
                    src_loc_file: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "srcLocFile",
                    )?)?,
                    src_loc_start_line: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "srcLocStartLine",
                    )?)?,
                    src_loc_start_col: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "srcLocStartCol",
                    )?)?,
                    src_loc_end_line: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "srcLocEndLine",
                    )?)?,
                    src_loc_end_col: rt::FromValue::from_value(rt::required_field(
                        value,
                        6usize,
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
                    microseconds: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
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
pub mod splice_amulet_0_1_14 {
    pub mod Splice_Types {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ForOwner {
            ///The Daml `dso` field.
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///The Daml `owner` field.
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
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)?,
                    owner: rt::FromValue::from_value(rt::required_field(value, 1usize, "owner")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ForRound {
            ///The Daml `dso` field.
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///The Daml `round` field.
            #[serde(rename = "round")]
            pub round: crate::splice_amulet_0_1_14::Splice_Types::Round,
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
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)?,
                    round: rt::FromValue::from_value(rt::required_field(value, 1usize, "round")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ForDso {
            ///The Daml `dso` field.
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
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Round {
            ///The Daml `number` field.
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
                    number: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "number",
                    )?)?,
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
            ///The Daml `TransferPreapproval_CancelResult` value.
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
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferPreapproval_ExpireResult {}
        impl rt::ToValue for TransferPreapproval_ExpireResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for TransferPreapproval_ExpireResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferPreapproval_RenewResult {
            ///The Daml `transferPreapprovalCid` field.
            #[serde(rename = "transferPreapprovalCid")]
            pub transfer_preapproval_cid: rt::ContractId<
                crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferPreapproval,
            >,
            ///The Daml `transferResult` field.
            #[serde(rename = "transferResult")]
            pub transfer_result: crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferResult,
            ///The Daml `receiver` field.
            #[serde(rename = "receiver")]
            pub receiver: rt::Party,
            ///The Daml `provider` field.
            #[serde(rename = "provider")]
            pub provider: rt::Party,
            ///The Daml `amuletPaid` field.
            #[serde(rename = "amuletPaid")]
            pub amulet_paid: rt::Numeric,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta: ::core::option::Option<
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
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
                    )?)?,
                    transfer_result: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "transferResult",
                    )?)?,
                    receiver: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "receiver",
                    )?)?,
                    provider: rt::FromValue::from_value(rt::required_field(
                        value, 3usize, "provider",
                    )?)?,
                    amulet_paid: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "amuletPaid",
                    )?)?,
                    meta: rt::optional_field(value, 5usize, "meta")?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferPreapproval_SendResult {
            ///The Daml `result` field.
            #[serde(rename = "result")]
            pub result: crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferResult,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta: ::core::option::Option<
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
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
                    result: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "result",
                    )?)?,
                    meta: rt::optional_field(value, 1usize, "meta")?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferPreapproval_Cancel {
            ///The Daml `p` field.
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
                    p: rt::FromValue::from_value(rt::required_field(value, 0usize, "p")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferPreapproval_Expire {}
        impl rt::ToValue for TransferPreapproval_Expire {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for TransferPreapproval_Expire {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferPreapproval_Renew {
            ///The Daml `context` field.
            #[serde(rename = "context")]
            pub context: crate::splice_amulet_0_1_14::Splice_AmuletRules::PaymentTransferContext,
            ///The Daml `inputs` field.
            #[serde(rename = "inputs")]
            pub inputs:
                ::std::vec::Vec<crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferInput>,
            ///The Daml `newExpiresAt` field.
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
                    )?)?,
                    inputs: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "inputs",
                    )?)?,
                    new_expires_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "newExpiresAt",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferPreapproval_Send {
            ///The Daml `context` field.
            #[serde(rename = "context")]
            pub context: crate::splice_amulet_0_1_14::Splice_AmuletRules::PaymentTransferContext,
            ///The Daml `inputs` field.
            #[serde(rename = "inputs")]
            pub inputs:
                ::std::vec::Vec<crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferInput>,
            ///The Daml `amount` field.
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            ///The Daml `sender` field.
            #[serde(rename = "sender")]
            pub sender: rt::Party,
            ///The Daml `description` field.
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
                    )?)?,
                    inputs: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "inputs",
                    )?)?,
                    amount: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "amount",
                    )?)?,
                    sender: rt::FromValue::from_value(rt::required_field(
                        value, 3usize, "sender",
                    )?)?,
                    description: rt::optional_field(value, 4usize, "description")?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferPreapproval_Fetch {
            ///The Daml `p` field.
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
                    p: rt::FromValue::from_value(rt::required_field(value, 0usize, "p")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ExternalPartySetupProposal_WithdrawResult {
            ///The Daml `dummyArg` field.
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ExternalPartySetupProposal_RejectResult {
            ///The Daml `dummyArg` field.
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ExternalPartySetupProposal_AcceptResult {
            ///The Daml `validatorRightCid` field.
            #[serde(rename = "validatorRightCid")]
            pub validator_right_cid:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Amulet::ValidatorRight>,
            ///The Daml `transferPreapprovalCid` field.
            #[serde(rename = "transferPreapprovalCid")]
            pub transfer_preapproval_cid: rt::ContractId<
                crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferPreapproval,
            >,
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
                    )?)?,
                    transfer_preapproval_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "transferPreapprovalCid",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ExternalPartySetupProposal_Withdraw {
            ///The Daml `reason` field.
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
                    reason: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "reason",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ExternalPartySetupProposal_Reject {
            ///The Daml `reason` field.
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
                    reason: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "reason",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ExternalPartySetupProposal_Accept {}
        impl rt::ToValue for ExternalPartySetupProposal_Accept {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for ExternalPartySetupProposal_Accept {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct BalanceChange {
            ///The Daml `changeToInitialAmountAsOfRoundZero` field.
            #[serde(rename = "changeToInitialAmountAsOfRoundZero")]
            pub change_to_initial_amount_as_of_round_zero: rt::Numeric,
            ///The Daml `changeToHoldingFeesRate` field.
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
                    )?,
                    change_to_holding_fees_rate: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "changeToHoldingFeesRate",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferSummary {
            ///The Daml `inputAppRewardAmount` field.
            #[serde(rename = "inputAppRewardAmount")]
            pub input_app_reward_amount: rt::Numeric,
            ///The Daml `inputValidatorRewardAmount` field.
            #[serde(rename = "inputValidatorRewardAmount")]
            pub input_validator_reward_amount: rt::Numeric,
            ///The Daml `inputSvRewardAmount` field.
            #[serde(rename = "inputSvRewardAmount")]
            pub input_sv_reward_amount: rt::Numeric,
            ///The Daml `inputAmuletAmount` field.
            #[serde(rename = "inputAmuletAmount")]
            pub input_amulet_amount: rt::Numeric,
            ///The Daml `balanceChanges` field.
            #[serde(rename = "balanceChanges")]
            pub balance_changes: rt::GenMap<
                rt::Party,
                crate::splice_amulet_0_1_14::Splice_AmuletRules::BalanceChange,
            >,
            ///The Daml `holdingFees` field.
            #[serde(rename = "holdingFees")]
            pub holding_fees: rt::Numeric,
            ///The Daml `outputFees` field.
            #[serde(rename = "outputFees")]
            pub output_fees: ::std::vec::Vec<rt::Numeric>,
            ///The Daml `senderChangeFee` field.
            #[serde(rename = "senderChangeFee")]
            pub sender_change_fee: rt::Numeric,
            ///The Daml `senderChangeAmount` field.
            #[serde(rename = "senderChangeAmount")]
            pub sender_change_amount: rt::Numeric,
            ///The Daml `amuletPrice` field.
            #[serde(rename = "amuletPrice")]
            pub amulet_price: rt::Numeric,
            ///The Daml `inputValidatorFaucetAmount` field.
            #[serde(rename = "inputValidatorFaucetAmount")]
            pub input_validator_faucet_amount: ::core::option::Option<rt::Numeric>,
            ///The Daml `inputUnclaimedActivityRecordAmount` field.
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
                    )?)?,
                    input_validator_reward_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "inputValidatorRewardAmount",
                    )?)?,
                    input_sv_reward_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "inputSvRewardAmount",
                    )?)?,
                    input_amulet_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "inputAmuletAmount",
                    )?)?,
                    balance_changes: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "balanceChanges",
                    )?)?,
                    holding_fees: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "holdingFees",
                    )?)?,
                    output_fees: rt::FromValue::from_value(rt::required_field(
                        value,
                        6usize,
                        "outputFees",
                    )?)?,
                    sender_change_fee: rt::FromValue::from_value(rt::required_field(
                        value,
                        7usize,
                        "senderChangeFee",
                    )?)?,
                    sender_change_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        8usize,
                        "senderChangeAmount",
                    )?)?,
                    amulet_price: rt::FromValue::from_value(rt::required_field(
                        value,
                        9usize,
                        "amuletPrice",
                    )?)?,
                    input_validator_faucet_amount: rt::optional_field(
                        value,
                        10usize,
                        "inputValidatorFaucetAmount",
                    )?,
                    input_unclaimed_activity_record_amount: rt::optional_field(
                        value,
                        11usize,
                        "inputUnclaimedActivityRecordAmount",
                    )?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_CreateTransferPreapprovalResult {
            ///The Daml `transferPreapprovalCid` field.
            #[serde(rename = "transferPreapprovalCid")]
            pub transfer_preapproval_cid: rt::ContractId<
                crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferPreapproval,
            >,
            ///The Daml `transferResult` field.
            #[serde(rename = "transferResult")]
            pub transfer_result: crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferResult,
            ///The Daml `amuletPaid` field.
            #[serde(rename = "amuletPaid")]
            pub amulet_paid: rt::Numeric,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta: ::core::option::Option<
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
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
                    )?)?,
                    transfer_result: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "transferResult",
                    )?)?,
                    amulet_paid: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "amuletPaid",
                    )?)?,
                    meta: rt::optional_field(value, 3usize, "meta")?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_CreateExternalPartySetupProposalResult {
            ///The Daml `proposalCid` field.
            #[serde(rename = "proposalCid")]
            pub proposal_cid: rt::ContractId<
                crate::splice_amulet_0_1_14::Splice_AmuletRules::ExternalPartySetupProposal,
            >,
            ///The Daml `user` field.
            #[serde(rename = "user")]
            pub user: rt::Party,
            ///The Daml `validator` field.
            #[serde(rename = "validator")]
            pub validator: rt::Party,
            ///The Daml `transferResult` field.
            #[serde(rename = "transferResult")]
            pub transfer_result: crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferResult,
            ///The Daml `amuletPaid` field.
            #[serde(rename = "amuletPaid")]
            pub amulet_paid: rt::Numeric,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta: ::core::option::Option<
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
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
                    )?)?,
                    user: rt::FromValue::from_value(rt::required_field(value, 1usize, "user")?)?,
                    validator: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "validator",
                    )?)?,
                    transfer_result: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "transferResult",
                    )?)?,
                    amulet_paid: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "amuletPaid",
                    )?)?,
                    meta: rt::optional_field(value, 5usize, "meta")?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_BuyMemberTrafficResult {
            ///The Daml `round` field.
            #[serde(rename = "round")]
            pub round: crate::splice_amulet_0_1_14::Splice_Types::Round,
            ///The Daml `summary` field.
            #[serde(rename = "summary")]
            pub summary: crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferSummary,
            ///The Daml `amuletPaid` field.
            #[serde(rename = "amuletPaid")]
            pub amulet_paid: rt::Numeric,
            ///The Daml `purchasedTraffic` field.
            #[serde(rename = "purchasedTraffic")]
            pub purchased_traffic: rt::ContractId<
                crate::splice_amulet_0_1_14::Splice_DecentralizedSynchronizer::MemberTraffic,
            >,
            ///The Daml `senderChangeAmulet` field.
            #[serde(rename = "senderChangeAmulet")]
            pub sender_change_amulet: ::core::option::Option<
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Amulet::Amulet>,
            >,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta: ::core::option::Option<
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
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
                    round: rt::FromValue::from_value(rt::required_field(value, 0usize, "round")?)?,
                    summary: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "summary",
                    )?)?,
                    amulet_paid: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "amuletPaid",
                    )?)?,
                    purchased_traffic: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "purchasedTraffic",
                    )?)?,
                    sender_change_amulet: rt::optional_field(value, 4usize, "senderChangeAmulet")?,
                    meta: rt::optional_field(value, 5usize, "meta")?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferResult {
            ///The Daml `round` field.
            #[serde(rename = "round")]
            pub round: crate::splice_amulet_0_1_14::Splice_Types::Round,
            ///The Daml `summary` field.
            #[serde(rename = "summary")]
            pub summary: crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferSummary,
            ///The Daml `createdAmulets` field.
            #[serde(rename = "createdAmulets")]
            pub created_amulets:
                ::std::vec::Vec<crate::splice_amulet_0_1_14::Splice_AmuletRules::CreatedAmulet>,
            ///The Daml `senderChangeAmulet` field.
            #[serde(rename = "senderChangeAmulet")]
            pub sender_change_amulet: ::core::option::Option<
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Amulet::Amulet>,
            >,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta: ::core::option::Option<
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
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
                    round: rt::FromValue::from_value(rt::required_field(value, 0usize, "round")?)?,
                    summary: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "summary",
                    )?)?,
                    created_amulets: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "createdAmulets",
                    )?)?,
                    sender_change_amulet: rt::optional_field(value, 3usize, "senderChangeAmulet")?,
                    meta: rt::optional_field(value, 4usize, "meta")?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferOutput {
            ///The Daml `receiver` field.
            #[serde(rename = "receiver")]
            pub receiver: rt::Party,
            ///The Daml `receiverFeeRatio` field.
            #[serde(rename = "receiverFeeRatio")]
            pub receiver_fee_ratio: rt::Numeric,
            ///The Daml `amount` field.
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            ///The Daml `lock` field.
            #[serde(rename = "lock")]
            pub lock: ::core::option::Option<crate::splice_amulet_0_1_14::Splice_Expiry::TimeLock>,
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
                    )?)?,
                    receiver_fee_ratio: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "receiverFeeRatio",
                    )?)?,
                    amount: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "amount",
                    )?)?,
                    lock: rt::optional_field(value, 3usize, "lock")?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Transfer {
            ///The Daml `sender` field.
            #[serde(rename = "sender")]
            pub sender: rt::Party,
            ///The Daml `provider` field.
            #[serde(rename = "provider")]
            pub provider: rt::Party,
            ///The Daml `inputs` field.
            #[serde(rename = "inputs")]
            pub inputs: ::std::vec::Vec<
                crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferInput,
            >,
            ///The Daml `outputs` field.
            #[serde(rename = "outputs")]
            pub outputs: ::std::vec::Vec<
                crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferOutput,
            >,
            ///The Daml `beneficiaries` field.
            #[serde(rename = "beneficiaries")]
            pub beneficiaries: ::core::option::Option<
                ::std::vec::Vec<
                    crate::splice_api_featured_app_v1_1_0_0::Splice_Api_FeaturedAppRightV1::AppRewardBeneficiary,
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
                    sender: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "sender",
                    )?)?,
                    provider: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "provider",
                    )?)?,
                    inputs: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "inputs",
                    )?)?,
                    outputs: rt::FromValue::from_value(rt::required_field(
                        value, 3usize, "outputs",
                    )?)?,
                    beneficiaries: rt::optional_field(value, 4usize, "beneficiaries")?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum TransferInput {
            ///The Daml `InputAppRewardCoupon` constructor.
            #[serde(rename = "InputAppRewardCoupon")]
            InputAppRewardCoupon(
                rt::ContractId<
                    crate::splice_amulet_0_1_14::Splice_Amulet::AppRewardCoupon,
                >,
            ),
            ///The Daml `InputValidatorRewardCoupon` constructor.
            #[serde(rename = "InputValidatorRewardCoupon")]
            InputValidatorRewardCoupon(
                rt::ContractId<
                    crate::splice_amulet_0_1_14::Splice_Amulet::ValidatorRewardCoupon,
                >,
            ),
            ///The Daml `InputSvRewardCoupon` constructor.
            #[serde(rename = "InputSvRewardCoupon")]
            InputSvRewardCoupon(
                rt::ContractId<
                    crate::splice_amulet_0_1_14::Splice_Amulet::SvRewardCoupon,
                >,
            ),
            ///The Daml `InputAmulet` constructor.
            #[serde(rename = "InputAmulet")]
            InputAmulet(
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Amulet::Amulet>,
            ),
            ///The Daml `ExtTransferInput` constructor.
            #[serde(rename = "ExtTransferInput")]
            ExtTransferInput(
                crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferInput_ExtTransferInput,
            ),
            ///The Daml `InputValidatorLivenessActivityRecord` constructor.
            #[serde(rename = "InputValidatorLivenessActivityRecord")]
            InputValidatorLivenessActivityRecord(
                rt::ContractId<
                    crate::splice_amulet_0_1_14::Splice_ValidatorLicense::ValidatorLivenessActivityRecord,
                >,
            ),
            ///The Daml `InputUnclaimedActivityRecord` constructor.
            #[serde(rename = "InputUnclaimedActivityRecord")]
            InputUnclaimedActivityRecord(
                rt::ContractId<
                    crate::splice_amulet_0_1_14::Splice_Amulet::UnclaimedActivityRecord,
                >,
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
                    "InputAppRewardCoupon" => ::core::result::Result::Ok(
                        TransferInput::InputAppRewardCoupon(rt::FromValue::from_value(payload)?),
                    ),
                    "InputValidatorRewardCoupon" => {
                        ::core::result::Result::Ok(TransferInput::InputValidatorRewardCoupon(
                            rt::FromValue::from_value(payload)?,
                        ))
                    }
                    "InputSvRewardCoupon" => ::core::result::Result::Ok(
                        TransferInput::InputSvRewardCoupon(rt::FromValue::from_value(payload)?),
                    ),
                    "InputAmulet" => ::core::result::Result::Ok(TransferInput::InputAmulet(
                        rt::FromValue::from_value(payload)?,
                    )),
                    "ExtTransferInput" => ::core::result::Result::Ok(
                        TransferInput::ExtTransferInput(rt::FromValue::from_value(payload)?),
                    ),
                    "InputValidatorLivenessActivityRecord" => ::core::result::Result::Ok(
                        TransferInput::InputValidatorLivenessActivityRecord(
                            rt::FromValue::from_value(payload)?,
                        ),
                    ),
                    "InputUnclaimedActivityRecord" => {
                        ::core::result::Result::Ok(TransferInput::InputUnclaimedActivityRecord(
                            rt::FromValue::from_value(payload)?,
                        ))
                    }
                    other => ::core::result::Result::Err(rt::unexpected_constructor(
                        "TransferInput",
                        other,
                    )),
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferInput_ExtTransferInput {
            ///The Daml `dummyUnitField` field.
            #[serde(rename = "dummyUnitField")]
            pub dummy_unit_field: rt::Unit,
            ///The Daml `optInputValidatorFaucetCoupon` field.
            #[serde(rename = "optInputValidatorFaucetCoupon")]
            pub opt_input_validator_faucet_coupon: ::core::option::Option<
                rt::ContractId<
                    crate::splice_amulet_0_1_14::Splice_ValidatorLicense::ValidatorFaucetCoupon,
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
                    )?)?,
                    opt_input_validator_faucet_coupon: rt::optional_field(
                        value,
                        1usize,
                        "optInputValidatorFaucetCoupon",
                    )?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum CreatedAmulet {
            ///The Daml `TransferResultAmulet` constructor.
            #[serde(rename = "TransferResultAmulet")]
            TransferResultAmulet(
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Amulet::Amulet>,
            ),
            ///The Daml `TransferResultLockedAmulet` constructor.
            #[serde(rename = "TransferResultLockedAmulet")]
            TransferResultLockedAmulet(
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Amulet::LockedAmulet>,
            ),
            ///The Daml `ExtCreatedAmulet` constructor.
            #[serde(rename = "ExtCreatedAmulet")]
            ExtCreatedAmulet(
                crate::splice_amulet_0_1_14::Splice_AmuletRules::CreatedAmulet_ExtCreatedAmulet,
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
                    "TransferResultAmulet" => ::core::result::Result::Ok(
                        CreatedAmulet::TransferResultAmulet(rt::FromValue::from_value(payload)?),
                    ),
                    "TransferResultLockedAmulet" => {
                        ::core::result::Result::Ok(CreatedAmulet::TransferResultLockedAmulet(
                            rt::FromValue::from_value(payload)?,
                        ))
                    }
                    "ExtCreatedAmulet" => ::core::result::Result::Ok(
                        CreatedAmulet::ExtCreatedAmulet(rt::FromValue::from_value(payload)?),
                    ),
                    other => ::core::result::Result::Err(rt::unexpected_constructor(
                        "CreatedAmulet",
                        other,
                    )),
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct CreatedAmulet_ExtCreatedAmulet {
            ///The Daml `dummyUnitField` field.
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferContext {
            ///The Daml `openMiningRound` field.
            #[serde(rename = "openMiningRound")]
            pub open_mining_round:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Round::OpenMiningRound>,
            ///The Daml `issuingMiningRounds` field.
            #[serde(rename = "issuingMiningRounds")]
            pub issuing_mining_rounds: rt::GenMap<
                crate::splice_amulet_0_1_14::Splice_Types::Round,
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Round::IssuingMiningRound>,
            >,
            ///The Daml `validatorRights` field.
            #[serde(rename = "validatorRights")]
            pub validator_rights: rt::GenMap<
                rt::Party,
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Amulet::ValidatorRight>,
            >,
            ///The Daml `featuredAppRight` field.
            #[serde(rename = "featuredAppRight")]
            pub featured_app_right: ::core::option::Option<
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Amulet::FeaturedAppRight>,
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
                    )?)?,
                    issuing_mining_rounds: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "issuingMiningRounds",
                    )?)?,
                    validator_rights: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "validatorRights",
                    )?)?,
                    featured_app_right: rt::optional_field(value, 3usize, "featuredAppRight")?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct PaymentTransferContext {
            ///The Daml `amuletRules` field.
            #[serde(rename = "amuletRules")]
            pub amulet_rules:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules>,
            ///The Daml `context` field.
            #[serde(rename = "context")]
            pub context: crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferContext,
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
                    )?)?,
                    context: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "context",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AppTransferContext {
            ///The Daml `amuletRules` field.
            #[serde(rename = "amuletRules")]
            pub amulet_rules:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules>,
            ///The Daml `openMiningRound` field.
            #[serde(rename = "openMiningRound")]
            pub open_mining_round:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Round::OpenMiningRound>,
            ///The Daml `featuredAppRight` field.
            #[serde(rename = "featuredAppRight")]
            pub featured_app_right: ::core::option::Option<
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Amulet::FeaturedAppRight>,
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
                    )?)?,
                    open_mining_round: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "openMiningRound",
                    )?)?,
                    featured_app_right: rt::optional_field(value, 2usize, "featuredAppRight")?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct PreprocessedTransferOutput {
            ///The Daml `owner` field.
            #[serde(rename = "owner")]
            pub owner: rt::Party,
            ///The Daml `outputFee` field.
            #[serde(rename = "outputFee")]
            pub output_fee: rt::Numeric,
            ///The Daml `amount` field.
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            ///The Daml `lock` field.
            #[serde(rename = "lock")]
            pub lock: ::core::option::Option<crate::splice_amulet_0_1_14::Splice_Expiry::TimeLock>,
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
                    owner: rt::FromValue::from_value(rt::required_field(value, 0usize, "owner")?)?,
                    output_fee: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "outputFee",
                    )?)?,
                    amount: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "amount",
                    )?)?,
                    lock: rt::optional_field(value, 3usize, "lock")?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferInputsSummary {
            ///The Daml `totalAmuletAmount` field.
            #[serde(rename = "totalAmuletAmount")]
            pub total_amulet_amount: rt::Numeric,
            ///The Daml `totalAppRewardAmount` field.
            #[serde(rename = "totalAppRewardAmount")]
            pub total_app_reward_amount: rt::Numeric,
            ///The Daml `totalValidatorRewardAmount` field.
            #[serde(rename = "totalValidatorRewardAmount")]
            pub total_validator_reward_amount: rt::Numeric,
            ///The Daml `totalValidatorFaucetAmount` field.
            #[serde(rename = "totalValidatorFaucetAmount")]
            pub total_validator_faucet_amount: rt::Numeric,
            ///The Daml `totalSvRewardAmount` field.
            #[serde(rename = "totalSvRewardAmount")]
            pub total_sv_reward_amount: rt::Numeric,
            ///The Daml `totalHoldingFees` field.
            #[serde(rename = "totalHoldingFees")]
            pub total_holding_fees: rt::Numeric,
            ///The Daml `amountArchivedAsOfRoundZero` field.
            #[serde(rename = "amountArchivedAsOfRoundZero")]
            pub amount_archived_as_of_round_zero: rt::Numeric,
            ///The Daml `changeToHoldingFeesRate` field.
            #[serde(rename = "changeToHoldingFeesRate")]
            pub change_to_holding_fees_rate: rt::Numeric,
            ///The Daml `totalUnclaimedActivityRecordAmount` field.
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
                    )?)?,
                    total_app_reward_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "totalAppRewardAmount",
                    )?)?,
                    total_validator_reward_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "totalValidatorRewardAmount",
                    )?)?,
                    total_validator_faucet_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "totalValidatorFaucetAmount",
                    )?)?,
                    total_sv_reward_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "totalSvRewardAmount",
                    )?)?,
                    total_holding_fees: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "totalHoldingFees",
                    )?)?,
                    amount_archived_as_of_round_zero: rt::FromValue::from_value(
                        rt::required_field(value, 6usize, "amountArchivedAsOfRoundZero")?,
                    )?,
                    change_to_holding_fees_rate: rt::FromValue::from_value(rt::required_field(
                        value,
                        7usize,
                        "changeToHoldingFeesRate",
                    )?)?,
                    total_unclaimed_activity_record_amount: rt::optional_field(
                        value,
                        8usize,
                        "totalUnclaimedActivityRecordAmount",
                    )?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferContextSummary {
            ///The Daml `featuredAppProvider` field.
            #[serde(rename = "featuredAppProvider")]
            pub featured_app_provider: ::core::option::Option<rt::Party>,
            ///The Daml `config` field.
            #[serde(rename = "config")]
            pub config: crate::splice_amulet_0_1_14::Splice_AmuletConfig::TransferConfig<
                crate::splice_amulet_0_1_14::Splice_Amulet::Amulet,
            >,
            ///The Daml `openRound` field.
            #[serde(rename = "openRound")]
            pub open_round: crate::splice_amulet_0_1_14::Splice_Round::OpenMiningRound,
            ///The Daml `issuingMiningRounds` field.
            #[serde(rename = "issuingMiningRounds")]
            pub issuing_mining_rounds: rt::GenMap<
                crate::splice_amulet_0_1_14::Splice_Types::Round,
                crate::splice_amulet_0_1_14::Splice_Round::IssuingMiningRound,
            >,
            ///The Daml `validatorRights` field.
            #[serde(rename = "validatorRights")]
            pub validator_rights: rt::GenMap<
                rt::Party,
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Amulet::ValidatorRight>,
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
                    featured_app_provider: rt::optional_field(
                        value,
                        0usize,
                        "featuredAppProvider",
                    )?,
                    config: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "config",
                    )?)?,
                    open_round: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "openRound",
                    )?)?,
                    issuing_mining_rounds: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "issuingMiningRounds",
                    )?)?,
                    validator_rights: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "validatorRights",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct RewardsIssuanceConfig {
            ///The Daml `issueAppRewards` field.
            #[serde(rename = "issueAppRewards")]
            pub issue_app_rewards: bool,
            ///The Daml `issueValidatorRewards` field.
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
                    )?)?,
                    issue_validator_rewards: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "issueValidatorRewards",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct InvalidTransfer {
            ///The Daml `reason` field.
            #[serde(rename = "reason")]
            pub reason: crate::splice_amulet_0_1_14::Splice_AmuletRules::InvalidTransferReason,
        }
        impl rt::ToValue for InvalidTransfer {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("reason", rt::ToValue::to_value(&self.reason)),])
            }
        }
        impl rt::FromValue for InvalidTransfer {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    reason: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "reason",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum InvalidTransferReason {
            ///The Daml `ITR_InsufficientFunds` constructor.
            #[serde(rename = "ITR_InsufficientFunds")]
            ITR_InsufficientFunds(
                crate::splice_amulet_0_1_14::Splice_AmuletRules::InvalidTransferReason_ITR_InsufficientFunds,
            ),
            ///The Daml `ITR_UnknownSynchronizer` constructor.
            #[serde(rename = "ITR_UnknownSynchronizer")]
            ITR_UnknownSynchronizer(
                crate::splice_amulet_0_1_14::Splice_AmuletRules::InvalidTransferReason_ITR_UnknownSynchronizer,
            ),
            ///The Daml `ITR_InsufficientTopupAmount` constructor.
            #[serde(rename = "ITR_InsufficientTopupAmount")]
            ITR_InsufficientTopupAmount(
                crate::splice_amulet_0_1_14::Splice_AmuletRules::InvalidTransferReason_ITR_InsufficientTopupAmount,
            ),
            ///The Daml `ITR_Other` constructor.
            #[serde(rename = "ITR_Other")]
            ITR_Other(
                crate::splice_amulet_0_1_14::Splice_AmuletRules::InvalidTransferReason_ITR_Other,
            ),
            ///The Daml `ExtInvalidTransferReason` constructor.
            #[serde(rename = "ExtInvalidTransferReason")]
            ExtInvalidTransferReason(
                crate::splice_amulet_0_1_14::Splice_AmuletRules::InvalidTransferReason_ExtInvalidTransferReason,
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
                            rt::FromValue::from_value(payload)?,
                        ))
                    }
                    "ITR_UnknownSynchronizer" => {
                        ::core::result::Result::Ok(InvalidTransferReason::ITR_UnknownSynchronizer(
                            rt::FromValue::from_value(payload)?,
                        ))
                    }
                    "ITR_InsufficientTopupAmount" => ::core::result::Result::Ok(
                        InvalidTransferReason::ITR_InsufficientTopupAmount(
                            rt::FromValue::from_value(payload)?,
                        ),
                    ),
                    "ITR_Other" => ::core::result::Result::Ok(InvalidTransferReason::ITR_Other(
                        rt::FromValue::from_value(payload)?,
                    )),
                    "ExtInvalidTransferReason" => {
                        ::core::result::Result::Ok(InvalidTransferReason::ExtInvalidTransferReason(
                            rt::FromValue::from_value(payload)?,
                        ))
                    }
                    other => ::core::result::Result::Err(rt::unexpected_constructor(
                        "InvalidTransferReason",
                        other,
                    )),
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct InvalidTransferReason_ITR_InsufficientFunds {
            ///The Daml `missingAmount` field.
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct InvalidTransferReason_ITR_UnknownSynchronizer {
            ///The Daml `synchronizerId` field.
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct InvalidTransferReason_ITR_InsufficientTopupAmount {
            ///The Daml `requestedTopupAmount` field.
            #[serde(rename = "requestedTopupAmount")]
            pub requested_topup_amount: rt::Int64,
            ///The Daml `minTopupAmount` field.
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
                    )?)?,
                    min_topup_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "minTopupAmount",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct InvalidTransferReason_ITR_Other {
            ///The Daml `description` field.
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct InvalidTransferReason_ExtInvalidTransferReason {
            ///The Daml `dummyUnitField` field.
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_ConvertFeaturedAppActivityMarkers {
            ///The Daml `markerCids` field.
            #[serde(rename = "markerCids")]
            pub marker_cids: ::std::vec::Vec<
                rt::ContractId<
                    crate::splice_amulet_0_1_14::Splice_Amulet::FeaturedAppActivityMarker,
                >,
            >,
            ///The Daml `openMiningRoundCid` field.
            #[serde(rename = "openMiningRoundCid")]
            pub open_mining_round_cid:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Round::OpenMiningRound>,
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
                    )?)?,
                    open_mining_round_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "openMiningRoundCid",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_UpdateFutureAmuletConfigSchedule {
            ///The Daml `scheduleItem` field.
            #[serde(rename = "scheduleItem")]
            pub schedule_item: crate::daml_prim_DA_Types_1_0_0::DA_Types::Tuple2<
                rt::Timestamp,
                crate::splice_amulet_0_1_14::Splice_AmuletConfig::AmuletConfig<
                    crate::splice_amulet_0_1_14::Splice_AmuletConfig::USD,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_RemoveFutureAmuletConfigSchedule {
            ///The Daml `scheduleTime` field.
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_AddFutureAmuletConfigSchedule {
            ///The Daml `newScheduleItem` field.
            #[serde(rename = "newScheduleItem")]
            pub new_schedule_item: crate::daml_prim_DA_Types_1_0_0::DA_Types::Tuple2<
                rt::Timestamp,
                crate::splice_amulet_0_1_14::Splice_AmuletConfig::AmuletConfig<
                    crate::splice_amulet_0_1_14::Splice_AmuletConfig::USD,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_SetConfig {
            ///The Daml `newConfig` field.
            #[serde(rename = "newConfig")]
            pub new_config: crate::splice_amulet_0_1_14::Splice_AmuletConfig::AmuletConfig<
                crate::splice_amulet_0_1_14::Splice_AmuletConfig::USD,
            >,
            ///The Daml `baseConfig` field.
            #[serde(rename = "baseConfig")]
            pub base_config: crate::splice_amulet_0_1_14::Splice_AmuletConfig::AmuletConfig<
                crate::splice_amulet_0_1_14::Splice_AmuletConfig::USD,
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
                    )?)?,
                    base_config: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "baseConfig",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_Fetch {
            ///The Daml `p` field.
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
                    p: rt::FromValue::from_value(rt::required_field(value, 0usize, "p")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_MergeUnclaimedRewards {
            ///The Daml `unclaimedRewardCids` field.
            #[serde(rename = "unclaimedRewardCids")]
            pub unclaimed_reward_cids: ::std::vec::Vec<
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Amulet::UnclaimedReward>,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_ClaimExpiredRewards {
            ///The Daml `closedRoundCid` field.
            #[serde(rename = "closedRoundCid")]
            pub closed_round_cid: rt::ContractId<
                crate::splice_amulet_0_1_14::Splice_Round::ClosedMiningRound,
            >,
            ///The Daml `validatorRewardCouponCids` field.
            #[serde(rename = "validatorRewardCouponCids")]
            pub validator_reward_coupon_cids: ::std::vec::Vec<
                rt::ContractId<
                    crate::splice_amulet_0_1_14::Splice_Amulet::ValidatorRewardCoupon,
                >,
            >,
            ///The Daml `appCouponCids` field.
            #[serde(rename = "appCouponCids")]
            pub app_coupon_cids: ::std::vec::Vec<
                rt::ContractId<
                    crate::splice_amulet_0_1_14::Splice_Amulet::AppRewardCoupon,
                >,
            >,
            ///The Daml `svRewardCouponCids` field.
            #[serde(rename = "svRewardCouponCids")]
            pub sv_reward_coupon_cids: ::std::vec::Vec<
                rt::ContractId<
                    crate::splice_amulet_0_1_14::Splice_Amulet::SvRewardCoupon,
                >,
            >,
            ///The Daml `optValidatorFaucetCouponCids` field.
            #[serde(rename = "optValidatorFaucetCouponCids")]
            pub opt_validator_faucet_coupon_cids: ::core::option::Option<
                ::std::vec::Vec<
                    rt::ContractId<
                        crate::splice_amulet_0_1_14::Splice_ValidatorLicense::ValidatorFaucetCoupon,
                    >,
                >,
            >,
            ///The Daml `optValidatorLivenessActivityRecordCids` field.
            #[serde(rename = "optValidatorLivenessActivityRecordCids")]
            pub opt_validator_liveness_activity_record_cids: ::core::option::Option<
                ::std::vec::Vec<
                    rt::ContractId<
                        crate::splice_amulet_0_1_14::Splice_ValidatorLicense::ValidatorLivenessActivityRecord,
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
                    )?)?,
                    validator_reward_coupon_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "validatorRewardCouponCids",
                    )?)?,
                    app_coupon_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "appCouponCids",
                    )?)?,
                    sv_reward_coupon_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "svRewardCouponCids",
                    )?)?,
                    opt_validator_faucet_coupon_cids: rt::optional_field(
                        value,
                        4usize,
                        "optValidatorFaucetCouponCids",
                    )?,
                    opt_validator_liveness_activity_record_cids: rt::optional_field(
                        value,
                        5usize,
                        "optValidatorLivenessActivityRecordCids",
                    )?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_MiningRound_Archive {
            ///The Daml `closedRoundCid` field.
            #[serde(rename = "closedRoundCid")]
            pub closed_round_cid:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Round::ClosedMiningRound>,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_MiningRound_Close {
            ///The Daml `issuingRoundCid` field.
            #[serde(rename = "issuingRoundCid")]
            pub issuing_round_cid:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Round::IssuingMiningRound>,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_MiningRound_StartIssuing {
            ///The Daml `miningRoundCid` field.
            #[serde(rename = "miningRoundCid")]
            pub mining_round_cid:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Round::SummarizingMiningRound>,
            ///The Daml `summary` field.
            #[serde(rename = "summary")]
            pub summary: crate::splice_amulet_0_1_14::Splice_Issuance::OpenMiningRoundSummary,
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
                    )?)?,
                    summary: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "summary",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_AdvanceOpenMiningRounds {
            ///The Daml `amuletPrice` field.
            #[serde(rename = "amuletPrice")]
            pub amulet_price: rt::Numeric,
            ///The Daml `roundToArchiveCid` field.
            #[serde(rename = "roundToArchiveCid")]
            pub round_to_archive_cid:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Round::OpenMiningRound>,
            ///The Daml `middleRoundCid` field.
            #[serde(rename = "middleRoundCid")]
            pub middle_round_cid:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Round::OpenMiningRound>,
            ///The Daml `latestRoundCid` field.
            #[serde(rename = "latestRoundCid")]
            pub latest_round_cid:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Round::OpenMiningRound>,
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
                    )?)?,
                    round_to_archive_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "roundToArchiveCid",
                    )?)?,
                    middle_round_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "middleRoundCid",
                    )?)?,
                    latest_round_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "latestRoundCid",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_Bootstrap_Rounds {
            ///The Daml `amuletPrice` field.
            #[serde(rename = "amuletPrice")]
            pub amulet_price: rt::Numeric,
            ///The Daml `round0Duration` field.
            #[serde(rename = "round0Duration")]
            pub round0_duration: crate::daml_stdlib_DA_Time_Types_1_0_0::DA_Time_Types::RelTime,
            ///The Daml `initialRound` field.
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
                    )?)?,
                    round0_duration: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "round0Duration",
                    )?)?,
                    initial_round: rt::optional_field(value, 2usize, "initialRound")?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_DevNet_FeatureApp {
            ///The Daml `provider` field.
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_DevNet_Tap {
            ///The Daml `receiver` field.
            #[serde(rename = "receiver")]
            pub receiver: rt::Party,
            ///The Daml `amount` field.
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            ///The Daml `openRound` field.
            #[serde(rename = "openRound")]
            pub open_round:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Round::OpenMiningRound>,
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
                    )?)?,
                    amount: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "amount",
                    )?)?,
                    open_round: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "openRound",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_Mint {
            ///The Daml `receiver` field.
            #[serde(rename = "receiver")]
            pub receiver: rt::Party,
            ///The Daml `amount` field.
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            ///The Daml `openRound` field.
            #[serde(rename = "openRound")]
            pub open_round:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Round::OpenMiningRound>,
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
                    )?)?,
                    amount: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "amount",
                    )?)?,
                    open_round: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "openRound",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_MergeMemberTrafficContracts {
            ///The Daml `trafficCids` field.
            #[serde(rename = "trafficCids")]
            pub traffic_cids: ::std::vec::Vec<
                rt::ContractId<
                    crate::splice_amulet_0_1_14::Splice_DecentralizedSynchronizer::MemberTraffic,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_BuyMemberTraffic {
            ///The Daml `inputs` field.
            #[serde(rename = "inputs")]
            pub inputs:
                ::std::vec::Vec<crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferInput>,
            ///The Daml `context` field.
            #[serde(rename = "context")]
            pub context: crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferContext,
            ///The Daml `provider` field.
            #[serde(rename = "provider")]
            pub provider: rt::Party,
            ///The Daml `memberId` field.
            #[serde(rename = "memberId")]
            pub member_id: ::std::string::String,
            ///The Daml `synchronizerId` field.
            #[serde(rename = "synchronizerId")]
            pub synchronizer_id: ::std::string::String,
            ///The Daml `migrationId` field.
            #[serde(rename = "migrationId")]
            pub migration_id: rt::Int64,
            ///The Daml `trafficAmount` field.
            #[serde(rename = "trafficAmount")]
            pub traffic_amount: rt::Int64,
            ///The Daml `expectedDso` field.
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
                    inputs: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "inputs",
                    )?)?,
                    context: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "context",
                    )?)?,
                    provider: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "provider",
                    )?)?,
                    member_id: rt::FromValue::from_value(rt::required_field(
                        value, 3usize, "memberId",
                    )?)?,
                    synchronizer_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "synchronizerId",
                    )?)?,
                    migration_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "migrationId",
                    )?)?,
                    traffic_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        6usize,
                        "trafficAmount",
                    )?)?,
                    expected_dso: rt::optional_field(value, 7usize, "expectedDso")?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_CreateTransferPreapproval {
            ///The Daml `context` field.
            #[serde(rename = "context")]
            pub context: crate::splice_amulet_0_1_14::Splice_AmuletRules::PaymentTransferContext,
            ///The Daml `inputs` field.
            #[serde(rename = "inputs")]
            pub inputs:
                ::std::vec::Vec<crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferInput>,
            ///The Daml `receiver` field.
            #[serde(rename = "receiver")]
            pub receiver: rt::Party,
            ///The Daml `provider` field.
            #[serde(rename = "provider")]
            pub provider: rt::Party,
            ///The Daml `expiresAt` field.
            #[serde(rename = "expiresAt")]
            pub expires_at: rt::Timestamp,
            ///The Daml `expectedDso` field.
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
                    )?)?,
                    inputs: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "inputs",
                    )?)?,
                    receiver: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "receiver",
                    )?)?,
                    provider: rt::FromValue::from_value(rt::required_field(
                        value, 3usize, "provider",
                    )?)?,
                    expires_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "expiresAt",
                    )?)?,
                    expected_dso: rt::optional_field(value, 5usize, "expectedDso")?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_CreateExternalPartySetupProposal {
            ///The Daml `context` field.
            #[serde(rename = "context")]
            pub context: crate::splice_amulet_0_1_14::Splice_AmuletRules::PaymentTransferContext,
            ///The Daml `inputs` field.
            #[serde(rename = "inputs")]
            pub inputs:
                ::std::vec::Vec<crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferInput>,
            ///The Daml `user` field.
            #[serde(rename = "user")]
            pub user: rt::Party,
            ///The Daml `validator` field.
            #[serde(rename = "validator")]
            pub validator: rt::Party,
            ///The Daml `preapprovalExpiresAt` field.
            #[serde(rename = "preapprovalExpiresAt")]
            pub preapproval_expires_at: rt::Timestamp,
            ///The Daml `expectedDso` field.
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
                    )?)?,
                    inputs: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "inputs",
                    )?)?,
                    user: rt::FromValue::from_value(rt::required_field(value, 2usize, "user")?)?,
                    validator: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "validator",
                    )?)?,
                    preapproval_expires_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "preapprovalExpiresAt",
                    )?)?,
                    expected_dso: rt::optional_field(value, 5usize, "expectedDso")?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_Transfer {
            ///The Daml `transfer` field.
            #[serde(rename = "transfer")]
            pub transfer: crate::splice_amulet_0_1_14::Splice_AmuletRules::Transfer,
            ///The Daml `context` field.
            #[serde(rename = "context")]
            pub context: crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferContext,
            ///The Daml `expectedDso` field.
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
                    )?)?,
                    context: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "context",
                    )?)?,
                    expected_dso: rt::optional_field(value, 2usize, "expectedDso")?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_ComputeFees {
            ///The Daml `context` field.
            #[serde(rename = "context")]
            pub context: crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferContext,
            ///The Daml `sender` field.
            #[serde(rename = "sender")]
            pub sender: rt::Party,
            ///The Daml `outputs` field.
            #[serde(rename = "outputs")]
            pub outputs:
                ::std::vec::Vec<crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferOutput>,
            ///The Daml `expectedDso` field.
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
                    )?)?,
                    sender: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "sender",
                    )?)?,
                    outputs: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "outputs",
                    )?)?,
                    expected_dso: rt::optional_field(value, 3usize, "expectedDso")?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_ConvertFeaturedAppActivityMarkersResult {
            ///The Daml `appRewardCouponCids` field.
            #[serde(rename = "appRewardCouponCids")]
            pub app_reward_coupon_cids: ::std::vec::Vec<
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Amulet::AppRewardCoupon>,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_UpdateFutureAmuletConfigScheduleResult {
            ///The Daml `newAmuletRules` field.
            #[serde(rename = "newAmuletRules")]
            pub new_amulet_rules:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules>,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_RemoveFutureAmuletConfigScheduleResult {
            ///The Daml `newAmuletRules` field.
            #[serde(rename = "newAmuletRules")]
            pub new_amulet_rules:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules>,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_AddFutureAmuletConfigScheduleResult {
            ///The Daml `newAmuletRules` field.
            #[serde(rename = "newAmuletRules")]
            pub new_amulet_rules:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules>,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_SetConfigResult {
            ///The Daml `newAmuletRules` field.
            #[serde(rename = "newAmuletRules")]
            pub new_amulet_rules:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules>,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_MergeUnclaimedRewardsResult {
            ///The Daml `unclaimedRewardCid` field.
            #[serde(rename = "unclaimedRewardCid")]
            pub unclaimed_reward_cid:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Amulet::UnclaimedReward>,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_ClaimExpiredRewardsResult {
            ///The Daml `unclaimedRewardCid` field.
            #[serde(rename = "unclaimedRewardCid")]
            pub unclaimed_reward_cid: ::core::option::Option<
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Amulet::UnclaimedReward>,
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
                    unclaimed_reward_cid: rt::optional_field(value, 0usize, "unclaimedRewardCid")?,
                })
            }
        }
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize,
        )]
        #[serde(crate = "rt::serde")]
        pub enum AmuletRules_MiningRound_ArchiveResult {
            ///The Daml `AmuletRules_MiningRound_ArchiveResult` value.
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
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_MiningRound_CloseResult {
            ///The Daml `closedRoundCid` field.
            #[serde(rename = "closedRoundCid")]
            pub closed_round_cid:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Round::ClosedMiningRound>,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_MiningRound_StartIssuingResult {
            ///The Daml `issuingRoundCid` field.
            #[serde(rename = "issuingRoundCid")]
            pub issuing_round_cid:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Round::IssuingMiningRound>,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_AdvanceOpenMiningRoundsResult {
            ///The Daml `summarizingRoundCid` field.
            #[serde(rename = "summarizingRoundCid")]
            pub summarizing_round_cid:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Round::SummarizingMiningRound>,
            ///The Daml `openRoundCid` field.
            #[serde(rename = "openRoundCid")]
            pub open_round_cid:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Round::OpenMiningRound>,
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
                    )?)?,
                    open_round_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "openRoundCid",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_Bootstrap_RoundsResult {
            ///The Daml `openMiningRoundCid` field.
            #[serde(rename = "openMiningRoundCid")]
            pub open_mining_round_cid:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Round::OpenMiningRound>,
            ///The Daml `initialRound` field.
            #[serde(rename = "initialRound")]
            pub initial_round:
                ::core::option::Option<crate::splice_amulet_0_1_14::Splice_Types::Round>,
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
                    )?)?,
                    initial_round: rt::optional_field(value, 1usize, "initialRound")?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_DevNet_FeatureAppResult {
            ///The Daml `featuredAppRightCid` field.
            #[serde(rename = "featuredAppRightCid")]
            pub featured_app_right_cid:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Amulet::FeaturedAppRight>,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_DevNet_TapResult {
            ///The Daml `amuletSum` field.
            #[serde(rename = "amuletSum")]
            pub amulet_sum: crate::splice_amulet_0_1_14::Splice_Amulet::AmuletCreateSummary<
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Amulet::Amulet>,
            >,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta: ::core::option::Option<
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
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
                    )?)?,
                    meta: rt::optional_field(value, 1usize, "meta")?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_MintResult {
            ///The Daml `amuletSum` field.
            #[serde(rename = "amuletSum")]
            pub amulet_sum: crate::splice_amulet_0_1_14::Splice_Amulet::AmuletCreateSummary<
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Amulet::Amulet>,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_MergeMemberTrafficContractsResult {
            ///The Daml `mergedTrafficCid` field.
            #[serde(rename = "mergedTrafficCid")]
            pub merged_traffic_cid: rt::ContractId<
                crate::splice_amulet_0_1_14::Splice_DecentralizedSynchronizer::MemberTraffic,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules_ComputeFeesResult {
            ///The Daml `fees` field.
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
                    fees: rt::FromValue::from_value(rt::required_field(value, 0usize, "fees")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletRules {
            ///The Daml `dso` field.
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///The Daml `configSchedule` field.
            #[serde(rename = "configSchedule")]
            pub config_schedule: crate::splice_amulet_0_1_14::Splice_Schedule::Schedule<
                rt::Timestamp,
                crate::splice_amulet_0_1_14::Splice_AmuletConfig::AmuletConfig<
                    crate::splice_amulet_0_1_14::Splice_AmuletConfig::USD,
                >,
            >,
            ///The Daml `isDevNet` field.
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
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)?,
                    config_schedule: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "configSchedule",
                    )?)?,
                    is_dev_net: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "isDevNet",
                    )?)?,
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
        impl rt::Template for AmuletRules {}
        ///The `AmuletRules_ComputeFees` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules>
            for crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_ComputeFees
        {
            type Return =
                crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_ComputeFeesResult;
            const NAME: &'static str = "AmuletRules_ComputeFees";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_Transfer` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules>
            for crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_Transfer
        {
            type Return = crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferResult;
            const NAME: &'static str = "AmuletRules_Transfer";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_CreateExternalPartySetupProposal` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules>
        for crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_CreateExternalPartySetupProposal {
            type Return = crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_CreateExternalPartySetupProposalResult;
            const NAME: &'static str = "AmuletRules_CreateExternalPartySetupProposal";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_CreateTransferPreapproval` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules>
        for crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_CreateTransferPreapproval {
            type Return = crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_CreateTransferPreapprovalResult;
            const NAME: &'static str = "AmuletRules_CreateTransferPreapproval";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_BuyMemberTraffic` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules>
            for crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_BuyMemberTraffic
        {
            type Return =
                crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_BuyMemberTrafficResult;
            const NAME: &'static str = "AmuletRules_BuyMemberTraffic";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_MergeMemberTrafficContracts` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules>
        for crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_MergeMemberTrafficContracts {
            type Return = crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_MergeMemberTrafficContractsResult;
            const NAME: &'static str = "AmuletRules_MergeMemberTrafficContracts";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_Mint` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules> for crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_Mint {
            type Return = crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_MintResult;
            const NAME: &'static str = "AmuletRules_Mint";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_DevNet_Tap` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules>
            for crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_DevNet_Tap
        {
            type Return =
                crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_DevNet_TapResult;
            const NAME: &'static str = "AmuletRules_DevNet_Tap";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_DevNet_FeatureApp` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules>
            for crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_DevNet_FeatureApp
        {
            type Return = crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_DevNet_FeatureAppResult;
            const NAME: &'static str = "AmuletRules_DevNet_FeatureApp";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_Bootstrap_Rounds` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules>
            for crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_Bootstrap_Rounds
        {
            type Return =
                crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_Bootstrap_RoundsResult;
            const NAME: &'static str = "AmuletRules_Bootstrap_Rounds";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_AdvanceOpenMiningRounds` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules>
            for crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_AdvanceOpenMiningRounds
        {
            type Return = crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_AdvanceOpenMiningRoundsResult;
            const NAME: &'static str = "AmuletRules_AdvanceOpenMiningRounds";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_MiningRound_StartIssuing` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules>
        for crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_MiningRound_StartIssuing {
            type Return = crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_MiningRound_StartIssuingResult;
            const NAME: &'static str = "AmuletRules_MiningRound_StartIssuing";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_MiningRound_Close` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules>
            for crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_MiningRound_Close
        {
            type Return = crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_MiningRound_CloseResult;
            const NAME: &'static str = "AmuletRules_MiningRound_Close";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_MiningRound_Archive` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules>
            for crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_MiningRound_Archive
        {
            type Return = crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_MiningRound_ArchiveResult;
            const NAME: &'static str = "AmuletRules_MiningRound_Archive";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_ClaimExpiredRewards` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules>
            for crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_ClaimExpiredRewards
        {
            type Return = crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_ClaimExpiredRewardsResult;
            const NAME: &'static str = "AmuletRules_ClaimExpiredRewards";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_MergeUnclaimedRewards` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules>
            for crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_MergeUnclaimedRewards
        {
            type Return = crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_MergeUnclaimedRewardsResult;
            const NAME: &'static str = "AmuletRules_MergeUnclaimedRewards";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_SetConfig` choice on [`AmuletRules`] (consuming).
        impl rt::Choice<AmuletRules>
            for crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_SetConfig
        {
            type Return =
                crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_SetConfigResult;
            const NAME: &'static str = "AmuletRules_SetConfig";
            const CONSUMING: bool = true;
        }
        ///The `AmuletRules_ConvertFeaturedAppActivityMarkers` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules>
        for crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_ConvertFeaturedAppActivityMarkers {
            type Return = crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_ConvertFeaturedAppActivityMarkersResult;
            const NAME: &'static str = "AmuletRules_ConvertFeaturedAppActivityMarkers";
            const CONSUMING: bool = false;
        }
        ///The `Archive` choice on [`AmuletRules`] (consuming).
        impl rt::Choice<AmuletRules>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `AmuletRules_Fetch` choice on [`AmuletRules`] (non-consuming).
        impl rt::Choice<AmuletRules>
            for crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_Fetch
        {
            type Return = crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules;
            const NAME: &'static str = "AmuletRules_Fetch";
            const CONSUMING: bool = false;
        }
        ///The `AmuletRules_AddFutureAmuletConfigSchedule` choice on [`AmuletRules`] (consuming).
        impl rt::Choice<AmuletRules>
        for crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_AddFutureAmuletConfigSchedule {
            type Return = crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_AddFutureAmuletConfigScheduleResult;
            const NAME: &'static str = "AmuletRules_AddFutureAmuletConfigSchedule";
            const CONSUMING: bool = true;
        }
        ///The `AmuletRules_RemoveFutureAmuletConfigSchedule` choice on [`AmuletRules`] (consuming).
        impl rt::Choice<AmuletRules>
        for crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_RemoveFutureAmuletConfigSchedule {
            type Return = crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_RemoveFutureAmuletConfigScheduleResult;
            const NAME: &'static str = "AmuletRules_RemoveFutureAmuletConfigSchedule";
            const CONSUMING: bool = true;
        }
        ///The `AmuletRules_UpdateFutureAmuletConfigSchedule` choice on [`AmuletRules`] (consuming).
        impl rt::Choice<AmuletRules>
        for crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_UpdateFutureAmuletConfigSchedule {
            type Return = crate::splice_amulet_0_1_14::Splice_AmuletRules::AmuletRules_UpdateFutureAmuletConfigScheduleResult;
            const NAME: &'static str = "AmuletRules_UpdateFutureAmuletConfigSchedule";
            const CONSUMING: bool = true;
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ExternalPartySetupProposal {
            ///The Daml `validator` field.
            #[serde(rename = "validator")]
            pub validator: rt::Party,
            ///The Daml `user` field.
            #[serde(rename = "user")]
            pub user: rt::Party,
            ///The Daml `dso` field.
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///The Daml `createdAt` field.
            #[serde(rename = "createdAt")]
            pub created_at: rt::Timestamp,
            ///The Daml `preapprovalExpiresAt` field.
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
                    )?)?,
                    user: rt::FromValue::from_value(rt::required_field(value, 1usize, "user")?)?,
                    dso: rt::FromValue::from_value(rt::required_field(value, 2usize, "dso")?)?,
                    created_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "createdAt",
                    )?)?,
                    preapproval_expires_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "preapprovalExpiresAt",
                    )?)?,
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
        impl rt::Template for ExternalPartySetupProposal {}
        ///The `ExternalPartySetupProposal_Accept` choice on [`ExternalPartySetupProposal`] (consuming).
        impl rt::Choice<ExternalPartySetupProposal>
            for crate::splice_amulet_0_1_14::Splice_AmuletRules::ExternalPartySetupProposal_Accept
        {
            type Return = crate::splice_amulet_0_1_14::Splice_AmuletRules::ExternalPartySetupProposal_AcceptResult;
            const NAME: &'static str = "ExternalPartySetupProposal_Accept";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`ExternalPartySetupProposal`] (consuming).
        impl rt::Choice<ExternalPartySetupProposal>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `ExternalPartySetupProposal_Reject` choice on [`ExternalPartySetupProposal`] (consuming).
        impl rt::Choice<ExternalPartySetupProposal>
            for crate::splice_amulet_0_1_14::Splice_AmuletRules::ExternalPartySetupProposal_Reject
        {
            type Return = crate::splice_amulet_0_1_14::Splice_AmuletRules::ExternalPartySetupProposal_RejectResult;
            const NAME: &'static str = "ExternalPartySetupProposal_Reject";
            const CONSUMING: bool = true;
        }
        ///The `ExternalPartySetupProposal_Withdraw` choice on [`ExternalPartySetupProposal`] (consuming).
        impl rt::Choice<ExternalPartySetupProposal>
            for crate::splice_amulet_0_1_14::Splice_AmuletRules::ExternalPartySetupProposal_Withdraw
        {
            type Return = crate::splice_amulet_0_1_14::Splice_AmuletRules::ExternalPartySetupProposal_WithdrawResult;
            const NAME: &'static str = "ExternalPartySetupProposal_Withdraw";
            const CONSUMING: bool = true;
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferPreapproval {
            ///The Daml `dso` field.
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///The Daml `receiver` field.
            #[serde(rename = "receiver")]
            pub receiver: rt::Party,
            ///The Daml `provider` field.
            #[serde(rename = "provider")]
            pub provider: rt::Party,
            ///The Daml `validFrom` field.
            #[serde(rename = "validFrom")]
            pub valid_from: rt::Timestamp,
            ///The Daml `lastRenewedAt` field.
            #[serde(rename = "lastRenewedAt")]
            pub last_renewed_at: rt::Timestamp,
            ///The Daml `expiresAt` field.
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
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)?,
                    receiver: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "receiver",
                    )?)?,
                    provider: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "provider",
                    )?)?,
                    valid_from: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "validFrom",
                    )?)?,
                    last_renewed_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "lastRenewedAt",
                    )?)?,
                    expires_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "expiresAt",
                    )?)?,
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
        impl rt::Template for TransferPreapproval {}
        ///The `TransferPreapproval_Renew` choice on [`TransferPreapproval`] (consuming).
        impl rt::Choice<TransferPreapproval>
            for crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferPreapproval_Renew
        {
            type Return =
                crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferPreapproval_RenewResult;
            const NAME: &'static str = "TransferPreapproval_Renew";
            const CONSUMING: bool = true;
        }
        ///The `TransferPreapproval_Send` choice on [`TransferPreapproval`] (non-consuming).
        impl rt::Choice<TransferPreapproval>
            for crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferPreapproval_Send
        {
            type Return =
                crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferPreapproval_SendResult;
            const NAME: &'static str = "TransferPreapproval_Send";
            const CONSUMING: bool = false;
        }
        ///The `TransferPreapproval_Expire` choice on [`TransferPreapproval`] (consuming).
        impl rt::Choice<TransferPreapproval>
            for crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferPreapproval_Expire
        {
            type Return =
                crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferPreapproval_ExpireResult;
            const NAME: &'static str = "TransferPreapproval_Expire";
            const CONSUMING: bool = true;
        }
        ///The `TransferPreapproval_Cancel` choice on [`TransferPreapproval`] (consuming).
        impl rt::Choice<TransferPreapproval>
            for crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferPreapproval_Cancel
        {
            type Return =
                crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferPreapproval_CancelResult;
            const NAME: &'static str = "TransferPreapproval_Cancel";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`TransferPreapproval`] (consuming).
        impl rt::Choice<TransferPreapproval>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `TransferPreapproval_Fetch` choice on [`TransferPreapproval`] (non-consuming).
        impl rt::Choice<TransferPreapproval>
            for crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferPreapproval_Fetch
        {
            type Return = crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferPreapproval;
            const NAME: &'static str = "TransferPreapproval_Fetch";
            const CONSUMING: bool = false;
        }
    }
    pub mod Splice_Amulet {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct UnclaimedActivityRecord_DsoExpire {}
        impl rt::ToValue for UnclaimedActivityRecord_DsoExpire {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for UnclaimedActivityRecord_DsoExpire {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SvRewardCoupon_ArchiveAsBeneficiary {}
        impl rt::ToValue for SvRewardCoupon_ArchiveAsBeneficiary {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for SvRewardCoupon_ArchiveAsBeneficiary {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SvRewardCoupon_DsoExpire {
            ///The Daml `closedRoundCid` field.
            #[serde(rename = "closedRoundCid")]
            pub closed_round_cid:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Round::ClosedMiningRound>,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorRewardCoupon_ArchiveAsValidator {
            ///The Daml `validator` field.
            #[serde(rename = "validator")]
            pub validator: rt::Party,
            ///The Daml `rightCid` field.
            #[serde(rename = "rightCid")]
            pub right_cid:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Amulet::ValidatorRight>,
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
                    )?)?,
                    right_cid: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "rightCid",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorRewardCoupon_DsoExpire {
            ///The Daml `closedRoundCid` field.
            #[serde(rename = "closedRoundCid")]
            pub closed_round_cid:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Round::ClosedMiningRound>,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AppRewardCoupon_DsoExpire {
            ///The Daml `closedRoundCid` field.
            #[serde(rename = "closedRoundCid")]
            pub closed_round_cid:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Round::ClosedMiningRound>,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct FeaturedAppRight_Cancel {}
        impl rt::ToValue for FeaturedAppRight_Cancel {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for FeaturedAppRight_Cancel {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct FeaturedAppRight_Withdraw {
            ///The Daml `reason` field.
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
                    reason: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "reason",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorRight_ArchiveAsUser {}
        impl rt::ToValue for ValidatorRight_ArchiveAsUser {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for ValidatorRight_ArchiveAsUser {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorRight_ArchiveAsValidator {}
        impl rt::ToValue for ValidatorRight_ArchiveAsValidator {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for ValidatorRight_ArchiveAsValidator {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct LockedAmulet_ExpireAmulet {
            ///The Daml `roundCid` field.
            #[serde(rename = "roundCid")]
            pub round_cid:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Round::OpenMiningRound>,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct LockedAmulet_OwnerExpireLock {
            ///The Daml `openRoundCid` field.
            #[serde(rename = "openRoundCid")]
            pub open_round_cid:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Round::OpenMiningRound>,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct LockedAmulet_Unlock {
            ///The Daml `openRoundCid` field.
            #[serde(rename = "openRoundCid")]
            pub open_round_cid:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Round::OpenMiningRound>,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Amulet_Expire {
            ///The Daml `roundCid` field.
            #[serde(rename = "roundCid")]
            pub round_cid:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Round::OpenMiningRound>,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct UnclaimedActivityRecord_DsoExpireResult {
            ///The Daml `unclaimedRewardCid` field.
            #[serde(rename = "unclaimedRewardCid")]
            pub unclaimed_reward_cid:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Amulet::UnclaimedReward>,
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
                    )?)?,
                })
            }
        }
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize,
        )]
        #[serde(crate = "rt::serde")]
        pub enum UnclaimedActivityRecord_ArchiveAsBeneficiaryResult {
            ///The Daml `UnclaimedActivityRecord_ArchiveAsBeneficiaryResult` value.
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
            ///The Daml `SvRewardCoupon_ArchiveAsBeneficiaryResult` value.
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
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SvRewardCoupon_DsoExpireResult {
            ///The Daml `weight` field.
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
                    weight: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "weight",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorRewardCoupon_ArchiveAsValidatorResult {}
        impl rt::ToValue for ValidatorRewardCoupon_ArchiveAsValidatorResult {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for ValidatorRewardCoupon_ArchiveAsValidatorResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorRewardCoupon_DsoExpireResult {
            ///The Daml `amount` field.
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
                    amount: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "amount",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AppRewardCoupon_DsoExpireResult {
            ///The Daml `featured` field.
            #[serde(rename = "featured")]
            pub featured: bool,
            ///The Daml `amount` field.
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
                    )?)?,
                    amount: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "amount",
                    )?)?,
                })
            }
        }
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize,
        )]
        #[serde(crate = "rt::serde")]
        pub enum FeaturedAppRight_CancelResult {
            ///The Daml `FeaturedAppRight_CancelResult` value.
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
            ///The Daml `FeaturedAppRight_WithdrawResult` value.
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
            ///The Daml `ValidatorRight_ArchiveAsUserResult` value.
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
            ///The Daml `ValidatorRight_ArchiveAsValidatorResult` value.
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
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct LockedAmulet_ExpireAmuletResult {
            ///The Daml `expireSum` field.
            #[serde(rename = "expireSum")]
            pub expire_sum: crate::splice_amulet_0_1_14::Splice_Amulet::AmuletExpireSummary,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta: ::core::option::Option<
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
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
                    )?)?,
                    meta: rt::optional_field(value, 1usize, "meta")?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct LockedAmulet_OwnerExpireLockResult {
            ///The Daml `amuletSum` field.
            #[serde(rename = "amuletSum")]
            pub amulet_sum: crate::splice_amulet_0_1_14::Splice_Amulet::AmuletCreateSummary<
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Amulet::Amulet>,
            >,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta: ::core::option::Option<
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
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
                    )?)?,
                    meta: rt::optional_field(value, 1usize, "meta")?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct LockedAmulet_UnlockResult {
            ///The Daml `amuletSum` field.
            #[serde(rename = "amuletSum")]
            pub amulet_sum: crate::splice_amulet_0_1_14::Splice_Amulet::AmuletCreateSummary<
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Amulet::Amulet>,
            >,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta: ::core::option::Option<
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
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
                    )?)?,
                    meta: rt::optional_field(value, 1usize, "meta")?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Amulet_ExpireResult {
            ///The Daml `expireSum` field.
            #[serde(rename = "expireSum")]
            pub expire_sum: crate::splice_amulet_0_1_14::Splice_Amulet::AmuletExpireSummary,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta: ::core::option::Option<
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
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
                    )?)?,
                    meta: rt::optional_field(value, 1usize, "meta")?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletCreateSummary<AmuletContractId> {
            ///The Daml `amulet` field.
            #[serde(rename = "amulet")]
            pub amulet: AmuletContractId,
            ///The Daml `amuletPrice` field.
            #[serde(rename = "amuletPrice")]
            pub amulet_price: rt::Numeric,
            ///The Daml `round` field.
            #[serde(rename = "round")]
            pub round: crate::splice_amulet_0_1_14::Splice_Types::Round,
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
                    amulet: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "amulet",
                    )?)?,
                    amulet_price: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "amuletPrice",
                    )?)?,
                    round: rt::FromValue::from_value(rt::required_field(value, 2usize, "round")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletExpireSummary {
            ///The Daml `owner` field.
            #[serde(rename = "owner")]
            pub owner: rt::Party,
            ///The Daml `round` field.
            #[serde(rename = "round")]
            pub round: crate::splice_amulet_0_1_14::Splice_Types::Round,
            ///The Daml `changeToInitialAmountAsOfRoundZero` field.
            #[serde(rename = "changeToInitialAmountAsOfRoundZero")]
            pub change_to_initial_amount_as_of_round_zero: rt::Numeric,
            ///The Daml `changeToHoldingFeesRate` field.
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
                    owner: rt::FromValue::from_value(rt::required_field(value, 0usize, "owner")?)?,
                    round: rt::FromValue::from_value(rt::required_field(value, 1usize, "round")?)?,
                    change_to_initial_amount_as_of_round_zero: rt::FromValue::from_value(
                        rt::required_field(value, 2usize, "changeToInitialAmountAsOfRoundZero")?,
                    )?,
                    change_to_holding_fees_rate: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "changeToHoldingFeesRate",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Amulet {
            ///The Daml `dso` field.
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///The Daml `owner` field.
            #[serde(rename = "owner")]
            pub owner: rt::Party,
            ///The Daml `amount` field.
            #[serde(rename = "amount")]
            pub amount: crate::splice_amulet_0_1_14::Splice_Fees::ExpiringAmount,
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
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)?,
                    owner: rt::FromValue::from_value(rt::required_field(value, 1usize, "owner")?)?,
                    amount: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "amount",
                    )?)?,
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
        impl rt::Template for Amulet {}
        ///The `Amulet_Expire` choice on [`Amulet`] (consuming).
        impl rt::Choice<Amulet> for crate::splice_amulet_0_1_14::Splice_Amulet::Amulet_Expire {
            type Return = crate::splice_amulet_0_1_14::Splice_Amulet::Amulet_ExpireResult;
            const NAME: &'static str = "Amulet_Expire";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`Amulet`] (consuming).
        impl rt::Choice<Amulet>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AppRewardCoupon {
            ///The Daml `dso` field.
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///The Daml `provider` field.
            #[serde(rename = "provider")]
            pub provider: rt::Party,
            ///The Daml `featured` field.
            #[serde(rename = "featured")]
            pub featured: bool,
            ///The Daml `amount` field.
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            ///The Daml `round` field.
            #[serde(rename = "round")]
            pub round: crate::splice_amulet_0_1_14::Splice_Types::Round,
            ///The Daml `beneficiary` field.
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
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)?,
                    provider: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "provider",
                    )?)?,
                    featured: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "featured",
                    )?)?,
                    amount: rt::FromValue::from_value(rt::required_field(
                        value, 3usize, "amount",
                    )?)?,
                    round: rt::FromValue::from_value(rt::required_field(value, 4usize, "round")?)?,
                    beneficiary: rt::optional_field(value, 5usize, "beneficiary")?,
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
        impl rt::Template for AppRewardCoupon {}
        ///The `AppRewardCoupon_DsoExpire` choice on [`AppRewardCoupon`] (consuming).
        impl rt::Choice<AppRewardCoupon>
            for crate::splice_amulet_0_1_14::Splice_Amulet::AppRewardCoupon_DsoExpire
        {
            type Return =
                crate::splice_amulet_0_1_14::Splice_Amulet::AppRewardCoupon_DsoExpireResult;
            const NAME: &'static str = "AppRewardCoupon_DsoExpire";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`AppRewardCoupon`] (consuming).
        impl rt::Choice<AppRewardCoupon>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct FeaturedAppActivityMarker {
            ///The Daml `dso` field.
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///The Daml `provider` field.
            #[serde(rename = "provider")]
            pub provider: rt::Party,
            ///The Daml `beneficiary` field.
            #[serde(rename = "beneficiary")]
            pub beneficiary: rt::Party,
            ///The Daml `weight` field.
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
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)?,
                    provider: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "provider",
                    )?)?,
                    beneficiary: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "beneficiary",
                    )?)?,
                    weight: rt::FromValue::from_value(rt::required_field(
                        value, 3usize, "weight",
                    )?)?,
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
        impl rt::Template for FeaturedAppActivityMarker {}
        ///The `Archive` choice on [`FeaturedAppActivityMarker`] (consuming).
        impl rt::Choice<FeaturedAppActivityMarker>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct FeaturedAppRight {
            ///The Daml `dso` field.
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///The Daml `provider` field.
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
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)?,
                    provider: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "provider",
                    )?)?,
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
        impl rt::Template for FeaturedAppRight {}
        ///The `FeaturedAppRight_Withdraw` choice on [`FeaturedAppRight`] (consuming).
        impl rt::Choice<FeaturedAppRight>
            for crate::splice_amulet_0_1_14::Splice_Amulet::FeaturedAppRight_Withdraw
        {
            type Return =
                crate::splice_amulet_0_1_14::Splice_Amulet::FeaturedAppRight_WithdrawResult;
            const NAME: &'static str = "FeaturedAppRight_Withdraw";
            const CONSUMING: bool = true;
        }
        ///The `FeaturedAppRight_Cancel` choice on [`FeaturedAppRight`] (consuming).
        impl rt::Choice<FeaturedAppRight>
            for crate::splice_amulet_0_1_14::Splice_Amulet::FeaturedAppRight_Cancel
        {
            type Return = crate::splice_amulet_0_1_14::Splice_Amulet::FeaturedAppRight_CancelResult;
            const NAME: &'static str = "FeaturedAppRight_Cancel";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`FeaturedAppRight`] (consuming).
        impl rt::Choice<FeaturedAppRight>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct LockedAmulet {
            ///The Daml `amulet` field.
            #[serde(rename = "amulet")]
            pub amulet: crate::splice_amulet_0_1_14::Splice_Amulet::Amulet,
            ///The Daml `lock` field.
            #[serde(rename = "lock")]
            pub lock: crate::splice_amulet_0_1_14::Splice_Expiry::TimeLock,
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
                    amulet: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "amulet",
                    )?)?,
                    lock: rt::FromValue::from_value(rt::required_field(value, 1usize, "lock")?)?,
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
        impl rt::Template for LockedAmulet {}
        ///The `LockedAmulet_Unlock` choice on [`LockedAmulet`] (consuming).
        impl rt::Choice<LockedAmulet> for crate::splice_amulet_0_1_14::Splice_Amulet::LockedAmulet_Unlock {
            type Return = crate::splice_amulet_0_1_14::Splice_Amulet::LockedAmulet_UnlockResult;
            const NAME: &'static str = "LockedAmulet_Unlock";
            const CONSUMING: bool = true;
        }
        ///The `LockedAmulet_OwnerExpireLock` choice on [`LockedAmulet`] (consuming).
        impl rt::Choice<LockedAmulet>
            for crate::splice_amulet_0_1_14::Splice_Amulet::LockedAmulet_OwnerExpireLock
        {
            type Return =
                crate::splice_amulet_0_1_14::Splice_Amulet::LockedAmulet_OwnerExpireLockResult;
            const NAME: &'static str = "LockedAmulet_OwnerExpireLock";
            const CONSUMING: bool = true;
        }
        ///The `LockedAmulet_ExpireAmulet` choice on [`LockedAmulet`] (consuming).
        impl rt::Choice<LockedAmulet>
            for crate::splice_amulet_0_1_14::Splice_Amulet::LockedAmulet_ExpireAmulet
        {
            type Return =
                crate::splice_amulet_0_1_14::Splice_Amulet::LockedAmulet_ExpireAmuletResult;
            const NAME: &'static str = "LockedAmulet_ExpireAmulet";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`LockedAmulet`] (consuming).
        impl rt::Choice<LockedAmulet>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SvRewardCoupon {
            ///The Daml `dso` field.
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///The Daml `sv` field.
            #[serde(rename = "sv")]
            pub sv: rt::Party,
            ///The Daml `beneficiary` field.
            #[serde(rename = "beneficiary")]
            pub beneficiary: rt::Party,
            ///The Daml `round` field.
            #[serde(rename = "round")]
            pub round: crate::splice_amulet_0_1_14::Splice_Types::Round,
            ///The Daml `weight` field.
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
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)?,
                    sv: rt::FromValue::from_value(rt::required_field(value, 1usize, "sv")?)?,
                    beneficiary: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "beneficiary",
                    )?)?,
                    round: rt::FromValue::from_value(rt::required_field(value, 3usize, "round")?)?,
                    weight: rt::FromValue::from_value(rt::required_field(
                        value, 4usize, "weight",
                    )?)?,
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
        impl rt::Template for SvRewardCoupon {}
        ///The `SvRewardCoupon_DsoExpire` choice on [`SvRewardCoupon`] (consuming).
        impl rt::Choice<SvRewardCoupon>
            for crate::splice_amulet_0_1_14::Splice_Amulet::SvRewardCoupon_DsoExpire
        {
            type Return =
                crate::splice_amulet_0_1_14::Splice_Amulet::SvRewardCoupon_DsoExpireResult;
            const NAME: &'static str = "SvRewardCoupon_DsoExpire";
            const CONSUMING: bool = true;
        }
        ///The `SvRewardCoupon_ArchiveAsBeneficiary` choice on [`SvRewardCoupon`] (consuming).
        impl rt::Choice<SvRewardCoupon>
            for crate::splice_amulet_0_1_14::Splice_Amulet::SvRewardCoupon_ArchiveAsBeneficiary
        {
            type Return = crate::splice_amulet_0_1_14::Splice_Amulet::SvRewardCoupon_ArchiveAsBeneficiaryResult;
            const NAME: &'static str = "SvRewardCoupon_ArchiveAsBeneficiary";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`SvRewardCoupon`] (consuming).
        impl rt::Choice<SvRewardCoupon>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct UnclaimedActivityRecord {
            ///The Daml `dso` field.
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///The Daml `beneficiary` field.
            #[serde(rename = "beneficiary")]
            pub beneficiary: rt::Party,
            ///The Daml `amount` field.
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            ///The Daml `reason` field.
            #[serde(rename = "reason")]
            pub reason: ::std::string::String,
            ///The Daml `expiresAt` field.
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
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)?,
                    beneficiary: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "beneficiary",
                    )?)?,
                    amount: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "amount",
                    )?)?,
                    reason: rt::FromValue::from_value(rt::required_field(
                        value, 3usize, "reason",
                    )?)?,
                    expires_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "expiresAt",
                    )?)?,
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
        impl rt::Template for UnclaimedActivityRecord {}
        ///The `UnclaimedActivityRecord_DsoExpire` choice on [`UnclaimedActivityRecord`] (consuming).
        impl rt::Choice<UnclaimedActivityRecord>
            for crate::splice_amulet_0_1_14::Splice_Amulet::UnclaimedActivityRecord_DsoExpire
        {
            type Return =
                crate::splice_amulet_0_1_14::Splice_Amulet::UnclaimedActivityRecord_DsoExpireResult;
            const NAME: &'static str = "UnclaimedActivityRecord_DsoExpire";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`UnclaimedActivityRecord`] (consuming).
        impl rt::Choice<UnclaimedActivityRecord>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct UnclaimedReward {
            ///The Daml `dso` field.
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///The Daml `amount` field.
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
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)?,
                    amount: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "amount",
                    )?)?,
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
        impl rt::Template for UnclaimedReward {}
        ///The `Archive` choice on [`UnclaimedReward`] (consuming).
        impl rt::Choice<UnclaimedReward>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorRewardCoupon {
            ///The Daml `dso` field.
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///The Daml `user` field.
            #[serde(rename = "user")]
            pub user: rt::Party,
            ///The Daml `amount` field.
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            ///The Daml `round` field.
            #[serde(rename = "round")]
            pub round: crate::splice_amulet_0_1_14::Splice_Types::Round,
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
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)?,
                    user: rt::FromValue::from_value(rt::required_field(value, 1usize, "user")?)?,
                    amount: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "amount",
                    )?)?,
                    round: rt::FromValue::from_value(rt::required_field(value, 3usize, "round")?)?,
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
        impl rt::Template for ValidatorRewardCoupon {}
        ///The `ValidatorRewardCoupon_DsoExpire` choice on [`ValidatorRewardCoupon`] (consuming).
        impl rt::Choice<ValidatorRewardCoupon>
            for crate::splice_amulet_0_1_14::Splice_Amulet::ValidatorRewardCoupon_DsoExpire
        {
            type Return =
                crate::splice_amulet_0_1_14::Splice_Amulet::ValidatorRewardCoupon_DsoExpireResult;
            const NAME: &'static str = "ValidatorRewardCoupon_DsoExpire";
            const CONSUMING: bool = true;
        }
        ///The `ValidatorRewardCoupon_ArchiveAsValidator` choice on [`ValidatorRewardCoupon`] (consuming).
        impl rt::Choice<ValidatorRewardCoupon>
            for crate::splice_amulet_0_1_14::Splice_Amulet::ValidatorRewardCoupon_ArchiveAsValidator
        {
            type Return = crate::splice_amulet_0_1_14::Splice_Amulet::ValidatorRewardCoupon_ArchiveAsValidatorResult;
            const NAME: &'static str = "ValidatorRewardCoupon_ArchiveAsValidator";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`ValidatorRewardCoupon`] (consuming).
        impl rt::Choice<ValidatorRewardCoupon>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorRight {
            ///The Daml `dso` field.
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///The Daml `user` field.
            #[serde(rename = "user")]
            pub user: rt::Party,
            ///The Daml `validator` field.
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
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)?,
                    user: rt::FromValue::from_value(rt::required_field(value, 1usize, "user")?)?,
                    validator: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "validator",
                    )?)?,
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
        impl rt::Template for ValidatorRight {}
        ///The `ValidatorRight_ArchiveAsValidator` choice on [`ValidatorRight`] (consuming).
        impl rt::Choice<ValidatorRight>
            for crate::splice_amulet_0_1_14::Splice_Amulet::ValidatorRight_ArchiveAsValidator
        {
            type Return =
                crate::splice_amulet_0_1_14::Splice_Amulet::ValidatorRight_ArchiveAsValidatorResult;
            const NAME: &'static str = "ValidatorRight_ArchiveAsValidator";
            const CONSUMING: bool = true;
        }
        ///The `ValidatorRight_ArchiveAsUser` choice on [`ValidatorRight`] (consuming).
        impl rt::Choice<ValidatorRight>
            for crate::splice_amulet_0_1_14::Splice_Amulet::ValidatorRight_ArchiveAsUser
        {
            type Return =
                crate::splice_amulet_0_1_14::Splice_Amulet::ValidatorRight_ArchiveAsUserResult;
            const NAME: &'static str = "ValidatorRight_ArchiveAsUser";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`ValidatorRight`] (consuming).
        impl rt::Choice<ValidatorRight>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
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
            ///The Daml `TxKind_Transfer` value.
            #[serde(rename = "TxKind_Transfer")]
            TxKind_Transfer,
            ///The Daml `TxKind_Unlock` value.
            #[serde(rename = "TxKind_Unlock")]
            TxKind_Unlock,
            ///The Daml `TxKind_MergeSplit` value.
            #[serde(rename = "TxKind_MergeSplit")]
            TxKind_MergeSplit,
            ///The Daml `TxKind_Burn` value.
            #[serde(rename = "TxKind_Burn")]
            TxKind_Burn,
            ///The Daml `TxKind_Mint` value.
            #[serde(rename = "TxKind_Mint")]
            TxKind_Mint,
            ///The Daml `TxKind_ExpireDust` value.
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
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletTransferInstruction {
            ///The Daml `lockedAmulet` field.
            #[serde(rename = "lockedAmulet")]
            pub locked_amulet: rt::ContractId<
                crate::splice_amulet_0_1_14::Splice_Amulet::LockedAmulet,
            >,
            ///The Daml `transfer` field.
            #[serde(rename = "transfer")]
            pub transfer: crate::splice_api_token_transfer_instruction_v1_1_0_0::Splice_Api_Token_TransferInstructionV1::Transfer,
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
                    )?)?,
                    transfer: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "transfer",
                    )?)?,
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
        impl rt::Template for AmuletTransferInstruction {}
        ///The `Archive` choice on [`AmuletTransferInstruction`] (consuming).
        impl rt::Choice<AmuletTransferInstruction>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
    }
    pub mod Splice_AmuletConfig {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct PackageConfig {
            ///The Daml `amulet` field.
            #[serde(rename = "amulet")]
            pub amulet: ::std::string::String,
            ///The Daml `amuletNameService` field.
            #[serde(rename = "amuletNameService")]
            pub amulet_name_service: ::std::string::String,
            ///The Daml `dsoGovernance` field.
            #[serde(rename = "dsoGovernance")]
            pub dso_governance: ::std::string::String,
            ///The Daml `validatorLifecycle` field.
            #[serde(rename = "validatorLifecycle")]
            pub validator_lifecycle: ::std::string::String,
            ///The Daml `wallet` field.
            #[serde(rename = "wallet")]
            pub wallet: ::std::string::String,
            ///The Daml `walletPayments` field.
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
                    amulet: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "amulet",
                    )?)?,
                    amulet_name_service: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "amuletNameService",
                    )?)?,
                    dso_governance: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "dsoGovernance",
                    )?)?,
                    validator_lifecycle: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "validatorLifecycle",
                    )?)?,
                    wallet: rt::FromValue::from_value(rt::required_field(
                        value, 4usize, "wallet",
                    )?)?,
                    wallet_payments: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "walletPayments",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletConfig<Unit> {
            ///The Daml `transferConfig` field.
            #[serde(rename = "transferConfig")]
            pub transfer_config: crate::splice_amulet_0_1_14::Splice_AmuletConfig::TransferConfig<
                Unit,
            >,
            ///The Daml `issuanceCurve` field.
            #[serde(rename = "issuanceCurve")]
            pub issuance_curve: crate::splice_amulet_0_1_14::Splice_Schedule::Schedule<
                crate::daml_stdlib_DA_Time_Types_1_0_0::DA_Time_Types::RelTime,
                crate::splice_amulet_0_1_14::Splice_Issuance::IssuanceConfig,
            >,
            ///The Daml `decentralizedSynchronizer` field.
            #[serde(rename = "decentralizedSynchronizer")]
            pub decentralized_synchronizer: crate::splice_amulet_0_1_14::Splice_DecentralizedSynchronizer::AmuletDecentralizedSynchronizerConfig,
            ///The Daml `tickDuration` field.
            #[serde(rename = "tickDuration")]
            pub tick_duration: crate::daml_stdlib_DA_Time_Types_1_0_0::DA_Time_Types::RelTime,
            ///The Daml `packageConfig` field.
            #[serde(rename = "packageConfig")]
            pub package_config: crate::splice_amulet_0_1_14::Splice_AmuletConfig::PackageConfig,
            ///The Daml `transferPreapprovalFee` field.
            #[serde(rename = "transferPreapprovalFee")]
            pub transfer_preapproval_fee: ::core::option::Option<rt::Numeric>,
            ///The Daml `featuredAppActivityMarkerAmount` field.
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
                    )?)?,
                    issuance_curve: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "issuanceCurve",
                    )?)?,
                    decentralized_synchronizer: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "decentralizedSynchronizer",
                    )?)?,
                    tick_duration: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "tickDuration",
                    )?)?,
                    package_config: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "packageConfig",
                    )?)?,
                    transfer_preapproval_fee: rt::optional_field(
                        value,
                        5usize,
                        "transferPreapprovalFee",
                    )?,
                    featured_app_activity_marker_amount: rt::optional_field(
                        value,
                        6usize,
                        "featuredAppActivityMarkerAmount",
                    )?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferConfig<Unit> {
            ///The Daml `createFee` field.
            #[serde(rename = "createFee")]
            pub create_fee: crate::splice_amulet_0_1_14::Splice_Fees::FixedFee,
            ///The Daml `holdingFee` field.
            #[serde(rename = "holdingFee")]
            pub holding_fee: crate::splice_amulet_0_1_14::Splice_Fees::RatePerRound,
            ///The Daml `transferFee` field.
            #[serde(rename = "transferFee")]
            pub transfer_fee: crate::splice_amulet_0_1_14::Splice_Fees::SteppedRate,
            ///The Daml `lockHolderFee` field.
            #[serde(rename = "lockHolderFee")]
            pub lock_holder_fee: crate::splice_amulet_0_1_14::Splice_Fees::FixedFee,
            ///The Daml `extraFeaturedAppRewardAmount` field.
            #[serde(rename = "extraFeaturedAppRewardAmount")]
            pub extra_featured_app_reward_amount: rt::Numeric,
            ///The Daml `maxNumInputs` field.
            #[serde(rename = "maxNumInputs")]
            pub max_num_inputs: rt::Int64,
            ///The Daml `maxNumOutputs` field.
            #[serde(rename = "maxNumOutputs")]
            pub max_num_outputs: rt::Int64,
            ///The Daml `maxNumLockHolders` field.
            #[serde(rename = "maxNumLockHolders")]
            pub max_num_lock_holders: rt::Int64,
            #[doc(hidden)]
            #[serde(skip)]
            pub _phantom: ::core::marker::PhantomData<(Unit,)>,
        }
        impl<Unit> rt::ToValue for TransferConfig<Unit>
        where
            Unit: rt::ToValue,
        {
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
        impl<Unit> rt::FromValue for TransferConfig<Unit>
        where
            Unit: rt::FromValue,
        {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    create_fee: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "createFee",
                    )?)?,
                    holding_fee: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "holdingFee",
                    )?)?,
                    transfer_fee: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "transferFee",
                    )?)?,
                    lock_holder_fee: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "lockHolderFee",
                    )?)?,
                    extra_featured_app_reward_amount: rt::FromValue::from_value(
                        rt::required_field(value, 4usize, "extraFeaturedAppRewardAmount")?,
                    )?,
                    max_num_inputs: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "maxNumInputs",
                    )?)?,
                    max_num_outputs: rt::FromValue::from_value(rt::required_field(
                        value,
                        6usize,
                        "maxNumOutputs",
                    )?)?,
                    max_num_lock_holders: rt::FromValue::from_value(rt::required_field(
                        value,
                        7usize,
                        "maxNumLockHolders",
                    )?)?,
                    _phantom: ::core::marker::PhantomData,
                })
            }
        }
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize,
        )]
        #[serde(crate = "rt::serde")]
        pub enum USD {
            ///The Daml `USD` value.
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
            ///The Daml `Amulet` value.
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
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferCommand_ExpireResult {
            ///The Daml `sender` field.
            #[serde(rename = "sender")]
            pub sender: rt::Party,
            ///The Daml `nonce` field.
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
                    sender: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "sender",
                    )?)?,
                    nonce: rt::FromValue::from_value(rt::required_field(value, 1usize, "nonce")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferCommand_WithdrawResult {
            ///The Daml `sender` field.
            #[serde(rename = "sender")]
            pub sender: rt::Party,
            ///The Daml `nonce` field.
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
                    sender: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "sender",
                    )?)?,
                    nonce: rt::FromValue::from_value(rt::required_field(value, 1usize, "nonce")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum TransferCommandResult {
            ///The Daml `TransferCommandResultFailure` constructor.
            #[serde(rename = "TransferCommandResultFailure")]
            TransferCommandResultFailure(
                crate::splice_amulet_0_1_14::Splice_ExternalPartyAmuletRules::TransferCommandResult_TransferCommandResultFailure,
            ),
            ///The Daml `TransferCommandResultSuccess` constructor.
            #[serde(rename = "TransferCommandResultSuccess")]
            TransferCommandResultSuccess(
                crate::splice_amulet_0_1_14::Splice_ExternalPartyAmuletRules::TransferCommandResult_TransferCommandResultSuccess,
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
                            rt::FromValue::from_value(payload)?,
                        ),
                    ),
                    "TransferCommandResultSuccess" => ::core::result::Result::Ok(
                        TransferCommandResult::TransferCommandResultSuccess(
                            rt::FromValue::from_value(payload)?,
                        ),
                    ),
                    other => ::core::result::Result::Err(rt::unexpected_constructor(
                        "TransferCommandResult",
                        other,
                    )),
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferCommandResult_TransferCommandResultFailure {
            ///The Daml `reason` field.
            #[serde(rename = "reason")]
            pub reason: crate::splice_amulet_0_1_14::Splice_AmuletRules::InvalidTransferReason,
        }
        impl rt::ToValue for TransferCommandResult_TransferCommandResultFailure {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("reason", rt::ToValue::to_value(&self.reason)),])
            }
        }
        impl rt::FromValue for TransferCommandResult_TransferCommandResultFailure {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    reason: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "reason",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferCommandResult_TransferCommandResultSuccess {
            ///The Daml `result` field.
            #[serde(rename = "result")]
            pub result: crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferResult,
        }
        impl rt::ToValue for TransferCommandResult_TransferCommandResultSuccess {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![("result", rt::ToValue::to_value(&self.result)),])
            }
        }
        impl rt::FromValue for TransferCommandResult_TransferCommandResultSuccess {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    result: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "result",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferCommand_SendResult {
            ///The Daml `result` field.
            #[serde(rename = "result")]
            pub result:
                crate::splice_amulet_0_1_14::Splice_ExternalPartyAmuletRules::TransferCommandResult,
            ///The Daml `sender` field.
            #[serde(rename = "sender")]
            pub sender: rt::Party,
            ///The Daml `nonce` field.
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
                    result: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "result",
                    )?)?,
                    sender: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "sender",
                    )?)?,
                    nonce: rt::FromValue::from_value(rt::required_field(value, 2usize, "nonce")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferCommand_Expire {
            ///The Daml `p` field.
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
                    p: rt::FromValue::from_value(rt::required_field(value, 0usize, "p")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferCommand_Withdraw {}
        impl rt::ToValue for TransferCommand_Withdraw {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for TransferCommand_Withdraw {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferCommand_Send {
            ///The Daml `context` field.
            #[serde(rename = "context")]
            pub context: crate::splice_amulet_0_1_14::Splice_AmuletRules::PaymentTransferContext,
            ///The Daml `inputs` field.
            #[serde(rename = "inputs")]
            pub inputs: ::std::vec::Vec<
                crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferInput,
            >,
            ///The Daml `transferPreapprovalCidO` field.
            #[serde(rename = "transferPreapprovalCidO")]
            pub transfer_preapproval_cid_o: ::core::option::Option<
                rt::ContractId<
                    crate::splice_amulet_0_1_14::Splice_AmuletRules::TransferPreapproval,
                >,
            >,
            ///The Daml `transferCounterCid` field.
            #[serde(rename = "transferCounterCid")]
            pub transfer_counter_cid: rt::ContractId<
                crate::splice_amulet_0_1_14::Splice_ExternalPartyAmuletRules::TransferCommandCounter,
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
                    )?)?,
                    inputs: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "inputs",
                    )?)?,
                    transfer_preapproval_cid_o: rt::optional_field(
                        value,
                        2usize,
                        "transferPreapprovalCidO",
                    )?,
                    transfer_counter_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "transferCounterCid",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ExternalPartyAmuletRules_CreateTransferCommandResult {
            ///The Daml `transferCommandCid` field.
            #[serde(rename = "transferCommandCid")]
            pub transfer_command_cid: rt::ContractId<
                crate::splice_amulet_0_1_14::Splice_ExternalPartyAmuletRules::TransferCommand,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ExternalPartyAmuletRules_CreateTransferCommand {
            ///The Daml `sender` field.
            #[serde(rename = "sender")]
            pub sender: rt::Party,
            ///The Daml `receiver` field.
            #[serde(rename = "receiver")]
            pub receiver: rt::Party,
            ///The Daml `delegate` field.
            #[serde(rename = "delegate")]
            pub delegate: rt::Party,
            ///The Daml `amount` field.
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            ///The Daml `expiresAt` field.
            #[serde(rename = "expiresAt")]
            pub expires_at: rt::Timestamp,
            ///The Daml `nonce` field.
            #[serde(rename = "nonce")]
            pub nonce: rt::Int64,
            ///The Daml `description` field.
            #[serde(rename = "description")]
            pub description: ::core::option::Option<::std::string::String>,
            ///The Daml `expectedDso` field.
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
                    sender: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "sender",
                    )?)?,
                    receiver: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "receiver",
                    )?)?,
                    delegate: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "delegate",
                    )?)?,
                    amount: rt::FromValue::from_value(rt::required_field(
                        value, 3usize, "amount",
                    )?)?,
                    expires_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "expiresAt",
                    )?)?,
                    nonce: rt::FromValue::from_value(rt::required_field(value, 5usize, "nonce")?)?,
                    description: rt::optional_field(value, 6usize, "description")?,
                    expected_dso: rt::optional_field(value, 7usize, "expectedDso")?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ExternalPartyAmuletRules {
            ///The Daml `dso` field.
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
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)?,
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
        impl rt::Template for ExternalPartyAmuletRules {}
        ///The `ExternalPartyAmuletRules_CreateTransferCommand` choice on [`ExternalPartyAmuletRules`] (non-consuming).
        impl rt::Choice<ExternalPartyAmuletRules>
        for crate::splice_amulet_0_1_14::Splice_ExternalPartyAmuletRules::ExternalPartyAmuletRules_CreateTransferCommand {
            type Return = crate::splice_amulet_0_1_14::Splice_ExternalPartyAmuletRules::ExternalPartyAmuletRules_CreateTransferCommandResult;
            const NAME: &'static str = "ExternalPartyAmuletRules_CreateTransferCommand";
            const CONSUMING: bool = false;
        }
        ///The `Archive` choice on [`ExternalPartyAmuletRules`] (consuming).
        impl rt::Choice<ExternalPartyAmuletRules>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferCommand {
            ///The Daml `dso` field.
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///The Daml `sender` field.
            #[serde(rename = "sender")]
            pub sender: rt::Party,
            ///The Daml `receiver` field.
            #[serde(rename = "receiver")]
            pub receiver: rt::Party,
            ///The Daml `delegate` field.
            #[serde(rename = "delegate")]
            pub delegate: rt::Party,
            ///The Daml `amount` field.
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            ///The Daml `expiresAt` field.
            #[serde(rename = "expiresAt")]
            pub expires_at: rt::Timestamp,
            ///The Daml `nonce` field.
            #[serde(rename = "nonce")]
            pub nonce: rt::Int64,
            ///The Daml `description` field.
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
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)?,
                    sender: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "sender",
                    )?)?,
                    receiver: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "receiver",
                    )?)?,
                    delegate: rt::FromValue::from_value(rt::required_field(
                        value, 3usize, "delegate",
                    )?)?,
                    amount: rt::FromValue::from_value(rt::required_field(
                        value, 4usize, "amount",
                    )?)?,
                    expires_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "expiresAt",
                    )?)?,
                    nonce: rt::FromValue::from_value(rt::required_field(value, 6usize, "nonce")?)?,
                    description: rt::optional_field(value, 7usize, "description")?,
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
        impl rt::Template for TransferCommand {}
        ///The `TransferCommand_Expire` choice on [`TransferCommand`] (consuming).
        impl rt::Choice<TransferCommand>
            for crate::splice_amulet_0_1_14::Splice_ExternalPartyAmuletRules::TransferCommand_Expire
        {
            type Return = crate::splice_amulet_0_1_14::Splice_ExternalPartyAmuletRules::TransferCommand_ExpireResult;
            const NAME: &'static str = "TransferCommand_Expire";
            const CONSUMING: bool = true;
        }
        ///The `TransferCommand_Send` choice on [`TransferCommand`] (consuming).
        impl rt::Choice<TransferCommand>
            for crate::splice_amulet_0_1_14::Splice_ExternalPartyAmuletRules::TransferCommand_Send
        {
            type Return = crate::splice_amulet_0_1_14::Splice_ExternalPartyAmuletRules::TransferCommand_SendResult;
            const NAME: &'static str = "TransferCommand_Send";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`TransferCommand`] (consuming).
        impl rt::Choice<TransferCommand>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `TransferCommand_Withdraw` choice on [`TransferCommand`] (consuming).
        impl rt::Choice<TransferCommand>
        for crate::splice_amulet_0_1_14::Splice_ExternalPartyAmuletRules::TransferCommand_Withdraw {
            type Return = crate::splice_amulet_0_1_14::Splice_ExternalPartyAmuletRules::TransferCommand_WithdrawResult;
            const NAME: &'static str = "TransferCommand_Withdraw";
            const CONSUMING: bool = true;
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferCommandCounter {
            ///The Daml `dso` field.
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///The Daml `sender` field.
            #[serde(rename = "sender")]
            pub sender: rt::Party,
            ///The Daml `nextNonce` field.
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
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)?,
                    sender: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "sender",
                    )?)?,
                    next_nonce: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "nextNonce",
                    )?)?,
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
        impl rt::Template for TransferCommandCounter {}
        ///The `Archive` choice on [`TransferCommandCounter`] (consuming).
        impl rt::Choice<TransferCommandCounter>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
    }
    pub mod Splice_Schedule {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Schedule<T, A> {
            ///The Daml `initialValue` field.
            #[serde(rename = "initialValue")]
            pub initial_value: A,
            ///The Daml `futureValues` field.
            #[serde(rename = "futureValues")]
            pub future_values:
                ::std::vec::Vec<crate::daml_prim_DA_Types_1_0_0::DA_Types::Tuple2<T, A>>,
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
                    )?)?,
                    future_values: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "futureValues",
                    )?)?,
                })
            }
        }
    }
    pub mod Splice_Fees {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct RatePerDay {
            ///The Daml `rate` field.
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
                    rate: rt::FromValue::from_value(rt::required_field(value, 0usize, "rate")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ExpiringAmount {
            ///The Daml `initialAmount` field.
            #[serde(rename = "initialAmount")]
            pub initial_amount: rt::Numeric,
            ///The Daml `createdAt` field.
            #[serde(rename = "createdAt")]
            pub created_at: crate::splice_amulet_0_1_14::Splice_Types::Round,
            ///The Daml `ratePerRound` field.
            #[serde(rename = "ratePerRound")]
            pub rate_per_round: crate::splice_amulet_0_1_14::Splice_Fees::RatePerRound,
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
                    )?)?,
                    created_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "createdAt",
                    )?)?,
                    rate_per_round: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "ratePerRound",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SteppedRate {
            ///The Daml `initialRate` field.
            #[serde(rename = "initialRate")]
            pub initial_rate: rt::Numeric,
            ///The Daml `steps` field.
            #[serde(rename = "steps")]
            pub steps: ::std::vec::Vec<
                crate::daml_prim_DA_Types_1_0_0::DA_Types::Tuple2<rt::Numeric, rt::Numeric>,
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
                    )?)?,
                    steps: rt::FromValue::from_value(rt::required_field(value, 1usize, "steps")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct FixedFee {
            ///The Daml `fee` field.
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
                    fee: rt::FromValue::from_value(rt::required_field(value, 0usize, "fee")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct RatePerRound {
            ///The Daml `rate` field.
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
                    rate: rt::FromValue::from_value(rt::required_field(value, 0usize, "rate")?)?,
                })
            }
        }
    }
    pub mod Splice_Expiry {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TimeLock {
            ///The Daml `holders` field.
            #[serde(rename = "holders")]
            pub holders: ::std::vec::Vec<rt::Party>,
            ///The Daml `expiresAt` field.
            #[serde(rename = "expiresAt")]
            pub expires_at: rt::Timestamp,
            ///The Daml `optContext` field.
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
                    )?)?,
                    expires_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "expiresAt",
                    )?)?,
                    opt_context: rt::optional_field(value, 2usize, "optContext")?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum BoundedSet<A> {
            ///The Daml `Singleton` constructor.
            #[serde(rename = "Singleton")]
            Singleton(A),
            ///The Daml `AfterMaxBound` constructor.
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
                        rt::FromValue::from_value(payload)?,
                    )),
                    "AfterMaxBound" => ::core::result::Result::Ok(BoundedSet::AfterMaxBound(
                        rt::FromValue::from_value(payload)?,
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
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletAllocation {
            ///The Daml `lockedAmulet` field.
            #[serde(rename = "lockedAmulet")]
            pub locked_amulet: rt::ContractId<
                crate::splice_amulet_0_1_14::Splice_Amulet::LockedAmulet,
            >,
            ///The Daml `allocation` field.
            #[serde(rename = "allocation")]
            pub allocation: crate::splice_api_token_allocation_v1_1_0_0::Splice_Api_Token_AllocationV1::AllocationSpecification,
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
                    )?)?,
                    allocation: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "allocation",
                    )?)?,
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
        impl rt::Template for AmuletAllocation {}
        ///The `Archive` choice on [`AmuletAllocation`] (consuming).
        impl rt::Choice<AmuletAllocation>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
    }
    pub mod Splice_Issuance {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct IssuanceTranche {
            ///The Daml `rewardsToIssue` field.
            #[serde(rename = "rewardsToIssue")]
            pub rewards_to_issue: rt::Numeric,
            ///The Daml `issuancePerCoupon` field.
            #[serde(rename = "issuancePerCoupon")]
            pub issuance_per_coupon: rt::Numeric,
            ///The Daml `unclaimedRewards` field.
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
                    )?)?,
                    issuance_per_coupon: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "issuancePerCoupon",
                    )?)?,
                    unclaimed_rewards: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "unclaimedRewards",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct IssuingRoundParameters {
            ///The Daml `issuancePerValidatorRewardCoupon` field.
            #[serde(rename = "issuancePerValidatorRewardCoupon")]
            pub issuance_per_validator_reward_coupon: rt::Numeric,
            ///The Daml `issuancePerFeaturedAppRewardCoupon` field.
            #[serde(rename = "issuancePerFeaturedAppRewardCoupon")]
            pub issuance_per_featured_app_reward_coupon: rt::Numeric,
            ///The Daml `issuancePerUnfeaturedAppRewardCoupon` field.
            #[serde(rename = "issuancePerUnfeaturedAppRewardCoupon")]
            pub issuance_per_unfeatured_app_reward_coupon: rt::Numeric,
            ///The Daml `issuancePerSvRewardCoupon` field.
            #[serde(rename = "issuancePerSvRewardCoupon")]
            pub issuance_per_sv_reward_coupon: rt::Numeric,
            ///The Daml `unclaimedAppRewards` field.
            #[serde(rename = "unclaimedAppRewards")]
            pub unclaimed_app_rewards: rt::Numeric,
            ///The Daml `unclaimedValidatorRewards` field.
            #[serde(rename = "unclaimedValidatorRewards")]
            pub unclaimed_validator_rewards: rt::Numeric,
            ///The Daml `unclaimedSvRewards` field.
            #[serde(rename = "unclaimedSvRewards")]
            pub unclaimed_sv_rewards: rt::Numeric,
            ///The Daml `issuancePerValidatorFaucetCoupon` field.
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
                    )?,
                    issuance_per_featured_app_reward_coupon: rt::FromValue::from_value(
                        rt::required_field(value, 1usize, "issuancePerFeaturedAppRewardCoupon")?,
                    )?,
                    issuance_per_unfeatured_app_reward_coupon: rt::FromValue::from_value(
                        rt::required_field(value, 2usize, "issuancePerUnfeaturedAppRewardCoupon")?,
                    )?,
                    issuance_per_sv_reward_coupon: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "issuancePerSvRewardCoupon",
                    )?)?,
                    unclaimed_app_rewards: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "unclaimedAppRewards",
                    )?)?,
                    unclaimed_validator_rewards: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "unclaimedValidatorRewards",
                    )?)?,
                    unclaimed_sv_rewards: rt::FromValue::from_value(rt::required_field(
                        value,
                        6usize,
                        "unclaimedSvRewards",
                    )?)?,
                    issuance_per_validator_faucet_coupon: rt::FromValue::from_value(
                        rt::required_field(value, 7usize, "issuancePerValidatorFaucetCoupon")?,
                    )?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct OpenMiningRoundSummary {
            ///The Daml `totalValidatorRewardCoupons` field.
            #[serde(rename = "totalValidatorRewardCoupons")]
            pub total_validator_reward_coupons: rt::Numeric,
            ///The Daml `totalFeaturedAppRewardCoupons` field.
            #[serde(rename = "totalFeaturedAppRewardCoupons")]
            pub total_featured_app_reward_coupons: rt::Numeric,
            ///The Daml `totalUnfeaturedAppRewardCoupons` field.
            #[serde(rename = "totalUnfeaturedAppRewardCoupons")]
            pub total_unfeatured_app_reward_coupons: rt::Numeric,
            ///The Daml `totalSvRewardWeight` field.
            #[serde(rename = "totalSvRewardWeight")]
            pub total_sv_reward_weight: rt::Int64,
            ///The Daml `optTotalValidatorFaucetCoupons` field.
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
                    )?)?,
                    total_featured_app_reward_coupons: rt::FromValue::from_value(
                        rt::required_field(value, 1usize, "totalFeaturedAppRewardCoupons")?,
                    )?,
                    total_unfeatured_app_reward_coupons: rt::FromValue::from_value(
                        rt::required_field(value, 2usize, "totalUnfeaturedAppRewardCoupons")?,
                    )?,
                    total_sv_reward_weight: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "totalSvRewardWeight",
                    )?)?,
                    opt_total_validator_faucet_coupons: rt::optional_field(
                        value,
                        4usize,
                        "optTotalValidatorFaucetCoupons",
                    )?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct IssuanceConfig {
            ///The Daml `amuletToIssuePerYear` field.
            #[serde(rename = "amuletToIssuePerYear")]
            pub amulet_to_issue_per_year: rt::Numeric,
            ///The Daml `validatorRewardPercentage` field.
            #[serde(rename = "validatorRewardPercentage")]
            pub validator_reward_percentage: rt::Numeric,
            ///The Daml `appRewardPercentage` field.
            #[serde(rename = "appRewardPercentage")]
            pub app_reward_percentage: rt::Numeric,
            ///The Daml `validatorRewardCap` field.
            #[serde(rename = "validatorRewardCap")]
            pub validator_reward_cap: rt::Numeric,
            ///The Daml `featuredAppRewardCap` field.
            #[serde(rename = "featuredAppRewardCap")]
            pub featured_app_reward_cap: rt::Numeric,
            ///The Daml `unfeaturedAppRewardCap` field.
            #[serde(rename = "unfeaturedAppRewardCap")]
            pub unfeatured_app_reward_cap: rt::Numeric,
            ///The Daml `optValidatorFaucetCap` field.
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
                    )?)?,
                    validator_reward_percentage: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "validatorRewardPercentage",
                    )?)?,
                    app_reward_percentage: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "appRewardPercentage",
                    )?)?,
                    validator_reward_cap: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "validatorRewardCap",
                    )?)?,
                    featured_app_reward_cap: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "featuredAppRewardCap",
                    )?)?,
                    unfeatured_app_reward_cap: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "unfeaturedAppRewardCap",
                    )?)?,
                    opt_validator_faucet_cap: rt::optional_field(
                        value,
                        6usize,
                        "optValidatorFaucetCap",
                    )?,
                })
            }
        }
    }
    pub mod Splice_ValidatorLicense {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorLivenessActivityRecord_DsoExpire {
            ///The Daml `closedRoundCid` field.
            #[serde(rename = "closedRoundCid")]
            pub closed_round_cid:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Round::ClosedMiningRound>,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorFaucetCoupon_DsoExpire {
            ///The Daml `closedRoundCid` field.
            #[serde(rename = "closedRoundCid")]
            pub closed_round_cid:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Round::ClosedMiningRound>,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorLicense_ReportActive {}
        impl rt::ToValue for ValidatorLicense_ReportActive {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![])
            }
        }
        impl rt::FromValue for ValidatorLicense_ReportActive {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {})
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorLicense_UpdateMetadata {
            ///The Daml `version` field.
            #[serde(rename = "version")]
            pub version: ::std::string::String,
            ///The Daml `contactPoint` field.
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
                    )?)?,
                    contact_point: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "contactPoint",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorLicense_Cancel {
            ///The Daml `reason` field.
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
                    reason: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "reason",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorLicense_Withdraw {
            ///The Daml `reason` field.
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
                    reason: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "reason",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorLicense_RecordValidatorLivenessActivity {
            ///The Daml `openRoundCid` field.
            #[serde(rename = "openRoundCid")]
            pub open_round_cid:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Round::OpenMiningRound>,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorLicense_ReceiveFaucetCoupon {
            ///The Daml `openRoundCid` field.
            #[serde(rename = "openRoundCid")]
            pub open_round_cid:
                rt::ContractId<crate::splice_amulet_0_1_14::Splice_Round::OpenMiningRound>,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorLicenseMetadata {
            ///The Daml `lastUpdatedAt` field.
            #[serde(rename = "lastUpdatedAt")]
            pub last_updated_at: rt::Timestamp,
            ///The Daml `version` field.
            #[serde(rename = "version")]
            pub version: ::std::string::String,
            ///The Daml `contactPoint` field.
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
                    )?)?,
                    version: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "version",
                    )?)?,
                    contact_point: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "contactPoint",
                    )?)?,
                })
            }
        }
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize,
        )]
        #[serde(crate = "rt::serde")]
        pub enum ValidatorLivenessActivityRecord_DsoExpireResult {
            ///The Daml `ValidatorLivenessActivityRecord_DsoExpireResult` value.
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
            ///The Daml `ValidatorFaucetCoupon_DsoExpireResult` value.
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
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorLicense_ReportActiveResult {
            ///The Daml `licenseCid` field.
            #[serde(rename = "licenseCid")]
            pub license_cid: rt::ContractId<
                crate::splice_amulet_0_1_14::Splice_ValidatorLicense::ValidatorLicense,
            >,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorLicense_UpdateMetadataResult {
            ///The Daml `licenseCid` field.
            #[serde(rename = "licenseCid")]
            pub license_cid: rt::ContractId<
                crate::splice_amulet_0_1_14::Splice_ValidatorLicense::ValidatorLicense,
            >,
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
                    )?)?,
                })
            }
        }
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, rt::serde::Serialize, rt::serde::Deserialize,
        )]
        #[serde(crate = "rt::serde")]
        pub enum ValidatorLicense_CancelResult {
            ///The Daml `ValidatorLicense_CancelResult` value.
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
            ///The Daml `ValidatorLicense_WithdrawResult` value.
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
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorLicense_RecordValidatorLivenessActivityResult {
            ///The Daml `licenseCid` field.
            #[serde(rename = "licenseCid")]
            pub license_cid: rt::ContractId<
                crate::splice_amulet_0_1_14::Splice_ValidatorLicense::ValidatorLicense,
            >,
            ///The Daml `couponCid` field.
            #[serde(rename = "couponCid")]
            pub coupon_cid: rt::ContractId<
                crate::splice_amulet_0_1_14::Splice_ValidatorLicense::ValidatorLivenessActivityRecord,
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
                    )?)?,
                    coupon_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "couponCid",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorLicense_ReceiveFaucetCouponResult {
            ///The Daml `licenseCid` field.
            #[serde(rename = "licenseCid")]
            pub license_cid: rt::ContractId<
                crate::splice_amulet_0_1_14::Splice_ValidatorLicense::ValidatorLicense,
            >,
            ///The Daml `couponCid` field.
            #[serde(rename = "couponCid")]
            pub coupon_cid: rt::ContractId<
                crate::splice_amulet_0_1_14::Splice_ValidatorLicense::ValidatorFaucetCoupon,
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
                    )?)?,
                    coupon_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "couponCid",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct FaucetState {
            ///The Daml `firstReceivedFor` field.
            #[serde(rename = "firstReceivedFor")]
            pub first_received_for: crate::splice_amulet_0_1_14::Splice_Types::Round,
            ///The Daml `lastReceivedFor` field.
            #[serde(rename = "lastReceivedFor")]
            pub last_received_for: crate::splice_amulet_0_1_14::Splice_Types::Round,
            ///The Daml `numCouponsMissed` field.
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
                    )?)?,
                    last_received_for: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "lastReceivedFor",
                    )?)?,
                    num_coupons_missed: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "numCouponsMissed",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorFaucetCoupon {
            ///The Daml `dso` field.
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///The Daml `validator` field.
            #[serde(rename = "validator")]
            pub validator: rt::Party,
            ///The Daml `round` field.
            #[serde(rename = "round")]
            pub round: crate::splice_amulet_0_1_14::Splice_Types::Round,
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
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)?,
                    validator: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "validator",
                    )?)?,
                    round: rt::FromValue::from_value(rt::required_field(value, 2usize, "round")?)?,
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
        impl rt::Template for ValidatorFaucetCoupon {}
        ///The `ValidatorFaucetCoupon_DsoExpire` choice on [`ValidatorFaucetCoupon`] (consuming).
        impl rt::Choice<ValidatorFaucetCoupon>
        for crate::splice_amulet_0_1_14::Splice_ValidatorLicense::ValidatorFaucetCoupon_DsoExpire {
            type Return = crate::splice_amulet_0_1_14::Splice_ValidatorLicense::ValidatorFaucetCoupon_DsoExpireResult;
            const NAME: &'static str = "ValidatorFaucetCoupon_DsoExpire";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`ValidatorFaucetCoupon`] (consuming).
        impl rt::Choice<ValidatorFaucetCoupon>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorLicense {
            ///The Daml `validator` field.
            #[serde(rename = "validator")]
            pub validator: rt::Party,
            ///The Daml `sponsor` field.
            #[serde(rename = "sponsor")]
            pub sponsor: rt::Party,
            ///The Daml `dso` field.
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///The Daml `faucetState` field.
            #[serde(rename = "faucetState")]
            pub faucet_state: ::core::option::Option<
                crate::splice_amulet_0_1_14::Splice_ValidatorLicense::FaucetState,
            >,
            ///The Daml `metadata` field.
            #[serde(rename = "metadata")]
            pub metadata: ::core::option::Option<
                crate::splice_amulet_0_1_14::Splice_ValidatorLicense::ValidatorLicenseMetadata,
            >,
            ///The Daml `lastActiveAt` field.
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
                    )?)?,
                    sponsor: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "sponsor",
                    )?)?,
                    dso: rt::FromValue::from_value(rt::required_field(value, 2usize, "dso")?)?,
                    faucet_state: rt::optional_field(value, 3usize, "faucetState")?,
                    metadata: rt::optional_field(value, 4usize, "metadata")?,
                    last_active_at: rt::optional_field(value, 5usize, "lastActiveAt")?,
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
        impl rt::Template for ValidatorLicense {}
        ///The `ValidatorLicense_ReceiveFaucetCoupon` choice on [`ValidatorLicense`] (consuming).
        impl rt::Choice<ValidatorLicense>
        for crate::splice_amulet_0_1_14::Splice_ValidatorLicense::ValidatorLicense_ReceiveFaucetCoupon {
            type Return = crate::splice_amulet_0_1_14::Splice_ValidatorLicense::ValidatorLicense_ReceiveFaucetCouponResult;
            const NAME: &'static str = "ValidatorLicense_ReceiveFaucetCoupon";
            const CONSUMING: bool = true;
        }
        ///The `ValidatorLicense_RecordValidatorLivenessActivity` choice on [`ValidatorLicense`] (consuming).
        impl rt::Choice<ValidatorLicense>
        for crate::splice_amulet_0_1_14::Splice_ValidatorLicense::ValidatorLicense_RecordValidatorLivenessActivity {
            type Return = crate::splice_amulet_0_1_14::Splice_ValidatorLicense::ValidatorLicense_RecordValidatorLivenessActivityResult;
            const NAME: &'static str = "ValidatorLicense_RecordValidatorLivenessActivity";
            const CONSUMING: bool = true;
        }
        ///The `ValidatorLicense_Withdraw` choice on [`ValidatorLicense`] (consuming).
        impl rt::Choice<ValidatorLicense>
            for crate::splice_amulet_0_1_14::Splice_ValidatorLicense::ValidatorLicense_Withdraw
        {
            type Return = crate::splice_amulet_0_1_14::Splice_ValidatorLicense::ValidatorLicense_WithdrawResult;
            const NAME: &'static str = "ValidatorLicense_Withdraw";
            const CONSUMING: bool = true;
        }
        ///The `ValidatorLicense_Cancel` choice on [`ValidatorLicense`] (consuming).
        impl rt::Choice<ValidatorLicense>
            for crate::splice_amulet_0_1_14::Splice_ValidatorLicense::ValidatorLicense_Cancel
        {
            type Return =
                crate::splice_amulet_0_1_14::Splice_ValidatorLicense::ValidatorLicense_CancelResult;
            const NAME: &'static str = "ValidatorLicense_Cancel";
            const CONSUMING: bool = true;
        }
        ///The `ValidatorLicense_UpdateMetadata` choice on [`ValidatorLicense`] (consuming).
        impl rt::Choice<ValidatorLicense>
        for crate::splice_amulet_0_1_14::Splice_ValidatorLicense::ValidatorLicense_UpdateMetadata {
            type Return = crate::splice_amulet_0_1_14::Splice_ValidatorLicense::ValidatorLicense_UpdateMetadataResult;
            const NAME: &'static str = "ValidatorLicense_UpdateMetadata";
            const CONSUMING: bool = true;
        }
        ///The `ValidatorLicense_ReportActive` choice on [`ValidatorLicense`] (consuming).
        impl rt::Choice<ValidatorLicense>
            for crate::splice_amulet_0_1_14::Splice_ValidatorLicense::ValidatorLicense_ReportActive
        {
            type Return = crate::splice_amulet_0_1_14::Splice_ValidatorLicense::ValidatorLicense_ReportActiveResult;
            const NAME: &'static str = "ValidatorLicense_ReportActive";
            const CONSUMING: bool = true;
        }
        ///The `Archive` choice on [`ValidatorLicense`] (consuming).
        impl rt::Choice<ValidatorLicense>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ValidatorLivenessActivityRecord {
            ///The Daml `dso` field.
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///The Daml `validator` field.
            #[serde(rename = "validator")]
            pub validator: rt::Party,
            ///The Daml `round` field.
            #[serde(rename = "round")]
            pub round: crate::splice_amulet_0_1_14::Splice_Types::Round,
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
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)?,
                    validator: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "validator",
                    )?)?,
                    round: rt::FromValue::from_value(rt::required_field(value, 2usize, "round")?)?,
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
        impl rt::Template for ValidatorLivenessActivityRecord {}
        ///The `Archive` choice on [`ValidatorLivenessActivityRecord`] (consuming).
        impl rt::Choice<ValidatorLivenessActivityRecord>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `ValidatorLivenessActivityRecord_DsoExpire` choice on [`ValidatorLivenessActivityRecord`] (consuming).
        impl rt::Choice<ValidatorLivenessActivityRecord>
        for crate::splice_amulet_0_1_14::Splice_ValidatorLicense::ValidatorLivenessActivityRecord_DsoExpire {
            type Return = crate::splice_amulet_0_1_14::Splice_ValidatorLicense::ValidatorLivenessActivityRecord_DsoExpireResult;
            const NAME: &'static str = "ValidatorLivenessActivityRecord_DsoExpire";
            const CONSUMING: bool = true;
        }
    }
    pub mod Splice_DecentralizedSynchronizer {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ForMemberTraffic {
            ///The Daml `dso` field.
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///The Daml `memberId` field.
            #[serde(rename = "memberId")]
            pub member_id: ::std::string::String,
            ///The Daml `synchronizerId` field.
            #[serde(rename = "synchronizerId")]
            pub synchronizer_id: ::std::string::String,
            ///The Daml `migrationId` field.
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
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)?,
                    member_id: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "memberId",
                    )?)?,
                    synchronizer_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "synchronizerId",
                    )?)?,
                    migration_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "migrationId",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SynchronizerFeesConfig {
            ///The Daml `baseRateTrafficLimits` field.
            #[serde(rename = "baseRateTrafficLimits")]
            pub base_rate_traffic_limits: crate::splice_amulet_0_1_14::Splice_DecentralizedSynchronizer::BaseRateTrafficLimits,
            ///The Daml `extraTrafficPrice` field.
            #[serde(rename = "extraTrafficPrice")]
            pub extra_traffic_price: rt::Numeric,
            ///The Daml `readVsWriteScalingFactor` field.
            #[serde(rename = "readVsWriteScalingFactor")]
            pub read_vs_write_scaling_factor: rt::Int64,
            ///The Daml `minTopupAmount` field.
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
                    )?)?,
                    extra_traffic_price: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "extraTrafficPrice",
                    )?)?,
                    read_vs_write_scaling_factor: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "readVsWriteScalingFactor",
                    )?)?,
                    min_topup_amount: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "minTopupAmount",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct BaseRateTrafficLimits {
            ///The Daml `burstAmount` field.
            #[serde(rename = "burstAmount")]
            pub burst_amount: rt::Int64,
            ///The Daml `burstWindow` field.
            #[serde(rename = "burstWindow")]
            pub burst_window: crate::daml_stdlib_DA_Time_Types_1_0_0::DA_Time_Types::RelTime,
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
                    )?)?,
                    burst_window: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "burstWindow",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AmuletDecentralizedSynchronizerConfig {
            ///The Daml `requiredSynchronizers` field.
            #[serde(rename = "requiredSynchronizers")]
            pub required_synchronizers: crate::daml_stdlib_DA_Set_Types_1_0_0::DA_Set_Types::Set<
                ::std::string::String,
            >,
            ///The Daml `activeSynchronizer` field.
            #[serde(rename = "activeSynchronizer")]
            pub active_synchronizer: ::std::string::String,
            ///The Daml `fees` field.
            #[serde(rename = "fees")]
            pub fees: crate::splice_amulet_0_1_14::Splice_DecentralizedSynchronizer::SynchronizerFeesConfig,
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
                    )?)?,
                    active_synchronizer: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "activeSynchronizer",
                    )?)?,
                    fees: rt::FromValue::from_value(rt::required_field(value, 2usize, "fees")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct MemberTraffic {
            ///The Daml `dso` field.
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///The Daml `memberId` field.
            #[serde(rename = "memberId")]
            pub member_id: ::std::string::String,
            ///The Daml `synchronizerId` field.
            #[serde(rename = "synchronizerId")]
            pub synchronizer_id: ::std::string::String,
            ///The Daml `migrationId` field.
            #[serde(rename = "migrationId")]
            pub migration_id: rt::Int64,
            ///The Daml `totalPurchased` field.
            #[serde(rename = "totalPurchased")]
            pub total_purchased: rt::Int64,
            ///The Daml `numPurchases` field.
            #[serde(rename = "numPurchases")]
            pub num_purchases: rt::Int64,
            ///The Daml `amuletSpent` field.
            #[serde(rename = "amuletSpent")]
            pub amulet_spent: rt::Numeric,
            ///The Daml `usdSpent` field.
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
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)?,
                    member_id: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "memberId",
                    )?)?,
                    synchronizer_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "synchronizerId",
                    )?)?,
                    migration_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "migrationId",
                    )?)?,
                    total_purchased: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "totalPurchased",
                    )?)?,
                    num_purchases: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "numPurchases",
                    )?)?,
                    amulet_spent: rt::FromValue::from_value(rt::required_field(
                        value,
                        6usize,
                        "amuletSpent",
                    )?)?,
                    usd_spent: rt::FromValue::from_value(rt::required_field(
                        value, 7usize, "usdSpent",
                    )?)?,
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
        impl rt::Template for MemberTraffic {}
        ///The `Archive` choice on [`MemberTraffic`] (consuming).
        impl rt::Choice<MemberTraffic>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
    }
    pub mod Splice_RelRound {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct RelRound {
            ///The Daml `diff` field.
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
                    diff: rt::FromValue::from_value(rt::required_field(value, 0usize, "diff")?)?,
                })
            }
        }
    }
    pub mod Splice_Round {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct OpenMiningRound_Fetch {
            ///The Daml `p` field.
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
                    p: rt::FromValue::from_value(rt::required_field(value, 0usize, "p")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct ClosedMiningRound {
            ///The Daml `dso` field.
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///The Daml `round` field.
            #[serde(rename = "round")]
            pub round: crate::splice_amulet_0_1_14::Splice_Types::Round,
            ///The Daml `issuancePerValidatorRewardCoupon` field.
            #[serde(rename = "issuancePerValidatorRewardCoupon")]
            pub issuance_per_validator_reward_coupon: rt::Numeric,
            ///The Daml `issuancePerFeaturedAppRewardCoupon` field.
            #[serde(rename = "issuancePerFeaturedAppRewardCoupon")]
            pub issuance_per_featured_app_reward_coupon: rt::Numeric,
            ///The Daml `issuancePerUnfeaturedAppRewardCoupon` field.
            #[serde(rename = "issuancePerUnfeaturedAppRewardCoupon")]
            pub issuance_per_unfeatured_app_reward_coupon: rt::Numeric,
            ///The Daml `issuancePerSvRewardCoupon` field.
            #[serde(rename = "issuancePerSvRewardCoupon")]
            pub issuance_per_sv_reward_coupon: rt::Numeric,
            ///The Daml `optIssuancePerValidatorFaucetCoupon` field.
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
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)?,
                    round: rt::FromValue::from_value(rt::required_field(value, 1usize, "round")?)?,
                    issuance_per_validator_reward_coupon: rt::FromValue::from_value(
                        rt::required_field(value, 2usize, "issuancePerValidatorRewardCoupon")?,
                    )?,
                    issuance_per_featured_app_reward_coupon: rt::FromValue::from_value(
                        rt::required_field(value, 3usize, "issuancePerFeaturedAppRewardCoupon")?,
                    )?,
                    issuance_per_unfeatured_app_reward_coupon: rt::FromValue::from_value(
                        rt::required_field(value, 4usize, "issuancePerUnfeaturedAppRewardCoupon")?,
                    )?,
                    issuance_per_sv_reward_coupon: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "issuancePerSvRewardCoupon",
                    )?)?,
                    opt_issuance_per_validator_faucet_coupon: rt::optional_field(
                        value,
                        6usize,
                        "optIssuancePerValidatorFaucetCoupon",
                    )?,
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
        impl rt::Template for ClosedMiningRound {}
        ///The `Archive` choice on [`ClosedMiningRound`] (consuming).
        impl rt::Choice<ClosedMiningRound>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct IssuingMiningRound {
            ///The Daml `dso` field.
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///The Daml `round` field.
            #[serde(rename = "round")]
            pub round: crate::splice_amulet_0_1_14::Splice_Types::Round,
            ///The Daml `issuancePerValidatorRewardCoupon` field.
            #[serde(rename = "issuancePerValidatorRewardCoupon")]
            pub issuance_per_validator_reward_coupon: rt::Numeric,
            ///The Daml `issuancePerFeaturedAppRewardCoupon` field.
            #[serde(rename = "issuancePerFeaturedAppRewardCoupon")]
            pub issuance_per_featured_app_reward_coupon: rt::Numeric,
            ///The Daml `issuancePerUnfeaturedAppRewardCoupon` field.
            #[serde(rename = "issuancePerUnfeaturedAppRewardCoupon")]
            pub issuance_per_unfeatured_app_reward_coupon: rt::Numeric,
            ///The Daml `issuancePerSvRewardCoupon` field.
            #[serde(rename = "issuancePerSvRewardCoupon")]
            pub issuance_per_sv_reward_coupon: rt::Numeric,
            ///The Daml `opensAt` field.
            #[serde(rename = "opensAt")]
            pub opens_at: rt::Timestamp,
            ///The Daml `targetClosesAt` field.
            #[serde(rename = "targetClosesAt")]
            pub target_closes_at: rt::Timestamp,
            ///The Daml `optIssuancePerValidatorFaucetCoupon` field.
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
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)?,
                    round: rt::FromValue::from_value(rt::required_field(value, 1usize, "round")?)?,
                    issuance_per_validator_reward_coupon: rt::FromValue::from_value(
                        rt::required_field(value, 2usize, "issuancePerValidatorRewardCoupon")?,
                    )?,
                    issuance_per_featured_app_reward_coupon: rt::FromValue::from_value(
                        rt::required_field(value, 3usize, "issuancePerFeaturedAppRewardCoupon")?,
                    )?,
                    issuance_per_unfeatured_app_reward_coupon: rt::FromValue::from_value(
                        rt::required_field(value, 4usize, "issuancePerUnfeaturedAppRewardCoupon")?,
                    )?,
                    issuance_per_sv_reward_coupon: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "issuancePerSvRewardCoupon",
                    )?)?,
                    opens_at: rt::FromValue::from_value(rt::required_field(
                        value, 6usize, "opensAt",
                    )?)?,
                    target_closes_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        7usize,
                        "targetClosesAt",
                    )?)?,
                    opt_issuance_per_validator_faucet_coupon: rt::optional_field(
                        value,
                        8usize,
                        "optIssuancePerValidatorFaucetCoupon",
                    )?,
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
        impl rt::Template for IssuingMiningRound {}
        ///The `Archive` choice on [`IssuingMiningRound`] (consuming).
        impl rt::Choice<IssuingMiningRound>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct OpenMiningRound {
            ///The Daml `dso` field.
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///The Daml `round` field.
            #[serde(rename = "round")]
            pub round: crate::splice_amulet_0_1_14::Splice_Types::Round,
            ///The Daml `amuletPrice` field.
            #[serde(rename = "amuletPrice")]
            pub amulet_price: rt::Numeric,
            ///The Daml `opensAt` field.
            #[serde(rename = "opensAt")]
            pub opens_at: rt::Timestamp,
            ///The Daml `targetClosesAt` field.
            #[serde(rename = "targetClosesAt")]
            pub target_closes_at: rt::Timestamp,
            ///The Daml `issuingFor` field.
            #[serde(rename = "issuingFor")]
            pub issuing_for: crate::daml_stdlib_DA_Time_Types_1_0_0::DA_Time_Types::RelTime,
            ///The Daml `transferConfigUsd` field.
            #[serde(rename = "transferConfigUsd")]
            pub transfer_config_usd:
                crate::splice_amulet_0_1_14::Splice_AmuletConfig::TransferConfig<
                    crate::splice_amulet_0_1_14::Splice_AmuletConfig::USD,
                >,
            ///The Daml `issuanceConfig` field.
            #[serde(rename = "issuanceConfig")]
            pub issuance_config: crate::splice_amulet_0_1_14::Splice_Issuance::IssuanceConfig,
            ///The Daml `tickDuration` field.
            #[serde(rename = "tickDuration")]
            pub tick_duration: crate::daml_stdlib_DA_Time_Types_1_0_0::DA_Time_Types::RelTime,
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
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)?,
                    round: rt::FromValue::from_value(rt::required_field(value, 1usize, "round")?)?,
                    amulet_price: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "amuletPrice",
                    )?)?,
                    opens_at: rt::FromValue::from_value(rt::required_field(
                        value, 3usize, "opensAt",
                    )?)?,
                    target_closes_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "targetClosesAt",
                    )?)?,
                    issuing_for: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "issuingFor",
                    )?)?,
                    transfer_config_usd: rt::FromValue::from_value(rt::required_field(
                        value,
                        6usize,
                        "transferConfigUsd",
                    )?)?,
                    issuance_config: rt::FromValue::from_value(rt::required_field(
                        value,
                        7usize,
                        "issuanceConfig",
                    )?)?,
                    tick_duration: rt::FromValue::from_value(rt::required_field(
                        value,
                        8usize,
                        "tickDuration",
                    )?)?,
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
        impl rt::Template for OpenMiningRound {}
        ///The `Archive` choice on [`OpenMiningRound`] (consuming).
        impl rt::Choice<OpenMiningRound>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `OpenMiningRound_Fetch` choice on [`OpenMiningRound`] (non-consuming).
        impl rt::Choice<OpenMiningRound>
            for crate::splice_amulet_0_1_14::Splice_Round::OpenMiningRound_Fetch
        {
            type Return = crate::splice_amulet_0_1_14::Splice_Round::OpenMiningRound;
            const NAME: &'static str = "OpenMiningRound_Fetch";
            const CONSUMING: bool = false;
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct SummarizingMiningRound {
            ///The Daml `dso` field.
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///The Daml `round` field.
            #[serde(rename = "round")]
            pub round: crate::splice_amulet_0_1_14::Splice_Types::Round,
            ///The Daml `amuletPrice` field.
            #[serde(rename = "amuletPrice")]
            pub amulet_price: rt::Numeric,
            ///The Daml `issuanceConfig` field.
            #[serde(rename = "issuanceConfig")]
            pub issuance_config: crate::splice_amulet_0_1_14::Splice_Issuance::IssuanceConfig,
            ///The Daml `tickDuration` field.
            #[serde(rename = "tickDuration")]
            pub tick_duration: crate::daml_stdlib_DA_Time_Types_1_0_0::DA_Time_Types::RelTime,
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
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)?,
                    round: rt::FromValue::from_value(rt::required_field(value, 1usize, "round")?)?,
                    amulet_price: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "amuletPrice",
                    )?)?,
                    issuance_config: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "issuanceConfig",
                    )?)?,
                    tick_duration: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "tickDuration",
                    )?)?,
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
        impl rt::Template for SummarizingMiningRound {}
        ///The `Archive` choice on [`SummarizingMiningRound`] (consuming).
        impl rt::Choice<SummarizingMiningRound>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
    }
    pub mod Splice_Amulet_TwoStepTransfer {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TwoStepTransfer {
            ///The Daml `dso` field.
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///The Daml `sender` field.
            #[serde(rename = "sender")]
            pub sender: rt::Party,
            ///The Daml `receiver` field.
            #[serde(rename = "receiver")]
            pub receiver: rt::Party,
            ///The Daml `amount` field.
            #[serde(rename = "amount")]
            pub amount: rt::Numeric,
            ///The Daml `lockContext` field.
            #[serde(rename = "lockContext")]
            pub lock_context: ::std::string::String,
            ///The Daml `transferBefore` field.
            #[serde(rename = "transferBefore")]
            pub transfer_before: rt::Timestamp,
            ///The Daml `transferBeforeDeadline` field.
            #[serde(rename = "transferBeforeDeadline")]
            pub transfer_before_deadline: ::std::string::String,
            ///The Daml `provider` field.
            #[serde(rename = "provider")]
            pub provider: rt::Party,
            ///The Daml `allowFeaturing` field.
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
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)?,
                    sender: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "sender",
                    )?)?,
                    receiver: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "receiver",
                    )?)?,
                    amount: rt::FromValue::from_value(rt::required_field(
                        value, 3usize, "amount",
                    )?)?,
                    lock_context: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "lockContext",
                    )?)?,
                    transfer_before: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "transferBefore",
                    )?)?,
                    transfer_before_deadline: rt::FromValue::from_value(rt::required_field(
                        value,
                        6usize,
                        "transferBeforeDeadline",
                    )?)?,
                    provider: rt::FromValue::from_value(rt::required_field(
                        value, 7usize, "provider",
                    )?)?,
                    allow_featuring: rt::FromValue::from_value(rt::required_field(
                        value,
                        8usize,
                        "allowFeaturing",
                    )?)?,
                })
            }
        }
    }
}
pub mod splice_api_featured_app_v1_1_0_0 {
    pub mod Splice_Api_FeaturedAppRightV1 {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct FeaturedAppActivityMarkerView {
            ///The Daml `dso` field.
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///The Daml `provider` field.
            #[serde(rename = "provider")]
            pub provider: rt::Party,
            ///The Daml `beneficiary` field.
            #[serde(rename = "beneficiary")]
            pub beneficiary: rt::Party,
            ///The Daml `weight` field.
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
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)?,
                    provider: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "provider",
                    )?)?,
                    beneficiary: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "beneficiary",
                    )?)?,
                    weight: rt::FromValue::from_value(rt::required_field(
                        value, 3usize, "weight",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct FeaturedAppRightView {
            ///The Daml `dso` field.
            #[serde(rename = "dso")]
            pub dso: rt::Party,
            ///The Daml `provider` field.
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
                    dso: rt::FromValue::from_value(rt::required_field(value, 0usize, "dso")?)?,
                    provider: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "provider",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct FeaturedAppRight_CreateActivityMarkerResult {
            ///The Daml `activityMarkerCids` field.
            #[serde(rename = "activityMarkerCids")]
            pub activity_marker_cids: ::std::vec::Vec<
                rt::ContractId<
                    crate::splice_api_featured_app_v1_1_0_0::Splice_Api_FeaturedAppRightV1::FeaturedAppActivityMarker,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct FeaturedAppRight_CreateActivityMarker {
            ///The Daml `beneficiaries` field.
            #[serde(rename = "beneficiaries")]
            pub beneficiaries: ::std::vec::Vec<
                crate::splice_api_featured_app_v1_1_0_0::Splice_Api_FeaturedAppRightV1::AppRewardBeneficiary,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AppRewardBeneficiary {
            ///The Daml `beneficiary` field.
            #[serde(rename = "beneficiary")]
            pub beneficiary: rt::Party,
            ///The Daml `weight` field.
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
                    )?)?,
                    weight: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "weight",
                    )?)?,
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
            type View = crate::splice_api_featured_app_v1_1_0_0::Splice_Api_FeaturedAppRightV1::FeaturedAppActivityMarkerView;
        }
        ///The `Archive` choice on [`FeaturedAppActivityMarker`] (consuming).
        impl rt::Choice<FeaturedAppActivityMarker>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
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
            type View = crate::splice_api_featured_app_v1_1_0_0::Splice_Api_FeaturedAppRightV1::FeaturedAppRightView;
        }
        ///The `Archive` choice on [`FeaturedAppRight`] (consuming).
        impl rt::Choice<FeaturedAppRight>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `FeaturedAppRight_CreateActivityMarker` choice on [`FeaturedAppRight`] (non-consuming).
        impl rt::Choice<FeaturedAppRight>
        for crate::splice_api_featured_app_v1_1_0_0::Splice_Api_FeaturedAppRightV1::FeaturedAppRight_CreateActivityMarker {
            type Return = crate::splice_api_featured_app_v1_1_0_0::Splice_Api_FeaturedAppRightV1::FeaturedAppRight_CreateActivityMarkerResult;
            const NAME: &'static str = "FeaturedAppRight_CreateActivityMarker";
            const CONSUMING: bool = false;
        }
    }
}
pub mod splice_api_token_allocation_instruction_v1_1_0_0 {
    pub mod Splice_Api_Token_AllocationInstructionV1 {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum AllocationInstructionResult_Output {
            ///The Daml `AllocationInstructionResult_Pending` constructor.
            #[serde(rename = "AllocationInstructionResult_Pending")]
            AllocationInstructionResult_Pending(
                crate::splice_api_token_allocation_instruction_v1_1_0_0::Splice_Api_Token_AllocationInstructionV1::AllocationInstructionResult_Output_AllocationInstructionResult_Pending,
            ),
            ///The Daml `AllocationInstructionResult_Completed` constructor.
            #[serde(rename = "AllocationInstructionResult_Completed")]
            AllocationInstructionResult_Completed(
                crate::splice_api_token_allocation_instruction_v1_1_0_0::Splice_Api_Token_AllocationInstructionV1::AllocationInstructionResult_Output_AllocationInstructionResult_Completed,
            ),
            ///The Daml `AllocationInstructionResult_Failed` constructor.
            #[serde(rename = "AllocationInstructionResult_Failed")]
            AllocationInstructionResult_Failed(rt::Unit),
        }
        impl rt::ToValue for AllocationInstructionResult_Output {
            fn to_value(&self) -> rt::Value {
                match self {
                    AllocationInstructionResult_Output::AllocationInstructionResult_Pending(
                        inner,
                    ) => rt::variant_value(
                        "AllocationInstructionResult_Pending",
                        rt::ToValue::to_value(inner),
                    ),
                    AllocationInstructionResult_Output::AllocationInstructionResult_Completed(
                        inner,
                    ) => rt::variant_value(
                        "AllocationInstructionResult_Completed",
                        rt::ToValue::to_value(inner),
                    ),
                    AllocationInstructionResult_Output::AllocationInstructionResult_Failed(
                        inner,
                    ) => rt::variant_value(
                        "AllocationInstructionResult_Failed",
                        rt::ToValue::to_value(inner),
                    ),
                }
            }
        }
        impl rt::FromValue for AllocationInstructionResult_Output {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                let (constructor, payload) = rt::variant_parts(value)?;
                match constructor {
                    "AllocationInstructionResult_Pending" => ::core::result::Result::Ok(
                        AllocationInstructionResult_Output::AllocationInstructionResult_Pending(
                            rt::FromValue::from_value(payload)?,
                        ),
                    ),
                    "AllocationInstructionResult_Completed" => ::core::result::Result::Ok(
                        AllocationInstructionResult_Output::AllocationInstructionResult_Completed(
                            rt::FromValue::from_value(payload)?,
                        ),
                    ),
                    "AllocationInstructionResult_Failed" => ::core::result::Result::Ok(
                        AllocationInstructionResult_Output::AllocationInstructionResult_Failed(
                            rt::FromValue::from_value(payload)?,
                        ),
                    ),
                    other => ::core::result::Result::Err(rt::unexpected_constructor(
                        "AllocationInstructionResult_Output",
                        other,
                    )),
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationInstructionResult_Output_AllocationInstructionResult_Pending {
            ///The Daml `allocationInstructionCid` field.
            #[serde(rename = "allocationInstructionCid")]
            pub allocation_instruction_cid: rt::ContractId<
                crate::splice_api_token_allocation_instruction_v1_1_0_0::Splice_Api_Token_AllocationInstructionV1::AllocationInstruction,
            >,
        }
        impl rt::ToValue for AllocationInstructionResult_Output_AllocationInstructionResult_Pending {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "allocationInstructionCid",
                    rt::ToValue::to_value(&self.allocation_instruction_cid)
                ),])
            }
        }
        impl rt::FromValue for AllocationInstructionResult_Output_AllocationInstructionResult_Pending {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    allocation_instruction_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "allocationInstructionCid",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationInstructionResult_Output_AllocationInstructionResult_Completed {
            ///The Daml `allocationCid` field.
            #[serde(rename = "allocationCid")]
            pub allocation_cid: rt::ContractId<
                crate::splice_api_token_allocation_v1_1_0_0::Splice_Api_Token_AllocationV1::Allocation,
            >,
        }
        impl rt::ToValue for AllocationInstructionResult_Output_AllocationInstructionResult_Completed {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "allocationCid",
                    rt::ToValue::to_value(&self.allocation_cid)
                ),])
            }
        }
        impl rt::FromValue for AllocationInstructionResult_Output_AllocationInstructionResult_Completed {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    allocation_cid: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "allocationCid",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationInstructionResult {
            ///The Daml `output` field.
            #[serde(rename = "output")]
            pub output: crate::splice_api_token_allocation_instruction_v1_1_0_0::Splice_Api_Token_AllocationInstructionV1::AllocationInstructionResult_Output,
            ///The Daml `senderChangeCids` field.
            #[serde(rename = "senderChangeCids")]
            pub sender_change_cids: ::std::vec::Vec<
                rt::ContractId<
                    crate::splice_api_token_holding_v1_1_0_0::Splice_Api_Token_HoldingV1::Holding,
                >,
            >,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta: crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for AllocationInstructionResult {
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
        impl rt::FromValue for AllocationInstructionResult {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    output: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "output",
                    )?)?,
                    sender_change_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "senderChangeCids",
                    )?)?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 2usize, "meta")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationFactory_PublicFetch {
            ///The Daml `expectedAdmin` field.
            #[serde(rename = "expectedAdmin")]
            pub expected_admin: rt::Party,
            ///The Daml `actor` field.
            #[serde(rename = "actor")]
            pub actor: rt::Party,
        }
        impl rt::ToValue for AllocationFactory_PublicFetch {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("expectedAdmin", rt::ToValue::to_value(&self.expected_admin)),
                    ("actor", rt::ToValue::to_value(&self.actor)),
                ])
            }
        }
        impl rt::FromValue for AllocationFactory_PublicFetch {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    expected_admin: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "expectedAdmin",
                    )?)?,
                    actor: rt::FromValue::from_value(rt::required_field(value, 1usize, "actor")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationFactory_Allocate {
            ///The Daml `expectedAdmin` field.
            #[serde(rename = "expectedAdmin")]
            pub expected_admin: rt::Party,
            ///The Daml `allocation` field.
            #[serde(rename = "allocation")]
            pub allocation: crate::splice_api_token_allocation_v1_1_0_0::Splice_Api_Token_AllocationV1::AllocationSpecification,
            ///The Daml `requestedAt` field.
            #[serde(rename = "requestedAt")]
            pub requested_at: rt::Timestamp,
            ///The Daml `inputHoldingCids` field.
            #[serde(rename = "inputHoldingCids")]
            pub input_holding_cids: ::std::vec::Vec<
                rt::ContractId<
                    crate::splice_api_token_holding_v1_1_0_0::Splice_Api_Token_HoldingV1::Holding,
                >,
            >,
            ///The Daml `extraArgs` field.
            #[serde(rename = "extraArgs")]
            pub extra_args: crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for AllocationFactory_Allocate {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("expectedAdmin", rt::ToValue::to_value(&self.expected_admin)),
                    ("allocation", rt::ToValue::to_value(&self.allocation)),
                    ("requestedAt", rt::ToValue::to_value(&self.requested_at)),
                    (
                        "inputHoldingCids",
                        rt::ToValue::to_value(&self.input_holding_cids)
                    ),
                    ("extraArgs", rt::ToValue::to_value(&self.extra_args)),
                ])
            }
        }
        impl rt::FromValue for AllocationFactory_Allocate {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    expected_admin: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "expectedAdmin",
                    )?)?,
                    allocation: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "allocation",
                    )?)?,
                    requested_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "requestedAt",
                    )?)?,
                    input_holding_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "inputHoldingCids",
                    )?)?,
                    extra_args: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "extraArgs",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationFactoryView {
            ///The Daml `admin` field.
            #[serde(rename = "admin")]
            pub admin: rt::Party,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta:
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for AllocationFactoryView {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("admin", rt::ToValue::to_value(&self.admin)),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for AllocationFactoryView {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    admin: rt::FromValue::from_value(rt::required_field(value, 0usize, "admin")?)?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 1usize, "meta")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationInstruction_Update {
            ///The Daml `extraActors` field.
            #[serde(rename = "extraActors")]
            pub extra_actors: ::std::vec::Vec<rt::Party>,
            ///The Daml `extraArgs` field.
            #[serde(rename = "extraArgs")]
            pub extra_args:
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for AllocationInstruction_Update {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    ("extraActors", rt::ToValue::to_value(&self.extra_actors)),
                    ("extraArgs", rt::ToValue::to_value(&self.extra_args)),
                ])
            }
        }
        impl rt::FromValue for AllocationInstruction_Update {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    extra_actors: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "extraActors",
                    )?)?,
                    extra_args: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "extraArgs",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationInstruction_Withdraw {
            ///The Daml `extraArgs` field.
            #[serde(rename = "extraArgs")]
            pub extra_args:
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::ExtraArgs,
        }
        impl rt::ToValue for AllocationInstruction_Withdraw {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![(
                    "extraArgs",
                    rt::ToValue::to_value(&self.extra_args)
                ),])
            }
        }
        impl rt::FromValue for AllocationInstruction_Withdraw {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    extra_args: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "extraArgs",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct AllocationInstructionView {
            ///The Daml `originalInstructionCid` field.
            #[serde(rename = "originalInstructionCid")]
            pub original_instruction_cid: ::core::option::Option<
                rt::ContractId<
                    crate::splice_api_token_allocation_instruction_v1_1_0_0::Splice_Api_Token_AllocationInstructionV1::AllocationInstruction,
                >,
            >,
            ///The Daml `allocation` field.
            #[serde(rename = "allocation")]
            pub allocation: crate::splice_api_token_allocation_v1_1_0_0::Splice_Api_Token_AllocationV1::AllocationSpecification,
            ///The Daml `pendingActions` field.
            #[serde(rename = "pendingActions")]
            pub pending_actions: rt::GenMap<rt::Party, ::std::string::String>,
            ///The Daml `requestedAt` field.
            #[serde(rename = "requestedAt")]
            pub requested_at: rt::Timestamp,
            ///The Daml `inputHoldingCids` field.
            #[serde(rename = "inputHoldingCids")]
            pub input_holding_cids: ::std::vec::Vec<
                rt::ContractId<
                    crate::splice_api_token_holding_v1_1_0_0::Splice_Api_Token_HoldingV1::Holding,
                >,
            >,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta: crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
        }
        impl rt::ToValue for AllocationInstructionView {
            fn to_value(&self) -> rt::Value {
                rt::record(::std::vec![
                    (
                        "originalInstructionCid",
                        rt::ToValue::to_value(&self.original_instruction_cid)
                    ),
                    ("allocation", rt::ToValue::to_value(&self.allocation)),
                    (
                        "pendingActions",
                        rt::ToValue::to_value(&self.pending_actions)
                    ),
                    ("requestedAt", rt::ToValue::to_value(&self.requested_at)),
                    (
                        "inputHoldingCids",
                        rt::ToValue::to_value(&self.input_holding_cids)
                    ),
                    ("meta", rt::ToValue::to_value(&self.meta)),
                ])
            }
        }
        impl rt::FromValue for AllocationInstructionView {
            fn from_value(value: &rt::Value) -> ::core::result::Result<Self, rt::ValueError> {
                ::core::result::Result::Ok(Self {
                    original_instruction_cid: rt::optional_field(
                        value,
                        0usize,
                        "originalInstructionCid",
                    )?,
                    allocation: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "allocation",
                    )?)?,
                    pending_actions: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "pendingActions",
                    )?)?,
                    requested_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "requestedAt",
                    )?)?,
                    input_holding_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "inputHoldingCids",
                    )?)?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 5usize, "meta")?)?,
                })
            }
        }
        ///Marker for the Daml interface `AllocationFactory` (held via `ContractId`).
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct AllocationFactory;
        ///Marker for the Daml interface `AllocationInstruction` (held via `ContractId`).
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct AllocationInstruction;
        impl rt::Contract for AllocationFactory {
            const PACKAGE_ID: &'static str =
                "275064aacfe99cea72ee0c80563936129563776f67415ef9f13e4297eecbc520";
            const PACKAGE_NAME: &'static str = "splice-api-token-allocation-instruction-v1";
            const MODULE_NAME: &'static str = "Splice.Api.Token.AllocationInstructionV1";
            const ENTITY_NAME: &'static str = "AllocationFactory";
        }
        impl rt::Interface for AllocationFactory {
            type View = crate::splice_api_token_allocation_instruction_v1_1_0_0::Splice_Api_Token_AllocationInstructionV1::AllocationFactoryView;
        }
        ///The `Archive` choice on [`AllocationFactory`] (consuming).
        impl rt::Choice<AllocationFactory>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `AllocationFactory_Allocate` choice on [`AllocationFactory`] (non-consuming).
        impl rt::Choice<AllocationFactory>
        for crate::splice_api_token_allocation_instruction_v1_1_0_0::Splice_Api_Token_AllocationInstructionV1::AllocationFactory_Allocate {
            type Return = crate::splice_api_token_allocation_instruction_v1_1_0_0::Splice_Api_Token_AllocationInstructionV1::AllocationInstructionResult;
            const NAME: &'static str = "AllocationFactory_Allocate";
            const CONSUMING: bool = false;
        }
        ///The `AllocationFactory_PublicFetch` choice on [`AllocationFactory`] (non-consuming).
        impl rt::Choice<AllocationFactory>
        for crate::splice_api_token_allocation_instruction_v1_1_0_0::Splice_Api_Token_AllocationInstructionV1::AllocationFactory_PublicFetch {
            type Return = crate::splice_api_token_allocation_instruction_v1_1_0_0::Splice_Api_Token_AllocationInstructionV1::AllocationFactoryView;
            const NAME: &'static str = "AllocationFactory_PublicFetch";
            const CONSUMING: bool = false;
        }
        impl rt::Contract for AllocationInstruction {
            const PACKAGE_ID: &'static str =
                "275064aacfe99cea72ee0c80563936129563776f67415ef9f13e4297eecbc520";
            const PACKAGE_NAME: &'static str = "splice-api-token-allocation-instruction-v1";
            const MODULE_NAME: &'static str = "Splice.Api.Token.AllocationInstructionV1";
            const ENTITY_NAME: &'static str = "AllocationInstruction";
        }
        impl rt::Interface for AllocationInstruction {
            type View = crate::splice_api_token_allocation_instruction_v1_1_0_0::Splice_Api_Token_AllocationInstructionV1::AllocationInstructionView;
        }
        ///The `Archive` choice on [`AllocationInstruction`] (consuming).
        impl rt::Choice<AllocationInstruction>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `AllocationInstruction_Withdraw` choice on [`AllocationInstruction`] (consuming).
        impl rt::Choice<AllocationInstruction>
        for crate::splice_api_token_allocation_instruction_v1_1_0_0::Splice_Api_Token_AllocationInstructionV1::AllocationInstruction_Withdraw {
            type Return = crate::splice_api_token_allocation_instruction_v1_1_0_0::Splice_Api_Token_AllocationInstructionV1::AllocationInstructionResult;
            const NAME: &'static str = "AllocationInstruction_Withdraw";
            const CONSUMING: bool = true;
        }
        ///The `AllocationInstruction_Update` choice on [`AllocationInstruction`] (consuming).
        impl rt::Choice<AllocationInstruction>
        for crate::splice_api_token_allocation_instruction_v1_1_0_0::Splice_Api_Token_AllocationInstructionV1::AllocationInstruction_Update {
            type Return = crate::splice_api_token_allocation_instruction_v1_1_0_0::Splice_Api_Token_AllocationInstructionV1::AllocationInstructionResult;
            const NAME: &'static str = "AllocationInstruction_Update";
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
            pub sender_holding_cids: ::std::vec::Vec<
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
                    sender_holding_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "senderHoldingCids",
                    )?)?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 1usize, "meta")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Allocation_CancelResult {
            ///The Daml `senderHoldingCids` field.
            #[serde(rename = "senderHoldingCids")]
            pub sender_holding_cids: ::std::vec::Vec<
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
                    sender_holding_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "senderHoldingCids",
                    )?)?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 1usize, "meta")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Allocation_ExecuteTransferResult {
            ///The Daml `senderHoldingCids` field.
            #[serde(rename = "senderHoldingCids")]
            pub sender_holding_cids: ::std::vec::Vec<
                rt::ContractId<
                    crate::splice_api_token_holding_v1_1_0_0::Splice_Api_Token_HoldingV1::Holding,
                >,
            >,
            ///The Daml `receiverHoldingCids` field.
            #[serde(rename = "receiverHoldingCids")]
            pub receiver_holding_cids: ::std::vec::Vec<
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
                    sender_holding_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "senderHoldingCids",
                    )?)?,
                    receiver_holding_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "receiverHoldingCids",
                    )?)?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 2usize, "meta")?)?,
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
                    extra_args: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "extraArgs",
                    )?)?,
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
                    extra_args: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "extraArgs",
                    )?)?,
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
                    extra_args: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "extraArgs",
                    )?)?,
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
            pub holding_cids: ::std::vec::Vec<
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
                    allocation: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "allocation",
                    )?)?,
                    holding_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "holdingCids",
                    )?)?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 2usize, "meta")?)?,
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
            pub transfer_leg_id: ::std::string::String,
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
                    settlement: rt::FromValue::from_value(rt::required_field(
                        value,
                        0usize,
                        "settlement",
                    )?)?,
                    transfer_leg_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "transferLegId",
                    )?)?,
                    transfer_leg: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
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
                    sender: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "sender",
                    )?)?,
                    receiver: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "receiver",
                    )?)?,
                    amount: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "amount",
                    )?)?,
                    instrument_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "instrumentId",
                    )?)?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 4usize, "meta")?)?,
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
                    executor: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "executor",
                    )?)?,
                    settlement_ref: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "settlementRef",
                    )?)?,
                    requested_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "requestedAt",
                    )?)?,
                    allocate_before: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "allocateBefore",
                    )?)?,
                    settle_before: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "settleBefore",
                    )?)?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 5usize, "meta")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Reference {
            ///The Daml `id` field.
            #[serde(rename = "id")]
            pub id: ::std::string::String,
            ///The Daml `cid` field.
            #[serde(rename = "cid")]
            pub cid: ::core::option::Option<
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
                    id: rt::FromValue::from_value(rt::required_field(value, 0usize, "id")?)?,
                    cid: rt::optional_field(value, 1usize, "cid")?,
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
            pub lock: ::core::option::Option<
                crate::splice_api_token_holding_v1_1_0_0::Splice_Api_Token_HoldingV1::Lock,
            >,
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
                    owner: rt::FromValue::from_value(rt::required_field(value, 0usize, "owner")?)?,
                    instrument_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "instrumentId",
                    )?)?,
                    amount: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "amount",
                    )?)?,
                    lock: rt::optional_field(value, 3usize, "lock")?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 4usize, "meta")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Lock {
            ///The Daml `holders` field.
            #[serde(rename = "holders")]
            pub holders: ::std::vec::Vec<rt::Party>,
            ///The Daml `expiresAt` field.
            #[serde(rename = "expiresAt")]
            pub expires_at: ::core::option::Option<rt::Timestamp>,
            ///The Daml `expiresAfter` field.
            #[serde(rename = "expiresAfter")]
            pub expires_after: ::core::option::Option<
                crate::daml_stdlib_DA_Time_Types_1_0_0::DA_Time_Types::RelTime,
            >,
            ///The Daml `context` field.
            #[serde(rename = "context")]
            pub context: ::core::option::Option<::std::string::String>,
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
                    holders: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "holders",
                    )?)?,
                    expires_at: rt::optional_field(value, 1usize, "expiresAt")?,
                    expires_after: rt::optional_field(value, 2usize, "expiresAfter")?,
                    context: rt::optional_field(value, 3usize, "context")?,
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
            pub id: ::std::string::String,
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
                    admin: rt::FromValue::from_value(rt::required_field(value, 0usize, "admin")?)?,
                    id: rt::FromValue::from_value(rt::required_field(value, 1usize, "id")?)?,
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
                    meta: rt::FromValue::from_value(rt::required_field(value, 0usize, "meta")?)?,
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
                    context: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "context",
                    )?)?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 1usize, "meta")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Metadata {
            ///The Daml `values` field.
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
                    values: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "values",
                    )?)?,
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
                    values: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "values",
                    )?)?,
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
            AV_Text(::std::string::String),
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
                ::std::vec::Vec<
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
pub mod splice_api_token_transfer_instruction_v1_1_0_0 {
    pub mod Splice_Api_Token_TransferInstructionV1 {
        use canton_daml as rt;
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferFactoryView {
            ///The Daml `admin` field.
            #[serde(rename = "admin")]
            pub admin: rt::Party,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta:
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
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
                    admin: rt::FromValue::from_value(rt::required_field(value, 0usize, "admin")?)?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 1usize, "meta")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferFactory_PublicFetch {
            ///The Daml `expectedAdmin` field.
            #[serde(rename = "expectedAdmin")]
            pub expected_admin: rt::Party,
            ///The Daml `actor` field.
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
                    )?)?,
                    actor: rt::FromValue::from_value(rt::required_field(value, 1usize, "actor")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferFactory_Transfer {
            ///The Daml `expectedAdmin` field.
            #[serde(rename = "expectedAdmin")]
            pub expected_admin: rt::Party,
            ///The Daml `transfer` field.
            #[serde(rename = "transfer")]
            pub transfer: crate::splice_api_token_transfer_instruction_v1_1_0_0::Splice_Api_Token_TransferInstructionV1::Transfer,
            ///The Daml `extraArgs` field.
            #[serde(rename = "extraArgs")]
            pub extra_args: crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::ExtraArgs,
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
                    )?)?,
                    transfer: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "transfer",
                    )?)?,
                    extra_args: rt::FromValue::from_value(rt::required_field(
                        value,
                        2usize,
                        "extraArgs",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferInstruction_Update {
            ///The Daml `extraActors` field.
            #[serde(rename = "extraActors")]
            pub extra_actors: ::std::vec::Vec<rt::Party>,
            ///The Daml `extraArgs` field.
            #[serde(rename = "extraArgs")]
            pub extra_args:
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::ExtraArgs,
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
                    )?)?,
                    extra_args: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "extraArgs",
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferInstruction_Withdraw {
            ///The Daml `extraArgs` field.
            #[serde(rename = "extraArgs")]
            pub extra_args:
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::ExtraArgs,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferInstruction_Reject {
            ///The Daml `extraArgs` field.
            #[serde(rename = "extraArgs")]
            pub extra_args:
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::ExtraArgs,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferInstruction_Accept {
            ///The Daml `extraArgs` field.
            #[serde(rename = "extraArgs")]
            pub extra_args:
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::ExtraArgs,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferInstructionView {
            ///The Daml `originalInstructionCid` field.
            #[serde(rename = "originalInstructionCid")]
            pub original_instruction_cid: ::core::option::Option<
                rt::ContractId<
                    crate::splice_api_token_transfer_instruction_v1_1_0_0::Splice_Api_Token_TransferInstructionV1::TransferInstruction,
                >,
            >,
            ///The Daml `transfer` field.
            #[serde(rename = "transfer")]
            pub transfer: crate::splice_api_token_transfer_instruction_v1_1_0_0::Splice_Api_Token_TransferInstructionV1::Transfer,
            ///The Daml `status` field.
            #[serde(rename = "status")]
            pub status: crate::splice_api_token_transfer_instruction_v1_1_0_0::Splice_Api_Token_TransferInstructionV1::TransferInstructionStatus,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta: crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
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
                    )?,
                    transfer: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "transfer",
                    )?)?,
                    status: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "status",
                    )?)?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 3usize, "meta")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum TransferInstructionStatus {
            ///The Daml `TransferPendingReceiverAcceptance` constructor.
            #[serde(rename = "TransferPendingReceiverAcceptance")]
            TransferPendingReceiverAcceptance(rt::Unit),
            ///The Daml `TransferPendingInternalWorkflow` constructor.
            #[serde(rename = "TransferPendingInternalWorkflow")]
            TransferPendingInternalWorkflow(
                crate::splice_api_token_transfer_instruction_v1_1_0_0::Splice_Api_Token_TransferInstructionV1::TransferInstructionStatus_TransferPendingInternalWorkflow,
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
                            rt::FromValue::from_value(payload)?,
                        ),
                    ),
                    "TransferPendingInternalWorkflow" => ::core::result::Result::Ok(
                        TransferInstructionStatus::TransferPendingInternalWorkflow(
                            rt::FromValue::from_value(payload)?,
                        ),
                    ),
                    other => ::core::result::Result::Err(rt::unexpected_constructor(
                        "TransferInstructionStatus",
                        other,
                    )),
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferInstructionStatus_TransferPendingInternalWorkflow {
            ///The Daml `pendingActions` field.
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde", tag = "tag", content = "value")]
        pub enum TransferInstructionResult_Output {
            ///The Daml `TransferInstructionResult_Pending` constructor.
            #[serde(rename = "TransferInstructionResult_Pending")]
            TransferInstructionResult_Pending(
                crate::splice_api_token_transfer_instruction_v1_1_0_0::Splice_Api_Token_TransferInstructionV1::TransferInstructionResult_Output_TransferInstructionResult_Pending,
            ),
            ///The Daml `TransferInstructionResult_Completed` constructor.
            #[serde(rename = "TransferInstructionResult_Completed")]
            TransferInstructionResult_Completed(
                crate::splice_api_token_transfer_instruction_v1_1_0_0::Splice_Api_Token_TransferInstructionV1::TransferInstructionResult_Output_TransferInstructionResult_Completed,
            ),
            ///The Daml `TransferInstructionResult_Failed` constructor.
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
                            rt::FromValue::from_value(payload)?,
                        ),
                    ),
                    "TransferInstructionResult_Completed" => ::core::result::Result::Ok(
                        TransferInstructionResult_Output::TransferInstructionResult_Completed(
                            rt::FromValue::from_value(payload)?,
                        ),
                    ),
                    "TransferInstructionResult_Failed" => ::core::result::Result::Ok(
                        TransferInstructionResult_Output::TransferInstructionResult_Failed(
                            rt::FromValue::from_value(payload)?,
                        ),
                    ),
                    other => ::core::result::Result::Err(rt::unexpected_constructor(
                        "TransferInstructionResult_Output",
                        other,
                    )),
                }
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferInstructionResult_Output_TransferInstructionResult_Pending {
            ///The Daml `transferInstructionCid` field.
            #[serde(rename = "transferInstructionCid")]
            pub transfer_instruction_cid: rt::ContractId<
                crate::splice_api_token_transfer_instruction_v1_1_0_0::Splice_Api_Token_TransferInstructionV1::TransferInstruction,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferInstructionResult_Output_TransferInstructionResult_Completed {
            ///The Daml `receiverHoldingCids` field.
            #[serde(rename = "receiverHoldingCids")]
            pub receiver_holding_cids: ::std::vec::Vec<
                rt::ContractId<
                    crate::splice_api_token_holding_v1_1_0_0::Splice_Api_Token_HoldingV1::Holding,
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
                    )?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct TransferInstructionResult {
            ///The Daml `output` field.
            #[serde(rename = "output")]
            pub output: crate::splice_api_token_transfer_instruction_v1_1_0_0::Splice_Api_Token_TransferInstructionV1::TransferInstructionResult_Output,
            ///The Daml `senderChangeCids` field.
            #[serde(rename = "senderChangeCids")]
            pub sender_change_cids: ::std::vec::Vec<
                rt::ContractId<
                    crate::splice_api_token_holding_v1_1_0_0::Splice_Api_Token_HoldingV1::Holding,
                >,
            >,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta: crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
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
                    output: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "output",
                    )?)?,
                    sender_change_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        1usize,
                        "senderChangeCids",
                    )?)?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 2usize, "meta")?)?,
                })
            }
        }
        #[derive(Clone, Debug, PartialEq, rt::serde::Serialize, rt::serde::Deserialize)]
        #[serde(crate = "rt::serde")]
        pub struct Transfer {
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
            ///The Daml `requestedAt` field.
            #[serde(rename = "requestedAt")]
            pub requested_at: rt::Timestamp,
            ///The Daml `executeBefore` field.
            #[serde(rename = "executeBefore")]
            pub execute_before: rt::Timestamp,
            ///The Daml `inputHoldingCids` field.
            #[serde(rename = "inputHoldingCids")]
            pub input_holding_cids: ::std::vec::Vec<
                rt::ContractId<
                    crate::splice_api_token_holding_v1_1_0_0::Splice_Api_Token_HoldingV1::Holding,
                >,
            >,
            ///The Daml `meta` field.
            #[serde(rename = "meta")]
            pub meta:
                crate::splice_api_token_metadata_v1_1_0_0::Splice_Api_Token_MetadataV1::Metadata,
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
                    sender: rt::FromValue::from_value(rt::required_field(
                        value, 0usize, "sender",
                    )?)?,
                    receiver: rt::FromValue::from_value(rt::required_field(
                        value, 1usize, "receiver",
                    )?)?,
                    amount: rt::FromValue::from_value(rt::required_field(
                        value, 2usize, "amount",
                    )?)?,
                    instrument_id: rt::FromValue::from_value(rt::required_field(
                        value,
                        3usize,
                        "instrumentId",
                    )?)?,
                    requested_at: rt::FromValue::from_value(rt::required_field(
                        value,
                        4usize,
                        "requestedAt",
                    )?)?,
                    execute_before: rt::FromValue::from_value(rt::required_field(
                        value,
                        5usize,
                        "executeBefore",
                    )?)?,
                    input_holding_cids: rt::FromValue::from_value(rt::required_field(
                        value,
                        6usize,
                        "inputHoldingCids",
                    )?)?,
                    meta: rt::FromValue::from_value(rt::required_field(value, 7usize, "meta")?)?,
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
            type View = crate::splice_api_token_transfer_instruction_v1_1_0_0::Splice_Api_Token_TransferInstructionV1::TransferFactoryView;
        }
        ///The `Archive` choice on [`TransferFactory`] (consuming).
        impl rt::Choice<TransferFactory>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `TransferFactory_Transfer` choice on [`TransferFactory`] (non-consuming).
        impl rt::Choice<TransferFactory>
        for crate::splice_api_token_transfer_instruction_v1_1_0_0::Splice_Api_Token_TransferInstructionV1::TransferFactory_Transfer {
            type Return = crate::splice_api_token_transfer_instruction_v1_1_0_0::Splice_Api_Token_TransferInstructionV1::TransferInstructionResult;
            const NAME: &'static str = "TransferFactory_Transfer";
            const CONSUMING: bool = false;
        }
        ///The `TransferFactory_PublicFetch` choice on [`TransferFactory`] (non-consuming).
        impl rt::Choice<TransferFactory>
        for crate::splice_api_token_transfer_instruction_v1_1_0_0::Splice_Api_Token_TransferInstructionV1::TransferFactory_PublicFetch {
            type Return = crate::splice_api_token_transfer_instruction_v1_1_0_0::Splice_Api_Token_TransferInstructionV1::TransferFactoryView;
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
            type View = crate::splice_api_token_transfer_instruction_v1_1_0_0::Splice_Api_Token_TransferInstructionV1::TransferInstructionView;
        }
        ///The `Archive` choice on [`TransferInstruction`] (consuming).
        impl rt::Choice<TransferInstruction>
            for crate::ghc_stdlib_DA_Internal_Template_1_0_0::DA_Internal_Template::Archive
        {
            type Return = rt::Unit;
            const NAME: &'static str = "Archive";
            const CONSUMING: bool = true;
        }
        ///The `TransferInstruction_Accept` choice on [`TransferInstruction`] (consuming).
        impl rt::Choice<TransferInstruction>
        for crate::splice_api_token_transfer_instruction_v1_1_0_0::Splice_Api_Token_TransferInstructionV1::TransferInstruction_Accept {
            type Return = crate::splice_api_token_transfer_instruction_v1_1_0_0::Splice_Api_Token_TransferInstructionV1::TransferInstructionResult;
            const NAME: &'static str = "TransferInstruction_Accept";
            const CONSUMING: bool = true;
        }
        ///The `TransferInstruction_Reject` choice on [`TransferInstruction`] (consuming).
        impl rt::Choice<TransferInstruction>
        for crate::splice_api_token_transfer_instruction_v1_1_0_0::Splice_Api_Token_TransferInstructionV1::TransferInstruction_Reject {
            type Return = crate::splice_api_token_transfer_instruction_v1_1_0_0::Splice_Api_Token_TransferInstructionV1::TransferInstructionResult;
            const NAME: &'static str = "TransferInstruction_Reject";
            const CONSUMING: bool = true;
        }
        ///The `TransferInstruction_Withdraw` choice on [`TransferInstruction`] (consuming).
        impl rt::Choice<TransferInstruction>
        for crate::splice_api_token_transfer_instruction_v1_1_0_0::Splice_Api_Token_TransferInstructionV1::TransferInstruction_Withdraw {
            type Return = crate::splice_api_token_transfer_instruction_v1_1_0_0::Splice_Api_Token_TransferInstructionV1::TransferInstructionResult;
            const NAME: &'static str = "TransferInstruction_Withdraw";
            const CONSUMING: bool = true;
        }
        ///The `TransferInstruction_Update` choice on [`TransferInstruction`] (consuming).
        impl rt::Choice<TransferInstruction>
        for crate::splice_api_token_transfer_instruction_v1_1_0_0::Splice_Api_Token_TransferInstructionV1::TransferInstruction_Update {
            type Return = crate::splice_api_token_transfer_instruction_v1_1_0_0::Splice_Api_Token_TransferInstructionV1::TransferInstructionResult;
            const NAME: &'static str = "TransferInstruction_Update";
            const CONSUMING: bool = true;
        }
    }
}
