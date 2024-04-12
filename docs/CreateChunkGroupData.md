# CreateChunkGroupData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Description to assign to the chunk_group. Convenience field for you to avoid having to remember what the group is for. | [optional]
**metadata** | Option<[**serde_json::Value**](.md)> | Optional metadata to assign to the chunk_group. This is a JSON object that can store any additional information you want to associate with the chunks inside of the chunk_group. | [optional]
**name** | **String** | Name to assign to the chunk_group. Does not need to be unique. | 
**tag_set** | Option<**Vec<String>**> | Optional tags to assign to the chunk_group. This is a list of strings that can be used to categorize the chunks inside the chunk_group. | [optional]
**tracking_id** | Option<**String**> | Optional tracking id to assign to the chunk_group. This is a unique identifier for the chunk_group. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


