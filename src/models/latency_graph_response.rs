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
pub struct LatencyGraphResponse {
    #[serde(rename = "latency_points")]
    pub latency_points: Vec<models::SearchLatencyGraph>,
}

impl LatencyGraphResponse {
    pub fn new(latency_points: Vec<models::SearchLatencyGraph>) -> LatencyGraphResponse {
        LatencyGraphResponse {
            latency_points,
        }
    }
}

