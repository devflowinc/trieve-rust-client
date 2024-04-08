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
pub struct SearchGroupsResult {
    #[serde(rename = "bookmarks")]
    pub bookmarks: Vec<models::ScoreChunkDto>,
    #[serde(rename = "group")]
    pub group: Box<models::ChunkGroup>,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
}

impl SearchGroupsResult {
    pub fn new(bookmarks: Vec<models::ScoreChunkDto>, group: models::ChunkGroup, total_pages: i64) -> SearchGroupsResult {
        SearchGroupsResult {
            bookmarks,
            group: Box::new(group),
            total_pages,
        }
    }
}
