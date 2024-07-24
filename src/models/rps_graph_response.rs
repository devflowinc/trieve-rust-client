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
pub struct RpsGraphResponse {
    #[serde(rename = "rps_points")]
    pub rps_points: Vec<models::SearchRpsGraph>,
}

impl RpsGraphResponse {
    pub fn new(rps_points: Vec<models::SearchRpsGraph>) -> RpsGraphResponse {
        RpsGraphResponse {
            rps_points,
        }
    }
}
