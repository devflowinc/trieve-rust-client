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
pub struct File {
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "dataset_id")]
    pub dataset_id: uuid::Uuid,
    #[serde(rename = "file_name")]
    pub file_name: String,
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "link", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub link: Option<Option<String>>,
    #[serde(rename = "metadata", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Option<serde_json::Value>>,
    #[serde(rename = "size")]
    pub size: i64,
    #[serde(rename = "tag_set", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tag_set: Option<Option<Vec<String>>>,
    #[serde(rename = "time_stamp", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<Option<String>>,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl File {
    pub fn new(created_at: String, dataset_id: uuid::Uuid, file_name: String, id: uuid::Uuid, size: i64, updated_at: String) -> File {
        File {
            created_at,
            dataset_id,
            file_name,
            id,
            link: None,
            metadata: None,
            size,
            tag_set: None,
            time_stamp: None,
            updated_at,
        }
    }
}

