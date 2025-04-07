//! Stripe Customer API
//!
//! This module provides functionality to create, retrieve, update, and delete customers,
//! as well as list all customers.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::client::Client;
use crate::error::Result;
use crate::types::{Currency, Id, List, Metadata, Timestamp};

/// A Stripe customer object
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Customer {
    /// Unique identifier for the object
    pub id: Id,
    
    /// String representing the object's type. Objects of the same type share the same value
    pub object: String,
    
    /// The customer's email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    
    /// The customer's full name or business name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    /// The customer's phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    
    /// Customer's description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// The customer's address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    
    /// The default currency for the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    
    /// ID of the default payment source for the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<Id>,
    
    /// Whether this object is deleted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    
    /// When the customer was created
    pub created: Timestamp,
    
    /// Set of key-value pairs attached to the object
    #[serde(default)]
    pub metadata: Metadata,
}

/// An address associated with a customer
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Address {
    /// City, district, suburb, town, or village
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    
    /// Two-letter country code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    
    /// Address line 1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,
    
    /// Address line 2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,
    
    /// ZIP or postal code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    
    /// State, county, province, or region
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// Parameters for creating a new customer
#[derive(Debug, Serialize, Default, Clone)]
pub struct CreateCustomer {
    /// The customer's email address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    
    /// The customer's full name or business name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    
    /// The customer's phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    
    /// An arbitrary string attached to the object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    /// The customer's address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<Address>,
    
    /// Customer's preferred currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    
    /// Set of key-value pairs attached to the object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,
}

/// Customer API implementation
impl Client {
    /// Create a new customer
    pub async fn create_customer(&self, params: &CreateCustomer) -> Result<Customer> {
        let url = format!("{}/customers", self.base_url());
        let response = self.http_client()
            .post(&url)
            .json(params)
            .send()
            .await?;
        
        let status = response.status();
        if !status.is_success() {
            let error: crate::error::ApiError = response.json().await?;
            return Err(error.into());
        }
        
        let customer: Customer = response.json().await?;
        Ok(customer)
    }
    
    /// Retrieve a customer by ID
    pub async fn get_customer(&self, customer_id: &str) -> Result<Customer> {
        let url = format!("{}/customers/{}", self.base_url(), customer_id);
        let response = self.http_client()
            .get(&url)
            .send()
            .await?;
        
        let status = response.status();
        if !status.is_success() {
            let error: crate::error::ApiError = response.json().await?;
            return Err(error.into());
        }
        
        let customer: Customer = response.json().await?;
        Ok(customer)
    }
    
    /// Update a customer by ID
    pub async fn update_customer(&self, customer_id: &str, params: &CreateCustomer) -> Result<Customer> {
        let url = format!("{}/customers/{}", self.base_url(), customer_id);
        let response = self.http_client()
            .post(&url)
            .json(params)
            .send()
            .await?;
        
        let status = response.status();
        if !status.is_success() {
            let error: crate::error::ApiError = response.json().await?;
            return Err(error.into());
        }
        
        let customer: Customer = response.json().await?;
        Ok(customer)
    }
    
    /// Delete a customer by ID
    pub async fn delete_customer(&self, customer_id: &str) -> Result<Customer> {
        let url = format!("{}/customers/{}", self.base_url(), customer_id);
        let response = self.http_client()
            .delete(&url)
            .send()
            .await?;
        
        let status = response.status();
        if !status.is_success() {
            let error: crate::error::ApiError = response.json().await?;
            return Err(error.into());
        }
        
        let customer: Customer = response.json().await?;
        Ok(customer)
    }
    
    /// List all customers
    pub async fn list_customers(&self, limit: Option<u32>) -> Result<List<Customer>> {
        let mut url = format!("{}/customers", self.base_url());
        
        if let Some(limit) = limit {
            url = format!("{}?limit={}", url, limit);
        }
        
        let response = self.http_client()
            .get(&url)
            .send()
            .await?;
        
        let status = response.status();
        if !status.is_success() {
            let error: crate::error::ApiError = response.json().await?;
            return Err(error.into());
        }
        
        let customers: List<Customer> = response.json().await?;
        Ok(customers)
    }
}