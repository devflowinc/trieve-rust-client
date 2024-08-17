# ChunkReqPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**chunk_html** | Option<**String**> | HTML content of the chunk. This can also be plaintext. The innerText of the HTML will be used to create the embedding vector. The point of using HTML is for convienience, as some users have applications where users submit HTML content. | [optional]
**convert_html_to_text** | Option<**bool**> | Convert HTML to raw text before processing to avoid adding noise to the vector embeddings. By default this is true. If you are using HTML content that you want to be included in the vector embeddings, set this to false. | [optional]
**fulltext_boost** | Option<[**models::FullTextBoost**](FullTextBoost.md)> |  | [optional]
**group_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Group ids are the Trieve generated ids of the groups that the chunk should be placed into. This is useful for when you want to create a chunk and add it to a group or multiple groups in one request. Groups with these Trieve generated ids must be created first, it cannot be arbitrarily created through this route. | [optional]
**group_tracking_ids** | Option<**Vec<String>**> | Group tracking_ids are the user-assigned tracking_ids of the groups that the chunk should be placed into. This is useful for when you want to create a chunk and add it to a group or multiple groups in one request. If a group with the tracking_id does not exist, it will be created. | [optional]
**image_urls** | Option<**Vec<String>**> | Image urls are a list of urls to images that are associated with the chunk. This is useful for when you want to associate images with a chunk. | [optional]
**link** | Option<**String**> | Link to the chunk. This can also be any string. Frequently, this is a link to the source of the chunk. The link value will not affect the embedding creation. | [optional]
**location** | Option<[**models::GeoInfo**](GeoInfo.md)> |  | [optional]
**metadata** | Option<[**serde_json::Value**](.md)> | Metadata is a JSON object which can be used to filter chunks. This is useful for when you want to filter chunks by arbitrary metadata. Unlike with tag filtering, there is a performance hit for filtering on metadata. | [optional]
**num_value** | Option<**f64**> | Num value is an arbitrary numerical value that can be used to filter chunks. This is useful for when you want to filter chunks by numerical value. There is no performance hit for filtering on num_value. | [optional]
**semantic_boost** | Option<[**models::SemanticBoost**](SemanticBoost.md)> |  | [optional]
**semantic_content** | Option<**String**> | If semantic_content is present, it will be used for creating semantic embeddings instead of the innerText `chunk_html`. `chunk_html` will still be the only thing stored and always used for fulltext functionality. | [optional]
**split_avg** | Option<**bool**> | Split avg is a boolean which tells the server to split the text in the chunk_html into smaller chunks and average their resulting vectors. This is useful for when you want to create a chunk from a large piece of text and want to split it into smaller chunks to create a more fuzzy average dense vector. The sparse vector will be generated normally with no averaging. By default this is false. | [optional]
**tag_set** | Option<**Vec<String>**> | Tag set is a list of tags. This can be used to filter chunks by tag. Unlike with metadata filtering, HNSW indices will exist for each tag such that there is not a performance hit for filtering on them. | [optional]
**time_stamp** | Option<**String**> | Time_stamp should be an ISO 8601 combined date and time without timezone. It is used for time window filtering and recency-biasing search results. | [optional]
**tracking_id** | Option<**String**> | Tracking_id is a string which can be used to identify a chunk. This is useful for when you are coordinating with an external system and want to use the tracking_id to identify the chunk. | [optional]
**upsert_by_tracking_id** | Option<**bool**> | Upsert when a chunk with the same tracking_id exists. By default this is false, and the request will fail if a chunk with the same tracking_id exists. If this is true, the chunk will be updated if a chunk with the same tracking_id exists. | [optional]
**weight** | Option<**f64**> | Weight is a float which can be used to bias search results. This is useful for when you want to bias search results for a chunk. The magnitude only matters relative to other chunks in the chunk's dataset dataset. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


