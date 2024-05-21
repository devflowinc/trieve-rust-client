# ChunkFilter

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**must** | Option<[**Vec<models::ConditionType>**](ConditionType.md)> | All of these field conditions have to match for the chunk to be included in the result set. | [optional]
**must_not** | Option<[**Vec<models::ConditionType>**](ConditionType.md)> | None of these field conditions can match for the chunk to be included in the result set. | [optional]
**should** | Option<[**Vec<models::ConditionType>**](ConditionType.md)> | Only one of these field conditions has to match for the chunk to be included in the result set. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


