# \AddressApi

All URIs are relative to *https://testnet.mvcapi.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**address_address_balance_get**](AddressApi.md#address_address_balance_get) | **GET** /address/{address}/balance | Get address balance by specific address.
[**address_address_tx_get**](AddressApi.md#address_address_tx_get) | **GET** /address/{address}/tx | Get address history by specific address(recent 10 days available).
[**address_address_utxo_get**](AddressApi.md#address_address_utxo_get) | **GET** /address/{address}/utxo | Get address utxos by specific address(100 per page).



## address_address_balance_get

> crate::models::AddressBalance address_address_balance_get(address)
Get address balance by specific address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | the requested address | [required] |

### Return type

[**crate::models::AddressBalance**](AddressBalance.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## address_address_tx_get

> Vec<crate::models::AddressTx> address_address_tx_get(address, limit, flag, confirmed)
Get address history by specific address(recent 10 days available).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | the requested address | [required] |
**limit** | Option<**i64**> | The max items returned in this query(max 100). |  |
**flag** | Option<**String**> | The last id of the last query(Paging flag) |  |
**confirmed** | Option<**bool**> | (default true) fetch confirmed tx, fetch unconfirmed tx if false |  |

### Return type

[**Vec<crate::models::AddressTx>**](AddressTx.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## address_address_utxo_get

> Vec<crate::models::AddressUtxo> address_address_utxo_get(address, flag)
Get address utxos by specific address(100 per page).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | the requested address | [required] |
**flag** | Option<**String**> | The last id of the last query(Paging flag) |  |

### Return type

[**Vec<crate::models::AddressUtxo>**](AddressUtxo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

