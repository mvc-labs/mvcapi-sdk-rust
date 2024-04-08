# TxInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**index** | Option<**i32**> | Input index of the tx. | [optional]
**utxo_txid** | Option<**String**> | The outpoint utxo txid that this input spent | [optional]
**utxo_index** | Option<**i32**> | The outpoint utxo index that this input spent | [optional]
**address** | Option<**String**> | Parsed address from pubkey(P2PKH input only). | [optional]
**value** | Option<**i64**> | satoshi value of this input. | [optional]
**unlock_script** | Option<**String**> | The hex of the input script. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


