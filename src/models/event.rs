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
pub struct Event {
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "dataset_id")]
    pub dataset_id: uuid::Uuid,
    #[serde(rename = "event_data", deserialize_with = "Option::deserialize")]
    pub event_data: Option<serde_json::Value>,
    #[serde(rename = "event_type")]
    pub event_type: String,
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl Event {
    pub fn new(created_at: String, dataset_id: uuid::Uuid, event_data: Option<serde_json::Value>, event_type: String, id: uuid::Uuid, updated_at: String) -> Event {
        Event {
            created_at,
            dataset_id,
            event_data,
            event_type,
            id,
            updated_at,
        }
    }
}

