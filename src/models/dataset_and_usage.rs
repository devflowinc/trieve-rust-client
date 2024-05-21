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
pub struct DatasetAndUsage {
    #[serde(rename = "dataset")]
    pub dataset: Box<models::DatasetDto>,
    #[serde(rename = "dataset_usage")]
    pub dataset_usage: Box<models::DatasetUsageCount>,
}

impl DatasetAndUsage {
    pub fn new(dataset: models::DatasetDto, dataset_usage: models::DatasetUsageCount) -> DatasetAndUsage {
        DatasetAndUsage {
            dataset: Box::new(dataset),
            dataset_usage: Box::new(dataset_usage),
        }
    }
}

