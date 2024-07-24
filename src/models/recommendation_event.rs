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
pub struct RecommendationEvent {
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "dataset_id")]
    pub dataset_id: uuid::Uuid,
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "negative_ids")]
    pub negative_ids: Vec<uuid::Uuid>,
    #[serde(rename = "negative_tracking_ids")]
    pub negative_tracking_ids: Vec<String>,
    #[serde(rename = "positive_ids")]
    pub positive_ids: Vec<uuid::Uuid>,
    #[serde(rename = "positive_tracking_ids")]
    pub positive_tracking_ids: Vec<String>,
    #[serde(rename = "recommendation_type")]
    pub recommendation_type: String,
    #[serde(rename = "request_params")]
    pub request_params: String,
    #[serde(rename = "results")]
    pub results: Vec<models::SearchResultType>,
    #[serde(rename = "top_score")]
    pub top_score: f32,
}

impl RecommendationEvent {
    pub fn new(created_at: String, dataset_id: uuid::Uuid, id: uuid::Uuid, negative_ids: Vec<uuid::Uuid>, negative_tracking_ids: Vec<String>, positive_ids: Vec<uuid::Uuid>, positive_tracking_ids: Vec<String>, recommendation_type: String, request_params: String, results: Vec<models::SearchResultType>, top_score: f32) -> RecommendationEvent {
        RecommendationEvent {
            created_at,
            dataset_id,
            id,
            negative_ids,
            negative_tracking_ids,
            positive_ids,
            positive_tracking_ids,
            recommendation_type,
            request_params,
            results,
            top_score,
        }
    }
}
