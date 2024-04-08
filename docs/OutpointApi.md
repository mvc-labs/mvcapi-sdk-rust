# \OutpointApi

All URIs are relative to *https://testnet.mvcapi.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**outpoint_txid_index_get**](OutpointApi.md#outpoint_txid_index_get) | **GET** /outpoint/{txid}/{index} | Get tx output(outpoint for vin) spent status.



## outpoint_txid_index_get

> crate::models::OutputInfo outpoint_txid_index_get(txid, index)
Get tx output(outpoint for vin) spent status.

Get detailed info for a utxo(or txo if spent), Only outputs spent no longer than one month are available. (Premium feature will support full history)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**txid** | **String** | The txid of the output | [required] |
**index** | **i32** | The index of the output in the tx. | [required] |

### Return type

[**crate::models::OutputInfo**](OutputInfo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

