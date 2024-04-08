# \MerchantApi

All URIs are relative to *https://testnet.mvcapi.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**merchant_utxo_post**](MerchantApi.md#merchant_utxo_post) | **POST** /merchant/utxo | Pick utxos by addresses and amount.



## merchant_utxo_post

> Vec<crate::models::AddressUtxo> merchant_utxo_post(utxo_pick_request)
Pick utxos by addresses and amount.

Selects a set of Utxos with total value higher than the given amount from a given address list . In case of HD wallets, multiple addresses can be specified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**utxo_pick_request** | Option<[**UtxoPickRequest**](UtxoPickRequest.md)> |  |  |

### Return type

[**Vec<crate::models::AddressUtxo>**](AddressUtxo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

