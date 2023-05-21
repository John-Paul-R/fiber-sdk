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
pub enum ForeignPlatform {
    Variant0,
    Variant1,
    Variant2,

}

impl ToString for ForeignPlatform {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0 => String::from("0"),
            Self::Variant1 => String::from("1"),
            Self::Variant2 => String::from("2"),
        }
    }
}

impl Default for ForeignPlatform {
    fn default() -> ForeignPlatform {
        Self::Variant0
    }
}




