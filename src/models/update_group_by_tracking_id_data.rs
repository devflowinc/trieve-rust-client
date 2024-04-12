/*
 * Trieve API
 *
 * Trieve OpenAPI Specification. This document describes all of the operations available through the Trieve API.
 *
 * The version of the OpenAPI document: 0.6.5
 * Contact: developers@trieve.ai
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateGroupByTrackingIdData {
    /// Description to assign to the chunk_group. Convenience field for you to avoid having to remember what the group is for. If not provided, the description will not be updated.
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// Optional metadata to assign to the chunk_group. This is a JSON object that can store any additional information you want to associate with the chunks inside of the chunk_group.
    #[serde(rename = "metadata", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Option<serde_json::Value>>,
    /// Name to assign to the chunk_group. Does not need to be unique. If not provided, the name will not be updated.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /// Optional tags to assign to the chunk_group. This is a list of strings that can be used to categorize the chunks inside the chunk_group.
    #[serde(rename = "tag_set", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tag_set: Option<Option<Vec<String>>>,
    /// Tracking Id of the chunk_group to update.
    #[serde(rename = "tracking_id")]
    pub tracking_id: String,
}

impl UpdateGroupByTrackingIdData {
    pub fn new(tracking_id: String) -> UpdateGroupByTrackingIdData {
        UpdateGroupByTrackingIdData {
            description: None,
            metadata: None,
            name: None,
            tag_set: None,
            tracking_id,
        }
    }
}

