# GetEventsData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**event_types** | Option<[**Vec<models::EventTypeRequest>**](EventTypeRequest.md)> | The types of events to get. Any combination of file_uploaded, chunk_uploaded, chunk_action_failed, chunk_updated, or qdrant_index_failed. Leave undefined to get all events. | [optional]
**page** | Option<**i64**> | The page number to get. Default is 1. | [optional]
**page_size** | Option<**i64**> | The number of items per page. Default is 10. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


