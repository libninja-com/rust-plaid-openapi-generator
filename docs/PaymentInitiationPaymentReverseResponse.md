# PaymentInitiationPaymentReverseResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**refund_id** | **String** | A unique ID identifying the refund | 
**status** | **String** | The status of the refund.  `PROCESSING`: The refund is currently being processed. The refund will automatically exit this state when processing is complete.  `INITIATED`: The refund has been successfully initiated.  `EXECUTED`: Indicates that the refund has been successfully executed.  `FAILED`: The refund has failed to be executed. This error is retryable once the root cause is resolved. | 
**request_id** | **String** | A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


