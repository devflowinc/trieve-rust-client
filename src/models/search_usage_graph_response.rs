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
pub struct SearchUsageGraphResponse {
    #[serde(rename = "usage_points")]
    pub usage_points: Vec<models::UsageGraphPoint>,
}

impl SearchUsageGraphResponse {
    pub fn new(usage_points: Vec<models::UsageGraphPoint>) -> SearchUsageGraphResponse {
        SearchUsageGraphResponse {
            usage_points,
        }
    }
}

