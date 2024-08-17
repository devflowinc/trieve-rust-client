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
pub enum RagTypes {
    #[serde(rename = "chosen_chunks")]
    ChosenChunks,
    #[serde(rename = "all_chunks")]
    AllChunks,

}

impl ToString for RagTypes {
    fn to_string(&self) -> String {
        match self {
            Self::ChosenChunks => String::from("chosen_chunks"),
            Self::AllChunks => String::from("all_chunks"),
        }
    }
}

impl Default for RagTypes {
    fn default() -> RagTypes {
        Self::ChosenChunks
    }
}

