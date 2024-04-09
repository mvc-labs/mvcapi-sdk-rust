# ContractNftSellUtxo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**address** | Option<**String**> | Address string of this utxo | [optional]
**contract_address** | Option<**String**> | Address calculated from contract hash(p2ch). | [optional]
**txid** | Option<**String**> | Txid for this utxo. | [optional]
**tx_index** | Option<**i32**> | Output index for the Utxo. | [optional]
**code_hash** | Option<**String**> | Codehash of this utxo. | [optional]
**genesis** | Option<**String**> | Genesis of this utxo. | [optional]
**token_index** | Option<**i64**> | The index of this NFT. | [optional]
**price** | Option<**i64**> | the price of nft. | [optional]
**satoshi** | Option<**i64**> | Mvc value of the utxo(Irrelavant to token value) | [optional]
**satoshi_string** | Option<**String**> | Mvc value of the utxo(In string format) | [optional]
**height** | Option<**i64**> | The height of this utxo, -1 for unconfirmed utxo. | [optional]
**is_ready** | Option<**bool**> | Is current nft transferred into sell contract, If not ready, the following fields will be null | [optional]
**sensible_id** | Option<**String**> | SensibleId of the token | [optional]
**meta_txid** | Option<**String**> | The metanet tx describing the nft. | [optional]
**meta_output_index** | Option<**i32**> | Symbol of the token. | [optional]
**token_supply** | Option<**i64**> | The total supply of this NFT. | [optional]
**flag** | Option<**String**> | Flag used for paging | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


