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

/// ChunkFilter : Filters is a JSON object which can be used to filter chunks. This is useful for when you want to filter chunks by arbitrary metadata. Unlike with tag filtering, there is a performance hit for filtering on metadata.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChunkFilter {
    /// JOSNB prefilter tells the server to perform a full scan over the metadata JSONB column instead of using the filtered HNSW. Datasets on the enterprise plan with custom metadata indices will perform better with the filtered HNSW instead. When false, the server will use the filtered HNSW index to filter chunks. When true, the server will perform a full scan over the metadata JSONB column to filter chunks. Default is true.
    #[serde(rename = "jsonb_prefilter", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub jsonb_prefilter: Option<Option<bool>>,
    /// All of these field conditions have to match for the chunk to be included in the result set.
    #[serde(rename = "must", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub must: Option<Option<Vec<models::ConditionType>>>,
    /// None of these field conditions can match for the chunk to be included in the result set.
    #[serde(rename = "must_not", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub must_not: Option<Option<Vec<models::ConditionType>>>,
    /// Only one of these field conditions has to match for the chunk to be included in the result set.
    #[serde(rename = "should", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub should: Option<Option<Vec<models::ConditionType>>>,
}

impl ChunkFilter {
    /// Filters is a JSON object which can be used to filter chunks. This is useful for when you want to filter chunks by arbitrary metadata. Unlike with tag filtering, there is a performance hit for filtering on metadata.
    pub fn new() -> ChunkFilter {
        ChunkFilter {
            jsonb_prefilter: None,
            must: None,
            must_not: None,
            should: None,
        }
    }
}

