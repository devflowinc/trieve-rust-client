# HighlightOptions

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**highlight_delimiters** | Option<**Vec<String>**> | Set highlight_delimiters to a list of strings to use as delimiters for highlighting. If not specified, this defaults to [\"?\", \",\", \".\", \"!\"]. These are the characters that will be used to split the chunk_html into splits for highlighting. These are the characters that will be used to split the chunk_html into splits for highlighting. | [optional]
**highlight_max_length** | Option<**i32**> | Set highlight_max_length to control the maximum number of tokens (typically whitespace separated strings, but sometimes also word stems) which can be present within a single highlight. If not specified, this defaults to 8. This is useful to shorten large splits which may have low scores due to length compared to the query. Set to something very large like 100 to highlight entire splits. | [optional]
**highlight_max_num** | Option<**i32**> | Set highlight_max_num to control the maximum number of highlights per chunk. If not specified, this defaults to 3. It may be less than 3 if no snippets score above the highlight_threshold. | [optional]
**highlight_results** | Option<**bool**> | Set highlight_results to false for a slight latency improvement (1-10ms). If not specified, this defaults to true. This will add `<b><mark>` tags to the chunk_html of the chunks to highlight matching splits and return the highlights on each scored chunk in the response. | [optional]
**highlight_strategy** | Option<[**models::HighlightStrategy**](HighlightStrategy.md)> |  | [optional]
**highlight_threshold** | Option<**f64**> | Set highlight_threshold to a lower or higher value to adjust the sensitivity of the highlights applied to the chunk html. If not specified, this defaults to 0.8. The range is 0.0 to 1.0. | [optional]
**highlight_window** | Option<**i32**> | Set highlight_window to a number to control the amount of words that are returned around the matched phrases. If not specified, this defaults to 0. This is useful for when you want to show more context around the matched words. When specified, window/2 whitespace separated words are added before and after each highlight in the response's highlights array. If an extended highlight overlaps with another highlight, the overlapping words are only included once. This parameter can be overriden to respect the highlight_max_length param. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


