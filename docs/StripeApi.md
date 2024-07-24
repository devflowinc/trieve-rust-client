# \StripeApi

All URIs are relative to *https://api.trieve.ai*

Method | HTTP request | Description
------------- | ------------- | -------------
[**cancel_subscription**](StripeApi.md#cancel_subscription) | **DELETE** /api/stripe/subscription/{subscription_id} | Cancel Subscription
[**create_setup_checkout_session**](StripeApi.md#create_setup_checkout_session) | **POST** /api/stripe/checkout/setup/{organization_id} | Create checkout session setup
[**direct_to_payment_link**](StripeApi.md#direct_to_payment_link) | **GET** /api/stripe/payment_link/{plan_id}/{organization_id} | Checkout
[**get_all_invoices**](StripeApi.md#get_all_invoices) | **GET** /api/stripe/invoices/{organization_id} | Get All Invoices
[**get_all_plans**](StripeApi.md#get_all_plans) | **GET** /api/stripe/plans | Get All Plans
[**update_subscription_plan**](StripeApi.md#update_subscription_plan) | **PATCH** /api/stripe/subscription_plan/{subscription_id}/{plan_id} | Update Subscription Plan



## cancel_subscription

> cancel_subscription(tr_organization, subscription_id)
Cancel Subscription

Cancel a subscription by its id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_organization** | **String** | The organization id to use for the request | [required] |
**subscription_id** | **uuid::Uuid** | id of the subscription you want to cancel | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## create_setup_checkout_session

> models::CreateSetupCheckoutSessionResPayload create_setup_checkout_session(organization_id)
Create checkout session setup

Create a checkout session (setup)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | The id of the organization to create setup checkout session for. | [required] |

### Return type

[**models::CreateSetupCheckoutSessionResPayload**](CreateSetupCheckoutSessionResPayload.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## direct_to_payment_link

> direct_to_payment_link(plan_id, organization_id)
Checkout

Get a 303 SeeOther redirect link to the stripe checkout page for the plan and organization

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**plan_id** | **uuid::Uuid** | id of the plan you want to subscribe to | [required] |
**organization_id** | **uuid::Uuid** | id of the organization you want to subscribe to the plan | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_invoices

> Vec<models::StripeInvoice> get_all_invoices(organization_id)
Get All Invoices

Get a list of all invoices

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**organization_id** | **uuid::Uuid** | The id of the organization to get invoices for. | [required] |

### Return type

[**Vec<models::StripeInvoice>**](StripeInvoice.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_all_plans

> Vec<models::StripePlan> get_all_plans()
Get All Plans

Get a list of all plans

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::StripePlan>**](StripePlan.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_subscription_plan

> update_subscription_plan(tr_organization, subscription_id, plan_id)
Update Subscription Plan

Update a subscription to a new plan

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tr_organization** | **String** | The organization id to use for the request | [required] |
**subscription_id** | **uuid::Uuid** | id of the subscription you want to update | [required] |
**plan_id** | **uuid::Uuid** | id of the plan you want to subscribe to | [required] |

### Return type

 (empty response body)

### Authorization

[ApiKey](../README.md#ApiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

