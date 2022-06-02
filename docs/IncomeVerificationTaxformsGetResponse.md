# IncomeVerificationTaxformsGetResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**request_id** | Option<**String**> | A unique identifier for the request, which can be used for troubleshooting. This identifier, like all Plaid identifiers, is case sensitive. | [optional]
**document_metadata** | [**Vec<crate::models::DocumentMetadata>**](DocumentMetadata.md) |  | 
**taxforms** | [**Vec<crate::models::Taxform>**](Taxform.md) | A list of forms. | 
**error** | Option<[**crate::models::PlaidError**](PlaidError.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)

