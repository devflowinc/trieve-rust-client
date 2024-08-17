# LlmOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**completion_first** | Option<**bool**> | Completion first decides whether the stream should contain the stream of the completion response or the chunks first. Default is false. Keep in mind that || is used to separate the chunks from the completion response. If || is in the completion then you may want to split on ||{ instead. | [optional]
**frequency_penalty** | Option<**f32**> | Frequency penalty is a number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim. Default is 0.7. | [optional]
**max_tokens** | Option<**i32**> | The maximum number of tokens to generate in the chat completion. Default is None. | [optional]
**presence_penalty** | Option<**f32**> | Presence penalty is a number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics. Default is 0.7. | [optional]
**stop_tokens** | Option<**Vec<String>**> | Stop tokens are up to 4 sequences where the API will stop generating further tokens. Default is None. | [optional]
**stream_response** | Option<**bool**> | Whether or not to stream the response. If this is set to true or not included, the response will be a stream. If this is set to false, the response will be a normal JSON response. Default is true. | [optional]
**system_prompt** | Option<**String**> | Optionally, override the system prompt in dataset server settings. | [optional]
**temperature** | Option<**f32**> | What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic. Default is 0.5. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


