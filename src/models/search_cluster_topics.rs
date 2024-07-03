/*
 * Trieve API
 *
 * Trieve OpenAPI Specification. This document describes all of the operations available through the Trieve API.
 *
 * The version of the OpenAPI document: 0.10.9
 * Contact: developers@trieve.ai
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchClusterTopics {
    #[serde(rename = "avg_score")]
    pub avg_score: f32,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "dataset_id")]
    pub dataset_id: uuid::Uuid,
    #[serde(rename = "density")]
    pub density: i32,
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "topic")]
    pub topic: String,
}

impl SearchClusterTopics {
    pub fn new(avg_score: f32, created_at: String, dataset_id: uuid::Uuid, density: i32, id: uuid::Uuid, topic: String) -> SearchClusterTopics {
        SearchClusterTopics {
            avg_score,
            created_at,
            dataset_id,
            density,
            id,
            topic,
        }
    }
}

