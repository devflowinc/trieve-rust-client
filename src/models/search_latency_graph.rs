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
pub struct SearchLatencyGraph {
    #[serde(rename = "average_latency")]
    pub average_latency: f64,
    #[serde(rename = "time_stamp")]
    pub time_stamp: String,
}

impl SearchLatencyGraph {
    pub fn new(average_latency: f64, time_stamp: String) -> SearchLatencyGraph {
        SearchLatencyGraph {
            average_latency,
            time_stamp,
        }
    }
}

