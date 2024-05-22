# \DefaultApi

All URIs are relative to *http://example.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**login_post**](DefaultApi.md#login_post) | **POST** /login | Authenticate user and return JWT
[**logout_post**](DefaultApi.md#logout_post) | **POST** /logout | Logout user
[**root_get**](DefaultApi.md#root_get) | **GET** / | Login/Sign-up UI
[**signup_post**](DefaultApi.md#signup_post) | **POST** /signup | Register a new user
[**verify2fa_post**](DefaultApi.md#verify2fa_post) | **POST** /verify-2fa | Verify 2FA token
[**verify_token_post**](DefaultApi.md#verify_token_post) | **POST** /verify-token | Verify JWT



## login_post

> login_post(login_post_request)
Authenticate user and return JWT

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**login_post_request** | [**LoginPostRequest**](LoginPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## logout_post

> logout_post(jwt)
Logout user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jwt** | **String** | JWT token for authentication | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## root_get

> String root_get()
Login/Sign-up UI

This route serves the login/signup UI

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/html

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## signup_post

> crate::models::SignupPost201Response signup_post(signup_post_request)
Register a new user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**signup_post_request** | [**SignupPostRequest**](SignupPostRequest.md) |  | [required] |

### Return type

[**crate::models::SignupPost201Response**](_signup_post_201_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify2fa_post

> verify2fa_post(verify2fa_post_request)
Verify 2FA token

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verify2fa_post_request** | [**Verify2faPostRequest**](Verify2faPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## verify_token_post

> verify_token_post(verify_token_post_request)
Verify JWT

Verifies if a JWT is valid

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verify_token_post_request** | [**VerifyTokenPostRequest**](VerifyTokenPostRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

