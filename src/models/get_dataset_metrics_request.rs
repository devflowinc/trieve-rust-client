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
pub struct GetDatasetMetricsRequest {
    #[serde(rename = "filter", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub filter: Option<Option<Box<models::SearchAnalyticsFilter>>>,
}

impl GetDatasetMetricsRequest {
    pub fn new() -> GetDatasetMetricsRequest {
        GetDatasetMetricsRequest {
            filter: None,
        }
    }
}
