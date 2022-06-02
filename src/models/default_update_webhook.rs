/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// DefaultUpdateWebhook : Fired when new transaction data is available for an Item. Plaid will typically check for new transaction data several times a day. 



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DefaultUpdateWebhook {
    /// `TRANSACTIONS`
    #[serde(rename = "webhook_type")]
    pub webhook_type: String,
    /// `DEFAULT_UPDATE`
    #[serde(rename = "webhook_code")]
    pub webhook_code: String,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<crate::models::PlaidError>>,
    /// The number of new transactions detected since the last time this webhook was fired.
    #[serde(rename = "new_transactions")]
    pub new_transactions: f32,
    /// The `item_id` of the Item the webhook relates to.
    #[serde(rename = "item_id")]
    pub item_id: String,
}

impl DefaultUpdateWebhook {
    /// Fired when new transaction data is available for an Item. Plaid will typically check for new transaction data several times a day. 
    pub fn new(webhook_type: String, webhook_code: String, new_transactions: f32, item_id: String) -> DefaultUpdateWebhook {
        DefaultUpdateWebhook {
            webhook_type,
            webhook_code,
            error: None,
            new_transactions,
            item_id,
        }
    }
}

