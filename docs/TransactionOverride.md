# TransactionOverride

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**date_transacted** | [**String**](string.md) | The date of the transaction, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format. Transactions in Sandbox will move from pending to posted once their transaction date has been reached. If a `date_transacted` is not provided by the institution, a transaction date may be available in the [`authorized_date`](https://plaid.com/docs/api/products/#transactions-get-response-transactions-authorized-date) field. | 
**date_posted** | [**String**](string.md) | The date the transaction posted, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) (YYYY-MM-DD) format. Posted dates in the past or present will result in posted transactions; posted dates in the future will result in pending transactions. | 
**amount** | **f32** | The transaction amount. Can be negative. | 
**description** | **String** | The transaction description. | 
**currency** | Option<**String**> | The ISO-4217 format currency code for the transaction. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


