# ScrollChunksReqPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**filters** | Option<[**models::ChunkFilter**](ChunkFilter.md)> |  | [optional]
**offset_chunk_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Offset chunk id is the id of the chunk to start the page from. If not specified, this defaults to the first chunk in the dataset sorted by id ascending. | [optional]
**page_size** | Option<**i64**> | Page size is the number of chunks to fetch. This can be used to fetch more than 10 chunks at a time. | [optional]
**sort_by** | Option<[**models::SortByField**](SortByField.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


