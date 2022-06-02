/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PaymentInitiationMetadata : Metadata that captures what specific payment configurations an institution supports when making Payment Initiation requests.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PaymentInitiationMetadata {
    /// Indicates whether the institution supports payments from a different country.
    #[serde(rename = "supports_international_payments")]
    pub supports_international_payments: bool,
    /// A mapping of currency to maximum payment amount (denominated in the smallest unit of currency) supported by the institution.  Example: `{\"GBP\": \"10000\"}` 
    #[serde(rename = "maximum_payment_amount")]
    pub maximum_payment_amount: ::std::collections::HashMap<String, String>,
    /// Indicates whether the institution supports returning refund details when initiating a payment.
    #[serde(rename = "supports_refund_details")]
    pub supports_refund_details: bool,
    #[serde(rename = "standing_order_metadata")]
    pub standing_order_metadata: Option<crate::models::PaymentInitiationStandingOrderMetadata>,
}

impl PaymentInitiationMetadata {
    /// Metadata that captures what specific payment configurations an institution supports when making Payment Initiation requests.
    pub fn new(supports_international_payments: bool, maximum_payment_amount: ::std::collections::HashMap<String, String>, supports_refund_details: bool, standing_order_metadata: Option<crate::models::PaymentInitiationStandingOrderMetadata>) -> PaymentInitiationMetadata {
        PaymentInitiationMetadata {
            supports_international_payments,
            maximum_payment_amount,
            supports_refund_details,
            standing_order_metadata,
        }
    }
}

