/*
 * FiberMC API
 *
 * API documentation for FiberMC
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: jp@jpcode.dev
 * Generated by: https://openapi-generator.tech
 */


use serde_repr::{Serialize_repr, Deserialize_repr};

/// 
#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize_repr, Deserialize_repr)]
pub enum FiberCategory {
    Variant0,
    Variant1,
    Variant2,
    Variant3,
    Variant4,
    Variant5,
    Variant6,
    Variant7,
    Variant8,
    Variant9,
    Variant10,
    Variant11,
    Variant12,
    Variant13,

}

impl ToString for FiberCategory {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0 => String::from("0"),
            Self::Variant1 => String::from("1"),
            Self::Variant2 => String::from("2"),
            Self::Variant3 => String::from("3"),
            Self::Variant4 => String::from("4"),
            Self::Variant5 => String::from("5"),
            Self::Variant6 => String::from("6"),
            Self::Variant7 => String::from("7"),
            Self::Variant8 => String::from("8"),
            Self::Variant9 => String::from("9"),
            Self::Variant10 => String::from("10"),
            Self::Variant11 => String::from("11"),
            Self::Variant12 => String::from("12"),
            Self::Variant13 => String::from("13"),
        }
    }
}

impl Default for FiberCategory {
    fn default() -> FiberCategory {
        Self::Variant0
    }
}




