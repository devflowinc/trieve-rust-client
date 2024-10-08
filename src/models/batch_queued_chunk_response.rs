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
pub struct BatchQueuedChunkResponse {
    #[serde(rename = "chunk_metadata")]
    pub chunk_metadata: Vec<models::ChunkMetadata>,
    /// The current position the last access item is in the queue
    #[serde(rename = "pos_in_queue")]
    pub pos_in_queue: i32,
}

impl BatchQueuedChunkResponse {
    pub fn new(chunk_metadata: Vec<models::ChunkMetadata>, pos_in_queue: i32) -> BatchQueuedChunkResponse {
        BatchQueuedChunkResponse {
            chunk_metadata,
            pos_in_queue,
        }
    }
}

