# \ChunkGroupApi

All URIs are relative to *https://api.trieve.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_chunk_to_group**](ChunkGroupApi.md#add_chunk_to_group) | **POST** /api/chunk_group/chunk/{group_id} | Add Chunk to Group
[**add_chunk_to_group_by_tracking_id**](ChunkGroupApi.md#add_chunk_to_group_by_tracking_id) | **POST** /api/chunk_group/tracking_id/{tracking_id} | Add Chunk to Group by Tracking ID
[**create_chunk_group**](ChunkGroupApi.md#create_chunk_group) | **POST** /api/chunk_group | Create Chunk Group
[**delete_chunk_group**](ChunkGroupApi.md#delete_chunk_group) | **DELETE** /api/chunk_group/{group_id} | Delete Group
[**delete_group_by_tracking_id**](ChunkGroupApi.md#delete_group_by_tracking_id) | **DELETE** /api/chunk_group/tracking_id/{tracking_id} | Delete Group by Tracking ID
[**get_chunk_group**](ChunkGroupApi.md#get_chunk_group) | **GET** /api/chunk_group/{group_id} | Get Group
[**get_chunks_in_group**](ChunkGroupApi.md#get_chunks_in_group) | **GET** /api/chunk_group/{group_id}/{page} | Get Chunks in Group
[**get_chunks_in_group_by_tracking_id**](ChunkGroupApi.md#get_chunks_in_group_by_tracking_id) | **GET** /api/chunk_group/tracking_id/{group_tracking_id}/{page} | Get Chunks in Group by Tracking ID
[**get_group_by_tracking_id**](ChunkGroupApi.md#get_group_by_tracking_id) | **GET** /api/chunk_group/tracking_id/{tracking_id} | Get Group by Tracking ID
[**get_groups_chunk_is_in**](ChunkGroupApi.md#get_groups_chunk_is_in) | **POST** /api/chunk_group/chunks | Get Groups for Chunks
[**get_recommended_groups**](ChunkGroupApi.md#get_recommended_groups) | **POST** /api/chunk_group/recommend | Get Recommended Groups
[**get_specific_dataset_chunk_groups**](ChunkGroupApi.md#get_specific_dataset_chunk_groups) | **GET** /api/dataset/groups/{dataset_id}/{page} | Get Groups for Dataset
[**remove_chunk_from_group**](ChunkGroupApi.md#remove_chunk_from_group) | **DELETE** /api/chunk_group/chunk/{group_id} | Remove Chunk from Group
[**search_over_groups**](ChunkGroupApi.md#search_over_groups) | **POST** /api/chunk_group/group_oriented_search | Search Over Groups
[**search_within_group**](ChunkGroupApi.md#search_within_group) | **POST** /api/chunk_group/search | Search Within Group
[**update_chunk_group**](ChunkGroupApi.md#update_chunk_group) | **PUT** /api/chunk_group | Update Group
[**update_group_by_tracking_id**](ChunkGroupApi.md#update_group_by_tracking_id) | **PUT** /api/chunk_group/tracking_id/{tracking_id} | Update Group by Tracking ID



## add_chunk_to_group

> add_chunk_to_group(tr_dataset, group_id, add_chunk_to_group_data)
Add Chunk to Group

Add Chunk to Group  Route to add a chunk to a group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**group_id** | **uuid::Uuid** | Id of the group to add the chunk to as a bookmark | [required] |
**add_chunk_to_group_data** | [**AddChunkToGroupData**](AddChunkToGroupData.md) | JSON request payload to add a chunk to a group (bookmark it) | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_chunk_to_group_by_tracking_id

> add_chunk_to_group_by_tracking_id(tr_dataset, tracking_id, add_chunk_to_group_data)
Add Chunk to Group by Tracking ID

Add Chunk to Group by Tracking ID  Route to add a chunk to a group by tracking id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**tracking_id** | **String** | Id of the group to add the chunk to as a bookmark | [required] |
**add_chunk_to_group_data** | [**AddChunkToGroupData**](AddChunkToGroupData.md) | JSON request payload to add a chunk to a group (bookmark it) | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_chunk_group

> models::ChunkGroup create_chunk_group(tr_dataset, create_chunk_group_data)
Create Chunk Group

Create Chunk Group  Create a new chunk_group. This is a way to group chunks together. If you try to create a chunk_group with the same tracking_id as an existing chunk_group, this operation will fail.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**create_chunk_group_data** | [**CreateChunkGroupData**](CreateChunkGroupData.md) | JSON request payload to cretea a chunkGroup | [required] |

### Return type

[**models::ChunkGroup**](ChunkGroup.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_chunk_group

> delete_chunk_group(tr_dataset, group_id, delete_chunks)
Delete Group

Delete Group  This will delete a chunk_group. If you set delete_chunks to true, it will also delete the chunks within the group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**group_id** | **uuid::Uuid** | Id of the group you want to fetch. | [required] |
**delete_chunks** | **bool** | Delete the chunks within the group | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_group_by_tracking_id

> delete_group_by_tracking_id(tr_dataset, tracking_id, delete_chunks)
Delete Group by Tracking ID

Delete Group by Tracking ID  Delete a chunk_group with the given tracking id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**tracking_id** | **String** | Tracking id of the chunk_group to delete | [required] |
**delete_chunks** | **bool** | Delete the chunks within the group | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chunk_group

> models::ChunkGroup get_chunk_group(tr_dataset, group_id)
Get Group

Get Group  Fetch the group with the given id. get_group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**group_id** | **uuid::Uuid** | Id of the group you want to fetch. | [required] |

### Return type

[**models::ChunkGroup**](ChunkGroup.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chunks_in_group

> models::BookmarkData get_chunks_in_group(tr_dataset, group_id, page)
Get Chunks in Group

Get Chunks in Group  Route to get all chunks for a group. The response is paginated, with each page containing 10 chunks. Page is 1-indexed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**group_id** | **uuid::Uuid** | Id of the group you want to fetch. | [required] |
**page** | Option<**i64**> | The page of chunks to get from the group | [required] |

### Return type

[**models::BookmarkData**](BookmarkData.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chunks_in_group_by_tracking_id

> models::BookmarkData get_chunks_in_group_by_tracking_id(tr_dataset, group_tracking_id, page)
Get Chunks in Group by Tracking ID

Get Chunks in Group by Tracking ID  Route to get all chunks for a group. The response is paginated, with each page containing 10 chunks. Support for custom page size is coming soon. Page is 1-indexed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**group_tracking_id** | **String** | The id of the group to get the chunks from | [required] |
**page** | **i64** | The page of chunks to get from the group | [required] |

### Return type

[**models::BookmarkData**](BookmarkData.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_by_tracking_id

> models::ChunkGroup get_group_by_tracking_id(tr_dataset, tracking_id)
Get Group by Tracking ID

Get Group by Tracking ID  Fetch the group with the given tracking id. get_group_by_tracking_id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**tracking_id** | **String** | The tracking id of the group to fetch. | [required] |

### Return type

[**models::ChunkGroup**](ChunkGroup.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_groups_chunk_is_in

> Vec<models::BookmarkGroupResult> get_groups_chunk_is_in(tr_dataset, get_groups_for_chunks_data)
Get Groups for Chunks

Get Groups for Chunks  Route to get the groups that a chunk is in.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**get_groups_for_chunks_data** | [**GetGroupsForChunksData**](GetGroupsForChunksData.md) | JSON request payload to get the groups that a chunk is in | [required] |

### Return type

[**Vec<models::BookmarkGroupResult>**](BookmarkGroupResult.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recommended_groups

> Vec<models::GroupScoreChunk> get_recommended_groups(tr_dataset, recommend_group_chunks_request)
Get Recommended Groups

Get Recommended Groups  Route to get recommended groups. This route will return groups which are similar to the groups in the request body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**recommend_group_chunks_request** | [**RecommendGroupChunksRequest**](RecommendGroupChunksRequest.md) | JSON request payload to get recommendations of chunks similar to the chunks in the request | [required] |

### Return type

[**Vec<models::GroupScoreChunk>**](GroupScoreChunk.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_specific_dataset_chunk_groups

> models::GroupData get_specific_dataset_chunk_groups(tr_dataset, dataset_id, page)
Get Groups for Dataset

Get Groups for Dataset  Fetch the groups which belong to a dataset specified by its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**dataset_id** | **uuid::Uuid** | The id of the dataset to fetch groups for. | [required] |
**page** | **i64** | The page of groups to fetch. Page is 1-indexed. | [required] |

### Return type

[**models::GroupData**](GroupData.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_chunk_from_group

> remove_chunk_from_group(tr_dataset, group_id, create_chunk_group_data)
Remove Chunk from Group

Remove Chunk from Group  Route to remove a chunk from a group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**group_id** | **uuid::Uuid** | Id of the group to remove the bookmark'ed chunk from | [required] |
**create_chunk_group_data** | [**CreateChunkGroupData**](CreateChunkGroupData.md) | JSON request payload to cretea a chunkGroup | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_over_groups

> models::SearchOverGroupsResults search_over_groups(tr_dataset, search_over_groups_data)
Search Over Groups

Search Over Groups  This route allows you to get groups as results instead of chunks. Each group returned will have the matching chunks sorted by similarity within the group. This is useful for when you want to get groups of chunks which are similar to the search query. If choosing hybrid search, the results will be re-ranked using BAAI/bge-reranker-large. Compatible with semantic, fulltext, or hybrid search modes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**search_over_groups_data** | [**SearchOverGroupsData**](SearchOverGroupsData.md) | JSON request payload to semantically search over groups | [required] |

### Return type

[**models::SearchOverGroupsResults**](SearchOverGroupsResults.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_within_group

> models::SearchWithinGroupResults search_within_group(tr_dataset, search_within_group_data)
Search Within Group

Search Within Group  This route allows you to search only within a group. This is useful for when you only want search results to contain chunks which are members of a specific group. If choosing hybrid search, the results will be re-ranked using BAAI/bge-reranker-large.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**search_within_group_data** | [**SearchWithinGroupData**](SearchWithinGroupData.md) | JSON request payload to semantically search a group | [required] |

### Return type

[**models::SearchWithinGroupResults**](SearchWithinGroupResults.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_chunk_group

> update_chunk_group(tr_dataset, update_chunk_group_data)
Update Group

Update Group  Update a chunk_group. If you try to change the tracking_id to one that already exists, this operation will fail.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**update_chunk_group_data** | [**UpdateChunkGroupData**](UpdateChunkGroupData.md) | JSON request payload to update a chunkGroup | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group_by_tracking_id

> update_group_by_tracking_id(tr_dataset, tracking_id, update_group_by_tracking_id_data)
Update Group by Tracking ID

Update Group by Tracking ID  Update a chunk_group with the given tracking id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**tracking_id** | **uuid::Uuid** | Tracking id of the chunk_group to update | [required] |
**update_group_by_tracking_id_data** | [**UpdateGroupByTrackingIdData**](UpdateGroupByTrackingIdData.md) | JSON request payload to update a chunkGroup | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

