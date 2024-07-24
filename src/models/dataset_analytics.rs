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
pub struct DatasetAnalytics {
    #[serde(rename = "avg_latency")]
    pub avg_latency: f64,
    #[serde(rename = "p50")]
    pub p50: f64,
    #[serde(rename = "p95")]
    pub p95: f64,
    #[serde(rename = "p99")]
    pub p99: f64,
    #[serde(rename = "search_rps")]
    pub search_rps: f64,
    #[serde(rename = "total_queries")]
    pub total_queries: i32,
}

impl DatasetAnalytics {
    pub fn new(avg_latency: f64, p50: f64, p95: f64, p99: f64, search_rps: f64, total_queries: i32) -> DatasetAnalytics {
        DatasetAnalytics {
            avg_latency,
            p50,
            p95,
            p99,
            search_rps,
            total_queries,
        }
    }
}

