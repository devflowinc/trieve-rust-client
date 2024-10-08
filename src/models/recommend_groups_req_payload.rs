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
pub struct RecommendGroupsReqPayload {
    #[serde(rename = "filters", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub filters: Option<Option<Box<models::ChunkFilter>>>,
    /// The number of chunks to fetch for each group. This is the number of chunks which will be returned in the response for each group. The default is 3. If this is set to a large number, we recommend setting slim_chunks to true to avoid returning the content and chunk_html of the chunks so as to reduce latency due to content download and serialization.
    #[serde(rename = "group_size", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub group_size: Option<Option<i32>>,
    /// The number of groups to return. This is the number of groups which will be returned in the response. The default is 10.
    #[serde(rename = "limit", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub limit: Option<Option<i64>>,
    /// The ids of the groups to be used as negative examples for the recommendation. The groups in this array will be used to filter out similar groups.
    #[serde(rename = "negative_group_ids", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub negative_group_ids: Option<Option<Vec<uuid::Uuid>>>,
    /// The ids of the groups to be used as negative examples for the recommendation. The groups in this array will be used to filter out similar groups.
    #[serde(rename = "negative_group_tracking_ids", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub negative_group_tracking_ids: Option<Option<Vec<String>>>,
    /// The ids of the groups to be used as positive examples for the recommendation. The groups in this array will be used to find similar groups.
    #[serde(rename = "positive_group_ids", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub positive_group_ids: Option<Option<Vec<uuid::Uuid>>>,
    /// The ids of the groups to be used as positive examples for the recommendation. The groups in this array will be used to find similar groups.
    #[serde(rename = "positive_group_tracking_ids", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub positive_group_tracking_ids: Option<Option<Vec<String>>>,
    #[serde(rename = "recommend_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub recommend_type: Option<Option<models::RecommendType>>,
    /// Set slim_chunks to true to avoid returning the content and chunk_html of the chunks. This is useful for when you want to reduce amount of data over the wire for latency improvement (typicall 10-50ms). Default is false.
    #[serde(rename = "slim_chunks", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub slim_chunks: Option<Option<bool>>,
    #[serde(rename = "strategy", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub strategy: Option<Option<models::RecommendationStrategy>>,
    /// The user_id is the id of the user who is making the request. This is used to track user interactions with the rrecommendation results.
    #[serde(rename = "user_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Option<String>>,
}

impl RecommendGroupsReqPayload {
    pub fn new() -> RecommendGroupsReqPayload {
        RecommendGroupsReqPayload {
            filters: None,
            group_size: None,
            limit: None,
            negative_group_ids: None,
            negative_group_tracking_ids: None,
            positive_group_ids: None,
            positive_group_tracking_ids: None,
            recommend_type: None,
            slim_chunks: None,
            strategy: None,
            user_id: None,
        }
    }
}

