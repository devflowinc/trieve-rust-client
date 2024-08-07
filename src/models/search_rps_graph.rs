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
pub struct SearchRpsGraph {
    #[serde(rename = "average_rps")]
    pub average_rps: f64,
    #[serde(rename = "time_stamp")]
    pub time_stamp: String,
}

impl SearchRpsGraph {
    pub fn new(average_rps: f64, time_stamp: String) -> SearchRpsGraph {
        SearchRpsGraph {
            average_rps,
            time_stamp,
        }
    }
}

