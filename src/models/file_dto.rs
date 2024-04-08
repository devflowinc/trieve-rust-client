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
pub struct FileDto {
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "file_name")]
    pub file_name: String,
    #[serde(rename = "id")]
    pub id: uuid::Uuid,
    #[serde(rename = "link", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub link: Option<Option<String>>,
    #[serde(rename = "metadata", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Option<serde_json::Value>>,
    #[serde(rename = "s3_url")]
    pub s3_url: String,
    #[serde(rename = "size")]
    pub size: i64,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
}

impl FileDto {
    pub fn new(created_at: String, file_name: String, id: uuid::Uuid, s3_url: String, size: i64, updated_at: String) -> FileDto {
        FileDto {
            created_at,
            file_name,
            id,
            link: None,
            metadata: None,
            s3_url,
            size,
            updated_at,
        }
    }
}
