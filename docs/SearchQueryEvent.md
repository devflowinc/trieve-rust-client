# SearchQueryEvent

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**created_at** | **String** |  | 
**dataset_id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**id** | [**uuid::Uuid**](uuid::Uuid.md) |  | 
**latency** | **f32** |  | 
**query** | **String** |  | 
**query_rating** | **String** |  | 
**request_params** | Option<[**serde_json::Value**](.md)> |  | 
**results** | [**Vec<serde_json::Value>**](serde_json::Value.md) |  | 
**search_type** | **String** |  | 
**top_score** | **f32** |  | 
**user_id** | **String** |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


