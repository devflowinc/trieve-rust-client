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
pub struct RagQueries {
    #[serde(rename = "filter", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub filter: Option<Option<Box<models::RagAnalyticsFilter>>>,
    #[serde(rename = "page", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub page: Option<Option<i32>>,
    #[serde(rename = "sort_by", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<Option<models::SortBy>>,
    #[serde(rename = "sort_order", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<Option<models::SortOrder>>,
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl RagQueries {
    pub fn new(r#type: Type) -> RagQueries {
        RagQueries {
            filter: None,
            page: None,
            sort_by: None,
            sort_order: None,
            r#type,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "rag_queries")]
    RagQueries,
}

impl Default for Type {
    fn default() -> Type {
        Self::RagQueries
    }
}

