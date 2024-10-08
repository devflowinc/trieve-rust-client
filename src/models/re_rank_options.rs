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
pub enum ReRankOptions {
    #[serde(rename = "semantic")]
    Semantic,
    #[serde(rename = "fulltext")]
    Fulltext,
    #[serde(rename = "bm25")]
    Bm25,
    #[serde(rename = "cross_encoder")]
    CrossEncoder,

}

impl ToString for ReRankOptions {
    fn to_string(&self) -> String {
        match self {
            Self::Semantic => String::from("semantic"),
            Self::Fulltext => String::from("fulltext"),
            Self::Bm25 => String::from("bm25"),
            Self::CrossEncoder => String::from("cross_encoder"),
        }
    }
}

impl Default for ReRankOptions {
    fn default() -> ReRankOptions {
        Self::Semantic
    }
}

