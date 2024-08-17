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
pub struct CtrRecommendationsWithoutClicksResponse {
    #[serde(rename = "recommendations")]
    pub recommendations: Vec<models::RecommendationsWithoutClicksCtrResponse>,
}

impl CtrRecommendationsWithoutClicksResponse {
    pub fn new(recommendations: Vec<models::RecommendationsWithoutClicksCtrResponse>) -> CtrRecommendationsWithoutClicksResponse {
        CtrRecommendationsWithoutClicksResponse {
            recommendations,
        }
    }
}
