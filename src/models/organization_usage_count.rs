/*
 * Trieve API
 *
 * Trieve OpenAPI Specification. This document describes all of the operations available through the Trieve API.
 *
 * The version of the OpenAPI document: 0.5.8
 * Contact: developers@trieve.ai
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrganizationUsageCount {
    #[serde(rename = "dataset_count")]
    pub dataset_count: i32,
    #[serde(rename = "file_storage")]
    pub file_storage: i64,
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "message_count")]
    pub message_count: i32,
    #[serde(rename = "org_id")]
    pub org_id: uuid::Uuid,
    #[serde(rename = "user_count")]
    pub user_count: i32,
}

impl OrganizationUsageCount {
    pub fn new(dataset_count: i32, file_storage: i64, id: uuid::Uuid, message_count: i32, org_id: uuid::Uuid, user_count: i32) -> OrganizationUsageCount {
        OrganizationUsageCount {
            dataset_count,
            file_storage,
            id,
            message_count,
            org_id,
            user_count,
        }
    }
}

