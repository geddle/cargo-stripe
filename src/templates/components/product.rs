//! Stripe Product API
//!
//! This module provides functionality to create, retrieve, update, and list products.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::stripe::client::Client;
use crate::stripe::error::Result;
use crate::stripe::types::{Id, List, Metadata, Timestamp};

/// A Stripe product object
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Product {
    /// Unique identifier for the object
    pub id: Id,

    /// String representing the object's type
    pub object: String,

    /// Whether the product is currently available for purchase
    pub active: bool,

    /// Time at which the object was created
    pub created: Timestamp,

    /// The product's description, meant to be displayable to the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// A list of up to 8 URLs of images for this product
    #[serde(default)]
    pub images: Vec<String>,

    /// Has the value true if the object exists in live mode or the value false if the object exists in test mode
    pub livemode: bool,

    /// Set of key-value pairs attached to the object
    #[serde(default)]
    pub metadata: Metadata,

    /// The product's name, meant to be displayable to the customer
    pub name: String,

    /// The dimensions of this product for shipping purposes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_dimensions: Option<PackageDimensions>,

    /// Whether this product is shipped (i.e., physical goods)
    pub shippable: Option<bool>,

    /// Extra information about a product which will appear on your customer's credit card statement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// A label that represents units of this product
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_label: Option<String>,

    /// Time at which the object was last updated
    pub updated: Timestamp,

    /// A URL of a publicly-accessible webpage for this product
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// The dimensions of a product for shipping purposes
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PackageDimensions {
    /// Height, in inches
    pub height: f64,

    /// Length, in inches
    pub length: f64,

    /// Weight, in ounces
    pub weight: f64,

    /// Width, in inches
    pub width: f64,
}

/// Parameters for creating a new product
#[derive(Debug, Serialize, Clone)]
pub struct CreateProduct {
    /// The product's name, meant to be displayable to the customer
    pub name: String,

    /// Whether the product is currently available for purchase
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// The product's description, meant to be displayable to the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// A list of up to 8 URLs of images for this product
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,

    /// Set of key-value pairs attached to the object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The dimensions of this product for shipping purposes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_dimensions: Option<PackageDimensions>,

    /// Whether this product is shipped (i.e., physical goods)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shippable: Option<bool>,

    /// Extra information about a product which will appear on your customer's credit card statement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// A label that represents units of this product
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_label: Option<String>,

    /// A URL of a publicly-accessible webpage for this product
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Parameters for updating a product
#[derive(Debug, Serialize, Default, Clone)]
pub struct UpdateProduct {
    /// The product's name, meant to be displayable to the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Whether the product is currently available for purchase
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// The product's description, meant to be displayable to the customer
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// A list of up to 8 URLs of images for this product
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,

    /// Set of key-value pairs attached to the object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// The dimensions of this product for shipping purposes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_dimensions: Option<PackageDimensions>,

    /// Whether this product is shipped (i.e., physical goods)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shippable: Option<bool>,

    /// Extra information about a product which will appear on your customer's credit card statement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// A label that represents units of this product
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_label: Option<String>,

    /// A URL of a publicly-accessible webpage for this product
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Product API implementation
impl Client {
    /// Create a new product
    pub async fn create_product(&self, params: &CreateProduct) -> Result<Product> {
        let url = format!("{}/products", self.base_url());
        let response = self.http_client().post(&url).json(params).send().await?;

        let status = response.status();
        if !status.is_success() {
            let error: crate::stripe::error::ApiError = response.json().await?;
            return Err(error.into());
        }

        let product: Product = response.json().await?;
        Ok(product)
    }

    /// Retrieve a product by ID
    pub async fn get_product(&self, product_id: &str) -> Result<Product> {
        let url = format!("{}/products/{}", self.base_url(), product_id);
        let response = self.http_client().get(&url).send().await?;

        let status = response.status();
        if !status.is_success() {
            let error: crate::stripe::error::ApiError = response.json().await?;
            return Err(error.into());
        }

        let product: Product = response.json().await?;
        Ok(product)
    }

    /// Update a product by ID
    pub async fn update_product(
        &self,
        product_id: &str,
        params: &UpdateProduct,
    ) -> Result<Product> {
        let url = format!("{}/products/{}", self.base_url(), product_id);
        let response = self.http_client().post(&url).json(params).send().await?;

        let status = response.status();
        if !status.is_success() {
            let error: crate::stripe::error::ApiError = response.json().await?;
            return Err(error.into());
        }

        let product: Product = response.json().await?;
        Ok(product)
    }

    /// Delete a product by ID
    pub async fn delete_product(&self, product_id: &str) -> Result<Product> {
        let url = format!("{}/products/{}", self.base_url(), product_id);
        let response = self.http_client().delete(&url).send().await?;

        let status = response.status();
        if !status.is_success() {
            let error: crate::stripe::error::ApiError = response.json().await?;
            return Err(error.into());
        }

        let product: Product = response.json().await?;
        Ok(product)
    }

    /// List all products
    pub async fn list_products(
        &self,
        limit: Option<u32>,
        active: Option<bool>,
    ) -> Result<List<Product>> {
        let mut url = format!("{}/products", self.base_url());

        let mut has_param = false;

        if let Some(limit) = limit {
            url = format!("{}?limit={}", url, limit);
            has_param = true;
        }

        if let Some(active) = active {
            let prefix = if has_param { "&" } else { "?" };
            url = format!("{}{}active={}", url, prefix, active);
        }

        let response = self.http_client().get(&url).send().await?;

        let status = response.status();
        if !status.is_success() {
            let error: crate::stripe::error::ApiError = response.json().await?;
            return Err(error.into());
        }

        let products: List<Product> = response.json().await?;
        Ok(products)
    }
}
