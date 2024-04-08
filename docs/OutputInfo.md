# OutputInfo

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**txid** | Option<**String**> | txid that this output is in. | [optional]
**index** | Option<**i32**> | index of this output in the tx. | [optional]
**address** | Option<**String**> | parsed address of this output, empty for non standard. | [optional]
**value** | Option<**i64**> | value of this output | [optional]
**spent** | Option<**bool**> | this output is spent or not, true if spent | [optional]
**spent_txid** | Option<**String**> | txid that spent this output | [optional]
**spent_index** | Option<**i32**> | vin index of the spent tx | [optional]
**spent_height** | Option<**i64**> | height of the spent tx(-1 if unconfirmed, 0 if unspent) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


