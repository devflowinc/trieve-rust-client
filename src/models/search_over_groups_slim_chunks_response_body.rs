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
pub struct SearchOverGroupsSlimChunksResponseBody {
    #[serde(rename = "group_chunks")]
    pub group_chunks: Vec<models::GroupSlimChunksDto>,
    #[serde(rename = "total_chunk_pages")]
    pub total_chunk_pages: i64,
}

impl SearchOverGroupsSlimChunksResponseBody {
    pub fn new(group_chunks: Vec<models::GroupSlimChunksDto>, total_chunk_pages: i64) -> SearchOverGroupsSlimChunksResponseBody {
        SearchOverGroupsSlimChunksResponseBody {
            group_chunks,
            total_chunk_pages,
        }
    }
}

