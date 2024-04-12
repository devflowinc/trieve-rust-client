/*
 * Trieve API
 *
 * Trieve OpenAPI Specification. This document describes all of the operations available through the Trieve API.
 *
 * The version of the OpenAPI document: 0.6.5
 * Contact: developers@trieve.ai
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChatMessageProxy {
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "role")]
    pub role: String,
}

impl ChatMessageProxy {
    pub fn new(content: String, role: String) -> ChatMessageProxy {
        ChatMessageProxy {
            content,
            role,
        }
    }
}

