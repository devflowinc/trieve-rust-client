# \AuthApi

All URIs are relative to *https://api.trieve.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**callback**](AuthApi.md#callback) | **GET** /api/auth/callback | OpenID Connect callback
[**get_me**](AuthApi.md#get_me) | **GET** /api/auth/me | Get Me
[**login**](AuthApi.md#login) | **GET** /api/auth | Login
[**logout**](AuthApi.md#logout) | **DELETE** /api/auth | Logout



## callback

> models::SlimUser callback()
OpenID Connect callback

This is the callback route for the OAuth provider, it should not be called directly. Redirects to browser with set-cookie header.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SlimUser**](SlimUser.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_me

> models::SlimUser get_me()
Get Me

Get the user corresponding to your current auth credentials.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::SlimUser**](SlimUser.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## login

> login(organization_id, redirect_uri, inv_code)
Login

This will redirect you to the OAuth provider for authentication with email/pass, SSO, Google, Github, etc.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | Option<**uuid::Uuid**> | ID of organization to authenticate into |  |
**redirect_uri** | Option<**String**> | URL to redirect to after successful login |  |
**inv_code** | Option<**uuid::Uuid**> | Code sent via email as a result of successful call to send_invitation |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logout

> logout()
Logout

Invalidate your current auth credential stored typically stored in a cookie. This does not invalidate your API key.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

