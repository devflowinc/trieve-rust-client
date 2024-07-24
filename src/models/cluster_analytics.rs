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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClusterAnalytics {
    #[serde(rename="ClusterTopics")]
    ClusterTopics(Box<models::ClusterTopics>),
    #[serde(rename="ClusterQueries")]
    ClusterQueries(Box<models::ClusterQueries>),
}

impl Default for ClusterAnalytics {
    fn default() -> Self {
        Self::ClusterTopics(Default::default())
    }
}


