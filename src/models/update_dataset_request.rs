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
pub struct UpdateDatasetRequest {
    /// The id of the dataset you want to update.
    #[serde(rename = "dataset_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub dataset_id: Option<Option<uuid::Uuid>>,
    /// The new name of the dataset. Must be unique within the organization. If not provided, the name will not be updated.
    #[serde(rename = "dataset_name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub dataset_name: Option<Option<String>>,
    /// Optional new tracking ID for the dataset. Can be used to track the dataset in external systems. Must be unique within the organization. If not provided, the tracking ID will not be updated. Strongly recommended to not use a valid uuid value as that will not work with the TR-Dataset header.
    #[serde(rename = "new_tracking_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub new_tracking_id: Option<Option<String>>,
    /// The configuration of the dataset. See the example request payload for the potential keys which can be set. It is possible to break your dataset's functionality by erroneously updating this field. We recommend updating through the settings panel for your dataset at dashboard.trieve.ai.
    #[serde(rename = "server_configuration", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub server_configuration: Option<Option<serde_json::Value>>,
    /// The tracking ID of the dataset you want to update.
    #[serde(rename = "tracking_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<Option<String>>,
}

impl UpdateDatasetRequest {
    pub fn new() -> UpdateDatasetRequest {
        UpdateDatasetRequest {
            dataset_id: None,
            dataset_name: None,
            new_tracking_id: None,
            server_configuration: None,
            tracking_id: None,
        }
    }
}

