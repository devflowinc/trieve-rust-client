/*
 * Trieve API
 *
 * Trieve OpenAPI Specification. This document describes all of the operations available through the Trieve API.
 *
 * The version of the OpenAPI document: 0.8.5
 * Contact: developers@trieve.ai
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SuggestedQueriesRequest {
    /// The query to base the generated suggested queries off of.
    #[serde(rename = "query")]
    pub query: String,
}

impl SuggestedQueriesRequest {
    pub fn new(query: String) -> SuggestedQueriesRequest {
        SuggestedQueriesRequest {
            query,
        }
    }
}

