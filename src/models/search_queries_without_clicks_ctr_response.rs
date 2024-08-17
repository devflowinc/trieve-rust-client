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
pub struct SearchQueriesWithoutClicksCtrResponse {
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "query")]
    pub query: String,
}

impl SearchQueriesWithoutClicksCtrResponse {
    pub fn new(created_at: String, query: String) -> SearchQueriesWithoutClicksCtrResponse {
        SearchQueriesWithoutClicksCtrResponse {
            created_at,
            query,
        }
    }
}
