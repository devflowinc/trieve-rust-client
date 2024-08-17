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

/// RecommendType : The type of recommendation to make. This lets you choose whether to recommend based off of `semantic` or `fulltext` similarity. The default is `semantic`.
/// The type of recommendation to make. This lets you choose whether to recommend based off of `semantic` or `fulltext` similarity. The default is `semantic`.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RecommendType {
    #[serde(rename = "semantic")]
    Semantic,
    #[serde(rename = "fulltext")]
    Fulltext,
    #[serde(rename = "bm25")]
    Bm25,

}

impl ToString for RecommendType {
    fn to_string(&self) -> String {
        match self {
            Self::Semantic => String::from("semantic"),
            Self::Fulltext => String::from("fulltext"),
            Self::Bm25 => String::from("bm25"),
        }
    }
}

impl Default for RecommendType {
    fn default() -> RecommendType {
        Self::Semantic
    }
}

