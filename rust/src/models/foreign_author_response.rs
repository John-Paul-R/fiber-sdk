/*
 * FibermcApi
 *
 * No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)
 *
 * The version of the OpenAPI document: 0.0.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ForeignAuthorResponse {
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<crate::models::ForeignPlatform>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "slug")]
    pub slug: String,
}

impl ForeignAuthorResponse {
    pub fn new(name: String, slug: String) -> ForeignAuthorResponse {
        ForeignAuthorResponse {
            platform: None,
            name,
            slug,
        }
    }
}

