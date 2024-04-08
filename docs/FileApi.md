# \FileApi

All URIs are relative to *https://api.trieve.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_file_handler**](FileApi.md#delete_file_handler) | **DELETE** /api/file/{file_id} | Delete File
[**get_dataset_files_handler**](FileApi.md#get_dataset_files_handler) | **GET** /api/dataset/files/{dataset_id}/{page} | Get Files for Dataset
[**get_file_handler**](FileApi.md#get_file_handler) | **GET** /api/file/{file_id} | Get File
[**upload_file_handler**](FileApi.md#upload_file_handler) | **POST** /api/file | Upload File



## delete_file_handler

> delete_file_handler(tr_dataset, file_id, delete_chunks)
Delete File

Delete File  Delete a file from S3 attached to the server based on its id. This will disassociate chunks from the file, but only delete them all together if you specify delete_chunks to be true. Auth'ed user must be an admin or owner of the dataset's organization to delete a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**file_id** | **uuid::Uuid** | The id of the file to delete | [required] |
**delete_chunks** | **bool** | Whether or not to delete the chunks associated with the file | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_dataset_files_handler

> Vec<models::File> get_dataset_files_handler(tr_dataset, dataset_id, page)
Get Files for Dataset

Get Files for Dataset  Get all files which belong to a given dataset specified by the dataset_id parameter. 10 files are returned per page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**dataset_id** | **uuid::Uuid** | The id of the dataset to fetch files for. | [required] |
**page** | **i64** | The page number of files you wish to fetch. Each page contains at most 10 files. | [required] |

### Return type

[**Vec<models::File>**](File.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_file_handler

> models::FileDto get_file_handler(tr_dataset, file_id)
Get File

Get File  Download a file based on its id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**file_id** | **uuid::Uuid** | The id of the file to fetch | [required] |

### Return type

[**models::FileDto**](FileDTO.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## upload_file_handler

> models::UploadFileResult upload_file_handler(tr_dataset, upload_file_data)
Upload File

Upload File  Upload a file to S3 attached to the server. The file will be converted to HTML with tika and chunked algorithmically, images will be OCR'ed with tesseract. The resulting chunks will be indexed and searchable. Optionally, you can only upload the file and manually create chunks associated to the file after. See docs.trieve.ai and/or contact us for more details and tips. Auth'ed user must be an admin or owner of the dataset's organization to upload a file.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**upload_file_data** | [**UploadFileData**](UploadFileData.md) | JSON request payload to upload a file | [required] |

### Return type

[**models::UploadFileResult**](UploadFileResult.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

