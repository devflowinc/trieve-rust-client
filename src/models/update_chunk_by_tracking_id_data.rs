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
pub struct UpdateChunkByTrackingIdData {
    /// HTML content of the chunk you want to update. This can also be plaintext. The innerText of the HTML will be used to create the embedding vector. The point of using HTML is for convienience, as some users have applications where users submit HTML content. If no chunk_html is provided, the existing chunk_html will be used.
    #[serde(rename = "chunk_html", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub chunk_html: Option<Option<String>>,
    /// Convert HTML to raw text before processing to avoid adding noise to the vector embeddings. By default this is true. If you are using HTML content that you want to be included in the vector embeddings, set this to false.
    #[serde(rename = "convert_html_to_text", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub convert_html_to_text: Option<Option<bool>>,
    /// Group ids are the ids of the groups that the chunk should be placed into. This is useful for when you want to update a chunk and add it to a group or multiple groups in one request.
    #[serde(rename = "group_ids", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Option<Vec<uuid::Uuid>>>,
    /// Group tracking_ids are the tracking_ids of the groups that the chunk should be placed into. This is useful for when you want to update a chunk and add it to a group or multiple groups in one request.
    #[serde(rename = "group_tracking_ids", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub group_tracking_ids: Option<Option<Vec<String>>>,
    /// Link of the chunk you want to update. This can also be any string. Frequently, this is a link to the source of the chunk. The link value will not affect the embedding creation. If no link is provided, the existing link will be used.
    #[serde(rename = "link", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub link: Option<Option<String>>,
    /// The metadata is a JSON object which can be used to filter chunks. This is useful for when you want to filter chunks by arbitrary metadata. Unlike with tag filtering, there is a performance hit for filtering on metadata. If no metadata is provided, the existing metadata will be used.
    #[serde(rename = "metadata", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Option<serde_json::Value>>,
    /// Time_stamp should be an ISO 8601 combined date and time without timezone. It is used for time window filtering and recency-biasing search results. If no time_stamp is provided, the existing time_stamp will be used.
    #[serde(rename = "time_stamp", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<Option<String>>,
    /// Tracking_id of the chunk you want to update. This is required to match an existing chunk.
    #[serde(rename = "tracking_id")]
    pub tracking_id: String,
    /// Weight is a float which can be used to bias search results. This is useful for when you want to bias search results for a chunk. The magnitude only matters relative to other chunks in the chunk's dataset dataset. If no weight is provided, the existing weight will be used.
    #[serde(rename = "weight", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub weight: Option<Option<f64>>,
}

impl UpdateChunkByTrackingIdData {
    pub fn new(tracking_id: String) -> UpdateChunkByTrackingIdData {
        UpdateChunkByTrackingIdData {
            chunk_html: None,
            convert_html_to_text: None,
            group_ids: None,
            group_tracking_ids: None,
            link: None,
            metadata: None,
            time_stamp: None,
            tracking_id,
            weight: None,
        }
    }
}

