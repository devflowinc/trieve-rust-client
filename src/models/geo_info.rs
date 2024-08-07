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
pub struct GeoInfo {
    #[serde(rename = "lat")]
    pub lat: Box<models::GeoTypes>,
    #[serde(rename = "lon")]
    pub lon: Box<models::GeoTypes>,
}

impl GeoInfo {
    pub fn new(lat: models::GeoTypes, lon: models::GeoTypes) -> GeoInfo {
        GeoInfo {
            lat: Box::new(lat),
            lon: Box::new(lon),
        }
    }
}

