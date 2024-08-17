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

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RagSortBy {
    #[serde(rename = "created_at")]
    CreatedAt,
    #[serde(rename = "latency")]
    Latency,

}

impl ToString for RagSortBy {
    fn to_string(&self) -> String {
        match self {
            Self::CreatedAt => String::from("created_at"),
            Self::Latency => String::from("latency"),
        }
    }
}

impl Default for RagSortBy {
    fn default() -> RagSortBy {
        Self::CreatedAt
    }
}

