/*
 * Trieve API
 *
 * Trieve OpenAPI Specification. This document describes all of the operations available through the Trieve API.
 *
 * The version of the OpenAPI document: 0.8.5
 * Contact: developers@trieve.ai
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SearchWithinGroupData {
    #[serde(rename = "filters", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub filters: Option<Option<Box<models::ChunkFilter>>>,
    /// Get total page count for the query accounting for the applied filters. Defaults to false, but can be set to true when the latency penalty is acceptable (typically 50-200ms).
    #[serde(rename = "get_total_pages", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub get_total_pages: Option<Option<bool>>,
    /// Group specifies the group to search within. Results will only consist of chunks which are bookmarks within the specified group.
    #[serde(rename = "group_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<Option<uuid::Uuid>>,
    /// Group_tracking_id specifies the group to search within by tracking id. Results will only consist of chunks which are bookmarks within the specified group. If both group_id and group_tracking_id are provided, group_id will be used.
    #[serde(rename = "group_tracking_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub group_tracking_id: Option<Option<String>>,
    /// Set highlight_delimiters to a list of strings to use as delimiters for highlighting. If not specified, this defaults to [\"?\", \",\", \".\", \"!\"].
    #[serde(rename = "highlight_delimiters", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub highlight_delimiters: Option<Option<Vec<String>>>,
    /// Set highlight_results to false for a slight latency improvement (1-10ms). If not specified, this defaults to true. This will add `<b><mark>` tags to the chunk_html of the chunks to highlight matching sub-sentences.
    #[serde(rename = "highlight_results", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub highlight_results: Option<Option<bool>>,
    /// Set highlight_threshold to a lower or higher value to adjust the sensitivity of the highlights applied to the chunk html. If not specified, this defaults to 0.8. The range is 0.0 to 1.0.
    #[serde(rename = "highlight_threshold", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub highlight_threshold: Option<Option<f64>>,
    /// The page of chunks to fetch. Page is 1-indexed.
    #[serde(rename = "page", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub page: Option<Option<i64>>,
    /// The page size is the number of chunks to fetch. This can be used to fetch more than 10 chunks at a time.
    #[serde(rename = "page_size", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<Option<i64>>,
    /// The query is the search query. This can be any string. The query will be used to create an embedding vector and/or SPLADE vector which will be used to find the result set.
    #[serde(rename = "query")]
    pub query: String,
    /// Recency Bias lets you determine how much of an effect the recency of chunks will have on the search results. If not specified, this defaults to 0.0.
    #[serde(rename = "recency_bias", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub recency_bias: Option<Option<f32>>,
    /// Set score_threshold to a float to filter out chunks with a score below the threshold.
    #[serde(rename = "score_threshold", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub score_threshold: Option<Option<f32>>,
    /// Search_type can be either \"semantic\", \"fulltext\", or \"hybrid\". \"hybrid\" will pull in one page (10 chunks) of both semantic and full-text results then re-rank them using BAAI/bge-reranker-large. \"semantic\" will pull in one page (10 chunks) of the nearest cosine distant vectors. \"fulltext\" will pull in one page (10 chunks) of full-text results based on SPLADE.
    #[serde(rename = "search_type")]
    pub search_type: String,
    /// Set slim_chunks to true to avoid returning the content and chunk_html of the chunks. This is useful for when you want to reduce amount of data over the wire for latency improvement (typicall 10-50ms). Default is false.
    #[serde(rename = "slim_chunks", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub slim_chunks: Option<Option<bool>>,
    /// Tag weights is a JSON object which can be used to boost the ranking of chunks with certain tags. This is useful for when you want to be able to bias towards chunks with a certain tag on the fly. The keys are the tag names and the values are the weights.
    #[serde(rename = "tag_weights", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tag_weights: Option<Option<std::collections::HashMap<String, f32>>>,
    /// Set use_weights to true to use the weights of the chunks in the result set in order to sort them. If not specified, this defaults to true.
    #[serde(rename = "use_weights", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub use_weights: Option<Option<bool>>,
}

impl SearchWithinGroupData {
    pub fn new(query: String, search_type: String) -> SearchWithinGroupData {
        SearchWithinGroupData {
            filters: None,
            get_total_pages: None,
            group_id: None,
            group_tracking_id: None,
            highlight_delimiters: None,
            highlight_results: None,
            highlight_threshold: None,
            page: None,
            page_size: None,
            query,
            recency_bias: None,
            score_threshold: None,
            search_type,
            slim_chunks: None,
            tag_weights: None,
            use_weights: None,
        }
    }
}

