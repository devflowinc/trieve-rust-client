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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchWithinGroupResponseTypes {
    SearchGroupsResult(Box<models::SearchGroupsResult>),
    SearchGroupSlimChunksResult(Box<models::SearchGroupSlimChunksResult>),
}

impl Default for SearchWithinGroupResponseTypes {
    fn default() -> Self {
        Self::SearchGroupsResult(Default::default())
    }
}

