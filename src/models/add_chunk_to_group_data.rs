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
pub struct AddChunkToGroupData {
    /// Id of the chunk to make a member of the group.
    #[serde(rename = "chunk_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub chunk_id: Option<Option<uuid::Uuid>>,
    /// Tracking Id of the chunk to make a member of the group.
    #[serde(rename = "tracking_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<Option<String>>,
}

impl AddChunkToGroupData {
    pub fn new() -> AddChunkToGroupData {
        AddChunkToGroupData {
            chunk_id: None,
            tracking_id: None,
        }
    }
}

