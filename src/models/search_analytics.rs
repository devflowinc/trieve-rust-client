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
#[serde(tag = "type")]
pub enum SearchAnalytics {
    #[serde(rename="LatencyGraph")]
    LatencyGraph(Box<models::LatencyGraph>),
    #[serde(rename="SearchUsageGraph")]
    SearchUsageGraph(Box<models::SearchUsageGraph>),
    #[serde(rename="SearchMetrics")]
    SearchMetrics(Box<models::SearchMetrics>),
    #[serde(rename="HeadQueries_1")]
    HeadQueries1(Box<models::HeadQueries1>),
    #[serde(rename="LowConfidenceQueries")]
    LowConfidenceQueries(Box<models::LowConfidenceQueries>),
    #[serde(rename="NoResultQueries")]
    NoResultQueries(Box<models::NoResultQueries>),
    #[serde(rename="SearchQueries")]
    SearchQueries(Box<models::SearchQueries>),
    #[serde(rename="CountQueries")]
    CountQueries(Box<models::CountQueries>),
    #[serde(rename="QueryDetails")]
    QueryDetails(Box<models::QueryDetails>),
    #[serde(rename="PopularFilters_1")]
    PopularFilters1(Box<models::PopularFilters1>),
}

impl Default for SearchAnalytics {
    fn default() -> Self {
        Self::LatencyGraph(Default::default())
    }
}


