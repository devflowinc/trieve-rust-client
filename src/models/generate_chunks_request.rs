/*
 * Trieve API
 *
 * Trieve OpenAPI Specification. This document describes all of the operations available through the Trieve API.
 *
 * The version of the OpenAPI document: 0.8.7
 * Contact: developers@trieve.ai
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GenerateChunksRequest {
    /// The ids of the chunks to be retrieved and injected into the context window for RAG.
    #[serde(rename = "chunk_ids")]
    pub chunk_ids: Vec<uuid::Uuid>,
    /// Set highlight_results to false for a slight latency improvement (1-10ms). If not specified, this defaults to true. This will add `<b><mark>`` tags to the chunk_html of the chunks to highlight matching sub-sentences.
    #[serde(rename = "highlight_results", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub highlight_results: Option<Option<bool>>,
    /// The previous messages to be placed into the chat history. The last message in this array will be the prompt for the model to inference on. The length of this array must be at least 1.
    #[serde(rename = "prev_messages")]
    pub prev_messages: Vec<models::ChatMessageProxy>,
    /// Prompt for the last message in the prev_messages array. This will be used to generate the next message in the chat. The default is 'Respond to the instruction and include the doc numbers that you used in square brackets at the end of the sentences that you used the docs for:'. You can also specify an empty string to leave the final message alone such that your user's final message can be used as the prompt. See docs.trieve.ai or contact us for more information.
    #[serde(rename = "prompt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub prompt: Option<Option<String>>,
    /// Whether or not to stream the response. If this is set to true or not included, the response will be a stream. If this is set to false, the response will be a normal JSON response. Default is true.
    #[serde(rename = "stream_response", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub stream_response: Option<Option<bool>>,
}

impl GenerateChunksRequest {
    pub fn new(chunk_ids: Vec<uuid::Uuid>, prev_messages: Vec<models::ChatMessageProxy>) -> GenerateChunksRequest {
        GenerateChunksRequest {
            chunk_ids,
            highlight_results: None,
            prev_messages,
            prompt: None,
            stream_response: None,
        }
    }
}

