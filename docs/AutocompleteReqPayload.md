# AutocompleteReqPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content_only** | Option<**bool**> | Set content_only to true to only returning the chunk_html of the chunks. This is useful for when you want to reduce amount of data over the wire for latency improvement (typically 10-50ms). Default is false. | [optional]
**extend_results** | Option<**bool**> | If specified to true, this will extend the search results to include non-exact prefix matches of the same search_type such that a full page_size of results are returned. Default is false. | [optional]
**filters** | Option<[**models::ChunkFilter**](ChunkFilter.md)> |  | [optional]
**highlight_options** | Option<[**models::HighlightOptions**](HighlightOptions.md)> |  | [optional]
**page_size** | Option<**i64**> | Page size is the number of chunks to fetch. This can be used to fetch more than 10 chunks at a time. | [optional]
**query** | **String** | Query is the search query. This can be any string. The query will be used to create an embedding vector and/or SPLADE vector which will be used to find the result set. | 
**remove_stop_words** | Option<**bool**> | If true, stop words (specified in server/src/stop-words.txt in the git repo) will be removed. Queries that are entirely stop words will be preserved. | [optional]
**score_threshold** | Option<**f32**> | Set score_threshold to a float to filter out chunks with a score below the threshold. This threshold applies before weight and bias modifications. If not specified, this defaults to 0.0. | [optional]
**search_type** | [**models::SearchMethod**](SearchMethod.md) |  | 
**slim_chunks** | Option<**bool**> | Set slim_chunks to true to avoid returning the content and chunk_html of the chunks. This is useful for when you want to reduce amount of data over the wire for latency improvement (typically 10-50ms). Default is false. | [optional]
**sort_options** | Option<[**models::SortOptions**](SortOptions.md)> |  | [optional]
**use_quote_negated_terms** | Option<**bool**> | If true, quoted and - prefixed words will be parsed from the queries and used as required and negated words respectively. Default is false. | [optional]
**user_id** | Option<**String**> | User ID is the id of the user who is making the request. This is used to track user interactions with the search results. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


