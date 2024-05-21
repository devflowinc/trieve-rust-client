/*
 * Trieve API
 *
 * Trieve OpenAPI Specification. This document describes all of the operations available through the Trieve API.
 *
 * The version of the OpenAPI document: 0.8.7
 * Contact: developers@trieve.ai
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InvitationData {
    /// The url of the app that the user will be directed to in order to set their password. Usually admin.trieve.ai, but may differ for local dev or self-hosted setups.
    #[serde(rename = "app_url")]
    pub app_url: String,
    /// The email of the user to invite. Must be a valid email as they will be sent an email to register.
    #[serde(rename = "email")]
    pub email: String,
    /// The id of the organization to invite the user to.
    #[serde(rename = "organization_id")]
    pub organization_id: uuid::Uuid,
    /// The url that the user will be redirected to after setting their password.
    #[serde(rename = "redirect_uri")]
    pub redirect_uri: String,
    /// The role the user will have in the organization. 0 = User, 1 = Admin, 2 = Owner.
    #[serde(rename = "user_role")]
    pub user_role: i32,
}

impl InvitationData {
    pub fn new(app_url: String, email: String, organization_id: uuid::Uuid, redirect_uri: String, user_role: i32) -> InvitationData {
        InvitationData {
            app_url,
            email,
            organization_id,
            redirect_uri,
            user_role,
        }
    }
}

