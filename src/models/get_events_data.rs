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
pub struct GetEventsData {
    /// The types of events to get. Any combination of file_uploaded, card_uploaded, card_action_failed, or card_updated. Leave undefined to get all events.
    #[serde(rename = "event_types", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Option<Vec<String>>>,
    /// The page number to get. Default is 1.
    #[serde(rename = "page", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub page: Option<Option<i64>>,
    /// The number of items per page. Default is 10.
    #[serde(rename = "page_size", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<Option<i64>>,
}

impl GetEventsData {
    pub fn new() -> GetEventsData {
        GetEventsData {
            event_types: None,
            page: None,
            page_size: None,
        }
    }
}

