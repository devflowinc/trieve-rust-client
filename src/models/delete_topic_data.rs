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
pub struct DeleteTopicData {
    /// The id of the topic to target.
    #[serde(rename = "topic_id")]
    pub topic_id: uuid::Uuid,
}

impl DeleteTopicData {
    pub fn new(topic_id: uuid::Uuid) -> DeleteTopicData {
        DeleteTopicData {
            topic_id,
        }
    }
}

