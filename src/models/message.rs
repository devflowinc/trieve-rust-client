/*
 * Trieve API
 *
 * Trieve OpenAPI Specification. This document describes all of the operations available through the Trieve API.
 *
 * The version of the OpenAPI document: 0.5.8
 * Contact: developers@trieve.ai
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "completion_tokens", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub completion_tokens: Option<Option<i32>>,
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "dataset_id")]
    pub dataset_id: uuid::Uuid,
    #[serde(rename = "deleted")]
    pub deleted: bool,
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "prompt_tokens", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub prompt_tokens: Option<Option<i32>>,
    #[serde(rename = "role")]
    pub role: String,
    #[serde(rename = "sort_order")]
    pub sort_order: i32,
    #[serde(rename = "topic_id")]
    pub topic_id: uuid::Uuid,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl Message {
    pub fn new(content: String, created_at: String, dataset_id: uuid::Uuid, deleted: bool, id: uuid::Uuid, role: String, sort_order: i32, topic_id: uuid::Uuid, updated_at: String) -> Message {
        Message {
            completion_tokens: None,
            content,
            created_at,
            dataset_id,
            deleted,
            id,
            prompt_tokens: None,
            role,
            sort_order,
            topic_id,
            updated_at,
        }
    }
}
