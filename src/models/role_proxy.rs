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
pub enum RoleProxy {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,

}

impl ToString for RoleProxy {
    fn to_string(&self) -> String {
        match self {
            Self::System => String::from("system"),
            Self::User => String::from("user"),
            Self::Assistant => String::from("assistant"),
        }
    }
}

impl Default for RoleProxy {
    fn default() -> RoleProxy {
        Self::System
    }
}

