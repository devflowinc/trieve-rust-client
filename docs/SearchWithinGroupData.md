# SearchWithinGroupData

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**date_bias** | Option<**bool**> | Set date_bias to true to bias search results towards more recent chunks. This will work best in hybrid search mode. | [optional]
**filters** | Option<[**models::ChunkFilter**](ChunkFilter.md)> |  | [optional]
**group_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Group specifies the group to search within. Results will only consist of chunks which are bookmarks within the specified group. | [optional]
**group_tracking_id** | Option<**String**> | Group_tracking_id specifies the group to search within by tracking id. Results will only consist of chunks which are bookmarks within the specified group. If both group_id and group_tracking_id are provided, group_id will be used. | [optional]
**highlight_delimiters** | Option<**Vec<String>**> | Set highlight_delimiters to a list of strings to use as delimiters for highlighting. If not specified, this defaults to [\"?\", \",\", \".\", \"!\"]. | [optional]
**highlight_results** | Option<**bool**> | Set highlight_results to true to highlight the results. If not specified, this defaults to true. | [optional]
**page** | Option<**i64**> | The page of chunks to fetch. Page is 1-indexed. | [optional]
**page_size** | Option<**i64**> | The page size is the number of chunks to fetch. This can be used to fetch more than 10 chunks at a time. | [optional]
**query** | **String** | The query is the search query. This can be any string. The query will be used to create an embedding vector and/or SPLADE vector which will be used to find the result set. | 
**score_threshold** | Option<**f32**> | Set score_threshold to a float to filter out chunks with a score below the threshold. | [optional]
**search_type** | **String** | Search_type can be either \"semantic\", \"fulltext\", or \"hybrid\". \"hybrid\" will pull in one page (10 chunks) of both semantic and full-text results then re-rank them using BAAI/bge-reranker-large. \"semantic\" will pull in one page (10 chunks) of the nearest cosine distant vectors. \"fulltext\" will pull in one page (10 chunks) of full-text results based on SPLADE. | 
**slim_chunks** | Option<**bool**> | Set slim_chunks to true to avoid returning the content and chunk_html of the chunks. This is useful for when you want to reduce amount of data over the wire for latency improvement. Default is false. | [optional]
**use_weights** | Option<**bool**> | Set use_weights to true to use the weights of the chunks in the result set in order to sort them. If not specified, this defaults to true. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


