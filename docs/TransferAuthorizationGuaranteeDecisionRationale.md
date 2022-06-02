# TransferAuthorizationGuaranteeDecisionRationale

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **String** | A code representing the reason Plaid declined to guarantee this transfer:  `RETURN_BANK`: The risk of a bank-initiated return (for example, an R01/NSF) is too high to guarantee this transfer.  `RETURN_CUSTOMER`: The risk of a customer-initiated return (for example, a R10/Unauthorized) is too high to guarantee this transfer.  `GUARANTEE_LIMIT_REACHED`: This transfer is low-risk, but Guaranteed ACH has exhausted an internal limit on the number or rate of guarantees that applies to this transfer.  `RISK_ESTIMATE_UNAVAILABLE`: A risk estimate is unavailable for this Item. | 
**description** | **String** | A human-readable description of why the transfer cannot be guaranteed. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


