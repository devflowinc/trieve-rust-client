/*
 * Trieve API
 *
 * Trieve OpenAPI Specification. This document describes all of the operations available through the Trieve API.
 *
 * The version of the OpenAPI document: 0.10.9
 * Contact: developers@trieve.ai
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SearchType {
    #[serde(rename = "search")]
    Search,
    #[serde(rename = "autocomplete")]
    Autocomplete,
    #[serde(rename = "search_over_groups")]
    SearchOverGroups,
    #[serde(rename = "search_within_groups")]
    SearchWithinGroups,

}

impl ToString for SearchType {
    fn to_string(&self) -> String {
        match self {
            Self::Search => String::from("search"),
            Self::Autocomplete => String::from("autocomplete"),
            Self::SearchOverGroups => String::from("search_over_groups"),
            Self::SearchWithinGroups => String::from("search_within_groups"),
        }
    }
}

impl Default for SearchType {
    fn default() -> SearchType {
        Self::Search
    }
}
