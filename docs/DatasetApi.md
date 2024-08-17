# \DatasetApi

All URIs are relative to *https://api.trieve.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**clear_dataset**](DatasetApi.md#clear_dataset) | **PUT** /api/dataset/clear/{dataset_id} | Clear Dataset
[**create_dataset**](DatasetApi.md#create_dataset) | **POST** /api/dataset | Create Dataset
[**delete_dataset**](DatasetApi.md#delete_dataset) | **DELETE** /api/dataset/{dataset_id} | Delete Dataset
[**delete_dataset_by_tracking_id**](DatasetApi.md#delete_dataset_by_tracking_id) | **DELETE** /api/dataset/tracking_id/{tracking_id} | Delete Dataset by Tracking ID
[**get_all_tags**](DatasetApi.md#get_all_tags) | **POST** /api/dataset/get_all_tags | Get All Tags
[**get_dataset**](DatasetApi.md#get_dataset) | **GET** /api/dataset/{dataset_id} | Get Dataset By ID
[**get_datasets_from_organization**](DatasetApi.md#get_datasets_from_organization) | **GET** /api/dataset/organization/{organization_id} | Get Datasets from Organization
[**get_usage_by_dataset_id**](DatasetApi.md#get_usage_by_dataset_id) | **GET** /api/dataset/usage/{dataset_id} | Get Usage By Dataset ID
[**update_dataset**](DatasetApi.md#update_dataset) | **PUT** /api/dataset | Update Dataset by ID or Tracking ID



## clear_dataset

> clear_dataset(tr_dataset, dataset_id)
Clear Dataset

Removes all chunks, files, and groups from the dataset while retaining the analytics and dataset itself. The auth'ed user must be an owner of the organization to clear a dataset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**dataset_id** | **uuid::Uuid** | The id of the dataset you want to clear. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_dataset

> models::Dataset create_dataset(tr_organization, create_dataset_request)
Create Dataset

Auth'ed user must be an owner of the organization to create a dataset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_organization** | **String** | The organization id to use for the request | [required] |
**create_dataset_request** | [**CreateDatasetRequest**](CreateDatasetRequest.md) | JSON request payload to create a new dataset | [required] |

### Return type

[**models::Dataset**](Dataset.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dataset

> delete_dataset(tr_dataset, dataset_id)
Delete Dataset

Auth'ed user must be an owner of the organization to delete a dataset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**dataset_id** | **uuid::Uuid** | The id of the dataset you want to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_dataset_by_tracking_id

> delete_dataset_by_tracking_id(tr_dataset, tracking_id)
Delete Dataset by Tracking ID

Auth'ed user must be an owner of the organization to delete a dataset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**tracking_id** | **String** | The tracking id of the dataset you want to delete. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_tags

> models::GetAllTagsResponse get_all_tags(tr_dataset, get_all_tags_req_payload)
Get All Tags

Scroll through all tags in the dataset and get the number of chunks in the dataset with that tag plus the total number of unique tags for the whole datset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**get_all_tags_req_payload** | [**GetAllTagsReqPayload**](GetAllTagsReqPayload.md) | JSON request payload to get items with the tag in the request | [required] |

### Return type

[**models::GetAllTagsResponse**](GetAllTagsResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dataset

> models::Dataset get_dataset(tr_dataset, dataset_id)
Get Dataset By ID

Auth'ed user or api key must have an admin or owner role for the specified dataset's organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**dataset_id** | **uuid::Uuid** | The id of the dataset you want to retrieve. | [required] |

### Return type

[**models::Dataset**](Dataset.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_datasets_from_organization

> Vec<models::DatasetAndUsage> get_datasets_from_organization(tr_organization, organization_id, limit, offset)
Get Datasets from Organization

Auth'ed user or api key must have an admin or owner role for the specified dataset's organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_organization** | **String** | The organization id to use for the request | [required] |
**organization_id** | **uuid::Uuid** | id of the organization you want to retrieve datasets for | [required] |
**limit** | Option<**i64**> | The number of records to return |  |
**offset** | Option<**i64**> | The number of records to skip |  |

### Return type

[**Vec<models::DatasetAndUsage>**](DatasetAndUsage.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_usage_by_dataset_id

> models::DatasetUsageCount get_usage_by_dataset_id(tr_dataset, dataset_id)
Get Usage By Dataset ID

Auth'ed user or api key must have an admin or owner role for the specified dataset's organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id or tracking_id to use for the request. We assume you intend to use an id if the value is a valid uuid. | [required] |
**dataset_id** | **uuid::Uuid** | The id of the dataset you want to retrieve usage for. | [required] |

### Return type

[**models::DatasetUsageCount**](DatasetUsageCount.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_dataset

> models::Dataset update_dataset(tr_organization, update_dataset_request)
Update Dataset by ID or Tracking ID

One of id or tracking_id must be provided. The auth'ed user must be an owner of the organization to update a dataset.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_organization** | **String** | The organization id to use for the request | [required] |
**update_dataset_request** | [**UpdateDatasetRequest**](UpdateDatasetRequest.md) | JSON request payload to update a dataset | [required] |

### Return type

[**models::Dataset**](Dataset.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

