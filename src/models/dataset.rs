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
pub struct Dataset {
    #[serde(rename = "client_configuration", deserialize_with = "Option::deserialize")]
    pub client_configuration: Option<serde_json::Value>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "organization_id")]
    pub organization_id: uuid::Uuid,
    #[serde(rename = "server_configuration", deserialize_with = "Option::deserialize")]
    pub server_configuration: Option<serde_json::Value>,
    #[serde(rename = "tracking_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<Option<String>>,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl Dataset {
    pub fn new(client_configuration: Option<serde_json::Value>, created_at: String, id: uuid::Uuid, name: String, organization_id: uuid::Uuid, server_configuration: Option<serde_json::Value>, updated_at: String) -> Dataset {
        Dataset {
            client_configuration,
            created_at,
            id,
            name,
            organization_id,
            server_configuration,
            tracking_id: None,
            updated_at,
        }
    }
}

