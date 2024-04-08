# XPubTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**xpub** | Option<**String**> | query xpub | [optional]
**txid** | Option<**String**> | Txid for this transaction. | [optional]
**max_receive_index** | Option<**i32**> | Max lookahead receive index when processing this transaction. | [optional]
**max_change_index** | Option<**i32**> | Max lookahead change index when processing this transaction. | [optional]
**income** | Option<**i64**> | Total received satoshis(Including all address) | [optional]
**outcome** | Option<**i64**> | Total spent satoshis(Including all address) | [optional]
**height** | Option<**i64**> | Height for this transaction. -1 for unconfirmed | [optional]
**block_index** | Option<**i32**> | Block index for this transaction, -1 for unconfirmed | [optional]
**block_time** | Option<**i64**> | Block timestamp for this transaction, if unconfirmed, the time is first seen time. | [optional]
**flag** | Option<**String**> | Paging flag, format blockTimestamp_blockIndex | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


