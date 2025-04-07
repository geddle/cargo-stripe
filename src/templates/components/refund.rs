//! Stripe Refund API
//!
//! This module provides functionality to create, retrieve, update, and list refunds.

use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::Result;
use crate::types::{Currency, Id, List, Metadata, Timestamp};

/// A Stripe refund object
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Refund {
    /// Unique identifier for the object
    pub id: Id,

    /// String representing the object's type
    pub object: String,

    /// Amount, in the currency's smallest unit
    pub amount: i64,

    /// Balance transaction that describes the impact on your account balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<Id>,

    /// ID of the charge that was refunded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<Id>,

    /// Time at which the object was created
    pub created: Timestamp,

    /// Three-letter ISO currency code
    pub currency: Currency,

    /// An arbitrary string attached to the object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Set of key-value pairs attached to the object
    #[serde(default)]
    pub metadata: Metadata,

    /// ID of the PaymentIntent that was refunded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<Id>,

    /// Reason for the refund
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<RefundReason>,

    /// Status of the refund
    pub status: RefundStatus,
}

/// The reason for a refund
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RefundReason {
    /// The customer requested the refund
    RequestedByCustomer,

    /// The refund is due to a duplicate charge
    Duplicate,

    /// The refund is due to fraudulent activity
    Fraudulent,

    /// Other reason for the refund
    #[serde(other)]
    Other,
}

/// The status of a refund
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RefundStatus {
    /// The refund is pending
    Pending,

    /// The refund has been successfully processed
    Succeeded,

    /// The refund was declined by the payment processor
    Failed,

    /// The refund has been canceled
    Canceled,

    /// Other refund status
    #[serde(other)]
    Other,
}

/// Parameters for creating a new refund
#[derive(Debug, Serialize, Default, Clone)]
pub struct CreateRefund {
    /// ID of the charge to refund
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<String>,

    /// ID of the PaymentIntent to refund
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<String>,

    /// A positive integer representing how much to refund
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// An arbitrary string attached to the object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Set of key-value pairs attached to the object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// String indicating the reason for the refund
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<RefundReason>,

    /// Boolean indicating whether the application fee should be refunded
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_application_fee: Option<bool>,

    /// Boolean indicating whether the transfer should be reversed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reverse_transfer: Option<bool>,
}

/// Refund API implementation
impl Client {
    /// Create a new refund
    pub async fn create_refund(&self, params: &CreateRefund) -> Result<Refund> {
        let url = format!("{}/refunds", self.base_url());
        let response = self.http_client().post(&url).json(params).send().await?;

        let status = response.status();
        if !status.is_success() {
            let error: crate::error::ApiError = response.json().await?;
            return Err(error.into());
        }

        let refund: Refund = response.json().await?;
        Ok(refund)
    }

    /// Retrieve a refund by ID
    pub async fn get_refund(&self, refund_id: &str) -> Result<Refund> {
        let url = format!("{}/refunds/{}", self.base_url(), refund_id);
        let response = self.http_client().get(&url).send().await?;

        let status = response.status();
        if !status.is_success() {
            let error: crate::error::ApiError = response.json().await?;
            return Err(error.into());
        }

        let refund: Refund = response.json().await?;
        Ok(refund)
    }

    /// Update a refund by ID
    pub async fn update_refund(&self, refund_id: &str, metadata: &Metadata) -> Result<Refund> {
        let url = format!("{}/refunds/{}", self.base_url(), refund_id);
        let response = self
            .http_client()
            .post(&url)
            .json(&serde_json::json!({ "metadata": metadata }))
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error: crate::error::ApiError = response.json().await?;
            return Err(error.into());
        }

        let refund: Refund = response.json().await?;
        Ok(refund)
    }

    /// List all refunds
    pub async fn list_refunds(
        &self,
        limit: Option<u32>,
        charge: Option<&str>,
    ) -> Result<List<Refund>> {
        let mut url = format!("{}/refunds", self.base_url());

        let mut has_param = false;

        if let Some(limit) = limit {
            url = format!("{}?limit={}", url, limit);
            has_param = true;
        }

        if let Some(charge) = charge {
            let prefix = if has_param { "&" } else { "?" };
            url = format!("{}{}charge={}", url, prefix, charge);
        }

        let response = self.http_client().get(&url).send().await?;

        let status = response.status();
        if !status.is_success() {
            let error: crate::error::ApiError = response.json().await?;
            return Err(error.into());
        }

        let refunds: List<Refund> = response.json().await?;
        Ok(refunds)
    }
}
