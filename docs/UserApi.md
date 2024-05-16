# \UserApi

All URIs are relative to *https://api.trieve.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_user_api_key**](UserApi.md#delete_user_api_key) | **DELETE** /api/user/api_key/{api_key_id} | Delete User Api Key
[**set_user_api_key**](UserApi.md#set_user_api_key) | **POST** /api/user/api_key | Set User Api Key
[**update_user**](UserApi.md#update_user) | **PUT** /api/user | Update User



## delete_user_api_key

> Vec<models::ApiKeyDto> delete_user_api_key(api_key_id)
Delete User Api Key

Delete User Api Key  Delete an api key for the auth'ed user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api_key_id** | **uuid::Uuid** | The id of the api key to delete | [required] |

### Return type

[**Vec<models::ApiKeyDto>**](ApiKeyDTO.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_user_api_key

> models::SetUserApiKeyResponse set_user_api_key(set_user_api_key_request)
Set User Api Key

Set User Api Key  Create a new api key for the auth'ed user. Successful response will contain the newly created api key. If a write role is assigned the api key will have permission level of the auth'ed user who calls this endpoint.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**set_user_api_key_request** | [**SetUserApiKeyRequest**](SetUserApiKeyRequest.md) | JSON request payload to create a new user api key | [required] |

### Return type

[**models::SetUserApiKeyResponse**](SetUserApiKeyResponse.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_user

> update_user(update_user_org_role_data)
Update User

Update User  Update a user's information. If the user_id is not provided, the auth'ed user will be updated. If the user_id is provided, the auth'ed user must be an admin (1) or owner (2) of the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**update_user_org_role_data** | [**UpdateUserOrgRoleData**](UpdateUserOrgRoleData.md) | JSON request payload to update user information for the auth'ed user | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

