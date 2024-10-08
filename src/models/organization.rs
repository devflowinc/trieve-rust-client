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
pub struct Organization {
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "deleted")]
    pub deleted: i32,
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "registerable", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub registerable: Option<Option<bool>>,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl Organization {
    pub fn new(created_at: String, deleted: i32, id: uuid::Uuid, name: String, updated_at: String) -> Organization {
        Organization {
            created_at,
            deleted,
            id,
            name,
            registerable: None,
            updated_at,
        }
    }
}

