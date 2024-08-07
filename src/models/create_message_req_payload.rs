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
pub struct CreateMessageReqPayload {
    /// Completion first decides whether the stream should contain the stream of the completion response or the chunks first. Default is false. Keep in mind that || is used to separate the chunks from the completion response. If || is in the completion then you may want to split on ||{ instead.
    #[serde(rename = "completion_first", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub completion_first: Option<Option<bool>>,
    /// If concat user messages query is set to true, all of the user messages in the topic will be concatenated together and used as the search query. If not specified, this defaults to false. Default is false.
    #[serde(rename = "concat_user_messages_query", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub concat_user_messages_query: Option<Option<bool>>,
    #[serde(rename = "filters", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub filters: Option<Option<Box<models::ChunkFilter>>>,
    /// Frequency penalty is a number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim. Default is 0.7.
    #[serde(rename = "frequency_penalty", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<Option<f32>>,
    /// The delimiters to use for highlighting the citations. If this is not included, the default delimiters will be used. Default is `[\".\", \"!\", \"?\", \"\\n\", \"\\t\", \",\"]`.
    #[serde(rename = "highlight_delimiters", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub highlight_delimiters: Option<Option<Vec<String>>>,
    /// Set highlight_results to false for a slight latency improvement (1-10ms). If not specified, this defaults to true. This will add `<b><mark>` tags to the chunk_html of the chunks to highlight matching splits and return the highlights on each scored chunk in the response.
    #[serde(rename = "highlight_results", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub highlight_results: Option<Option<bool>>,
    /// The maximum number of tokens to generate in the chat completion. Default is None.
    #[serde(rename = "max_tokens", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<Option<i32>>,
    /// The content of the user message to attach to the topic and then generate an assistant message in response to.
    #[serde(rename = "new_message_content")]
    pub new_message_content: String,
    /// Page size is the number of chunks to fetch during RAG. If 0, then no search will be performed. If specified, this will override the N retrievals to include in the dataset configuration. Default is None.
    #[serde(rename = "page_size", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<Option<i64>>,
    /// Presence penalty is a number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics. Default is 0.7.
    #[serde(rename = "presence_penalty", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<Option<f32>>,
    /// Set score_threshold to a float to filter out chunks with a score below the threshold. This threshold applies before weight and bias modifications. If not specified, this defaults to 0.0.
    #[serde(rename = "score_threshold", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub score_threshold: Option<Option<f32>>,
    /// Query is the search query. This can be any string. The search_query will be used to create a dense embedding vector and/or sparse vector which will be used to find the result set. If not specified, will default to the last user message or HyDE if HyDE is enabled in the dataset configuration. Default is None.
    #[serde(rename = "search_query", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub search_query: Option<Option<String>>,
    #[serde(rename = "search_type", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub search_type: Option<Option<models::SearchMethod>>,
    /// Stop tokens are up to 4 sequences where the API will stop generating further tokens. Default is None.
    #[serde(rename = "stop_tokens", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub stop_tokens: Option<Option<Vec<String>>>,
    /// Whether or not to stream the response. If this is set to true or not included, the response will be a stream. If this is set to false, the response will be a normal JSON response. Default is true.
    #[serde(rename = "stream_response", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub stream_response: Option<Option<bool>>,
    /// Optionally, override the system prompt in dataset server settings.
    #[serde(rename = "system_prompt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<Option<String>>,
    /// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. Default is 0.5.
    #[serde(rename = "temperature", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub temperature: Option<Option<f32>>,
    /// The ID of the topic to attach the message to.
    #[serde(rename = "topic_id")]
    pub topic_id: uuid::Uuid,
}

impl CreateMessageReqPayload {
    pub fn new(new_message_content: String, topic_id: uuid::Uuid) -> CreateMessageReqPayload {
        CreateMessageReqPayload {
            completion_first: None,
            concat_user_messages_query: None,
            filters: None,
            frequency_penalty: None,
            highlight_delimiters: None,
            highlight_results: None,
            max_tokens: None,
            new_message_content,
            page_size: None,
            presence_penalty: None,
            score_threshold: None,
            search_query: None,
            search_type: None,
            stop_tokens: None,
            stream_response: None,
            system_prompt: None,
            temperature: None,
            topic_id,
        }
    }
}

