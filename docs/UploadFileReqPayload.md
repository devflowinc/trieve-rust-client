# UploadFileReqPayload

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**base64_file** | **String** | Base64 encoded file. This is the standard base64url encoding. | 
**create_chunks** | Option<**bool**> | Create chunks is a boolean which determines whether or not to create chunks from the file. If false, you can manually chunk the file and send the chunks to the create_chunk endpoint with the file_id to associate chunks with the file. Meant mostly for advanced users. | [optional]
**description** | Option<**String**> | Description is an optional convience field so you do not have to remember what the file contains or is about. It will be included on the group resulting from the file which will hold its chunk. | [optional]
**file_name** | **String** | Name of the file being uploaded, including the extension. | 
**group_tracking_id** | Option<**String**> | Group tracking id is an optional field which allows you to specify the tracking id of the group that is created from the file. Chunks created will be created with the tracking id of `group_tracking_id|<index of chunk>` | [optional]
**link** | Option<**String**> | Link to the file. This can also be any string. This can be used to filter when searching for the file's resulting chunks. The link value will not affect embedding creation. | [optional]
**metadata** | Option<[**serde_json::Value**](.md)> | Metadata is a JSON object which can be used to filter chunks. This is useful for when you want to filter chunks by arbitrary metadata. Unlike with tag filtering, there is a performance hit for filtering on metadata. Will be passed down to the file's chunks. | [optional]
**rebalance_chunks** | Option<**bool**> | Rebalance chunks is an optional field which allows you to specify whether or not to rebalance the chunks created from the file. If not specified, the default true is used. If true, Trieve will evenly distribute remainder splits across chunks such that 66 splits with a `target_splits_per_chunk` of 20 will result in 3 chunks with 22 splits each. | [optional]
**split_delimiters** | Option<**Vec<String>**> | Split delimiters is an optional field which allows you to specify the delimiters to use when splitting the file before chunking the text. If not specified, the default [.!?\\n] are used to split into sentences. However, you may want to use spaces or other delimiters. | [optional]
**tag_set** | Option<**Vec<String>**> | Tag set is a comma separated list of tags which will be passed down to the chunks made from the file. Tags are used to filter chunks when searching. HNSW indices are created for each tag such that there is no performance loss when filtering on them. | [optional]
**target_splits_per_chunk** | Option<**i32**> | Target splits per chunk. This is an optional field which allows you to specify the number of splits you want per chunk. If not specified, the default 20 is used. However, you may want to use a different number. | [optional]
**time_stamp** | Option<**String**> | Time stamp should be an ISO 8601 combined date and time without timezone. Time_stamp is used for time window filtering and recency-biasing search results. Will be passed down to the file's chunks. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


