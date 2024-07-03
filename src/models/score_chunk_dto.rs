/*
 * Trieve API
 *
 * Trieve OpenAPI Specification. This document describes all of the operations available through the Trieve API.
 *
 * The version of the OpenAPI document: 0.10.9
 * Contact: developers@trieve.ai
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScoreChunkDto {
    #[serde(rename = "highlights", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub highlights: Option<Option<Vec<String>>>,
    #[serde(rename = "metadata")]
    pub metadata: Vec<models::ChunkMetadataTypes>,
    #[serde(rename = "score")]
    pub score: f64,
}

impl ScoreChunkDto {
    pub fn new(metadata: Vec<models::ChunkMetadataTypes>, score: f64) -> ScoreChunkDto {
        ScoreChunkDto {
            highlights: None,
            metadata,
            score,
        }
    }
}

