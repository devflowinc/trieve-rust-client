# RecommendChunksRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**filters** | Option<[**models::ChunkFilter**](ChunkFilter.md)> |  | [optional]
**limit** | Option<**i64**> | The number of chunks to return. This is the number of chunks which will be returned in the response. The default is 10. | [optional]
**negative_chunk_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | The ids of the chunks to be used as negative examples for the recommendation. The chunks in this array will be used to filter out similar chunks. | [optional]
**negative_tracking_ids** | Option<**Vec<String>**> | The tracking_ids of the chunks to be used as negative examples for the recommendation. The chunks in this array will be used to filter out similar chunks. | [optional]
**positive_chunk_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | The ids of the chunks to be used as positive examples for the recommendation. The chunks in this array will be used to find similar chunks. | [optional]
**positive_tracking_ids** | Option<**Vec<String>**> | The tracking_ids of the chunks to be used as positive examples for the recommendation. The chunks in this array will be used to find similar chunks. | [optional]
**recommend_type** | Option<[**models::RecommendType**](RecommendType.md)> |  | [optional]
**slim_chunks** | Option<**bool**> | Set slim_chunks to true to avoid returning the content and chunk_html of the chunks. This is useful for when you want to reduce amount of data over the wire for latency improvement (typicall 10-50ms). Default is false. | [optional]
**strategy** | Option<[**models::RecommendationStrategy**](RecommendationStrategy.md)> |  | [optional]
**user_id** | Option<**String**> | User ID is the id of the user who is making the request. This is used to track user interactions with the recommendation results. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


