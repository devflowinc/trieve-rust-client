# SearchWithinGroupReqPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content_only** | Option<**bool**> | Set content_only to true to only returning the chunk_html of the chunks. This is useful for when you want to reduce amount of data over the wire for latency improvement (typically 10-50ms). Default is false. | [optional]
**filters** | Option<[**models::ChunkFilter**](ChunkFilter.md)> |  | [optional]
**get_total_pages** | Option<**bool**> | Get total page count for the query accounting for the applied filters. Defaults to false, but can be set to true when the latency penalty is acceptable (typically 50-200ms). | [optional]
**group_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Group specifies the group to search within. Results will only consist of chunks which are bookmarks within the specified group. | [optional]
**group_tracking_id** | Option<**String**> | Group_tracking_id specifies the group to search within by tracking id. Results will only consist of chunks which are bookmarks within the specified group. If both group_id and group_tracking_id are provided, group_id will be used. | [optional]
**highlight_options** | Option<[**models::HighlightOptions**](HighlightOptions.md)> |  | [optional]
**page** | Option<**i64**> | The page of chunks to fetch. Page is 1-indexed. | [optional]
**page_size** | Option<**i64**> | The page size is the number of chunks to fetch. This can be used to fetch more than 10 chunks at a time. | [optional]
**query** | [**models::QueryTypes**](QueryTypes.md) |  | 
**remove_stop_words** | Option<**bool**> | If true, stop words (specified in server/src/stop-words.txt in the git repo) will be removed. Queries that are entirely stop words will be preserved. | [optional]
**score_threshold** | Option<**f32**> | Set score_threshold to a float to filter out chunks with a score below the threshold. This threshold applies before weight and bias modifications. If not specified, this defaults to 0.0. | [optional]
**search_type** | [**models::SearchMethod**](SearchMethod.md) |  | 
**slim_chunks** | Option<**bool**> | Set slim_chunks to true to avoid returning the content and chunk_html of the chunks. This is useful for when you want to reduce amount of data over the wire for latency improvement (typicall 10-50ms). Default is false. | [optional]
**sort_options** | Option<[**models::SortOptions**](SortOptions.md)> |  | [optional]
**use_quote_negated_terms** | Option<**bool**> | If true, quoted and - prefixed words will be parsed from the queries and used as required and negated words respectively. Default is false. | [optional]
**user_id** | Option<**String**> | The user_id is the id of the user who is making the request. This is used to track user interactions with the search results. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


