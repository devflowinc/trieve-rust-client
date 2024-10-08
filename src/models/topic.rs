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
pub struct Topic {
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "dataset_id")]
    pub dataset_id: uuid::Uuid,
    #[serde(rename = "deleted")]
    pub deleted: bool,
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "owner_id")]
    pub owner_id: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl Topic {
    pub fn new(created_at: String, dataset_id: uuid::Uuid, deleted: bool, id: uuid::Uuid, name: String, owner_id: String, updated_at: String) -> Topic {
        Topic {
            created_at,
            dataset_id,
            deleted,
            id,
            name,
            owner_id,
            updated_at,
        }
    }
}

