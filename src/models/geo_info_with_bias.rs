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
pub struct GeoInfoWithBias {
    #[serde(rename = "bias")]
    pub bias: f64,
    #[serde(rename = "location")]
    pub location: Box<models::GeoInfo>,
}

impl GeoInfoWithBias {
    pub fn new(bias: f64, location: models::GeoInfo) -> GeoInfoWithBias {
        GeoInfoWithBias {
            bias,
            location: Box::new(location),
        }
    }
}

