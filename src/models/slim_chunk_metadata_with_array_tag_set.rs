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
pub struct SlimChunkMetadataWithArrayTagSet {
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "dataset_id")]
    pub dataset_id: uuid::Uuid,
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "image_urls", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub image_urls: Option<Option<Vec<String>>>,
    #[serde(rename = "link", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub link: Option<Option<String>>,
    #[serde(rename = "location", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub location: Option<Option<Box<models::GeoInfo>>>,
    #[serde(rename = "metadata", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Option<serde_json::Value>>,
    #[serde(rename = "num_value", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub num_value: Option<Option<f64>>,
    #[serde(rename = "tag_set", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tag_set: Option<Option<Vec<String>>>,
    #[serde(rename = "time_stamp", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<Option<String>>,
    #[serde(rename = "tracking_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<Option<String>>,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "weight")]
    pub weight: f64,
}

impl SlimChunkMetadataWithArrayTagSet {
    pub fn new(created_at: String, dataset_id: uuid::Uuid, id: uuid::Uuid, updated_at: String, weight: f64) -> SlimChunkMetadataWithArrayTagSet {
        SlimChunkMetadataWithArrayTagSet {
            created_at,
            dataset_id,
            id,
            image_urls: None,
            link: None,
            location: None,
            metadata: None,
            num_value: None,
            tag_set: None,
            time_stamp: None,
            tracking_id: None,
            updated_at,
            weight,
        }
    }
}

