# \ContractApi

All URIs are relative to *https://testnet.mvcapi.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**contract_ft_address_address_balance_confirmed_get**](ContractApi.md#contract_ft_address_address_balance_confirmed_get) | **GET** /contract/ft/address/{address}/balance/confirmed | Get all contract token balances for specific address ignoring all unconfirmed txs.
[**contract_ft_address_address_balance_get**](ContractApi.md#contract_ft_address_address_balance_get) | **GET** /contract/ft/address/{address}/balance | Get all contract token balances for specific address.
[**contract_ft_address_address_code_hash_genesis_tx_get**](ContractApi.md#contract_ft_address_address_code_hash_genesis_tx_get) | **GET** /contract/ft/address/{address}/{codeHash}/{genesis}/tx | Get all contract token balances for specific address.
[**contract_ft_address_address_utxo_get**](ContractApi.md#contract_ft_address_address_utxo_get) | **GET** /contract/ft/address/{address}/utxo | Get all contract token utxos for specific address.
[**contract_ft_genesis_code_hash_genesis_circulation_get**](ContractApi.md#contract_ft_genesis_code_hash_genesis_circulation_get) | **GET** /contract/ft/genesis/{codeHash}/{genesis}/circulation | Get all sum of circulation ft token utxos by codeHash and genesisId(10min cached).
[**contract_nft_address_address_count_confirmed_get**](ContractApi.md#contract_nft_address_address_count_confirmed_get) | **GET** /contract/nft/address/{address}/count/confirmed | Get confirmed utxo count for specific nft(ignore all unconfirmed txs).
[**contract_nft_address_address_summary_get**](ContractApi.md#contract_nft_address_address_summary_get) | **GET** /contract/nft/address/{address}/summary | Get nft summary(NFT count group by genesis) for address.
[**contract_nft_address_address_utxo_get**](ContractApi.md#contract_nft_address_address_utxo_get) | **GET** /contract/nft/address/{address}/utxo | Get all contract nft token utxos for specific address.
[**contract_nft_genesis_code_hash_genesis_summary_get**](ContractApi.md#contract_nft_genesis_code_hash_genesis_summary_get) | **GET** /contract/nft/genesis/{codeHash}/{genesis}/summary | Get nft summary(count group by address) for specific codeHash and genesisId(result cached for 60s).
[**contract_nft_genesis_code_hash_genesis_utxo_get**](ContractApi.md#contract_nft_genesis_code_hash_genesis_utxo_get) | **GET** /contract/nft/genesis/{codeHash}/{genesis}/utxo | Get all contract nft token utxos by codeHash and genesisId.
[**contract_nft_sell_address_address_utxo_get**](ContractApi.md#contract_nft_sell_address_address_utxo_get) | **GET** /contract/nft/sell/address/{address}/utxo | Get all contract sell sell utxos for specific address.
[**contract_nft_sell_genesis_code_hash_genesis_utxo_get**](ContractApi.md#contract_nft_sell_genesis_code_hash_genesis_utxo_get) | **GET** /contract/nft/sell/genesis/{codeHash}/{genesis}/utxo | Get all contract nft token utxos by codeHash and genesisId.
[**contract_unique_genesis_code_hash_genesis_utxo_get**](ContractApi.md#contract_unique_genesis_code_hash_genesis_utxo_get) | **GET** /contract/unique/genesis/{codeHash}/{genesis}/utxo | Get contract unique utxos by codeHash and genesisId.



## contract_ft_address_address_balance_confirmed_get

> i64 contract_ft_address_address_balance_confirmed_get(address, code_hash, genesis)
Get all contract token balances for specific address ignoring all unconfirmed txs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | the requested address | [required] |
**code_hash** | **String** | Filter by contract code hash | [required] |
**genesis** | **String** | Filter by contract genesis | [required] |

### Return type

**i64**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contract_ft_address_address_balance_get

> Vec<crate::models::ContractFtBalance> contract_ft_address_address_balance_get(address, code_hash, genesis)
Get all contract token balances for specific address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | the requested address | [required] |
**code_hash** | Option<**String**> | Filter by contract code hash |  |
**genesis** | Option<**String**> | Filter by contract genesis |  |

### Return type

[**Vec<crate::models::ContractFtBalance>**](ContractFtBalance.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contract_ft_address_address_code_hash_genesis_tx_get

> Vec<crate::models::ContractFtAddressTx> contract_ft_address_address_code_hash_genesis_tx_get(address, code_hash, genesis, limit, flag)
Get all contract token balances for specific address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | the requested address | [required] |
**code_hash** | **String** | Filter by contract code hash | [required] |
**genesis** | **String** | Filter by contract genesis | [required] |
**limit** | Option<**i64**> | The max items returned in this query(max 100). |  |
**flag** | Option<**String**> | The last id of the last query(Paging flag) |  |

### Return type

[**Vec<crate::models::ContractFtAddressTx>**](ContractFtAddressTx.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contract_ft_address_address_utxo_get

> Vec<crate::models::ContractFtUtxo> contract_ft_address_address_utxo_get(address, code_hash, genesis, flag)
Get all contract token utxos for specific address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | the requested address | [required] |
**code_hash** | Option<**String**> | Filter by contract code hash |  |
**genesis** | Option<**String**> | Filter by contract genesis |  |
**flag** | Option<**String**> | The flag of the last query Item(Paging flag) |  |

### Return type

[**Vec<crate::models::ContractFtUtxo>**](ContractFtUtxo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contract_ft_genesis_code_hash_genesis_circulation_get

> crate::models::ContractFtGenesisCirculation contract_ft_genesis_code_hash_genesis_circulation_get(code_hash, genesis)
Get all sum of circulation ft token utxos by codeHash and genesisId(10min cached).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code_hash** | **String** | Code hash of the token. | [required] |
**genesis** | **String** | Contract genesis | [required] |

### Return type

[**crate::models::ContractFtGenesisCirculation**](ContractFtGenesisCirculation.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contract_nft_address_address_count_confirmed_get

> i32 contract_nft_address_address_count_confirmed_get(address, code_hash, genesis)
Get confirmed utxo count for specific nft(ignore all unconfirmed txs).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | the requested address | [required] |
**code_hash** | **String** | Filter by contract code hash | [required] |
**genesis** | **String** | Filter by contract genesis | [required] |

### Return type

**i32**

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contract_nft_address_address_summary_get

> Vec<crate::models::ContractNftAddressSummary> contract_nft_address_address_summary_get(address)
Get nft summary(NFT count group by genesis) for address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | the requested address | [required] |

### Return type

[**Vec<crate::models::ContractNftAddressSummary>**](ContractNftAddressSummary.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contract_nft_address_address_utxo_get

> Vec<crate::models::ContractNftUtxo> contract_nft_address_address_utxo_get(address, code_hash, genesis, limit, flag)
Get all contract nft token utxos for specific address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | the requested address | [required] |
**code_hash** | Option<**String**> | Filter by contract code hash |  |
**genesis** | Option<**String**> | Filter by contract genesis |  |
**limit** | Option<**i32**> | Limit the return count(no more than 300) |  |
**flag** | Option<**String**> | The flag of the last query Item(Paging flag) |  |

### Return type

[**Vec<crate::models::ContractNftUtxo>**](ContractNftUtxo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contract_nft_genesis_code_hash_genesis_summary_get

> Vec<crate::models::ContractNftGenesisSummary> contract_nft_genesis_code_hash_genesis_summary_get(code_hash, genesis)
Get nft summary(count group by address) for specific codeHash and genesisId(result cached for 60s).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code_hash** | **String** | Code hash of the token. | [required] |
**genesis** | **String** | Contract genesis | [required] |

### Return type

[**Vec<crate::models::ContractNftGenesisSummary>**](ContractNftGenesisSummary.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contract_nft_genesis_code_hash_genesis_utxo_get

> Vec<crate::models::ContractNftUtxo> contract_nft_genesis_code_hash_genesis_utxo_get(code_hash, genesis, token_index, max, min)
Get all contract nft token utxos by codeHash and genesisId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code_hash** | **String** | Code hash of the token. | [required] |
**genesis** | **String** | Contract genesis | [required] |
**token_index** | Option<**i64**> | Find exact token Index. |  |
**max** | Option<**i64**> | Token index not bigger than this(include this). |  |
**min** | Option<**i64**> | Token index not less than this(include this). |  |

### Return type

[**Vec<crate::models::ContractNftUtxo>**](ContractNftUtxo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contract_nft_sell_address_address_utxo_get

> Vec<crate::models::ContractNftSellUtxo> contract_nft_sell_address_address_utxo_get(address, code_hash, genesis, limit, flag)
Get all contract sell sell utxos for specific address.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | Owner address. | [required] |
**code_hash** | Option<**String**> | Filter by contract code hash |  |
**genesis** | Option<**String**> | Filter by contract genesis |  |
**limit** | Option<**i32**> | Limit the return count(no more than 300) |  |
**flag** | Option<**String**> | The flag of the last query Item(Paging flag) |  |

### Return type

[**Vec<crate::models::ContractNftSellUtxo>**](ContractNftSellUtxo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contract_nft_sell_genesis_code_hash_genesis_utxo_get

> Vec<crate::models::ContractNftSellUtxo> contract_nft_sell_genesis_code_hash_genesis_utxo_get(code_hash, genesis, token_index, max, min)
Get all contract nft token utxos by codeHash and genesisId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code_hash** | **String** | Code hash of the token. | [required] |
**genesis** | **String** | Contract genesis | [required] |
**token_index** | Option<**i64**> | Find exact token Index. |  |
**max** | Option<**i64**> | Token index not bigger than this(include this). |  |
**min** | Option<**i64**> | Token index not less than this(include this). |  |

### Return type

[**Vec<crate::models::ContractNftSellUtxo>**](ContractNftSellUtxo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## contract_unique_genesis_code_hash_genesis_utxo_get

> Vec<crate::models::ContractUniqueUtxo> contract_unique_genesis_code_hash_genesis_utxo_get(code_hash, genesis)
Get contract unique utxos by codeHash and genesisId.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**code_hash** | **String** | Code hash of the token. | [required] |
**genesis** | **String** | Contract genesis | [required] |

### Return type

[**Vec<crate::models::ContractUniqueUtxo>**](ContractUniqueUtxo.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

