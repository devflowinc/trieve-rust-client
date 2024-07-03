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
pub struct ApiKeyRespBody {
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "dataset_ids", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub dataset_ids: Option<Option<Vec<String>>>,
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "organization_ids", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub organization_ids: Option<Option<Vec<String>>>,
    #[serde(rename = "role")]
    pub role: i32,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "user_id")]
    pub user_id: uuid::Uuid,
}

impl ApiKeyRespBody {
    pub fn new(created_at: String, id: uuid::Uuid, name: String, role: i32, updated_at: String, user_id: uuid::Uuid) -> ApiKeyRespBody {
        ApiKeyRespBody {
            created_at,
            dataset_ids: None,
            id,
            name,
            organization_ids: None,
            role,
            updated_at,
            user_id,
        }
    }
}

