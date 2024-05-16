/*
 * Trieve API
 *
 * Trieve OpenAPI Specification. This document describes all of the operations available through the Trieve API.
 *
 * The version of the OpenAPI document: 0.8.5
 * Contact: developers@trieve.ai
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScoreChunkDto {
    #[serde(rename = "metadata")]
    pub metadata: Vec<models::ChunkMetadataTypes>,
    #[serde(rename = "score")]
    pub score: f64,
}

impl ScoreChunkDto {
    pub fn new(metadata: Vec<models::ChunkMetadataTypes>, score: f64) -> ScoreChunkDto {
        ScoreChunkDto {
            metadata,
            score,
        }
    }
}

