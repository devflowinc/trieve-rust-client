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
pub struct SearchUsageGraph {
    #[serde(rename = "filter", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub filter: Option<Option<Box<models::SearchAnalyticsFilter>>>,
    #[serde(rename = "granularity", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub granularity: Option<Option<models::Granularity>>,
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl SearchUsageGraph {
    pub fn new(r#type: Type) -> SearchUsageGraph {
        SearchUsageGraph {
            filter: None,
            granularity: None,
            r#type,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "search_usage_graph")]
    SearchUsageGraph,
}

impl Default for Type {
    fn default() -> Type {
        Self::SearchUsageGraph
    }
}
