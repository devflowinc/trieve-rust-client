/*
 * Trieve API
 *
 * Trieve OpenAPI Specification. This document describes all of the operations available through the Trieve API.
 *
 * The version of the OpenAPI document: 0.11.7
 * Contact: developers@trieve.ai
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchWithinGroupResults {
    #[serde(rename = "bookmarks")]
    pub bookmarks: Vec<models::ScoreChunkDto>,
    #[serde(rename = "group")]
    pub group: Box<models::ChunkGroupAndFileId>,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
}

impl SearchWithinGroupResults {
    pub fn new(bookmarks: Vec<models::ScoreChunkDto>, group: models::ChunkGroupAndFileId, total_pages: i64) -> SearchWithinGroupResults {
        SearchWithinGroupResults {
            bookmarks,
            group: Box::new(group),
            total_pages,
        }
    }
}

