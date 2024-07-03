# CreateTopicReqPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**first_user_message** | Option<**String**> | The first message which will belong to the topic. The topic name is generated based on this message similar to how it works in the OpenAI chat UX if a name is not explicitly provided on the name request body key. | [optional]
**name** | Option<**String**> | The name of the topic. If this is not provided, the topic name is generated from the first_user_message. | [optional]
**owner_id** | **String** | The owner_id of the topic. This is typically a browser fingerprint or your user's id. It is used to group topics together for a user. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


