# CreateMessageData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**highlight_citations** | Option<**bool**> | Whether or not to highlight the citations in the response. If this is set to true or not included, the citations will be highlighted. If this is set to false, the citations will not be highlighted. Default is true. | [optional]
**highlight_delimiters** | Option<**Vec<String>**> | The delimiters to use for highlighting the citations. If this is not included, the default delimiters will be used. Default is `[\".\", \"!\", \"?\", \"\\n\", \"\\t\", \",\"]`. | [optional]
**model** | Option<**String**> | The model to use for the assistant's messages. This can be any model from the openrouter model list. If no model is provided, gpt-3.5-turbo will be used. | [optional]
**new_message_content** | **String** | The content of the user message to attach to the topic and then generate an assistant message in response to. | 
**stream_response** | Option<**bool**> | Whether or not to stream the response. If this is set to true or not included, the response will be a stream. If this is set to false, the response will be a normal JSON response. Default is true. | [optional]
**topic_id** | [**uuid::Uuid**](uuid::Uuid.md) | The ID of the topic to attach the message to. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


