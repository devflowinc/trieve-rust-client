# CreateMessageReqPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**concat_user_messages_query** | Option<**bool**> | If concat user messages query is set to true, all of the user messages in the topic will be concatenated together and used as the search query. If not specified, this defaults to false. Default is false. | [optional]
**filters** | Option<[**models::ChunkFilter**](ChunkFilter.md)> |  | [optional]
**highlight_options** | Option<[**models::HighlightOptions**](HighlightOptions.md)> |  | [optional]
**llm_options** | Option<[**models::LlmOptions**](LLMOptions.md)> |  | [optional]
**new_message_content** | **String** | The content of the user message to attach to the topic and then generate an assistant message in response to. | 
**page_size** | Option<**i64**> | Page size is the number of chunks to fetch during RAG. If 0, then no search will be performed. If specified, this will override the N retrievals to include in the dataset configuration. Default is None. | [optional]
**score_threshold** | Option<**f32**> | Set score_threshold to a float to filter out chunks with a score below the threshold. This threshold applies before weight and bias modifications. If not specified, this defaults to 0.0. | [optional]
**search_query** | Option<**String**> | Query is the search query. This can be any string. The search_query will be used to create a dense embedding vector and/or sparse vector which will be used to find the result set. If not specified, will default to the last user message or HyDE if HyDE is enabled in the dataset configuration. Default is None. | [optional]
**search_type** | Option<[**models::SearchMethod**](SearchMethod.md)> |  | [optional]
**topic_id** | [**uuid::Uuid**](uuid::Uuid.md) | The ID of the topic to attach the message to. | 
**user_id** | Option<**String**> | The user_id is the id of the user who is making the request. This is used to track user interactions with the RAG results. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


