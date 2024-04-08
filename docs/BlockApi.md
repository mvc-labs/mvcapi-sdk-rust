# \BlockApi

All URIs are relative to *https://testnet.mvcapi.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**block_block_id_get**](BlockApi.md#block_block_id_get) | **GET** /block/{blockId} | Get block request by height or hash
[**block_get**](BlockApi.md#block_get) | **GET** /block | Get recent block list by paging. 30 items per page.
[**block_info_get**](BlockApi.md#block_info_get) | **GET** /block/info | Get current blockchain info from full node.



## block_block_id_get

> crate::models::BlockHeaderIndex block_block_id_get(block_id)
Get block request by height or hash

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**block_id** | **String** | The block id, height or hash acceptable. | [required] |

### Return type

[**crate::models::BlockHeaderIndex**](BlockHeaderIndex.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## block_get

> Vec<crate::models::BlockHeaderPage> block_get(last)
Get recent block list by paging. 30 items per page.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**last** | Option<**i64**> | paging flag, height of last item in last page |  |

### Return type

[**Vec<crate::models::BlockHeaderPage>**](BlockHeaderPage.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## block_info_get

> crate::models::BlockchainInfo block_info_get()
Get current blockchain info from full node.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::BlockchainInfo**](BlockchainInfo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

