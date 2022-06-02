# TransferAuthorizationDecisionRationale

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**code** | **String** | A code representing the rationale for permitting or declining the proposed transfer. Possible values are:  `NSF` – Transaction likely to result in a return due to insufficient funds.  `RISK` - Transaction is high-risk.  `MANUALLY_VERIFIED_ITEM` – Item created via same-day micro deposits, limited information available. Plaid can only offer `permitted` as a transaction decision.  `LOGIN_REQUIRED` – Unable to collect the account information required for an authorization decision due to Item staleness. Can be rectified using Link update mode.  `ERROR` – Unable to collect the account information required for an authorization decision due to an error. | 
**description** | **String** | A human-readable description of the code associated with a permitted transfer or transfer decline. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


