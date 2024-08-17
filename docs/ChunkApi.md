# \ChunkApi

All URIs are relative to *https://api.trieve.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**autocomplete**](ChunkApi.md#autocomplete) | **POST** /api/chunk/autocomplete | Autocomplete
[**count_chunks**](ChunkApi.md#count_chunks) | **POST** /api/chunk/count | Count chunks above threshold
[**create_chunk**](ChunkApi.md#create_chunk) | **POST** /api/chunk | Create or Upsert Chunk or Chunks
[**delete_chunk**](ChunkApi.md#delete_chunk) | **DELETE** /api/chunk/{chunk_id} | Delete Chunk
[**delete_chunk_by_tracking_id**](ChunkApi.md#delete_chunk_by_tracking_id) | **DELETE** /api/chunk/tracking_id/{tracking_id} | Delete Chunk By Tracking Id
[**generate_off_chunks**](ChunkApi.md#generate_off_chunks) | **POST** /api/chunk/generate | RAG on Specified Chunks
[**get_chunk_by_id**](ChunkApi.md#get_chunk_by_id) | **GET** /api/chunk/{chunk_id} | Get Chunk By Id
[**get_chunk_by_tracking_id**](ChunkApi.md#get_chunk_by_tracking_id) | **GET** /api/chunk/tracking_id/{tracking_id} | Get Chunk By Tracking Id
[**get_chunks_by_ids**](ChunkApi.md#get_chunks_by_ids) | **POST** /api/chunks | Get Chunks By Ids
[**get_chunks_by_tracking_ids**](ChunkApi.md#get_chunks_by_tracking_ids) | **POST** /api/chunks/tracking | Get Chunks By Tracking Ids
[**get_recommended_chunks**](ChunkApi.md#get_recommended_chunks) | **POST** /api/chunk/recommend | Get Recommended Chunks
[**get_suggested_queries**](ChunkApi.md#get_suggested_queries) | **POST** /api/chunk/suggestions | Generate suggested queries
[**scroll_dataset_chunks**](ChunkApi.md#scroll_dataset_chunks) | **POST** /api/chunks/scroll | Scroll Chunks
[**search_chunks**](ChunkApi.md#search_chunks) | **POST** /api/chunk/search | Search
[**update_chunk**](ChunkApi.md#update_chunk) | **PUT** /api/chunk | Update Chunk
[**update_chunk_by_tracking_id**](ChunkApi.md#update_chunk_by_tracking_id) | **PUT** /api/chunk/tracking_id/update | Update Chunk By Tracking Id



## autocomplete

> models::SearchResponseTypes autocomplete(tr_dataset, autocomplete_req_payload, x_api_version)
Autocomplete

This route provides the primary autocomplete functionality for the API. This prioritize prefix matching with semantic or full-text search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**autocomplete_req_payload** | [**AutocompleteReqPayload**](AutocompleteReqPayload.md) | JSON request payload to semantically search for chunks (chunks) | [required] |
**x_api_version** | Option<[**models::ApiVersion**](.md)> | The API version to use for this request. Defaults to V2 for orgs created after July 12, 2024 and V1 otherwise. |  |

### Return type

[**models::SearchResponseTypes**](SearchResponseTypes.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## count_chunks

> models::CountChunkQueryResponseBody count_chunks(tr_dataset, count_chunks_req_payload)
Count chunks above threshold

This route can be used to determine the number of chunk results that match a search query including score threshold and filters. It may be high latency for large limits. There is a dataset configuration imposed restriction on the maximum limit value (default 10,000) which is used to prevent DDOS attacks. Auth'ed user or api key must have an admin or owner role for the specified dataset's organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**count_chunks_req_payload** | [**CountChunksReqPayload**](CountChunksReqPayload.md) | JSON request payload to count chunks for a search query | [required] |

### Return type

[**models::CountChunkQueryResponseBody**](CountChunkQueryResponseBody.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_chunk

> models::ReturnQueuedChunk create_chunk(tr_dataset, create_chunk_req_payload_enum)
Create or Upsert Chunk or Chunks

Create new chunk(s). If the chunk has the same tracking_id as an existing chunk, the request will fail. Once a chunk is created, it can be searched for using the search endpoint. If uploading in bulk, the maximum amount of chunks that can be uploaded at once is 120 chunks. Auth'ed user or api key must have an admin or owner role for the specified dataset's organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**create_chunk_req_payload_enum** | [**CreateChunkReqPayloadEnum**](CreateChunkReqPayloadEnum.md) | JSON request payload to create a new chunk (chunk) | [required] |

### Return type

[**models::ReturnQueuedChunk**](ReturnQueuedChunk.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_chunk

> delete_chunk(tr_dataset, chunk_id)
Delete Chunk

Delete a chunk by its id. If deleting a root chunk which has a collision, the most recently created collision will become a new root chunk. Auth'ed user or api key must have an admin or owner role for the specified dataset's organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**chunk_id** | **uuid::Uuid** | Id of the chunk you want to fetch. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_chunk_by_tracking_id

> delete_chunk_by_tracking_id(tr_dataset, tracking_id)
Delete Chunk By Tracking Id

Delete a chunk by tracking_id. This is useful for when you are coordinating with an external system and want to use the tracking_id to identify the chunk. If deleting a root chunk which has a collision, the most recently created collision will become a new root chunk. Auth'ed user or api key must have an admin or owner role for the specified dataset's organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**tracking_id** | **String** | tracking_id of the chunk you want to delete | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## generate_off_chunks

> String generate_off_chunks(tr_dataset, generate_off_chunks_req_payload)
RAG on Specified Chunks

This endpoint exists as an alternative to the topic+message resource pattern where our Trieve handles chat memory. With this endpoint, the user is responsible for providing the context window and the prompt and the conversation is ephemeral.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**generate_off_chunks_req_payload** | [**GenerateOffChunksReqPayload**](GenerateOffChunksReqPayload.md) | JSON request payload to perform RAG on some chunks (chunks) | [required] |

### Return type

**String**

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chunk_by_id

> models::ChunkReturnTypes get_chunk_by_id(tr_dataset, chunk_id, x_api_version)
Get Chunk By Id

Get a singular chunk by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**chunk_id** | **uuid::Uuid** | Id of the chunk you want to fetch. | [required] |
**x_api_version** | Option<[**models::ApiVersion**](.md)> | The API version to use for this request. Defaults to V2 for orgs created after July 12, 2024 and V1 otherwise. |  |

### Return type

[**models::ChunkReturnTypes**](ChunkReturnTypes.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chunk_by_tracking_id

> models::ChunkReturnTypes get_chunk_by_tracking_id(tr_dataset, tracking_id, x_api_version)
Get Chunk By Tracking Id

Get a singular chunk by tracking_id. This is useful for when you are coordinating with an external system and want to use your own id as the primary reference for a chunk.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**tracking_id** | **String** | tracking_id of the chunk you want to fetch | [required] |
**x_api_version** | Option<[**models::ApiVersion**](.md)> | The API version to use for this request. Defaults to V2 for orgs created after July 12, 2024 and V1 otherwise. |  |

### Return type

[**models::ChunkReturnTypes**](ChunkReturnTypes.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chunks_by_ids

> Vec<models::ChunkReturnTypes> get_chunks_by_ids(tr_dataset, get_chunks_data, x_api_version)
Get Chunks By Ids

Get multiple chunks by multiple ids.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**get_chunks_data** | [**GetChunksData**](GetChunksData.md) | JSON request payload to get the chunks in the request | [required] |
**x_api_version** | Option<[**models::ApiVersion**](.md)> | The API version to use for this request. Defaults to V2 for orgs created after July 12, 2024 and V1 otherwise. |  |

### Return type

[**Vec<models::ChunkReturnTypes>**](ChunkReturnTypes.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chunks_by_tracking_ids

> Vec<models::ChunkReturnTypes> get_chunks_by_tracking_ids(tr_dataset, get_tracking_chunks_data, x_api_version)
Get Chunks By Tracking Ids

Get multiple chunks by ids.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**get_tracking_chunks_data** | [**GetTrackingChunksData**](GetTrackingChunksData.md) | JSON request payload to get the chunks in the request | [required] |
**x_api_version** | Option<[**models::ApiVersion**](.md)> | The API version to use for this request. Defaults to V2 for orgs created after July 12, 2024 and V1 otherwise. |  |

### Return type

[**Vec<models::ChunkReturnTypes>**](ChunkReturnTypes.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recommended_chunks

> models::RecommendResponseTypes get_recommended_chunks(tr_dataset, recommend_chunks_request, x_api_version)
Get Recommended Chunks

Get recommendations of chunks similar to the positive samples in the request and dissimilar to the negative.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**recommend_chunks_request** | [**RecommendChunksRequest**](RecommendChunksRequest.md) | JSON request payload to get recommendations of chunks similar to the chunks in the request | [required] |
**x_api_version** | Option<[**models::ApiVersion**](.md)> | The API version to use for this request. Defaults to V2 for orgs created after July 12, 2024 and V1 otherwise. |  |

### Return type

[**models::RecommendResponseTypes**](RecommendResponseTypes.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_suggested_queries

> models::SuggestedQueriesResponse get_suggested_queries(tr_dataset, suggested_queries_req_payload)
Generate suggested queries

This endpoint will generate 3 suggested queries based off a hybrid search using RAG with the query provided in the request body and return them as a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**suggested_queries_req_payload** | [**SuggestedQueriesReqPayload**](SuggestedQueriesReqPayload.md) | JSON request payload to get alternative suggested queries | [required] |

### Return type

[**models::SuggestedQueriesResponse**](SuggestedQueriesResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## scroll_dataset_chunks

> models::ScrollChunksResponseBody scroll_dataset_chunks(tr_dataset, scroll_chunks_req_payload)
Scroll Chunks

Get paginated chunks from your dataset with filters and custom sorting. If sort by is not specified, the results will sort by the id's of the chunks in ascending order. Sort by and offset_chunk_id cannot be used together; if you want to scroll with a sort by then you need to use a must_not filter with the ids you have already seen. There is a limit of 1000 id's in a must_not filter at a time.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**scroll_chunks_req_payload** | [**ScrollChunksReqPayload**](ScrollChunksReqPayload.md) | JSON request payload to scroll through chunks (chunks) | [required] |

### Return type

[**models::ScrollChunksResponseBody**](ScrollChunksResponseBody.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_chunks

> models::SearchResponseTypes search_chunks(tr_dataset, search_chunks_req_payload, x_api_version)
Search

This route provides the primary search functionality for the API. It can be used to search for chunks by semantic similarity, full-text similarity, or a combination of both. Results' `chunk_html` values will be modified with `<b><mark>` tags for sub-sentence highlighting.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**search_chunks_req_payload** | [**SearchChunksReqPayload**](SearchChunksReqPayload.md) | JSON request payload to semantically search for chunks (chunks) | [required] |
**x_api_version** | Option<[**models::ApiVersion**](.md)> | The API version to use for this request. Defaults to V2 for orgs created after July 12, 2024 and V1 otherwise. |  |

### Return type

[**models::SearchResponseTypes**](SearchResponseTypes.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_chunk

> update_chunk(tr_dataset, update_chunk_req_payload)
Update Chunk

Update a chunk. If you try to change the tracking_id of the chunk to have the same tracking_id as an existing chunk, the request will fail. Auth'ed user or api key must have an admin or owner role for the specified dataset's organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**update_chunk_req_payload** | [**UpdateChunkReqPayload**](UpdateChunkReqPayload.md) | JSON request payload to update a chunk (chunk) | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_chunk_by_tracking_id

> update_chunk_by_tracking_id(tr_dataset, update_chunk_by_tracking_id_data)
Update Chunk By Tracking Id

Update a chunk by tracking_id. This is useful for when you are coordinating with an external system and want to use the tracking_id to identify the chunk. Auth'ed user or api key must have an admin or owner role for the specified dataset's organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**update_chunk_by_tracking_id_data** | [**UpdateChunkByTrackingIdData**](UpdateChunkByTrackingIdData.md) | JSON request payload to update a chunk by tracking_id (chunks) | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

