/*
 * The Plaid API
 *
 * The Plaid REST API. Please see https://plaid.com/docs/api for more details.
 *
 * The version of the OpenAPI document: 2020-09-14_1.64.13
 * 
 * Generated by: https://openapi-generator.tech
 */

/// PayPeriodDetails : Details about the pay period.



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PayPeriodDetails {
    /// The amount of the paycheck.
    #[serde(rename = "check_amount", skip_serializing_if = "Option::is_none")]
    pub check_amount: Option<f32>,
    #[serde(rename = "distribution_breakdown", skip_serializing_if = "Option::is_none")]
    pub distribution_breakdown: Option<Vec<crate::models::DistributionBreakdown>>,
    /// The pay period end date, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format: \"yyyy-mm-dd\".
    #[serde(rename = "end_date", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Total earnings before tax/deductions.
    #[serde(rename = "gross_earnings", skip_serializing_if = "Option::is_none")]
    pub gross_earnings: Option<f32>,
    /// The date on which the paystub was issued, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (\"yyyy-mm-dd\").
    #[serde(rename = "pay_date", skip_serializing_if = "Option::is_none")]
    pub pay_date: Option<String>,
    /// The frequency at which an individual is paid.
    #[serde(rename = "pay_frequency", skip_serializing_if = "Option::is_none")]
    pub pay_frequency: Option<PayFrequency>,
    /// The date on which the paystub was issued, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format (\"yyyy-mm-dd\").
    #[serde(rename = "pay_day", skip_serializing_if = "Option::is_none")]
    pub pay_day: Option<String>,
    /// The pay period start date, in [ISO 8601](https://wikipedia.org/wiki/ISO_8601) format: \"yyyy-mm-dd\".
    #[serde(rename = "start_date", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
}

impl PayPeriodDetails {
    /// Details about the pay period.
    pub fn new() -> PayPeriodDetails {
        PayPeriodDetails {
            check_amount: None,
            distribution_breakdown: None,
            end_date: None,
            gross_earnings: None,
            pay_date: None,
            pay_frequency: None,
            pay_day: None,
            start_date: None,
        }
    }
}

/// The frequency at which an individual is paid.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PayFrequency {
    #[serde(rename = "PAY_FREQUENCY_UNKNOWN")]
    PAYFREQUENCYUNKNOWN,
    #[serde(rename = "PAY_FREQUENCY_WEEKLY")]
    PAYFREQUENCYWEEKLY,
    #[serde(rename = "PAY_FREQUENCY_BIWEEKLY")]
    PAYFREQUENCYBIWEEKLY,
    #[serde(rename = "PAY_FREQUENCY_SEMIMONTHLY")]
    PAYFREQUENCYSEMIMONTHLY,
    #[serde(rename = "PAY_FREQUENCY_MONTHLY")]
    PAYFREQUENCYMONTHLY,
    #[serde(rename = "null")]
    Null,
}

impl Default for PayFrequency {
    fn default() -> PayFrequency {
        Self::PAYFREQUENCYUNKNOWN
    }
}
