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
pub struct CreateDatasetRequest {
    /// Name of the dataset.
    #[serde(rename = "dataset_name")]
    pub dataset_name: String,
    /// Organization ID that the dataset will belong to.
    #[serde(rename = "organization_id")]
    pub organization_id: uuid::Uuid,
    #[serde(rename = "server_configuration", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub server_configuration: Option<Option<Box<models::DatasetConfigurationDto>>>,
    /// Optional tracking ID for the dataset. Can be used to track the dataset in external systems. Must be unique within the organization. Strongly recommended to not use a valid uuid value as that will not work with the TR-Dataset header.
    #[serde(rename = "tracking_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tracking_id: Option<Option<String>>,
}

impl CreateDatasetRequest {
    pub fn new(dataset_name: String, organization_id: uuid::Uuid) -> CreateDatasetRequest {
        CreateDatasetRequest {
            dataset_name,
            organization_id,
            server_configuration: None,
            tracking_id: None,
        }
    }
}

