# XpubUtxo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**xpub** | Option<**String**> | xpub of the utxo | [optional]
**address** | Option<**String**> | Address string of this utxo | [optional]
**address_type** | Option<**i32**> | Address type, 0 for receive address, 1 for change address. path is {{addressType}}/{{addressIndex}} | [optional]
**address_index** | Option<**i32**> | Address index. Address path in the xpub is {{addressType}}/{{addressIndex}} | [optional]
**txid** | Option<**String**> | Txid for this utxo. | [optional]
**tx_index** | Option<**i32**> | Output index for the Utxo. | [optional]
**value** | Option<**i64**> | Satoshi value of the utxo. | [optional]
**height** | Option<**i64**> | The height of this utxo, -1 for unconfirmed utxo. | [optional]
**flag** | Option<**i64**> | The paging flag of utxo | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


