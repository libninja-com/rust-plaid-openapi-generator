# TransferAuthorization

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | **String** | Plaid’s unique identifier for a transfer authorization. | 
**created** | **String** | The datetime representing when the authorization was created, in the format `2006-01-02T15:04:05Z`. | 
**decision** | **String** |  A decision regarding the proposed transfer.  `approved` – The proposed transfer has received the end user's consent and has been approved for processing. Plaid has also reviewed the proposed transfer and has approved it for processing.   `permitted` – Plaid was unable to fetch the information required to approve or decline the proposed transfer. You may proceed with the transfer, but further review is recommended. Plaid is awaiting further instructions from the client.  `declined` – Plaid reviewed the proposed transfer and declined processing. Refer to the `code` field in the `decision_rationale` object for details. | 
**decision_rationale** | Option<[**crate::models::TransferAuthorizationDecisionRationale**](TransferAuthorizationDecisionRationale.md)> |  | 
**guarantee_decision** | Option<[**crate::models::TransferAuthorizationGuaranteeDecision**](TransferAuthorizationGuaranteeDecision.md)> |  | 
**guarantee_decision_rationale** | Option<[**crate::models::TransferAuthorizationGuaranteeDecisionRationale**](TransferAuthorizationGuaranteeDecisionRationale.md)> |  | 
**proposed_transfer** | [**crate::models::TransferAuthorizationProposedTransfer**](TransferAuthorizationProposedTransfer.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


