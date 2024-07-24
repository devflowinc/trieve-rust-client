/*
 * Trieve API
 *
 * Trieve OpenAPI Specification. This document describes all of the operations available through the Trieve API.
 *
 * The version of the OpenAPI document: 0.11.6
 * Contact: developers@trieve.ai
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchClusterResponse {
    #[serde(rename = "clusters")]
    pub clusters: Vec<models::SearchClusterTopics>,
}

impl SearchClusterResponse {
    pub fn new(clusters: Vec<models::SearchClusterTopics>) -> SearchClusterResponse {
        SearchClusterResponse {
            clusters,
        }
    }
}

