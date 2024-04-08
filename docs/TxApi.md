# \TxApi

All URIs are relative to *https://testnet.mvcapi.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tx_broadcast_batch_post**](TxApi.md#tx_broadcast_batch_post) | **POST** /tx/broadcast/batch | Broadcast a batch of tx to MvcApi fullnode. This endpoint use rpc sendrawtransactions.
[**tx_broadcast_post**](TxApi.md#tx_broadcast_post) | **POST** /tx/broadcast | Broadcast tx to MvcApi fullnode.
[**tx_txid_get**](TxApi.md#tx_txid_get) | **GET** /tx/{txid} | Get transaction detail by specific txid.
[**tx_txid_raw_get**](TxApi.md#tx_txid_raw_get) | **GET** /tx/{txid}/raw | Get transaction raw hex by specific txid.
[**tx_txid_seen_get**](TxApi.md#tx_txid_seen_get) | **GET** /tx/{txid}/seen | Whether MvcApi have seen this tx before. This is a fast approach to know if the tx exist or not.
[**vin_txid_detail_get**](TxApi.md#vin_txid_detail_get) | **GET** /vin/{txid}/detail | Get all output point of vins in the tx with detailed output script.



## tx_broadcast_batch_post

> crate::models::BatchBroadcastResult tx_broadcast_batch_post(tx_raw)
Broadcast a batch of tx to MvcApi fullnode. This endpoint use rpc sendrawtransactions.

This api will broadcast to MvcApi fullnode directly.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tx_raw** | Option<[**Vec<crate::models::TxRaw>**](TxRaw.md)> |  |  |

### Return type

[**crate::models::BatchBroadcastResult**](BatchBroadcastResult.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tx_broadcast_post

> crate::models::BroadcastResult tx_broadcast_post(tx_raw)
Broadcast tx to MvcApi fullnode.

This api will broadcast to MvcApi fullnode directly. This endpoint use rpc sendrawtransaction.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**tx_raw** | Option<[**TxRaw**](TxRaw.md)> |  |  |

### Return type

[**crate::models::BroadcastResult**](BroadcastResult.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tx_txid_get

> crate::models::TxDetail tx_txid_get(txid, show_script)
Get transaction detail by specific txid.

This api is parsed from raw hex, you can use /tx/{txid}/raw to parse tx by yourself, If you want detail input info, use '/vin/{txid}' to get detailed input info including address and value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**txid** | **String** | the request ID to search, txhash | [required] |
**show_script** | Option<**bool**> | Return source script code or not (default false). |  |

### Return type

[**crate::models::TxDetail**](TxDetail.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tx_txid_raw_get

> crate::models::TxRaw tx_txid_raw_get(txid)
Get transaction raw hex by specific txid.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**txid** | **String** | the request ID to search, txhash | [required] |

### Return type

[**crate::models::TxRaw**](TxRaw.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tx_txid_seen_get

> bool tx_txid_seen_get(txid)
Whether MvcApi have seen this tx before. This is a fast approach to know if the tx exist or not.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**txid** | **String** | the request ID to search, txhash | [required] |

### Return type

**bool**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## vin_txid_detail_get

> Vec<crate::models::OutputInfoDetail> vin_txid_detail_get(txid)
Get all output point of vins in the tx with detailed output script.

Search output points by spent txid. Use this api to get detailed inputs for the tx.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**txid** | **String** | The txid of the vins | [required] |

### Return type

[**Vec<crate::models::OutputInfoDetail>**](OutputInfoDetail.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

