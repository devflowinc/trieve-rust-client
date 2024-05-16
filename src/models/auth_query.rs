/*
 * Trieve API
 *
 * Trieve OpenAPI Specification. This document describes all of the operations available through the Trieve API.
 *
 * The version of the OpenAPI document: 0.8.5
 * Contact: developers@trieve.ai
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AuthQuery {
    /// Code sent via email as a result of successful call to send_invitation
    #[serde(rename = "inv_code", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub inv_code: Option<Option<uuid::Uuid>>,
    /// ID of organization to authenticate into
    #[serde(rename = "organization_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<Option<uuid::Uuid>>,
    /// URL to redirect to after successful login
    #[serde(rename = "redirect_uri", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub redirect_uri: Option<Option<String>>,
}

impl AuthQuery {
    pub fn new() -> AuthQuery {
        AuthQuery {
            inv_code: None,
            organization_id: None,
            redirect_uri: None,
        }
    }
}

