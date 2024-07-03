# \EventsApi

All URIs are relative to *https://api.trieve.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_events**](EventsApi.md#get_events) | **POST** /api/events | Get events for the dataset



## get_events

> models::EventReturn get_events(tr_dataset, get_events_data)
Get events for the dataset

Get events for the dataset specified by the TR-Dataset header.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_dataset** | **String** | The dataset id to use for the request | [required] |
**get_events_data** | [**GetEventsData**](GetEventsData.md) | JSON request payload to get events for a dataset | [required] |

### Return type

[**models::EventReturn**](EventReturn.md)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

