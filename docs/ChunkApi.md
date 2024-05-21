# \ChunkApi

All URIs are relative to *https://api.trieve.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**autocomplete**](ChunkApi.md#autocomplete) | **POST** /api/chunk/autocomplete | Autocomplete
[**create_chunk**](ChunkApi.md#create_chunk) | **POST** /api/chunk | Create or Upsert Chunk or Chunks
[**create_suggested_queries_handler**](ChunkApi.md#create_suggested_queries_handler) | **POST** /api/chunk/gen_suggestions | Generate suggested queries
[**delete_chunk**](ChunkApi.md#delete_chunk) | **DELETE** /api/chunk/{chunk_id} | Delete Chunk
[**delete_chunk_by_tracking_id**](ChunkApi.md#delete_chunk_by_tracking_id) | **DELETE** /api/chunk/tracking_id/{tracking_id} | Delete Chunk By Tracking Id
[**generate_off_chunks**](ChunkApi.md#generate_off_chunks) | **POST** /api/chunk/generate | RAG on Specified Chunks
[**get_chunk_by_id**](ChunkApi.md#get_chunk_by_id) | **GET** /api/chunk/{chunk_id} | Get Chunk By Id
[**get_chunk_by_tracking_id**](ChunkApi.md#get_chunk_by_tracking_id) | **GET** /api/chunk/tracking_id/{tracking_id} | Get Chunk By Tracking Id
[**get_chunks_by_ids**](ChunkApi.md#get_chunks_by_ids) | **POST** /api/chunks | Get Chunks By Ids
[**get_chunks_by_tracking_ids**](ChunkApi.md#get_chunks_by_tracking_ids) | **POST** /api/chunks/tracking | Get Chunks By TrackingIds
[**get_recommended_chunks**](ChunkApi.md#get_recommended_chunks) | **POST** /api/chunk/recommend | Get Recommended Chunks
[**search_chunks**](ChunkApi.md#search_chunks) | **POST** /api/chunk/search | Search
[**update_chunk**](ChunkApi.md#update_chunk) | **PUT** /api/chunk | Update Chunk
[**update_chunk_by_tracking_id**](ChunkApi.md#update_chunk_by_tracking_id) | **PUT** /api/chunk/tracking_id/update | Update Chunk By Tracking Id



## autocomplete

> models::SearchChunkQueryResponseBody autocomplete(tr_dataset, autocomplete_data)
Autocomplete

Autocomplete  This route provides the primary autocomplete functionality for the API. This prioritize prefix matching with semantic or full-text search.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**autocomplete_data** | [**AutocompleteData**](AutocompleteData.md) | JSON request payload to semantically search for chunks (chunks) | [required] |

### Return type

[**models::SearchChunkQueryResponseBody**](SearchChunkQueryResponseBody.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_chunk

> models::ReturnQueuedChunk create_chunk(tr_dataset, create_chunk_data)
Create or Upsert Chunk or Chunks

Create or Upsert Chunk or Chunks  Create a new chunk. If the chunk has the same tracking_id as an existing chunk, the request will fail. Once a chunk is created, it can be searched for using the search endpoint. If uploading in bulk, the maximum amount of chunks that can be uploaded at once is 120 chunks.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**create_chunk_data** | [**CreateChunkData**](CreateChunkData.md) | JSON request payload to create a new chunk (chunk) | [required] |

### Return type

[**models::ReturnQueuedChunk**](ReturnQueuedChunk.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_suggested_queries_handler

> models::SuggestedQueriesResponse create_suggested_queries_handler(tr_dataset, suggested_queries_request)
Generate suggested queries

Generate suggested queries  This endpoint will generate 3 suggested queries based off the query provided in the request body and return them as a JSON object.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**suggested_queries_request** | [**SuggestedQueriesRequest**](SuggestedQueriesRequest.md) | JSON request payload to get alternative suggested queries | [required] |

### Return type

[**models::SuggestedQueriesResponse**](SuggestedQueriesResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_chunk

> delete_chunk(tr_dataset, chunk_id)
Delete Chunk

Delete Chunk  Delete a chunk by its id. If deleting a root chunk which has a collision, the most recently created collision will become a new root chunk.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
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

Delete Chunk By Tracking Id  Delete a chunk by tracking_id. This is useful for when you are coordinating with an external system and want to use the tracking_id to identify the chunk. If deleting a root chunk which has a collision, the most recently created collision will become a new root chunk.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
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

> String generate_off_chunks(tr_dataset, generate_chunks_request)
RAG on Specified Chunks

RAG on Specified Chunks  This endpoint exists as an alternative to the topic+message concept where our API handles chat memory. With this endpoint, the user is responsible for providing the context window and the prompt. See more in the \"search before generate\" page at docs.trieve.ai.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**generate_chunks_request** | [**GenerateChunksRequest**](GenerateChunksRequest.md) | JSON request payload to perform RAG on some chunks (chunks) | [required] |

### Return type

**String**

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: text/plain, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chunk_by_id

> models::ChunkMetadata get_chunk_by_id(tr_dataset, chunk_id)
Get Chunk By Id

Get Chunk By Id  Get a singular chunk by id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**chunk_id** | **uuid::Uuid** | Id of the chunk you want to fetch. | [required] |

### Return type

[**models::ChunkMetadata**](ChunkMetadata.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chunk_by_tracking_id

> models::ChunkMetadata get_chunk_by_tracking_id(tr_dataset, tracking_id)
Get Chunk By Tracking Id

Get Chunk By Tracking Id  Get a singular chunk by tracking_id. This is useful for when you are coordinating with an external system and want to use your own id as the primary reference for a chunk.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**tracking_id** | **String** | tracking_id of the chunk you want to fetch | [required] |

### Return type

[**models::ChunkMetadata**](ChunkMetadata.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chunks_by_ids

> Vec<models::ChunkMetadata> get_chunks_by_ids(tr_dataset, get_chunks_data)
Get Chunks By Ids

Get Chunks By Ids  Get multiple chunks by multiple ids.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**get_chunks_data** | [**GetChunksData**](GetChunksData.md) | JSON request payload to get the chunks in the request | [required] |

### Return type

[**Vec<models::ChunkMetadata>**](ChunkMetadata.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_chunks_by_tracking_ids

> models::ChunkMetadata get_chunks_by_tracking_ids(tr_dataset, get_tracking_chunks_data)
Get Chunks By TrackingIds

Get Chunks By TrackingIds  Get multiple chunks by ids.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**get_tracking_chunks_data** | [**GetTrackingChunksData**](GetTrackingChunksData.md) | JSON request payload to get the chunks in the request | [required] |

### Return type

[**models::ChunkMetadata**](ChunkMetadata.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recommended_chunks

> Vec<models::ChunkMetadataWithScore> get_recommended_chunks(tr_dataset, recommend_chunks_request)
Get Recommended Chunks

Get Recommended Chunks  Get recommendations of chunks similar to the chunks in the request. Think about this as a feature similar to the \"add to playlist\" recommendation feature on Spotify. This request pairs especially well with our groups endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**recommend_chunks_request** | [**RecommendChunksRequest**](RecommendChunksRequest.md) | JSON request payload to get recommendations of chunks similar to the chunks in the request | [required] |

### Return type

[**Vec<models::ChunkMetadataWithScore>**](ChunkMetadataWithScore.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## search_chunks

> models::SearchChunkQueryResponseBody search_chunks(tr_dataset, search_chunk_data)
Search

Search  This route provides the primary search functionality for the API. It can be used to search for chunks by semantic similarity, full-text similarity, or a combination of both. Results' `chunk_html` values will be modified with `<b><mark>` tags for sub-sentence highlighting.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**search_chunk_data** | [**SearchChunkData**](SearchChunkData.md) | JSON request payload to semantically search for chunks (chunks) | [required] |

### Return type

[**models::SearchChunkQueryResponseBody**](SearchChunkQueryResponseBody.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_chunk

> update_chunk(tr_dataset, update_chunk_data)
Update Chunk

Update Chunk  Update a chunk. If you try to change the tracking_id of the chunk to have the same tracking_id as an existing chunk, the request will fail.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**update_chunk_data** | [**UpdateChunkData**](UpdateChunkData.md) | JSON request payload to update a chunk (chunk) | [required] |

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

Update Chunk By Tracking Id  Update a chunk by tracking_id. This is useful for when you are coordinating with an external system and want to use the tracking_id to identify the chunk.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**update_chunk_by_tracking_id_data** | [**UpdateChunkByTrackingIdData**](UpdateChunkByTrackingIdData.md) | JSON request payload to update a chunk by tracking_id (chunks) | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

