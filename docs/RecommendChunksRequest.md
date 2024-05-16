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
**recommend_type** | Option<**String**> | The type of recommendation to make. This lets you choose whether to recommend based off of `semantic` or `fulltext` similarity. The default is `semantic`. | [optional]
**slim_chunks** | Option<**bool**> | Set slim_chunks to true to avoid returning the content and chunk_html of the chunks. This is useful for when you want to reduce amount of data over the wire for latency improvement (typicall 10-50ms). Default is false. | [optional]
**strategy** | Option<**String**> | Strategy to use for recommendations, either \"average_vector\" or \"best_score\". The default is \"average_vector\". The \"average_vector\" strategy will construct a single average vector from the positive and negative samples then use it to perform a pseudo-search. The \"best_score\" strategy is more advanced and navigates the HNSW with a heuristic of picking edges where the point is closer to the positive samples than it is the negatives. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


