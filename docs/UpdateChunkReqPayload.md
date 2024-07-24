# UpdateChunkReqPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**boost_phrase** | Option<[**models::BoostPhrase**](BoostPhrase.md)> |  | [optional]
**chunk_html** | Option<**String**> | HTML content of the chunk you want to update. This can also be plaintext. The innerText of the HTML will be used to create the embedding vector. The point of using HTML is for convienience, as some users have applications where users submit HTML content. If no chunk_html is provided, the existing chunk_html will be used. | [optional]
**chunk_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> | Id of the chunk you want to update. You can provide either the chunk_id or the tracking_id. If both are provided, the chunk_id will be used. | [optional]
**convert_html_to_text** | Option<**bool**> | Convert HTML to raw text before processing to avoid adding noise to the vector embeddings. By default this is true. If you are using HTML content that you want to be included in the vector embeddings, set this to false. | [optional]
**distance_phrase** | Option<[**models::DistancePhrase**](DistancePhrase.md)> |  | [optional]
**group_ids** | Option<[**Vec<uuid::Uuid>**](uuid::Uuid.md)> | Group ids are the ids of the groups that the chunk should be placed into. This is useful for when you want to update a chunk and add it to a group or multiple groups in one request. | [optional]
**group_tracking_ids** | Option<**Vec<String>**> | Group tracking_ids are the tracking_ids of the groups that the chunk should be placed into. This is useful for when you want to update a chunk and add it to a group or multiple groups in one request. | [optional]
**image_urls** | Option<**Vec<String>**> | Image urls are a list of urls to images that are associated with the chunk. This is useful for when you want to associate images with a chunk. If no image_urls are provided, the existing image_urls will be used. | [optional]
**link** | Option<**String**> | Link of the chunk you want to update. This can also be any string. Frequently, this is a link to the source of the chunk. The link value will not affect the embedding creation. If no link is provided, the existing link will be used. | [optional]
**location** | Option<[**models::GeoInfo**](GeoInfo.md)> |  | [optional]
**metadata** | Option<[**serde_json::Value**](.md)> | The metadata is a JSON object which can be used to filter chunks. This is useful for when you want to filter chunks by arbitrary metadata. Unlike with tag filtering, there is a performance hit for filtering on metadata. If no metadata is provided, the existing metadata will be used. | [optional]
**num_value** | Option<**f64**> | Num value is an arbitrary numerical value that can be used to filter chunks. This is useful for when you want to filter chunks by numerical value. If no num_value is provided, the existing num_value will be used. | [optional]
**tag_set** | Option<**Vec<String>**> | Tag set is a list of tags. This can be used to filter chunks by tag. Unlike with metadata filtering, HNSW indices will exist for each tag such that there is not a performance hit for filtering on them. If no tag_set is provided, the existing tag_set will be used. | [optional]
**time_stamp** | Option<**String**> | Time_stamp should be an ISO 8601 combined date and time without timezone. It is used for time window filtering and recency-biasing search results. If no time_stamp is provided, the existing time_stamp will be used. | [optional]
**tracking_id** | Option<**String**> | Tracking_id of the chunk you want to update. This is required to match an existing chunk. | [optional]
**weight** | Option<**f64**> | Weight is a float which can be used to bias search results. This is useful for when you want to bias search results for a chunk. The magnitude only matters relative to other chunks in the chunk's dataset dataset. If no weight is provided, the existing weight will be used. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


