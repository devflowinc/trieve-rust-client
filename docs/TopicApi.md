# \TopicApi

All URIs are relative to *https://api.trieve.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_topic**](TopicApi.md#create_topic) | **POST** /api/topic | Create Topic
[**delete_topic**](TopicApi.md#delete_topic) | **DELETE** /api/topic/{topic_id} | Delete Topic
[**get_all_topics_for_owner_id**](TopicApi.md#get_all_topics_for_owner_id) | **GET** /api/topic/owner/{owner_id} | Get All Topics for Owner ID
[**update_topic**](TopicApi.md#update_topic) | **PUT** /api/topic | Update Topic



## create_topic

> models::Topic create_topic(tr_dataset, create_topic_req_payload)
Create Topic

Create a new chat topic. Topics are attached to a owner_id's and act as a coordinator for conversation message history of gen-AI chat sessions. Auth'ed user or api key must have an admin or owner role for the specified dataset's organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**create_topic_req_payload** | [**CreateTopicReqPayload**](CreateTopicReqPayload.md) | JSON request payload to create chat topic | [required] |

### Return type

[**models::Topic**](Topic.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_topic

> delete_topic(tr_dataset, topic_id)
Delete Topic

Delete an existing chat topic. When a topic is deleted, all associated chat messages are also deleted. Auth'ed user or api key must have an admin or owner role for the specified dataset's organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**topic_id** | **uuid::Uuid** | The id of the topic you want to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_topics_for_owner_id

> Vec<models::Topic> get_all_topics_for_owner_id(owner_id, tr_dataset)
Get All Topics for Owner ID

Get all topics belonging to an arbitary owner_id. This is useful for managing message history and chat sessions. It is common to use a browser fingerprint or your user's id as the owner_id. Auth'ed user or api key must have an admin or owner role for the specified dataset's organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**owner_id** | **String** | The owner_id to get topics of; A common approach is to use a browser fingerprint or your user's id | [required] |
**tr_dataset** | **String** | The dataset id to use for the request | [required] |

### Return type

[**Vec<models::Topic>**](Topic.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_topic

> update_topic(tr_dataset, update_topic_req_payload)
Update Topic

Update an existing chat topic. Currently, only the name of the topic can be updated. Auth'ed user or api key must have an admin or owner role for the specified dataset's organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**update_topic_req_payload** | [**UpdateTopicReqPayload**](UpdateTopicReqPayload.md) | JSON request payload to update a chat topic | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

