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
pub struct DeleteUserApiKeyRequest {
    /// The id of the api key to delete.
    #[serde(rename = "api_key_id")]
    pub api_key_id: uuid::Uuid,
}

impl DeleteUserApiKeyRequest {
    pub fn new(api_key_id: uuid::Uuid) -> DeleteUserApiKeyRequest {
        DeleteUserApiKeyRequest {
            api_key_id,
        }
    }
}

