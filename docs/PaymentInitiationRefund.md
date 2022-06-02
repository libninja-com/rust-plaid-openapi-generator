# PaymentInitiationRefund

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**refund_id** | **String** | The ID of the refund. Like all Plaid identifiers, the `refund_id` is case sensitive. | 
**amount** | [**crate::models::PaymentAmount**](PaymentAmount.md) |  | 
**status** | **String** | The status of the refund.  `PROCESSING`: The refund is currently being processed. The refund will automatically exit this state when processing is complete.  `INITIATED`: The refund has been successfully initiated.  `EXECUTED`: Indicates that the refund has been successfully executed.  `FAILED`: The refund has failed to be executed. This error is retryable once the root cause is resolved. | 
**last_status_update** | **String** | The date and time of the last time the `status` was updated, in IS0 8601 format | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


