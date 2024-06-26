/*
 * Trieve API
 *
 * Trieve OpenAPI Specification. This document describes all of the operations available through the Trieve API.
 *
 * The version of the OpenAPI document: 0.8.7
 * Contact: developers@trieve.ai
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct BookmarkData {
    #[serde(rename = "chunks")]
    pub chunks: Vec<models::ChunkMetadata>,
    #[serde(rename = "group")]
    pub group: Box<models::ChunkGroup>,
    #[serde(rename = "total_pages")]
    pub total_pages: i64,
}

impl BookmarkData {
    pub fn new(chunks: Vec<models::ChunkMetadata>, group: models::ChunkGroup, total_pages: i64) -> BookmarkData {
        BookmarkData {
            chunks,
            group: Box::new(group),
            total_pages,
        }
    }
}

