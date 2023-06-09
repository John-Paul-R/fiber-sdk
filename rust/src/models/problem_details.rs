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



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ProblemDetails {
    #[serde(rename = "type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Option<String>>,
    #[serde(rename = "title", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub title: Option<Option<String>>,
    #[serde(rename = "status", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub status: Option<Option<i32>>,
    #[serde(rename = "detail", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub detail: Option<Option<String>>,
    #[serde(rename = "instance", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub instance: Option<Option<String>>,
}

impl ProblemDetails {
    pub fn new() -> ProblemDetails {
        ProblemDetails {
            r#type: None,
            title: None,
            status: None,
            detail: None,
            instance: None,
        }
    }
}


