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
pub struct RecommendationCtrMetrics {
    #[serde(rename = "avg_position_of_click")]
    pub avg_position_of_click: f64,
    #[serde(rename = "percent_recommendations_with_clicks")]
    pub percent_recommendations_with_clicks: f64,
    #[serde(rename = "recommendations_with_clicks")]
    pub recommendations_with_clicks: i64,
}

impl RecommendationCtrMetrics {
    pub fn new(avg_position_of_click: f64, percent_recommendations_with_clicks: f64, recommendations_with_clicks: i64) -> RecommendationCtrMetrics {
        RecommendationCtrMetrics {
            avg_position_of_click,
            percent_recommendations_with_clicks,
            recommendations_with_clicks,
        }
    }
}

