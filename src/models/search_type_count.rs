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
pub struct SearchTypeCount {
    #[serde(rename = "search_count")]
    pub search_count: i64,
    #[serde(rename = "search_method")]
    pub search_method: String,
    #[serde(rename = "search_type")]
    pub search_type: String,
}

impl SearchTypeCount {
    pub fn new(search_count: i64, search_method: String, search_type: String) -> SearchTypeCount {
        SearchTypeCount {
            search_count,
            search_method,
            search_type,
        }
    }
}

