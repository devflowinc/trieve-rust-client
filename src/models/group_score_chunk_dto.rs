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
pub struct GroupScoreChunkDto {
    #[serde(rename = "group_id")]
    pub group_id: uuid::Uuid,
    #[serde(rename = "group_tracking_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub group_tracking_id: Option<Option<String>>,
    #[serde(rename = "metadata")]
    pub metadata: Vec<models::ScoreChunkDto>,
}

impl GroupScoreChunkDto {
    pub fn new(group_id: uuid::Uuid, metadata: Vec<models::ScoreChunkDto>) -> GroupScoreChunkDto {
        GroupScoreChunkDto {
            group_id,
            group_tracking_id: None,
            metadata,
        }
    }
}
