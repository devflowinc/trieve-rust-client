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

/// MultiQuery : MultiQuery allows you to construct a dense vector from multiple queries with a weighted sum. This is useful for when you want to emphasize certain features of the query. This only works with Semantic Search and is not compatible with cross encoder re-ranking or highlights.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MultiQuery {
    /// Query to embed for the final weighted sum vector.
    #[serde(rename = "query")]
    pub query: String,
    /// Float value which is applies as a multiplier to the query vector when summing.
    #[serde(rename = "weight")]
    pub weight: f32,
}

impl MultiQuery {
    /// MultiQuery allows you to construct a dense vector from multiple queries with a weighted sum. This is useful for when you want to emphasize certain features of the query. This only works with Semantic Search and is not compatible with cross encoder re-ranking or highlights.
    pub fn new(query: String, weight: f32) -> MultiQuery {
        MultiQuery {
            query,
            weight,
        }
    }
}

