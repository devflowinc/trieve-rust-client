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
pub struct CreateDatasetRequest {
    /// Client configuration for the dataset, can be arbitrary JSON. We recommend setting to `{}` to start. See docs.trieve.ai for more information or adjust with the admin dashboard.
    #[serde(rename = "client_configuration", deserialize_with = "Option::deserialize")]
    pub client_configuration: Option<serde_json::Value>,
    /// Name of the dataset. Must be unique within the organization.
    #[serde(rename = "dataset_name")]
    pub dataset_name: String,
    /// Organization ID that the dataset will belong to.
    #[serde(rename = "organization_id")]
    pub organization_id: uuid::Uuid,
    /// Server configuration for the dataset, can be arbitrary JSON. We recommend setting to `{}` to start. See docs.trieve.ai for more information or adjust with the admin dashboard.
    #[serde(rename = "server_configuration", deserialize_with = "Option::deserialize")]
    pub server_configuration: Option<serde_json::Value>,
    /// Optional tracking ID for the dataset. Can be used to track the dataset in external systems.
    #[serde(rename = "tracking_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<Option<String>>,
}

impl CreateDatasetRequest {
    pub fn new(client_configuration: Option<serde_json::Value>, dataset_name: String, organization_id: uuid::Uuid, server_configuration: Option<serde_json::Value>) -> CreateDatasetRequest {
        CreateDatasetRequest {
            client_configuration,
            dataset_name,
            organization_id,
            server_configuration,
            tracking_id: None,
        }
    }
}

