# XpubDetail

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**xpub** | Option<**String**> | String encoded extended pubkey (xpub) | [optional]
**receive_index** | Option<**i32**> | Next unused receive index, path /0/index | [optional]
**max_receive_index** | Option<**i32**> | Max lookahead receive index. | [optional]
**change_index** | Option<**i32**> | Next unused change index, path /1/index | [optional]
**max_change_index** | Option<**i32**> | Max lookahead change index. | [optional]
**mode** | Option<**i32**> | Current xpub process mode, 0 means preparing(not ready), 1 means synchronizing(ready) | [optional]
**skip_height** | Option<**i64**> | Skip blocks before skipHeight while searching transactions. This will speed up sync time. | [optional]
**process_height** | Option<**i64**> | Xpub current processed height. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


