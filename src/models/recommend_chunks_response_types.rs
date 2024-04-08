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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RecommendChunksResponseTypes {
    RecommendChunkMetadata(Vec<models::ChunkMetadata>),
    RecommendSlimChunkMetadata(Vec<models::SlimChunkMetadata>),
}

impl Default for RecommendChunksResponseTypes {
    fn default() -> Self {
        Self::RecommendChunkMetadata(Default::default())
    }
}

