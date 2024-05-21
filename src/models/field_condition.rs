/*
 * Trieve API
 *
 * Trieve OpenAPI Specification. This document describes all of the operations available through the Trieve API.
 *
 * The version of the OpenAPI document: 0.8.7
 * Contact: developers@trieve.ai
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FieldCondition {
    #[serde(rename = "date_range", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub date_range: Option<Option<Box<models::DateRange>>>,
    /// Field is the name of the field to filter on. The field value will be used to check for an exact substring match on the metadata values for each existing chunk. This is useful for when you want to filter chunks by arbitrary metadata. To access fields inside of the metadata that you provide with the card, prefix the field name with `metadata.`.
    #[serde(rename = "field")]
    pub field: String,
    #[serde(rename = "geo_bounding_box", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub geo_bounding_box: Option<Option<Box<models::LocationBoundingBox>>>,
    #[serde(rename = "geo_polygon", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub geo_polygon: Option<Option<Box<models::LocationPolygon>>>,
    #[serde(rename = "geo_radius", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub geo_radius: Option<Option<Box<models::LocationRadius>>>,
    /// Match is the value to match on the field. The match value will be used to check for an exact substring match on the metadata values for each existing chunk. This is useful for when you want to filter chunks by arbitrary metadata.
    #[serde(rename = "match", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub r#match: Option<Option<Vec<models::MatchCondition>>>,
    #[serde(rename = "range", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub range: Option<Option<Box<models::Range>>>,
}

impl FieldCondition {
    pub fn new(field: String) -> FieldCondition {
        FieldCondition {
            date_range: None,
            field,
            geo_bounding_box: None,
            geo_polygon: None,
            geo_radius: None,
            r#match: None,
            range: None,
        }
    }
}

