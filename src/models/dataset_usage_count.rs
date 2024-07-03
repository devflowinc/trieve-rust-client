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
pub struct DatasetUsageCount {
    #[serde(rename = "chunk_count")]
    pub chunk_count: i32,
    #[serde(rename = "dataset_id")]
    pub dataset_id: uuid::Uuid,
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
}

impl DatasetUsageCount {
    pub fn new(chunk_count: i32, dataset_id: uuid::Uuid, id: uuid::Uuid) -> DatasetUsageCount {
        DatasetUsageCount {
            chunk_count,
            dataset_id,
            id,
        }
    }
}

