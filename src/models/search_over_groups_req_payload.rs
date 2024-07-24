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
pub struct SearchOverGroupsReqPayload {
    #[serde(rename = "filters", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub filters: Option<Option<Box<models::ChunkFilter>>>,
    /// Get total page count for the query accounting for the applied filters. Defaults to false, but can be set to true when the latency penalty is acceptable (typically 50-200ms).
    #[serde(rename = "get_total_pages", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub get_total_pages: Option<Option<bool>>,
    /// Group_size is the number of chunks to fetch for each group. The default is 3. If a group has less than group_size chunks, all chunks will be returned. If this is set to a large number, we recommend setting slim_chunks to true to avoid returning the content and chunk_html of the chunks so as to lower the amount of time required for content download and serialization.
    #[serde(rename = "group_size", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub group_size: Option<Option<i32>>,
    /// Set highlight_delimiters to a list of strings to use as delimiters for highlighting. If not specified, this defaults to [\"?\", \",\", \".\", \"!\"]. These are the characters that will be used to split the chunk_html into splits for highlighting.
    #[serde(rename = "highlight_delimiters", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub highlight_delimiters: Option<Option<Vec<String>>>,
    /// Set highlight_max_length to control the maximum number of tokens (typically whitespace separated strings, but sometimes also word stems) which can be present within a single highlight. If not specified, this defaults to 8. This is useful to shorten large splits which may have low scores due to length compared to the query. Set to something very large like 100 to highlight entire splits.
    #[serde(rename = "highlight_max_length", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub highlight_max_length: Option<Option<i32>>,
    /// Set highlight_max_num to control the maximum number of highlights per chunk. If not specified, this defaults to 3. It may be less than 3 if no snippets score above the highlight_threshold.
    #[serde(rename = "highlight_max_num", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub highlight_max_num: Option<Option<i32>>,
    /// Set highlight_results to false for a slight latency improvement (1-10ms). If not specified, this defaults to true. This will add `<b><mark>` tags to the chunk_html of the chunks to highlight matching splits and return the highlights on each scored chunk in the response.
    #[serde(rename = "highlight_results", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub highlight_results: Option<Option<bool>>,
    /// Set highlight_threshold to a lower or higher value to adjust the sensitivity of the highlights applied to the chunk html. If not specified, this defaults to 0.8. The range is 0.0 to 1.0.
    #[serde(rename = "highlight_threshold", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub highlight_threshold: Option<Option<f64>>,
    /// Set highlight_window to a number to control the amount of words that are returned around the matched phrases. If not specified, this defaults to 0. This is useful for when you want to show more context around the matched words. When specified, window/2 whitespace separated words are added before and after each highlight in the response's highlights array. If an extended highlight overlaps with another highlight, the overlapping words are only included once.
    #[serde(rename = "highlight_window", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub highlight_window: Option<Option<i32>>,
    /// Page of group results to fetch. Page is 1-indexed.
    #[serde(rename = "page", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub page: Option<Option<i64>>,
    /// Page size is the number of group results to fetch. The default is 10.
    #[serde(rename = "page_size", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<Option<i64>>,
    /// Query is the search query. This can be any string. The query will be used to create an embedding vector and/or SPLADE vector which will be used to find the result set.
    #[serde(rename = "query")]
    pub query: String,
    /// Set score_threshold to a float to filter out chunks with a score below the threshold. This threshold applies before weight and bias modifications. If not specified, this defaults to 0.0.
    #[serde(rename = "score_threshold", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub score_threshold: Option<Option<f32>>,
    #[serde(rename = "search_type")]
    pub search_type: models::SearchMethod,
    /// Set slim_chunks to true to avoid returning the content and chunk_html of the chunks. This is useful for when you want to reduce amount of data over the wire for latency improvement (typicall 10-50ms). Default is false.
    #[serde(rename = "slim_chunks", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub slim_chunks: Option<Option<bool>>,
    /// If true, quoted and - prefixed words will be parsed from the queries and used as required and negated words respectively. Default is false.
    #[serde(rename = "use_quote_negated_terms", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub use_quote_negated_terms: Option<Option<bool>>,
}

impl SearchOverGroupsReqPayload {
    pub fn new(query: String, search_type: models::SearchMethod) -> SearchOverGroupsReqPayload {
        SearchOverGroupsReqPayload {
            filters: None,
            get_total_pages: None,
            group_size: None,
            highlight_delimiters: None,
            highlight_max_length: None,
            highlight_max_num: None,
            highlight_results: None,
            highlight_threshold: None,
            highlight_window: None,
            page: None,
            page_size: None,
            query,
            score_threshold: None,
            search_type,
            slim_chunks: None,
            use_quote_negated_terms: None,
        }
    }
}
