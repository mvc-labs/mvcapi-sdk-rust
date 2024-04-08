# \XpubApi

All URIs are relative to *https://testnet.mvcapi.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**xpub_lite_xpub_address_address_get**](XpubApi.md#xpub_lite_xpub_address_address_get) | **GET** /xpubLite/{xpub}/address/{address} | Get xpub address type and index. Only index under /0/70 /1/30 is valid. Otherwise not found.
[**xpub_lite_xpub_balance_get**](XpubApi.md#xpub_lite_xpub_balance_get) | **GET** /xpubLite/{xpub}/balance | Get xpub balances including confirmed and unconfirmed.
[**xpub_lite_xpub_utxo_get**](XpubApi.md#xpub_lite_xpub_utxo_get) | **GET** /xpubLite/{xpub}/utxo | Get xpub utxos by specific xpub(300 per page).



## xpub_lite_xpub_address_address_get

> crate::models::XpubAddress xpub_lite_xpub_address_address_get(xpub, address)
Get xpub address type and index. Only index under /0/70 /1/30 is valid. Otherwise not found.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**xpub** | **String** | the requested xpub | [required] |
**address** | **String** | the requested address | [required] |

### Return type

[**crate::models::XpubAddress**](XpubAddress.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## xpub_lite_xpub_balance_get

> crate::models::XpubLiteBalance xpub_lite_xpub_balance_get(xpub)
Get xpub balances including confirmed and unconfirmed.

This api returns confirmed balance(same as address balance), not sumed utxos.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**xpub** | **String** | the xpub to search | [required] |

### Return type

[**crate::models::XpubLiteBalance**](XpubLiteBalance.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## xpub_lite_xpub_utxo_get

> Vec<crate::models::XpubUtxo> xpub_lite_xpub_utxo_get(xpub, limit)
Get xpub utxos by specific xpub(300 per page).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**xpub** | **String** | the requested xpub | [required] |
**limit** | Option<**i32**> | The max items returned in this query(default 300), not bigger than 5000. |  |

### Return type

[**Vec<crate::models::XpubUtxo>**](XpubUtxo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

