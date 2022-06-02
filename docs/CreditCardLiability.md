# CreditCardLiability

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**account_id** | Option<**String**> | The ID of the account that this liability belongs to. | 
**aprs** | [**Vec<crate::models::Apr>**](APR.md) | The various interest rates that apply to the account. | 
**is_overdue** | Option<**bool**> | true if a payment is currently overdue. Availability for this field is limited. | 
**last_payment_amount** | **f32** | The amount of the last payment. | 
**last_payment_date** | Option<[**String**](string.md)> | The date of the last payment. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). Availability for this field is limited. | 
**last_statement_issue_date** | [**String**](string.md) | The date of the last statement. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). | 
**last_statement_balance** | **f32** | The total amount owed as of the last statement issued | 
**minimum_payment_amount** | **f32** | The minimum payment due for the next billing cycle. | 
**next_payment_due_date** | Option<[**String**](string.md)> | The due date for the next payment. The due date is `null` if a payment is not expected. Dates are returned in an [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (YYYY-MM-DD). | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


