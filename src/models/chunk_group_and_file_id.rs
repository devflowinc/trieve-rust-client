/*
 * Trieve API
 *
 * Trieve OpenAPI Specification. This document describes all of the operations available through the Trieve API.
 *
 * The version of the OpenAPI document: 0.11.6
 * Contact: developers@trieve.ai
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChunkGroupAndFileId {
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "dataset_id")]
    pub dataset_id: uuid::Uuid,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "file_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub file_id: Option<Option<uuid::Uuid>>,
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "metadata", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Option<serde_json::Value>>,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "tag_set", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tag_set: Option<Option<Vec<String>>>,
    #[serde(rename = "tracking_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<Option<String>>,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl ChunkGroupAndFileId {
    pub fn new(created_at: String, dataset_id: uuid::Uuid, description: String, id: uuid::Uuid, name: String, updated_at: String) -> ChunkGroupAndFileId {
        ChunkGroupAndFileId {
            created_at,
            dataset_id,
            description,
            file_id: None,
            id,
            metadata: None,
            name,
            tag_set: None,
            tracking_id: None,
            updated_at,
        }
    }
}
