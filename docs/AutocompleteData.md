# AutocompleteData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**content_only** | Option<**bool**> | Set content_only to true to only returning the chunk_html of the chunks. This is useful for when you want to reduce amount of data over the wire for latency improvement (typically 10-50ms). Default is false. | [optional]
**filters** | Option<[**models::ChunkFilter**](ChunkFilter.md)> |  | [optional]
**highlight_delimiters** | Option<**Vec<String>**> | Set highlight_delimiters to a list of strings to use as delimiters for highlighting. If not specified, this defaults to [\"?\", \",\", \".\", \"!\"]. | [optional]
**highlight_results** | Option<**bool**> | Set highlight_results to false for a slight latency improvement (1-10ms). If not specified, this defaults to true. This will add `<b><mark>` tags to the chunk_html of the chunks to highlight matching sub-sentences. | [optional]
**highlight_threshold** | Option<**f64**> | Set highlight_threshold to a lower or higher value to adjust the sensitivity of the highlights applied to the chunk html. If not specified, this defaults to 0.8. The range is 0.0 to 1.0. | [optional]
**page_size** | Option<**i64**> | Page size is the number of chunks to fetch. This can be used to fetch more than 10 chunks at a time. | [optional]
**query** | **String** | Query is the search query. This can be any string. The query will be used to create an embedding vector and/or SPLADE vector which will be used to find the result set. | 
**recency_bias** | Option<**f32**> | Recency Bias lets you determine how much of an effect the recency of chunks will have on the search results. If not specified, this defaults to 0.0. We recommend setting this to 1.0 for a gentle reranking of the results, >3.0 for a strong reranking of the results. | [optional]
**score_threshold** | Option<**f32**> | Set score_threshold to a float to filter out chunks with a score below the threshold. | [optional]
**search_type** | **String** | Can be either \"semantic\", or \"fulltext\". \"semantic\" will pull in one page (10 chunks) of the nearest cosine distant vectors. \"fulltext\" will pull in one page (10 chunks) of full-text results based on SPLADE. | 
**slim_chunks** | Option<**bool**> | Set slim_chunks to true to avoid returning the content and chunk_html of the chunks. This is useful for when you want to reduce amount of data over the wire for latency improvement (typically 10-50ms). Default is false. | [optional]
**tag_weights** | Option<**std::collections::HashMap<String, f32>**> | Tag weights is a JSON object which can be used to boost the ranking of chunks with certain tags. This is useful for when you want to be able to bias towards chunks with a certain tag on the fly. The keys are the tag names and the values are the weights. | [optional]
**use_weights** | Option<**bool**> | Set use_weights to true to use the weights of the chunks in the result set in order to sort them. If not specified, this defaults to true. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


