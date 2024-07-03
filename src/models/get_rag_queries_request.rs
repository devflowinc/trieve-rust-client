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
pub struct GetRagQueriesRequest {
    #[serde(rename = "filter", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub filter: Option<Option<Box<models::RagAnalyticsFilter>>>,
    #[serde(rename = "page", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub page: Option<Option<i32>>,
    #[serde(rename = "sort_by", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<Option<String>>,
    #[serde(rename = "sort_order", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<Option<String>>,
}

impl GetRagQueriesRequest {
    pub fn new() -> GetRagQueriesRequest {
        GetRagQueriesRequest {
            filter: None,
            page: None,
            sort_by: None,
            sort_order: None,
        }
    }
}

