# \OrganizationApi

All URIs are relative to *https://api.trieve.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_organization**](OrganizationApi.md#create_organization) | **POST** /api/organization | Create Organization
[**delete_organization**](OrganizationApi.md#delete_organization) | **DELETE** /api/organization/{organization_id} | Delete Organization
[**get_organization**](OrganizationApi.md#get_organization) | **GET** /api/organization/{organization_id} | Get Organization
[**get_organization_usage**](OrganizationApi.md#get_organization_usage) | **GET** /api/organization/usage/{organization_id} | Get Organization Usage
[**get_organization_users**](OrganizationApi.md#get_organization_users) | **GET** /api/organization/users/{organization_id} | Get Organization Users
[**update_organization**](OrganizationApi.md#update_organization) | **PUT** /api/organization | Update Organization



## create_organization

> models::Organization create_organization(create_organization_data)
Create Organization

Create a new organization. The auth'ed user who creates the organization will be the default owner of the organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_organization_data** | [**CreateOrganizationData**](CreateOrganizationData.md) | The organization data that you want to create | [required] |

### Return type

[**models::Organization**](Organization.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_organization

> delete_organization(tr_organization, organization_id)
Delete Organization

Delete an organization by its id. The auth'ed user must be an owner of the organization to delete it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_organization** | **String** | The organization id to use for the request | [required] |
**organization_id** | **uuid::Uuid** | The id of the organization you want to fetch. | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization

> models::Organization get_organization(tr_organization, organization_id)
Get Organization

Fetch the details of an organization by its id. Auth'ed user or api key must have an admin or owner role for the specified dataset's organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_organization** | **String** | The organization id to use for the request | [required] |
**organization_id** | **uuid::Uuid** | The id of the organization you want to fetch. | [required] |

### Return type

[**models::Organization**](Organization.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_usage

> models::OrganizationUsageCount get_organization_usage(tr_organization, organization_id)
Get Organization Usage

Fetch the current usage specification of an organization by its id. Auth'ed user or api key must have an admin or owner role for the specified dataset's organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_organization** | **String** | The organization id to use for the request | [required] |
**organization_id** | **uuid::Uuid** | The id of the organization you want to fetch the usage of. | [required] |

### Return type

[**models::OrganizationUsageCount**](OrganizationUsageCount.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_organization_users

> Vec<models::SlimUser> get_organization_users(tr_organization, organization_id)
Get Organization Users

Fetch the users of an organization by its id. Auth'ed user or api key must have an admin or owner role for the specified dataset's organization.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_organization** | **String** | The organization id to use for the request | [required] |
**organization_id** | **uuid::Uuid** | The id of the organization you want to fetch the users of. | [required] |

### Return type

[**Vec<models::SlimUser>**](SlimUser.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_organization

> models::Organization update_organization(tr_organization, update_organization_data)
Update Organization

Update an organization. Only the owner of the organization can update it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_organization** | **String** | The organization id to use for the request | [required] |
**update_organization_data** | [**UpdateOrganizationData**](UpdateOrganizationData.md) | The organization data that you want to update | [required] |

### Return type

[**models::Organization**](Organization.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

