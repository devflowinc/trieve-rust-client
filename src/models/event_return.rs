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
pub struct EventReturn {
    #[serde(rename = "events")]
    pub events: Vec<models::WorkerEvent>,
    #[serde(rename = "page_count")]
    pub page_count: i32,
}

impl EventReturn {
    pub fn new(events: Vec<models::WorkerEvent>, page_count: i32) -> EventReturn {
        EventReturn {
            events,
            page_count,
        }
    }
}

