# CtrDataRequestBody

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**clicked_chunk_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | The ID of chunk that was clicked | [optional]
**clicked_chunk_tracking_id** | Option<**String**> | The tracking ID of the chunk that was clicked | [optional]
**ctr_type** | [**models::CtrType**](CTRType.md) |  | 
**metadata** | Option<[**serde_json::Value**](.md)> | Any metadata you want to include with the event i.e. action, user_id, etc. | [optional]
**position** | **i32** | The position of the clicked chunk | 
**request_id** | [**uuid::Uuid**](uuid::Uuid.md) | The request id for the CTR data | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


