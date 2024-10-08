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
pub struct ScrollChunksReqPayload {
    #[serde(rename = "filters", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub filters: Option<Option<Box<models::ChunkFilter>>>,
    /// Offset chunk id is the id of the chunk to start the page from. If not specified, this defaults to the first chunk in the dataset sorted by id ascending.
    #[serde(rename = "offset_chunk_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub offset_chunk_id: Option<Option<uuid::Uuid>>,
    /// Page size is the number of chunks to fetch. This can be used to fetch more than 10 chunks at a time.
    #[serde(rename = "page_size", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<Option<i64>>,
    #[serde(rename = "sort_by", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<Option<Box<models::SortByField>>>,
}

impl ScrollChunksReqPayload {
    pub fn new() -> ScrollChunksReqPayload {
        ScrollChunksReqPayload {
            filters: None,
            offset_chunk_id: None,
            page_size: None,
            sort_by: None,
        }
    }
}

