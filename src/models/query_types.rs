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

/// QueryTypes : Query is the search query. This can be any string. The query will be used to create an embedding vector and/or SPLADE vector which will be used to find the result set.  You can either provide one query, or multiple with weights. Multi-query only works with Semantic Search and is not compatible with cross encoder re-ranking or highlights.
/// Query is the search query. This can be any string. The query will be used to create an embedding vector and/or SPLADE vector which will be used to find the result set.  You can either provide one query, or multiple with weights. Multi-query only works with Semantic Search and is not compatible with cross encoder re-ranking or highlights.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum QueryTypes {
    String(String),
    Array(Vec<models::MultiQuery>),
}

impl Default for QueryTypes {
    fn default() -> Self {
        Self::String(Default::default())
    }
}

