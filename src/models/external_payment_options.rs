/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// ExternalPaymentOptions : Additional payment options



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct ExternalPaymentOptions {
    /// When `true`, Plaid will attempt to request refund details from the payee's financial institution.  Support varies between financial institutions and will not always be available.  If refund details could be retrieved, they will be available in the `/payment_initiation/payment/get` response.
    #[serde(rename = "request_refund_details", skip_serializing_if = "Option::is_none")]
    pub request_refund_details: Option<bool>,
    /// The International Bank Account Number (IBAN) for the payer's account. If provided, the end user will be able to send payments only from the specified bank account.
    #[serde(rename = "iban", skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(rename = "bacs", skip_serializing_if = "Option::is_none")]
    pub bacs: Option<Box<crate::models::PaymentInitiationOptionalRestrictionBacs>>,
    /// The EMI (E-Money Institution) wallet that this payment is associated with, if any. This wallet is used as an intermediary account to enable Plaid to reconcile the settlement of funds for Payment Initiation requests.
    #[serde(rename = "wallet_id", skip_serializing_if = "Option::is_none")]
    pub wallet_id: Option<String>,
    #[serde(rename = "scheme", skip_serializing_if = "Option::is_none")]
    pub scheme: Option<crate::models::PaymentScheme>,
}

impl ExternalPaymentOptions {
    /// Additional payment options
    pub fn new() -> ExternalPaymentOptions {
        ExternalPaymentOptions {
            request_refund_details: None,
            iban: None,
            bacs: None,
            wallet_id: None,
            scheme: None,
        }
    }
}

