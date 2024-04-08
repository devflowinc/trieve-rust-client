/*
 * Trieve API
 *
 * Trieve OpenAPI Specification. This document describes all of the operations available through the Trieve API.
 *
 * The version of the OpenAPI document: 0.5.8
 * Contact: developers@trieve.ai
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SetUserApiKeyResponse {
    /// The api key which was created. This is the value which should be used in the Authorization header.
    #[serde(rename = "api_key")]
    pub api_key: String,
}

impl SetUserApiKeyResponse {
    pub fn new(api_key: String) -> SetUserApiKeyResponse {
        SetUserApiKeyResponse {
            api_key,
        }
    }
}
