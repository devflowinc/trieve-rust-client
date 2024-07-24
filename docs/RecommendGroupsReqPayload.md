# RecommendGroupsReqPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**filters** | Option<[**models::ChunkFilter**](ChunkFilter.md)> |  | [optional]
**group_size** | Option<**i32**> | The number of chunks to fetch for each group. This is the number of chunks which will be returned in the response for each group. The default is 3. If this is set to a large number, we recommend setting slim_chunks to true to avoid returning the content and chunk_html of the chunks so as to reduce latency due to content download and serialization. | [optional]
**limit** | Option<**i64**> | The number of groups to return. This is the number of groups which will be returned in the response. The default is 10. | [optional]
**negative_group_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | The ids of the groups to be used as negative examples for the recommendation. The groups in this array will be used to filter out similar groups. | [optional]
**negative_group_tracking_ids** | Option<**Vec<String>**> | The ids of the groups to be used as negative examples for the recommendation. The groups in this array will be used to filter out similar groups. | [optional]
**positive_group_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | The ids of the groups to be used as positive examples for the recommendation. The groups in this array will be used to find similar groups. | [optional]
**positive_group_tracking_ids** | Option<**Vec<String>**> | The ids of the groups to be used as positive examples for the recommendation. The groups in this array will be used to find similar groups. | [optional]
**recommend_type** | Option<[**models::RecommendType**](RecommendType.md)> |  | [optional]
**slim_chunks** | Option<**bool**> | Set slim_chunks to true to avoid returning the content and chunk_html of the chunks. This is useful for when you want to reduce amount of data over the wire for latency improvement (typicall 10-50ms). Default is false. | [optional]
**strategy** | Option<[**models::RecommendationStrategy**](RecommendationStrategy.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


