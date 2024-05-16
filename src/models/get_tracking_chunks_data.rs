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
pub struct GetTrackingChunksData {
    #[serde(rename = "tracking_ids")]
    pub tracking_ids: Vec<String>,
}

impl GetTrackingChunksData {
    pub fn new(tracking_ids: Vec<String>) -> GetTrackingChunksData {
        GetTrackingChunksData {
            tracking_ids,
        }
    }
}
