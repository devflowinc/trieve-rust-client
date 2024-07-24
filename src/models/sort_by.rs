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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SortBy {
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "latency")]
    Latency,
    #[serde(rename = "top_score")]
    TopScore,

}

impl ToString for SortBy {
    fn to_string(&self) -> String {
        match self {
            Self::CreatedAt => String::from("created_at"),
            Self::Latency => String::from("latency"),
            Self::TopScore => String::from("top_score"),
        }
    }
}

impl Default for SortBy {
    fn default() -> SortBy {
        Self::CreatedAt
    }
}

