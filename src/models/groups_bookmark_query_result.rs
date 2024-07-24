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
pub struct GroupsBookmarkQueryResult {
    #[serde(rename = "chunks")]
    pub chunks: Vec<models::ChunkMetadataStringTagSet>,
    #[serde(rename = "group")]
    pub group: Box<models::ChunkGroupAndFileId>,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
}

impl GroupsBookmarkQueryResult {
    pub fn new(chunks: Vec<models::ChunkMetadataStringTagSet>, group: models::ChunkGroupAndFileId, total_pages: i64) -> GroupsBookmarkQueryResult {
        GroupsBookmarkQueryResult {
            chunks,
            group: Box::new(group),
            total_pages,
        }
    }
}

