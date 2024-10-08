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
pub struct SetUserApiKeyRequest {
    /// The dataset ids which the api key will have access to. If not provided or empty, the api key will have access to all datasets the auth'ed user has access to. If both dataset_ids and organization_ids are provided, the api key will have access to the intersection of the datasets and organizations.
    #[serde(rename = "dataset_ids", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub dataset_ids: Option<Option<Vec<uuid::Uuid>>>,
    /// The name which will be assigned to the new api key.
    #[serde(rename = "name")]
    pub name: String,
    /// The organization ids which the api key will have access to. If not provided or empty, the api key will have access to all organizations the auth'ed user has access to.
    #[serde(rename = "organization_ids", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub organization_ids: Option<Option<Vec<uuid::Uuid>>>,
    /// The role which will be assigned to the new api key. Either 0 (read), 1 (read and write at the level of the currently auth'ed user). The auth'ed user must have a role greater than or equal to the role being assigned which means they must be an admin (1) or owner (2) of the organization to assign write permissions with a role of 1.
    #[serde(rename = "role")]
    pub role: i32,
    /// The routes which the api key will have access to. If not provided or empty, the api key will have access to all routes the auth'ed user has access to. Specify the routes as a list of strings. For example, [\"GET /api/dataset\", \"POST /api/dataset\"].
    #[serde(rename = "scopes", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Option<Vec<String>>>,
}

impl SetUserApiKeyRequest {
    pub fn new(name: String, role: i32) -> SetUserApiKeyRequest {
        SetUserApiKeyRequest {
            dataset_ids: None,
            name,
            organization_ids: None,
            role,
            scopes: None,
        }
    }
}

