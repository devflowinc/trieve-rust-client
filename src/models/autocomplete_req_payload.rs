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
pub struct AutocompleteReqPayload {
    /// Set content_only to true to only returning the chunk_html of the chunks. This is useful for when you want to reduce amount of data over the wire for latency improvement (typically 10-50ms). Default is false.
    #[serde(rename = "content_only", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub content_only: Option<Option<bool>>,
    /// If specified to true, this will extend the search results to include non-exact prefix matches of the same search_type such that a full page_size of results are returned. Default is false.
    #[serde(rename = "extend_results", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub extend_results: Option<Option<bool>>,
    #[serde(rename = "filters", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub filters: Option<Option<Box<models::ChunkFilter>>>,
    #[serde(rename = "highlight_options", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub highlight_options: Option<Option<Box<models::HighlightOptions>>>,
    /// Page size is the number of chunks to fetch. This can be used to fetch more than 10 chunks at a time.
    #[serde(rename = "page_size", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<Option<i64>>,
    /// Query is the search query. This can be any string. The query will be used to create an embedding vector and/or SPLADE vector which will be used to find the result set.
    #[serde(rename = "query")]
    pub query: String,
    /// If true, stop words (specified in server/src/stop-words.txt in the git repo) will be removed. Queries that are entirely stop words will be preserved.
    #[serde(rename = "remove_stop_words", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub remove_stop_words: Option<Option<bool>>,
    /// Set score_threshold to a float to filter out chunks with a score below the threshold. This threshold applies before weight and bias modifications. If not specified, this defaults to 0.0.
    #[serde(rename = "score_threshold", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub score_threshold: Option<Option<f32>>,
    #[serde(rename = "search_type")]
    pub search_type: models::SearchMethod,
    /// Set slim_chunks to true to avoid returning the content and chunk_html of the chunks. This is useful for when you want to reduce amount of data over the wire for latency improvement (typically 10-50ms). Default is false.
    #[serde(rename = "slim_chunks", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub slim_chunks: Option<Option<bool>>,
    #[serde(rename = "sort_options", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub sort_options: Option<Option<Box<models::SortOptions>>>,
    /// If true, quoted and - prefixed words will be parsed from the queries and used as required and negated words respectively. Default is false.
    #[serde(rename = "use_quote_negated_terms", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub use_quote_negated_terms: Option<Option<bool>>,
    /// User ID is the id of the user who is making the request. This is used to track user interactions with the search results.
    #[serde(rename = "user_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Option<String>>,
}

impl AutocompleteReqPayload {
    pub fn new(query: String, search_type: models::SearchMethod) -> AutocompleteReqPayload {
        AutocompleteReqPayload {
            content_only: None,
            extend_results: None,
            filters: None,
            highlight_options: None,
            page_size: None,
            query,
            remove_stop_words: None,
            score_threshold: None,
            search_type,
            slim_chunks: None,
            sort_options: None,
            use_quote_negated_terms: None,
            user_id: None,
        }
    }
}

