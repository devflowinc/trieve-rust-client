/*
 * Trieve API
 *
 * Trieve OpenAPI Specification. This document describes all of the operations available through the Trieve API.
 *
 * The version of the OpenAPI document: 0.11.6
 * Contact: developers@trieve.ai
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetGroupsForChunksReqPayload {
    #[serde(rename = "chunk_ids")]
    pub chunk_ids: Vec<uuid::Uuid>,
}

impl GetGroupsForChunksReqPayload {
    pub fn new(chunk_ids: Vec<uuid::Uuid>) -> GetGroupsForChunksReqPayload {
        GetGroupsForChunksReqPayload {
            chunk_ids,
        }
    }
}
