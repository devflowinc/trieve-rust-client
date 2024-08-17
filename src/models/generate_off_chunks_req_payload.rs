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
pub struct GenerateOffChunksReqPayload {
    /// The ids of the chunks to be retrieved and injected into the context window for RAG.
    #[serde(rename = "chunk_ids")]
    pub chunk_ids: Vec<uuid::Uuid>,
    /// Frequency penalty is a number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim. Default is 0.7.
    #[serde(rename = "frequency_penalty", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<Option<f32>>,
    /// Set highlight_results to false for a slight latency improvement (1-10ms). If not specified, this defaults to true. This will add `<b><mark>` tags to the chunk_html of the chunks to highlight matching splits.
    #[serde(rename = "highlight_results", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub highlight_results: Option<Option<bool>>,
    /// The maximum number of tokens to generate in the chat completion. Default is None.
    #[serde(rename = "max_tokens", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<Option<i32>>,
    /// Presence penalty is a number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics. Default is 0.7.
    #[serde(rename = "presence_penalty", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<Option<f32>>,
    /// The previous messages to be placed into the chat history. There must be at least one previous message.
    #[serde(rename = "prev_messages")]
    pub prev_messages: Vec<models::ChatMessageProxy>,
    /// Prompt will be used to tell the model what to generate in the next message in the chat. The default is 'Respond to the previous instruction and include the doc numbers that you used in square brackets at the end of the sentences that you used the docs for:'. You can also specify an empty string to leave the final message alone such that your user's final message can be used as the prompt. See docs.trieve.ai or contact us for more information.
    #[serde(rename = "prompt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub prompt: Option<Option<String>>,
    /// Stop tokens are up to 4 sequences where the API will stop generating further tokens. Default is None.
    #[serde(rename = "stop_tokens", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub stop_tokens: Option<Option<Vec<String>>>,
    /// Whether or not to stream the response. If this is set to true or not included, the response will be a stream. If this is set to false, the response will be a normal JSON response. Default is true.
    #[serde(rename = "stream_response", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub stream_response: Option<Option<bool>>,
    /// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. Default is 0.5.
    #[serde(rename = "temperature", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Option<f32>>,
    /// User ID is the id of the user who is making the request. This is used to track user interactions with the RAG results.
    #[serde(rename = "user_id", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Option<String>>,
}

impl GenerateOffChunksReqPayload {
    pub fn new(chunk_ids: Vec<uuid::Uuid>, prev_messages: Vec<models::ChatMessageProxy>) -> GenerateOffChunksReqPayload {
        GenerateOffChunksReqPayload {
            chunk_ids,
            frequency_penalty: None,
            highlight_results: None,
            max_tokens: None,
            presence_penalty: None,
            prev_messages,
            prompt: None,
            stop_tokens: None,
            stream_response: None,
            temperature: None,
            user_id: None,
        }
    }
}

