# \ChunkGroupApi

All URIs are relative to *https://api.trieve.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_chunk_to_group**](ChunkGroupApi.md#add_chunk_to_group) | **POST** /api/chunk_group/chunk/{group_id} | Add Chunk to Group
[**add_chunk_to_group_by_tracking_id**](ChunkGroupApi.md#add_chunk_to_group_by_tracking_id) | **POST** /api/chunk_group/tracking_id/{tracking_id} | Add Chunk to Group by Tracking ID
[**create_chunk_group**](ChunkGroupApi.md#create_chunk_group) | **POST** /api/chunk_group | Create or Upsert Group or Groups
[**delete_chunk_group**](ChunkGroupApi.md#delete_chunk_group) | **DELETE** /api/chunk_group/{group_id} | Delete Group
[**delete_group_by_tracking_id**](ChunkGroupApi.md#delete_group_by_tracking_id) | **DELETE** /api/chunk_group/tracking_id/{tracking_id} | Delete Group by Tracking ID
[**get_chunk_group**](ChunkGroupApi.md#get_chunk_group) | **GET** /api/chunk_group/{group_id} | Get Group
[**get_chunks_in_group**](ChunkGroupApi.md#get_chunks_in_group) | **GET** /api/chunk_group/{group_id}/{page} | Get Chunks in Group
[**get_chunks_in_group_by_tracking_id**](ChunkGroupApi.md#get_chunks_in_group_by_tracking_id) | **GET** /api/chunk_group/tracking_id/{group_tracking_id}/{page} | Get Chunks in Group by Tracking ID
[**get_group_by_tracking_id**](ChunkGroupApi.md#get_group_by_tracking_id) | **GET** /api/chunk_group/tracking_id/{tracking_id} | Get Group by Tracking ID
[**get_groups_for_chunks**](ChunkGroupApi.md#get_groups_for_chunks) | **POST** /api/chunk_group/chunks | Get Groups for Chunks
[**get_groups_for_dataset**](ChunkGroupApi.md#get_groups_for_dataset) | **GET** /api/dataset/groups/{dataset_id}/{page} | Get Groups for Dataset
[**get_recommended_groups**](ChunkGroupApi.md#get_recommended_groups) | **POST** /api/chunk_group/recommend | Get Recommended Groups
[**remove_chunk_from_group**](ChunkGroupApi.md#remove_chunk_from_group) | **DELETE** /api/chunk_group/chunk/{group_id} | Remove Chunk from Group
[**search_over_groups**](ChunkGroupApi.md#search_over_groups) | **POST** /api/chunk_group/group_oriented_search | Search Over Groups
[**search_within_group**](ChunkGroupApi.md#search_within_group) | **POST** /api/chunk_group/search | Search Within Group
[**update_chunk_group**](ChunkGroupApi.md#update_chunk_group) | **PUT** /api/chunk_group | Update Group
[**update_group_by_tracking_id**](ChunkGroupApi.md#update_group_by_tracking_id) | **PUT** /api/chunk_group/tracking_id/{tracking_id} | Update Group by Tracking ID



## add_chunk_to_group

> add_chunk_to_group(tr_dataset, group_id, add_chunk_to_group_req_payload)
Add Chunk to Group

Route to add a chunk to a group. One of chunk_id or chunk_tracking_id must be provided. Auth'ed user or api key must have an admin or owner role for the specified dataset's organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**group_id** | **uuid::Uuid** | Id of the group to add the chunk to as a bookmark | [required] |
**add_chunk_to_group_req_payload** | [**AddChunkToGroupReqPayload**](AddChunkToGroupReqPayload.md) | JSON request payload to add a chunk to a group (bookmark it) | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## add_chunk_to_group_by_tracking_id

> add_chunk_to_group_by_tracking_id(tr_dataset, tracking_id, add_chunk_to_group_req_payload)
Add Chunk to Group by Tracking ID

Route to add a chunk to a group by tracking id. One of chunk_id or chunk_tracking_id must be provided. Auth'ed user or api key must have an admin or owner role for the specified dataset's organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**tracking_id** | **String** | Tracking id of the group to add the chunk to as a bookmark | [required] |
**add_chunk_to_group_req_payload** | [**AddChunkToGroupReqPayload**](AddChunkToGroupReqPayload.md) | JSON request payload to add a chunk to a group via tracking_id | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_chunk_group

> models::CreateChunkGroupResponseEnum create_chunk_group(tr_dataset, create_chunk_group_req_payload_enum)
Create or Upsert Group or Groups

Create new chunk_group(s). This is a way to group chunks together. If you try to create a chunk_group with the same tracking_id as an existing chunk_group, this operation will fail. Only 1000 chunk groups can be created at a time. Auth'ed user or api key must have an admin or owner role for the specified dataset's organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**create_chunk_group_req_payload_enum** | [**CreateChunkGroupReqPayloadEnum**](CreateChunkGroupReqPayloadEnum.md) | JSON request payload to cretea a chunk_group(s) | [required] |

### Return type

[**models::CreateChunkGroupResponseEnum**](CreateChunkGroupResponseEnum.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_chunk_group

> delete_chunk_group(tr_dataset, group_id, delete_chunks)
Delete Group

This will delete a chunk_group. If you set delete_chunks to true, it will also delete the chunks within the group. Auth'ed user or api key must have an admin or owner role for the specified dataset's organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
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

Delete a chunk_group with the given tracking id. Auth'ed user or api key must have an admin or owner role for the specified dataset's organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
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

> models::ChunkGroupAndFileId get_chunk_group(tr_dataset, group_id)
Get Group

Fetch the group with the given id. get_group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**group_id** | **uuid::Uuid** | Id of the group you want to fetch. | [required] |

### Return type

[**models::ChunkGroupAndFileId**](ChunkGroupAndFileId.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chunks_in_group

> models::GetChunksInGroupResponse get_chunks_in_group(tr_dataset, group_id, page, x_api_version)
Get Chunks in Group

Route to get all chunks for a group. The response is paginated, with each page containing 10 chunks. Page is 1-indexed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**group_id** | **uuid::Uuid** | Id of the group you want to fetch. | [required] |
**page** | Option<**i64**> | The page of chunks to get from the group | [required] |
**x_api_version** | Option<[**models::ApiVersion**](.md)> | The version of the API to use for the request |  |

### Return type

[**models::GetChunksInGroupResponse**](GetChunksInGroupResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chunks_in_group_by_tracking_id

> models::GetChunksInGroupResponse get_chunks_in_group_by_tracking_id(tr_dataset, group_tracking_id, page, x_api_version)
Get Chunks in Group by Tracking ID

Route to get all chunks for a group. The response is paginated, with each page containing 10 chunks. Support for custom page size is coming soon. Page is 1-indexed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**group_tracking_id** | **String** | The id of the group to get the chunks from | [required] |
**page** | **i64** | The page of chunks to get from the group | [required] |
**x_api_version** | Option<[**models::ApiVersion**](.md)> | The version of the API to use for the request |  |

### Return type

[**models::GetChunksInGroupResponse**](GetChunksInGroupResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_group_by_tracking_id

> models::ChunkGroupAndFileId get_group_by_tracking_id(tr_dataset, tracking_id)
Get Group by Tracking ID

Fetch the group with the given tracking id. get_group_by_tracking_id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**tracking_id** | **String** | The tracking id of the group to fetch. | [required] |

### Return type

[**models::ChunkGroupAndFileId**](ChunkGroupAndFileId.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_groups_for_chunks

> Vec<models::GroupsForChunk> get_groups_for_chunks(tr_dataset, get_groups_for_chunks_req_payload)
Get Groups for Chunks

Route to get the groups that a chunk is in.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**get_groups_for_chunks_req_payload** | [**GetGroupsForChunksReqPayload**](GetGroupsForChunksReqPayload.md) | JSON request payload to get the groups that a chunk is in | [required] |

### Return type

[**Vec<models::GroupsForChunk>**](GroupsForChunk.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_groups_for_dataset

> models::GroupData get_groups_for_dataset(tr_dataset, dataset_id, page)
Get Groups for Dataset

Fetch the groups which belong to a dataset specified by its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
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


## get_recommended_groups

> models::RecommendGroupsResponse get_recommended_groups(tr_dataset, recommend_groups_req_payload, x_api_version)
Get Recommended Groups

Route to get recommended groups. This route will return groups which are similar to the groups in the request body. You must provide at least one positive group id or group tracking id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**recommend_groups_req_payload** | [**RecommendGroupsReqPayload**](RecommendGroupsReqPayload.md) | JSON request payload to get recommendations of chunks similar to the chunks in the request | [required] |
**x_api_version** | Option<[**models::ApiVersion**](.md)> | The API version to use for this request. Defaults to V2 for orgs created after July 12, 2024 and V1 otherwise. |  |

### Return type

[**models::RecommendGroupsResponse**](RecommendGroupsResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## remove_chunk_from_group

> remove_chunk_from_group(tr_dataset, group_id, chunk_id, remove_chunk_from_group_req_payload)
Remove Chunk from Group

Route to remove a chunk from a group. Auth'ed user or api key must be an admin or owner of the dataset's organization to remove a chunk from a group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**group_id** | **uuid::Uuid** | Id of the group you want to remove the chunk from. | [required] |
**chunk_id** | Option<**uuid::Uuid**> | Id of the chunk you want to remove from the group |  |
**remove_chunk_from_group_req_payload** | Option<[**RemoveChunkFromGroupReqPayload**](RemoveChunkFromGroupReqPayload.md)> | JSON request payload to remove a chunk from a group |  |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_over_groups

> models::SearchOverGroupsResponseTypes search_over_groups(tr_dataset, search_over_groups_req_payload, x_api_version)
Search Over Groups

This route allows you to get groups as results instead of chunks. Each group returned will have the matching chunks sorted by similarity within the group. This is useful for when you want to get groups of chunks which are similar to the search query. If choosing hybrid search, the results will be re-ranked using scores from a cross encoder model. Compatible with semantic, fulltext, or hybrid search modes.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**search_over_groups_req_payload** | [**SearchOverGroupsReqPayload**](SearchOverGroupsReqPayload.md) | JSON request payload to semantically search over groups | [required] |
**x_api_version** | Option<[**models::ApiVersion**](.md)> | The API version to use for this request. Defaults to V2 for orgs created after July 12, 2024 and V1 otherwise. |  |

### Return type

[**models::SearchOverGroupsResponseTypes**](SearchOverGroupsResponseTypes.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_within_group

> models::SearchGroupResponseTypes search_within_group(tr_dataset, search_within_group_req_payload, x_api_version)
Search Within Group

This route allows you to search only within a group. This is useful for when you only want search results to contain chunks which are members of a specific group. If choosing hybrid search, the results will be re-ranked using scores from a cross encoder model.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**search_within_group_req_payload** | [**SearchWithinGroupReqPayload**](SearchWithinGroupReqPayload.md) | JSON request payload to semantically search a group | [required] |
**x_api_version** | Option<[**models::ApiVersion**](.md)> | The API version to use for this request. Defaults to V2 for orgs created after July 12, 2024 and V1 otherwise. |  |

### Return type

[**models::SearchGroupResponseTypes**](SearchGroupResponseTypes.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_chunk_group

> update_chunk_group(tr_dataset, update_chunk_group_req_payload)
Update Group

Update a chunk_group. One of group_id or tracking_id must be provided. If you try to change the tracking_id to one that already exists, this operation will fail. Auth'ed user or api key must have an admin or owner role for the specified dataset's organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**update_chunk_group_req_payload** | [**UpdateChunkGroupReqPayload**](UpdateChunkGroupReqPayload.md) | JSON request payload to update a chunkGroup | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_group_by_tracking_id

> update_group_by_tracking_id(tr_dataset, tracking_id, update_group_by_tracking_id_req_payload)
Update Group by Tracking ID

Update a chunk_group with the given tracking id. Auth'ed user or api key must have an admin or owner role for the specified dataset's organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**tracking_id** | **uuid::Uuid** | Tracking id of the chunk_group to update | [required] |
**update_group_by_tracking_id_req_payload** | [**UpdateGroupByTrackingIdReqPayload**](UpdateGroupByTrackingIdReqPayload.md) | JSON request payload to update a chunkGroup | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

