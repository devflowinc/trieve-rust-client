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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RecommendResponseTypes {
    RecommendChunksResponseBody(Box<models::RecommendChunksResponseBody>),
    Array(Vec<models::ChunkMetadataWithScore>),
}

impl Default for RecommendResponseTypes {
    fn default() -> Self {
        Self::RecommendChunksResponseBody(Default::default())
    }
}

