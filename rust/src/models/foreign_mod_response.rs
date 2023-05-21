/*
 * FiberMC API
 *
 * API documentation for FiberMC
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: jp@jpcode.dev
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ForeignModResponse {
    #[serde(rename = "platform", skip_serializing_if = "Option::is_none")]
    pub platform: Option<crate::models::ForeignPlatform>,
    #[serde(rename = "platformModId", skip_serializing_if = "Option::is_none")]
    pub platform_mod_id: Option<String>,
    #[serde(rename = "fiberModId", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub fiber_mod_id: Option<Option<uuid::Uuid>>,
    #[serde(rename = "platformModSlug")]
    pub platform_mod_slug: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "summary")]
    pub summary: String,
    #[serde(rename = "downloadCount")]
    pub download_count: i32,
    #[serde(rename = "dateReleased", skip_serializing_if = "Option::is_none")]
    pub date_released: Option<String>,
    #[serde(rename = "dateModified")]
    pub date_modified: String,
    #[serde(rename = "deletedAuto", skip_serializing_if = "Option::is_none")]
    pub deleted_auto: Option<bool>,
    #[serde(rename = "deletedManual", skip_serializing_if = "Option::is_none")]
    pub deleted_manual: Option<bool>,
    #[serde(rename = "authors")]
    pub authors: Vec<crate::models::ForeignAuthorResponse>,
    #[serde(rename = "minecraftVersions")]
    pub minecraft_versions: Vec<String>,
    #[serde(rename = "fabricVersions", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub fabric_versions: Option<Option<Vec<String>>>,
    #[serde(rename = "categories")]
    pub categories: Vec<String>,
}

impl ForeignModResponse {
    pub fn new(platform_mod_slug: String, name: String, summary: String, download_count: i32, date_modified: String, authors: Vec<crate::models::ForeignAuthorResponse>, minecraft_versions: Vec<String>, categories: Vec<String>) -> ForeignModResponse {
        ForeignModResponse {
            platform: None,
            platform_mod_id: None,
            fiber_mod_id: None,
            platform_mod_slug,
            name,
            summary,
            download_count,
            date_released: None,
            date_modified,
            deleted_auto: None,
            deleted_manual: None,
            authors,
            minecraft_versions,
            fabric_versions: None,
            categories,
        }
    }
}


