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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CtrAnalyticsResponse {
    SearchCtrMetrics(Box<models::SearchCtrMetrics>),
    CtrSearchQueryWithoutClicksResponse(Box<models::CtrSearchQueryWithoutClicksResponse>),
    CtrSearchQueryWithClicksResponse(Box<models::CtrSearchQueryWithClicksResponse>),
    RecommendationCtrMetrics(Box<models::RecommendationCtrMetrics>),
    CtrRecommendationsWithoutClicksResponse(Box<models::CtrRecommendationsWithoutClicksResponse>),
    CtrRecommendationsWithClicksResponse(Box<models::CtrRecommendationsWithClicksResponse>),
}

impl Default for CtrAnalyticsResponse {
    fn default() -> Self {
        Self::SearchCtrMetrics(Default::default())
    }
}

