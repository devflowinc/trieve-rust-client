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
pub struct SearchQueriesWithClicksCtrResponse {
    #[serde(rename = "clicked_chunk")]
    pub clicked_chunk: Box<models::ChunkMetadata>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "position")]
    pub position: i32,
    #[serde(rename = "query")]
    pub query: String,
}

impl SearchQueriesWithClicksCtrResponse {
    pub fn new(clicked_chunk: models::ChunkMetadata, created_at: String, position: i32, query: String) -> SearchQueriesWithClicksCtrResponse {
        SearchQueriesWithClicksCtrResponse {
            clicked_chunk: Box::new(clicked_chunk),
            created_at,
            position,
            query,
        }
    }
}

