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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CreateTopicReqPayload {
    /// The first message which will belong to the topic. The topic name is generated based on this message similar to how it works in the OpenAI chat UX if a name is not explicitly provided on the name request body key.
    #[serde(rename = "first_user_message", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub first_user_message: Option<Option<String>>,
    /// The name of the topic. If this is not provided, the topic name is generated from the first_user_message.
    #[serde(rename = "name", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub name: Option<Option<String>>,
    /// The owner_id of the topic. This is typically a browser fingerprint or your user's id. It is used to group topics together for a user.
    #[serde(rename = "owner_id")]
    pub owner_id: String,
}

impl CreateTopicReqPayload {
    pub fn new(owner_id: String) -> CreateTopicReqPayload {
        CreateTopicReqPayload {
            first_user_message: None,
            name: None,
            owner_id,
        }
    }
}

