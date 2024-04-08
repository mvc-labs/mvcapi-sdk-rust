# \TreasuryApi

All URIs are relative to *https://testnet.mvcapi.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**treasury_get**](TreasuryApi.md#treasury_get) | **GET** /treasury | Get current treasury info.
[**treasury_history_get**](TreasuryApi.md#treasury_history_get) | **GET** /treasury/history | Get all treasury history.



## treasury_get

> crate::models::TreasuryInfo treasury_get()
Get current treasury info.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::TreasuryInfo**](TreasuryInfo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## treasury_history_get

> Vec<crate::models::TreasuryHistory> treasury_history_get()
Get all treasury history.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::TreasuryHistory>**](TreasuryHistory.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

