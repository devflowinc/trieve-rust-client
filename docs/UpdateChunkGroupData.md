# UpdateChunkGroupData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**description** | Option<**String**> | Description to assign to the chunk_group. Convenience field for you to avoid having to remember what the group is for. If not provided, the description will not be updated. | [optional]
**group_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Id of the chunk_group to update. | [optional]
**metadata** | Option<[**serde_json::Value**](.md)> | Optional metadata to assign to the chunk_group. This is a JSON object that can store any additional information you want to associate with the chunks inside of the chunk_group. | [optional]
**name** | Option<**String**> | Name to assign to the chunk_group. Does not need to be unique. If not provided, the name will not be updated. | [optional]
**tag_set** | Option<**Vec<String>**> | Optional tags to assign to the chunk_group. This is a list of strings that can be used to categorize the chunks inside the chunk_group. | [optional]
**tracking_id** | Option<**String**> | Tracking Id of the chunk_group to update. | [optional]
**update_chunks** | Option<**bool**> | Flag to update the chunks in the group. If true, each chunk in the group will be updated by appending the group's tags to the chunk's tags. Default is false. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


