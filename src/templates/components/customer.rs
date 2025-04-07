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
    
    /// Default payment method for this customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<InvoiceSettings>,
    
    /// Whether this object is deleted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    
    /// When the customer was created
    pub created: Timestamp,
    
    /// Set of key-value pairs attached to the object
    #[serde(default)]
    pub metadata: Metadata,
    
    /// The customer's shipping information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<ShippingInfo>,
    
    /// The customer's tax information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<TaxInfo>,
    
    /// The customer's tax exemption status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<TaxExemptStatus>,
    
    /// Describes the customer's tax IDs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<List<TaxId>>,
    
    /// Current balance, if any, being stored on the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<i64>,
    
    /// The customer's payment methods, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<List<PaymentSource>>,
    
    /// The customer's preferred locales
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locales: Option<Vec<String>>,
    
    /// The customer's next invoice sequence
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_invoice_sequence: Option<i64>,
    
    /// The customer's default payment method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<Id>,
    
    /// The customer's discount, if any
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<Discount>,
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

/// Customer's invoice settings
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct InvoiceSettings {
    /// ID of the default payment method used for subscriptions and invoices for the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<Id>,
    
    /// Default footer to be displayed on invoices for this customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    
    /// Default custom fields to be displayed on invoices for this customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
}

/// Custom field for invoice settings
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CustomField {
    /// The name of the custom field
    pub name: String,
    
    /// The value of the custom field
    pub value: String,
}

/// Shipping information
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ShippingInfo {
    /// Customer shipping address
    pub address: Address,
    
    /// Customer name
    pub name: String,
    
    /// Customer phone (including extension)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

/// Tax information
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TaxInfo {
    /// Tax ID type
    pub tax_id_type: String,
    
    /// Tax ID value
    pub tax_id_value: String,
    
    /// Verification status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<TaxIdVerification>,
}

/// Tax ID verification
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TaxIdVerification {
    /// Verification status
    pub status: String,
    
    /// Verification details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_name: Option<String>,
    
    /// Verification failure reason
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_address: Option<String>,
}

/// Tax ID
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TaxId {
    /// Unique identifier for the object
    pub id: Id,
    
    /// String representing the object's type
    pub object: String,
    
    /// Two-letter ISO code representing the country of the tax ID
    pub country: String,
    
    /// Date when the tax ID was created
    pub created: Timestamp,
    
    /// ID of the customer
    pub customer: Id,
    
    /// Whether this object is deleted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,
    
    /// Has the value true if the object exists in live mode
    pub livemode: bool,
    
    /// Type of the tax ID
    pub type_: String,
    
    /// Value of the tax ID
    pub value: String,
    
    /// Tax ID verification information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<TaxIdVerification>,
}

/// Tax exemption status
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TaxExemptStatus {
    /// Tax exempt
    Exempt,
    
    /// Not exempt
    None,
    
    /// Reverse charge
    Reverse,
}

/// Payment source
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PaymentSource {
    /// Unique identifier for the object
    pub id: Id,
    
    /// String representing the object's type
    pub object: String,
    
    /// Extra fields specific to the payment method type
    #[serde(flatten)]
    pub details: HashMap<String, serde_json::Value>,
}

/// Discount
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Discount {
    /// ID of the discount object
    pub id: Id,
    
    /// String representing the object's type
    pub object: String,
    
    /// ID of the coupon for this discount
    pub coupon: Id,
    
    /// Customer that this discount is for
    pub customer: Id,
    
    /// Date that the coupon was applied
    pub start: Timestamp,
    
    /// Date that the discount ends
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<Timestamp>,
    
    /// Subscription ID if this is a subscription discount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<Id>,
    
    /// Invoice item ID if this is an invoice item discount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_item: Option<Id>,
    
    /// Invoice ID if this is an invoice discount
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<Id>,
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
    
    /// The ID of the payment method to attach to the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    
    /// The customer's shipping information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<ShippingInfo>,
    
    /// The customer's tax exemption status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<TaxExemptStatus>,
    
    /// The customer's tax ID information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_data: Option<Vec<TaxIdData>>,
    
    /// Default invoice settings for this customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<CustomerInvoiceSettings>,
    
    /// The customer's preferred locales
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locales: Option<Vec<String>>,
    
    /// A payment source to attach to the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    
    /// The customer's current balance
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<i64>,
    
    /// ID of a promotion code to apply to the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
    
    /// The customer's next invoice sequence
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_invoice_sequence: Option<i64>,
    
    /// The coupon to apply to this customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,
}

/// Tax ID data for customer creation or update
#[derive(Debug, Serialize, Clone)]
pub struct TaxIdData {
    /// Type of the tax ID
    pub type_: String,
    
    /// Value of the tax ID
    pub value: String,
}

/// Customer invoice settings for creation or update
#[derive(Debug, Serialize, Default, Clone)]
pub struct CustomerInvoiceSettings {
    /// ID of the default payment method for the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,
    
    /// Default footer to be displayed on invoices for this customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    
    /// Default custom fields to be displayed on invoices for this customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomField>>,
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
    pub async fn list_customers(&self, limit: Option<u32>, email: Option<&str>, starting_after: Option<&str>, ending_before: Option<&str>) -> Result<List<Customer>> {
        let mut url = format!("{}/customers", self.base_url());
        
        // Build query parameters
        let mut query_params = Vec::new();
        
        if let Some(limit) = limit {
            query_params.push(format!("limit={}", limit));
        }
        
        if let Some(email) = email {
            query_params.push(format!("email={}", email));
        }
        
        if let Some(starting_after) = starting_after {
            query_params.push(format!("starting_after={}", starting_after));
        }
        
        if let Some(ending_before) = ending_before {
            query_params.push(format!("ending_before={}", ending_before));
        }
        
        // Add query parameters to URL if there are any
        if !query_params.is_empty() {
            url = format!("{}?{}", url, query_params.join("&"));
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
    
    /// Search customers by query
    pub async fn search_customers(&self, query: &str, limit: Option<u32>, page: Option<&str>) -> Result<List<Customer>> {
        let mut url = format!("{}/customers/search", self.base_url());
        
        // Build query parameters
        let mut query_params = Vec::new();
        
        query_params.push(format!("query={}", query));
        
        if let Some(limit) = limit {
            query_params.push(format!("limit={}", limit));
        }
        
        if let Some(page) = page {
            query_params.push(format!("page={}", page));
        }
        
        // Add query parameters to URL
        url = format!("{}?{}", url, query_params.join("&"));
        
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