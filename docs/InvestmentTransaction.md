# InvestmentTransaction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**investment_transaction_id** | **String** | The ID of the Investment transaction, unique across all Plaid transactions. Like all Plaid identifiers, the `investment_transaction_id` is case sensitive. | 
**cancel_transaction_id** | Option<**String**> | A legacy field formerly used internally by Plaid to identify certain canceled transactions. | [optional]
**account_id** | **String** | The `account_id` of the account against which this transaction posted. | 
**security_id** | Option<**String**> | The `security_id` to which this transaction is related. | 
**date** | [**String**](string.md) | The [ISO 8601](https://wikipedia.org/wiki/ISO_8601) posting date for the transaction. | 
**name** | **String** | The institution’s description of the transaction. | 
**quantity** | **f32** | The number of units of the security involved in this transaction. | 
**amount** | **f32** | The complete value of the transaction. Positive values when cash is debited, e.g. purchases of stock; negative values when cash is credited, e.g. sales of stock. Treatment remains the same for cash-only movements unassociated with securities. | 
**price** | **f32** | The price of the security at which this transaction occurred. | 
**fees** | Option<**f32**> | The combined value of all fees applied to this transaction | 
**_type** | **String** | Value is one of the following: `buy`: Buying an investment `sell`: Selling an investment `cancel`: A cancellation of a pending transaction `cash`: Activity that modifies a cash position `fee`: A fee on the account `transfer`: Activity which modifies a position, but not through buy/sell activity e.g. options exercise, portfolio transfer  For descriptions of possible transaction types and subtypes, see the [Investment transaction types schema](https://plaid.com/docs/api/accounts/#investment-transaction-types-schema). | 
**subtype** | **String** | For descriptions of possible transaction types and subtypes, see the [Investment transaction types schema](https://plaid.com/docs/api/accounts/#investment-transaction-types-schema). | 
**iso_currency_code** | Option<**String**> | The ISO-4217 currency code of the transaction. Always `null` if `unofficial_currency_code` is non-`null`. | 
**unofficial_currency_code** | Option<**String**> | The unofficial currency code associated with the holding. Always `null` if `iso_currency_code` is non-`null`. Unofficial currency codes are used for currencies that do not have official ISO currency codes, such as cryptocurrencies and the currencies of certain countries.  See the [currency code schema](https://plaid.com/docs/api/accounts#currency-code-schema) for a full listing of supported `iso_currency_code`s. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


