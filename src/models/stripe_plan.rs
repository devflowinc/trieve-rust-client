/*
 * Trieve API
 *
 * Trieve OpenAPI Specification. This document describes all of the operations available through the Trieve API.
 *
 * The version of the OpenAPI document: 0.8.5
 * Contact: developers@trieve.ai
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StripePlan {
    #[serde(rename = "amount")]
    pub amount: i64,
    #[serde(rename = "chunk_count")]
    pub chunk_count: i32,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "dataset_count")]
    pub dataset_count: i32,
    #[serde(rename = "file_storage")]
    pub file_storage: i64,
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "message_count")]
    pub message_count: i32,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "stripe_id")]
    pub stripe_id: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "user_count")]
    pub user_count: i32,
}

impl StripePlan {
    pub fn new(amount: i64, chunk_count: i32, created_at: String, dataset_count: i32, file_storage: i64, id: uuid::Uuid, message_count: i32, name: String, stripe_id: String, updated_at: String, user_count: i32) -> StripePlan {
        StripePlan {
            amount,
            chunk_count,
            created_at,
            dataset_count,
            file_storage,
            id,
            message_count,
            name,
            stripe_id,
            updated_at,
            user_count,
        }
    }
}

