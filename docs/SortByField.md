# SortByField

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**direction** | Option<[**models::SortOrder**](SortOrder.md)> |  | [optional]
**field** | **String** | Field to sort by. This has to be a numeric field with a Qdrant `Range` index on it. i.e. num_value and timestamp | 
**prefetch_amount** | Option<**i64**> | How many results to pull in before the sort | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


