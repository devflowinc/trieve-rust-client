# RegenerateMessageReqPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**completion_first** | Option<**bool**> | Completion first decides whether the stream should contain the stream of the completion response or the chunks first. Default is false. Keep in mind that || is used to separate the chunks from the completion response. If || is in the completion then you may want to split on ||{ instead. | [optional]
**concat_user_messages_query** | Option<**bool**> | If concat user messages query is set to true, all of the user messages in the topic will be concatenated together and used as the search query. If not specified, this defaults to false. Default is false. | [optional]
**filters** | Option<[**models::ChunkFilter**](ChunkFilter.md)> |  | [optional]
**frequency_penalty** | Option<**f32**> | Frequency penalty is a number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim. Default is 0.7. | [optional]
**highlight_citations** | Option<**bool**> | Whether or not to highlight the citations in the response. If this is set to true or not included, the citations will be highlighted. If this is set to false, the citations will not be highlighted. Default is true. | [optional]
**highlight_delimiters** | Option<**Vec<String>**> | The delimiters to use for highlighting the citations. If this is not included, the default delimiters will be used. Default is `[\".\", \"!\", \"?\", \"\\n\", \"\\t\", \",\"]`. | [optional]
**max_tokens** | Option<**i32**> | The maximum number of tokens to generate in the chat completion. | [optional]
**page_size** | Option<**i64**> | Page size is the number of chunks to fetch during RAG. If 0, then no search will be performed. If specified, this will override the N retrievals to include in the dataset configuration. Default is None. | [optional]
**presence_penalty** | Option<**f32**> | Presence penalty is a number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics. | [optional]
**score_threshold** | Option<**f32**> | Set score_threshold to a float to filter out chunks with a score below the threshold. This threshold applies before weight and bias modifications. If not specified, this defaults to 0.0. | [optional]
**search_query** | Option<**String**> | Query is the search query. This can be any string. The search_query will be used to create a dense embedding vector and/or sparse vector which will be used to find the result set. If not specified, will default to the last user message or HyDE if HyDE is enabled in the dataset configuration. Default is None. | [optional]
**search_type** | Option<[**models::SearchMethod**](SearchMethod.md)> |  | [optional]
**stop_tokens** | Option<**Vec<String>**> | Stop tokens are up to 4 sequences where the API will stop generating further tokens. | [optional]
**stream_response** | Option<**bool**> | Whether or not to stream the response. If this is set to true or not included, the response will be a stream. If this is set to false, the response will be a normal JSON response. Default is true. | [optional]
**temperature** | Option<**f32**> | What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. Default is 0.7. | [optional]
**topic_id** | [**uuid::Uuid**](uuid::Uuid.md) | The id of the topic to regenerate the last message for. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


