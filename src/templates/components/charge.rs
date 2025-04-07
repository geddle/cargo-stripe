//! Stripe Charge API
//!
//! This module provides functionality to create, retrieve, update, and list charges.
//! Charges represent a payment that has been processed by Stripe.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::stripe::client::Client;
use crate::stripe::error::Result;
use crate::stripe::types::{Currency, Id, List, Metadata, Timestamp};

/// A Stripe charge object
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Charge {
    /// Unique identifier for the object
    pub id: Id,
    
    /// String representing the object's type. Objects of the same type share the same value
    pub object: String,
    
    /// Amount charged (in the smallest currency unit)
    pub amount: u64,
    
    /// Amount in cents refunded
    pub amount_refunded: u64,
    
    /// ID of the balance transaction that describes the impact of this charge on your account balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<Id>,
    
    /// Whether the charge has been captured or not
    pub captured: bool,
    
    /// Time at which the object was created
    pub created: Timestamp,
    
    /// Three-letter ISO currency code
    pub currency: Currency,
    
    /// ID of the customer this charge is for if one exists
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<Id>,
    
    /// An arbitrary string attached to the object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// ID of the payment method used in this charge
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<Id>,
    
    /// Whether the charge has been paid
    pub paid: bool,
    
    /// Message to explain a failed charge
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    
    /// Code explaining the reason for a charge failure
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    
    /// True if the charge was created without capturing, false otherwise
    pub livemode: bool,
    
    /// Set of key-value pairs attached to the object
    #[serde(default)]
    pub metadata: Metadata,
    
    /// ID of the invoice this charge is for if one exists
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Id>,
    
    /// Current charge status
    pub status: ChargeStatus,
}

/// The status of a charge
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ChargeStatus {
    /// The payment has been completed successfully
    Succeeded,
    
    /// The payment was not captured
    Pending,
    
    /// The payment failed
    Failed,
}

/// Parameters for creating a new charge
#[derive(Debug, Serialize, Default, Clone)]
pub struct CreateCharge {
    /// Amount to charge (in the smallest currency unit)
    pub amount: u64,
    
    /// Three-letter ISO currency code
    pub currency: Currency,
    
    /// ID of the customer this charge is for if one exists
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    
    /// ID of the payment method to attach to this charge
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    
    /// Whether to immediately capture the charge
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture: Option<bool>,
    
    /// An arbitrary string attached to the object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Set of key-value pairs attached to the object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    
    /// For non-card charges, you can use this value as the complete description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    
    /// An arbitrary string to be displayed on your customer's statement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<String>,
    
    /// The email address to which this charge's receipt will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<String>,
}

/// Parameters for updating a charge
#[derive(Debug, Serialize, Default, Clone)]
pub struct UpdateCharge {
    /// An arbitrary string attached to the object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// Set of key-value pairs attached to the object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
    
    /// The email address to which this charge's receipt will be sent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<String>,
}

/// Parameters for capturing a charge
#[derive(Debug, Serialize, Default, Clone)]
pub struct CaptureCharge {
    /// The amount to capture, which must be less than or equal to the original amount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u64>,
    
    /// An application fee to apply to the payment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<u64>,
    
    /// The email address to send this charge's receipt to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<String>,
    
    /// An arbitrary string to be displayed on your customer's credit card statement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    
    /// Additional information to be displayed on your customer's credit card statement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<String>,
}

/// Charge API implementation
impl Client {
    /// Create a new charge
    pub async fn create_charge(&self, params: &CreateCharge) -> Result<Charge> {
        let url = format!("{}/charges", self.base_url());
        let response = self.http_client()
            .post(&url)
            .json(params)
            .send()
            .await?;
        
        let status = response.status();
        if !status.is_success() {
            let error: crate::stripe::error::ApiError = response.json().await?;
            return Err(error.into());
        }
        
        let charge: Charge = response.json().await?;
        Ok(charge)
    }
    
    /// Retrieve a charge by ID
    pub async fn get_charge(&self, charge_id: &str) -> Result<Charge> {
        let url = format!("{}/charges/{}", self.base_url(), charge_id);
        let response = self.http_client()
            .get(&url)
            .send()
            .await?;
        
        let status = response.status();
        if !status.is_success() {
            let error: crate::stripe::error::ApiError = response.json().await?;
            return Err(error.into());
        }
        
        let charge: Charge = response.json().await?;
        Ok(charge)
    }
    
    /// Update a charge by ID
    pub async fn update_charge(&self, charge_id: &str, params: &UpdateCharge) -> Result<Charge> {
        let url = format!("{}/charges/{}", self.base_url(), charge_id);
        let response = self.http_client()
            .post(&url)
            .json(params)
            .send()
            .await?;
        
        let status = response.status();
        if !status.is_success() {
            let error: crate::stripe::error::ApiError = response.json().await?;
            return Err(error.into());
        }
        
        let charge: Charge = response.json().await?;
        Ok(charge)
    }
    
    /// Capture a charge that was created with capture set to false
    pub async fn capture_charge(&self, charge_id: &str, params: Option<&CaptureCharge>) -> Result<Charge> {
        let url = format!("{}/charges/{}/capture", self.base_url(), charge_id);
        let mut request = self.http_client().post(&url);
        
        if let Some(params) = params {
            request = request.json(params);
        }
        
        let response = request.send().await?;
        
        let status = response.status();
        if !status.is_success() {
            let error: crate::stripe::error::ApiError = response.json().await?;
            return Err(error.into());
        }
        
        let charge: Charge = response.json().await?;
        Ok(charge)
    }
    
    /// List all charges
    pub async fn list_charges(&self, limit: Option<u32>, customer: Option<&str>) -> Result<List<Charge>> {
        let mut url = format!("{}/charges", self.base_url());
        
        // Add query parameters
        let mut params = Vec::new();
        
        if let Some(limit) = limit {
            params.push(format!("limit={}", limit));
        }
        
        if let Some(customer) = customer {
            params.push(format!("customer={}", customer));
        }
        
        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }
        
        let response = self.http_client()
            .get(&url)
            .send()
            .await?;
        
        let status = response.status();
        if !status.is_success() {
            let error: crate::stripe::error::ApiError = response.json().await?;
            return Err(error.into());
        }
        
        let charges: List<Charge> = response.json().await?;
        Ok(charges)
    }
}