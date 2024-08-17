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
pub struct RecommendationsWithClicksCtrResponse {
    #[serde(rename = "clicked_chunk")]
    pub clicked_chunk: Box<models::ChunkMetadata>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "negative_ids", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub negative_ids: Option<Option<Vec<String>>>,
    #[serde(rename = "negative_tracking_ids", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub negative_tracking_ids: Option<Option<Vec<String>>>,
    #[serde(rename = "position")]
    pub position: i32,
    #[serde(rename = "positive_ids", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub positive_ids: Option<Option<Vec<String>>>,
    #[serde(rename = "positive_tracking_ids", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub positive_tracking_ids: Option<Option<Vec<String>>>,
}

impl RecommendationsWithClicksCtrResponse {
    pub fn new(clicked_chunk: models::ChunkMetadata, created_at: String, position: i32) -> RecommendationsWithClicksCtrResponse {
        RecommendationsWithClicksCtrResponse {
            clicked_chunk: Box::new(clicked_chunk),
            created_at,
            negative_ids: None,
            negative_tracking_ids: None,
            position,
            positive_ids: None,
            positive_tracking_ids: None,
        }
    }
}
