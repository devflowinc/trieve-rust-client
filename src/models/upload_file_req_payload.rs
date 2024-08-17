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
pub struct UploadFileReqPayload {
    /// Base64 encoded file. This is the standard base64url encoding.
    #[serde(rename = "base64_file")]
    pub base64_file: String,
    /// Create chunks is a boolean which determines whether or not to create chunks from the file. If false, you can manually chunk the file and send the chunks to the create_chunk endpoint with the file_id to associate chunks with the file. Meant mostly for advanced users.
    #[serde(rename = "create_chunks", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub create_chunks: Option<Option<bool>>,
    /// Description is an optional convience field so you do not have to remember what the file contains or is about. It will be included on the group resulting from the file which will hold its chunk.
    #[serde(rename = "description", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub description: Option<Option<String>>,
    /// Name of the file being uploaded, including the extension.
    #[serde(rename = "file_name")]
    pub file_name: String,
    /// Group tracking id is an optional field which allows you to specify the tracking id of the group that is created from the file. Chunks created will be created with the tracking id of `group_tracking_id|<index of chunk>`
    #[serde(rename = "group_tracking_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub group_tracking_id: Option<Option<String>>,
    /// Link to the file. This can also be any string. This can be used to filter when searching for the file's resulting chunks. The link value will not affect embedding creation.
    #[serde(rename = "link", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub link: Option<Option<String>>,
    /// Metadata is a JSON object which can be used to filter chunks. This is useful for when you want to filter chunks by arbitrary metadata. Unlike with tag filtering, there is a performance hit for filtering on metadata. Will be passed down to the file's chunks.
    #[serde(rename = "metadata", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Option<serde_json::Value>>,
    /// Rebalance chunks is an optional field which allows you to specify whether or not to rebalance the chunks created from the file. If not specified, the default true is used. If true, Trieve will evenly distribute remainder splits across chunks such that 66 splits with a `target_splits_per_chunk` of 20 will result in 3 chunks with 22 splits each.
    #[serde(rename = "rebalance_chunks", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub rebalance_chunks: Option<Option<bool>>,
    /// Split delimiters is an optional field which allows you to specify the delimiters to use when splitting the file before chunking the text. If not specified, the default [.!?\\n] are used to split into sentences. However, you may want to use spaces or other delimiters.
    #[serde(rename = "split_delimiters", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub split_delimiters: Option<Option<Vec<String>>>,
    /// Tag set is a comma separated list of tags which will be passed down to the chunks made from the file. Tags are used to filter chunks when searching. HNSW indices are created for each tag such that there is no performance loss when filtering on them.
    #[serde(rename = "tag_set", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub tag_set: Option<Option<Vec<String>>>,
    /// Target splits per chunk. This is an optional field which allows you to specify the number of splits you want per chunk. If not specified, the default 20 is used. However, you may want to use a different number.
    #[serde(rename = "target_splits_per_chunk", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub target_splits_per_chunk: Option<Option<i32>>,
    /// Time stamp should be an ISO 8601 combined date and time without timezone. Time_stamp is used for time window filtering and recency-biasing search results. Will be passed down to the file's chunks.
    #[serde(rename = "time_stamp", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub time_stamp: Option<Option<String>>,
}

impl UploadFileReqPayload {
    pub fn new(base64_file: String, file_name: String) -> UploadFileReqPayload {
        UploadFileReqPayload {
            base64_file,
            create_chunks: None,
            description: None,
            file_name,
            group_tracking_id: None,
            link: None,
            metadata: None,
            rebalance_chunks: None,
            split_delimiters: None,
            tag_set: None,
            target_splits_per_chunk: None,
            time_stamp: None,
        }
    }
}

