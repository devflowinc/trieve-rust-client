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
pub struct UpdateTopicData {
    /// The new name of the topic. A name is not generated from this field, it is used as-is.
    #[serde(rename = "name")]
    pub name: String,
    /// The id of the topic to target.
    #[serde(rename = "topic_id")]
    pub topic_id: uuid::Uuid,
}

impl UpdateTopicData {
    pub fn new(name: String, topic_id: uuid::Uuid) -> UpdateTopicData {
        UpdateTopicData {
            name,
            topic_id,
        }
    }
}

