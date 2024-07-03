# ChunkFilter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**jsonb_prefilter** | Option<**bool**> | JOSNB prefilter tells the server to perform a full scan over the metadata JSONB column instead of using the filtered HNSW. Datasets on the enterprise plan with custom metadata indices will perform better with the filtered HNSW instead. When false, the server will use the filtered HNSW index to filter chunks. When true, the server will perform a full scan over the metadata JSONB column to filter chunks. Default is true. | [optional]
**must** | Option<[**Vec<models::ConditionType>**](ConditionType.md)> | All of these field conditions have to match for the chunk to be included in the result set. | [optional]
**must_not** | Option<[**Vec<models::ConditionType>**](ConditionType.md)> | None of these field conditions can match for the chunk to be included in the result set. | [optional]
**should** | Option<[**Vec<models::ConditionType>**](ConditionType.md)> | Only one of these field conditions has to match for the chunk to be included in the result set. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


