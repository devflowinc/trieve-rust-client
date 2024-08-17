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
pub struct RecommendationsWithoutClicks {
    #[serde(rename = "filter", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub filter: Option<Option<Box<models::RecommendationAnalyticsFilter>>>,
    #[serde(rename = "page", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub page: Option<Option<i32>>,
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl RecommendationsWithoutClicks {
    pub fn new(r#type: Type) -> RecommendationsWithoutClicks {
        RecommendationsWithoutClicks {
            filter: None,
            page: None,
            r#type,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "recommendations_without_clicks")]
    RecommendationsWithoutClicks,
}

impl Default for Type {
    fn default() -> Type {
        Self::RecommendationsWithoutClicks
    }
}
