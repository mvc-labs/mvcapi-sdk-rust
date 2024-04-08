# BlockHeaderIndex

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**block_hash** | Option<**String**> | Block hash. | [optional]
**height** | Option<**i64**> | Block height. | [optional]
**version** | Option<**i64**> | Block version. | [optional]
**prev_block_hash** | Option<**String**> | The block hash of the previous block. | [optional]
**merkle_root** | Option<**String**> | Hex formatted block merkle root. | [optional]
**timestamp** | Option<**i64**> | Block timestamp. | [optional]
**median_time** | Option<**i64**> | Block median timestamp. | [optional]
**reward** | Option<**i64**> | Miner total rewards, including miner fee. | [optional]
**miner** | Option<**String**> | Guessed miner name. | [optional]
**miner_address** | Option<**String**> | Miner address that received rewards. | [optional]
**tx_count** | Option<**i32**> | Total txs count included in the block. | [optional]
**input_count** | Option<**i32**> | Total input count in the block. | [optional]
**output_count** | Option<**i32**> | Total output count in the block | [optional]
**size** | Option<**i64**> | Size of the block | [optional]
**bits** | Option<**i64**> | Target bits. | [optional]
**nonce** | Option<**i64**> | Block nonce | [optional]
**coinbase** | Option<**String**> | Hex formated coinbase info. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


