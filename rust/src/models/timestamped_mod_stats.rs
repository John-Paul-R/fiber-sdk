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
pub struct TimestampedModStats {
    #[serde(rename = "downloads")]
    pub downloads: i64,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
}

impl TimestampedModStats {
    pub fn new(downloads: i64, timestamp: String) -> TimestampedModStats {
        TimestampedModStats {
            downloads,
            timestamp,
        }
    }
}


