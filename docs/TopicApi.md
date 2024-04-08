# \TopicApi

All URIs are relative to *https://api.trieve.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_topic**](TopicApi.md#create_topic) | **POST** /api/topic | Create Topic
[**delete_topic**](TopicApi.md#delete_topic) | **DELETE** /api/topic/{topic_id} | Delete Topic
[**get_all_topics_for_user**](TopicApi.md#get_all_topics_for_user) | **GET** /api/topic/user/{user_id} | Get All Topics for User
[**update_topic**](TopicApi.md#update_topic) | **PUT** /api/topic | Update Topic



## create_topic

> models::Topic create_topic(tr_dataset, create_topic_data)
Create Topic

Create Topic  Create a new chat topic. Topics are attached to a user and act as a coordinator for memory of gen-AI chat sessions. We are considering refactoring this resource of the API soon.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**create_topic_data** | [**CreateTopicData**](CreateTopicData.md) | JSON request payload to create chat topic | [required] |

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

Delete Topic  Delete an existing chat topic. When a topic is deleted, all associated chat messages are also deleted.

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


## get_all_topics_for_user

> Vec<models::Topic> get_all_topics_for_user(user_id, tr_dataset)
Get All Topics for User

Get All Topics for User  Get all topics belonging to a the auth'ed user. Soon, we plan to allow specification of the user for this route and include pagination.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **uuid::Uuid** | The id of the user to get topics for | [required] |
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

> update_topic(tr_dataset, update_topic_data)
Update Topic

Update Topic  Update an existing chat topic. Currently, only the name of the topic can be updated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**update_topic_data** | [**UpdateTopicData**](UpdateTopicData.md) | JSON request payload to update a chat topic | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

