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

/// GeoInfo : Location that you want to use as the center of the search.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GeoInfo {
    #[serde(rename = "lat")]
    pub lat: Box<models::GeoTypes>,
    #[serde(rename = "lon")]
    pub lon: Box<models::GeoTypes>,
}

impl GeoInfo {
    /// Location that you want to use as the center of the search.
    pub fn new(lat: models::GeoTypes, lon: models::GeoTypes) -> GeoInfo {
        GeoInfo {
            lat: Box::new(lat),
            lon: Box::new(lon),
        }
    }
}

