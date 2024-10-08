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
pub struct LocationBoundingBox {
    #[serde(rename = "bottom_right")]
    pub bottom_right: Box<models::GeoInfo>,
    #[serde(rename = "top_left")]
    pub top_left: Box<models::GeoInfo>,
}

impl LocationBoundingBox {
    pub fn new(bottom_right: models::GeoInfo, top_left: models::GeoInfo) -> LocationBoundingBox {
        LocationBoundingBox {
            bottom_right: Box::new(bottom_right),
            top_left: Box::new(top_left),
        }
    }
}

