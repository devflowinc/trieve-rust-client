# \AnalyticsApi

All URIs are relative to *https://api.trieve.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_all_queries**](AnalyticsApi.md#get_all_queries) | **POST** /api/analytics/{dataset_id}/queries | Get All Search Queries
[**get_head_queries**](AnalyticsApi.md#get_head_queries) | **POST** /api/analytics/{dataset_id}/query/head | Get Head Queries
[**get_latency_graph**](AnalyticsApi.md#get_latency_graph) | **POST** /api/analytics/{dataset_id}/latency | Get Latency Graph
[**get_low_confidence_queries**](AnalyticsApi.md#get_low_confidence_queries) | **POST** /api/analytics/{dataset_id}/query/low_confidence | Get Low Confidence Queries
[**get_overall_topics**](AnalyticsApi.md#get_overall_topics) | **GET** /api/analytics/{dataset_id}/topics | Get Cluster Topics for a Dataset
[**get_queries_for_topic**](AnalyticsApi.md#get_queries_for_topic) | **GET** /api/analytics/{dataset_id}/{cluster_id}/{page} | Get Queries for a Topic
[**get_query**](AnalyticsApi.md#get_query) | **GET** /api/analytics/{dataset_id}/query/{search_id} | Get a Query
[**get_rag_queries**](AnalyticsApi.md#get_rag_queries) | **POST** /api/analytics/{dataset_id}/rag | 
[**get_rag_usage**](AnalyticsApi.md#get_rag_usage) | **GET** /api/analytics/{dataset_id}/rag/usage | 
[**get_rps_graph**](AnalyticsApi.md#get_rps_graph) | **POST** /api/analytics/{dataset_id}/rps | Get RPS Graph
[**get_search_metrics**](AnalyticsApi.md#get_search_metrics) | **POST** /api/analytics/{dataset_id}/metrics | Get Search Metrics



## get_all_queries

> Vec<models::SearchQueryEvent> get_all_queries(tr_dataset, dataset_id, get_all_queries_request)
Get All Search Queries

This route allows you to get all search queries and sort them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**dataset_id** | **uuid::Uuid** | The id of the dataset you want to get queries for. | [required] |
**get_all_queries_request** | [**GetAllQueriesRequest**](GetAllQueriesRequest.md) | JSON request payload to filter the queries | [required] |

### Return type

[**Vec<models::SearchQueryEvent>**](SearchQueryEvent.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_head_queries

> Vec<models::HeadQueries> get_head_queries(tr_dataset, dataset_id, get_head_queries_request)
Get Head Queries

This route allows you to get the most common queries for a dataset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**dataset_id** | **uuid::Uuid** | The id of the dataset you want to get head queries for. | [required] |
**get_head_queries_request** | [**GetHeadQueriesRequest**](GetHeadQueriesRequest.md) | JSON request payload to filter the analytics | [required] |

### Return type

[**Vec<models::HeadQueries>**](HeadQueries.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_latency_graph

> Vec<models::SearchLatencyGraph> get_latency_graph(tr_dataset, dataset_id, get_rps_graph_request)
Get Latency Graph

This route allows you to get the latency graph for a dataset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**dataset_id** | **uuid::Uuid** | The id of the dataset you want to get latency graph for. | [required] |
**get_rps_graph_request** | [**GetRpsGraphRequest**](GetRpsGraphRequest.md) | JSON request payload to filter the graph | [required] |

### Return type

[**Vec<models::SearchLatencyGraph>**](SearchLatencyGraph.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_low_confidence_queries

> Vec<models::SearchQueryEvent> get_low_confidence_queries(tr_dataset, dataset_id, get_head_queries_request)
Get Low Confidence Queries

This route allows you to get the queries that have the lowest confidence scores.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**dataset_id** | **uuid::Uuid** | The id of the dataset you want to get low confidence queries for. | [required] |
**get_head_queries_request** | [**GetHeadQueriesRequest**](GetHeadQueriesRequest.md) | JSON request payload to filter the analytics | [required] |

### Return type

[**Vec<models::SearchQueryEvent>**](SearchQueryEvent.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_overall_topics

> models::SearchClusterTopics get_overall_topics(tr_dataset, dataset_id)
Get Cluster Topics for a Dataset

This route allows you to view the top 15 topics for a dataset based on the clustering of the queries in the dataset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**dataset_id** | **uuid::Uuid** | The id of the dataset you want to get query clusters for. | [required] |

### Return type

[**models::SearchClusterTopics**](SearchClusterTopics.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_queries_for_topic

> models::SearchQueryEvent get_queries_for_topic(tr_dataset, dataset_id, page, cluster_id)
Get Queries for a Topic

This route allows you to view the queries that are associated with a specific topic.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**dataset_id** | **uuid::Uuid** | The id of the dataset you want to get query clusters for. | [required] |
**page** | **i32** | The page number to get the queries for the topic | [required] |
**cluster_id** | **uuid::Uuid** | The id of the cluster you want to get queries for. | [required] |

### Return type

[**models::SearchQueryEvent**](SearchQueryEvent.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_query

> models::SearchQueryEvent get_query(tr_dataset, dataset_id, search_id)
Get a Query

This route allows you to view the details of a specific query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**dataset_id** | **uuid::Uuid** | The id of the dataset you want to get the search for. | [required] |
**search_id** | **uuid::Uuid** | The id of the search. | [required] |

### Return type

[**models::SearchQueryEvent**](SearchQueryEvent.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rag_queries

> Vec<models::GetRagQueriesRequest> get_rag_queries(tr_dataset, dataset_id, get_rps_graph_request)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**dataset_id** | **uuid::Uuid** | The id of the dataset you want to get RAG queries for. | [required] |
**get_rps_graph_request** | [**GetRpsGraphRequest**](GetRpsGraphRequest.md) | JSON request payload to filter the graph | [required] |

### Return type

[**Vec<models::GetRagQueriesRequest>**](GetRagQueriesRequest.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rag_usage

> models::RagUsageResponse get_rag_usage(tr_dataset, dataset_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**dataset_id** | **uuid::Uuid** | The id of the dataset you want to get RAG usage for. | [required] |

### Return type

[**models::RagUsageResponse**](RAGUsageResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rps_graph

> Vec<models::SearchRpsGraph> get_rps_graph(tr_dataset, dataset_id, get_rps_graph_request)
Get RPS Graph

This route allows you to get the RPS graph for a dataset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**dataset_id** | **uuid::Uuid** | The id of the dataset you want to get RPS graph for. | [required] |
**get_rps_graph_request** | [**GetRpsGraphRequest**](GetRpsGraphRequest.md) | JSON request payload to filter the analytics | [required] |

### Return type

[**Vec<models::SearchRpsGraph>**](SearchRPSGraph.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_metrics

> models::DatasetAnalytics get_search_metrics(tr_dataset, dataset_id, get_dataset_metrics_request)
Get Search Metrics

This route allows you to get the search metrics for a dataset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**dataset_id** | **uuid::Uuid** | The id of the dataset you want to get search metrics for. | [required] |
**get_dataset_metrics_request** | [**GetDatasetMetricsRequest**](GetDatasetMetricsRequest.md) | JSON request payload to filter the analytics | [required] |

### Return type

[**models::DatasetAnalytics**](DatasetAnalytics.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

