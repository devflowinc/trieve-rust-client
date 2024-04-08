# CreateTopicData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**first_user_message** | Option<**String**> | The first message which will belong to the topic. The topic name is generated based on this message similar to how it works in the OpenAI chat UX if a name is not explicitly provided on the name request body key. | [optional]
**model** | Option<**String**> | The model to use for the assistant's messages. This can be any model from the openrouter model list. If no model is provided, the gpt-3.5-turbo will be used. | [optional]
**name** | Option<**String**> | The name of the topic. If this is not provided, the topic name is generated from the first_user_message. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


