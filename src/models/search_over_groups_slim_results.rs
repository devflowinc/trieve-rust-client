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
pub struct SearchOverGroupsSlimResults {
    #[serde(rename = "group_chunks")]
    pub group_chunks: Vec<models::GroupScoreSlimChunks>,
    #[serde(rename = "total_chunk_pages")]
    pub total_chunk_pages: i64,
}

impl SearchOverGroupsSlimResults {
    pub fn new(group_chunks: Vec<models::GroupScoreSlimChunks>, total_chunk_pages: i64) -> SearchOverGroupsSlimResults {
        SearchOverGroupsSlimResults {
            group_chunks,
            total_chunk_pages,
        }
    }
}

