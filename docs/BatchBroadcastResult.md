# BatchBroadcastResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**known** | Option<**Vec<String>**> | Already known transactions detected during processing (if there are any) | [optional]
**evicted** | Option<**Vec<String>**> | Transactions accepted by the mempool and then evicted due to insufficient fee (if there are any) | [optional]
**invalid** | Option<[**Vec<crate::models::InvalidBroadCast>**](InvalidBroadCast.md)> | Transactions that failed to be accepted by the mempool (if there are any) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


