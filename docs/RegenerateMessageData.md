# RegenerateMessageData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**highlight_citations** | Option<**bool**> | Whether or not to highlight the citations in the response. If this is set to true or not included, the citations will be highlighted. If this is set to false, the citations will not be highlighted. Default is true. | [optional]
**highlight_delimiters** | Option<**Vec<String>**> | The delimiters to use for highlighting the citations. If this is not included, the default delimiters will be used. Default is `[\".\", \"!\", \"?\", \"\\n\", \"\\t\", \",\"]`. | [optional]
**stream_response** | Option<**bool**> | Whether or not to stream the response. If this is set to true or not included, the response will be a stream. If this is set to false, the response will be a normal JSON response. Default is true. | [optional]
**topic_id** | [**uuid::Uuid**](uuid::Uuid.md) | The id of the topic to regenerate the last message for. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


