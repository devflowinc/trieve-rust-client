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
pub struct QueryDetails {
    #[serde(rename = "search_id")]
    pub search_id: uuid::Uuid,
    #[serde(rename = "type")]
    pub r#type: Type,
}

impl QueryDetails {
    pub fn new(search_id: uuid::Uuid, r#type: Type) -> QueryDetails {
        QueryDetails {
            search_id,
            r#type,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "query_details")]
    QueryDetails,
}

impl Default for Type {
    fn default() -> Type {
        Self::QueryDetails
    }
}
