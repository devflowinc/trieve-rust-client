# \AnalyticsApi

All URIs are relative to *https://api.trieve.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_cluster_analytics**](AnalyticsApi.md#get_cluster_analytics) | **POST** /api/analytics/search/cluster | Get Cluster Analytics
[**get_ctr_analytics**](AnalyticsApi.md#get_ctr_analytics) | **POST** /api/analytics/ctr | Get CTR Analytics
[**get_rag_analytics**](AnalyticsApi.md#get_rag_analytics) | **POST** /api/analytics/rag | Get RAG Analytics
[**get_recommendation_analytics**](AnalyticsApi.md#get_recommendation_analytics) | **POST** /api/analytics/recommendations | Get Recommendation Analytics
[**get_search_analytics**](AnalyticsApi.md#get_search_analytics) | **POST** /api/analytics/search | Get Search Analytics
[**send_ctr_data**](AnalyticsApi.md#send_ctr_data) | **PUT** /api/analytics/ctr | Send CTR Data
[**set_query_rating**](AnalyticsApi.md#set_query_rating) | **PUT** /api/analytics/search | Rate Query



## get_cluster_analytics

> models::ClusterAnalyticsResponse get_cluster_analytics(tr_dataset, cluster_analytics)
Get Cluster Analytics

This route allows you to view the cluster analytics for a dataset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**cluster_analytics** | [**ClusterAnalytics**](ClusterAnalytics.md) | JSON request payload to filter the graph | [required] |

### Return type

[**models::ClusterAnalyticsResponse**](ClusterAnalyticsResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ctr_analytics

> models::CtrAnalyticsResponse get_ctr_analytics(tr_dataset, ctr_analytics)
Get CTR Analytics

This route allows you to view the CTR analytics for a dataset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**ctr_analytics** | [**CtrAnalytics**](CtrAnalytics.md) | JSON request payload to filter the graph | [required] |

### Return type

[**models::CtrAnalyticsResponse**](CTRAnalyticsResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_rag_analytics

> models::RagAnalyticsResponse get_rag_analytics(tr_dataset, rag_analytics)
Get RAG Analytics

This route allows you to view the RAG analytics for a dataset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**rag_analytics** | [**RagAnalytics**](RagAnalytics.md) | JSON request payload to filter the graph | [required] |

### Return type

[**models::RagAnalyticsResponse**](RAGAnalyticsResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_recommendation_analytics

> models::RecommendationAnalyticsResponse get_recommendation_analytics(tr_dataset, recommendation_analytics)
Get Recommendation Analytics

This route allows you to view the recommendation analytics for a dataset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**recommendation_analytics** | [**RecommendationAnalytics**](RecommendationAnalytics.md) | JSON request payload to filter the graph | [required] |

### Return type

[**models::RecommendationAnalyticsResponse**](RecommendationAnalyticsResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_search_analytics

> models::SearchAnalyticsResponse get_search_analytics(tr_dataset, search_analytics)
Get Search Analytics

This route allows you to view the search analytics for a dataset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**search_analytics** | [**SearchAnalytics**](SearchAnalytics.md) | JSON request payload to filter the graph | [required] |

### Return type

[**models::SearchAnalyticsResponse**](SearchAnalyticsResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## send_ctr_data

> send_ctr_data(tr_dataset, ctr_data_request_body)
Send CTR Data

This route allows you to send CTR data to the system.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**ctr_data_request_body** | [**CtrDataRequestBody**](CtrDataRequestBody.md) | JSON request payload to send CTR data | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_query_rating

> set_query_rating(tr_dataset, rate_query_request)
Rate Query

This route allows you to Rate a query.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**rate_query_request** | [**RateQueryRequest**](RateQueryRequest.md) | JSON request payload to rate a query | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

