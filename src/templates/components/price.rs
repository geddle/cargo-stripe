//! Stripe Price API
//!
//! This module provides functionality to create, retrieve, update, and list prices.

use serde::{Deserialize, Serialize};

use crate::stripe::client::Client;
use crate::stripe::error::Result;
use crate::stripe::types::{Currency, Id, List, Metadata, Timestamp};

/// A Stripe price object
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Price {
    /// Unique identifier for the object
    pub id: Id,

    /// String representing the object's type
    pub object: String,

    /// Whether the price can be used for new purchases
    pub active: bool,

    /// Three-letter ISO currency code
    pub currency: Currency,

    /// One of `one_time` or `recurring` depending on whether the price is for a one-time purchase or a recurring (subscription) purchase
    pub type_: PriceType,

    /// The unit amount in the currency's smallest unit
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    /// The unit amount as a formatted string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,

    /// Time at which the object was created
    pub created: Timestamp,

    /// Has the value true if the object exists in live mode or the value false if the object exists in test mode
    pub livemode: bool,

    /// A lookup key used to retrieve prices dynamically
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_key: Option<String>,

    /// Set of key-value pairs attached to the object
    #[serde(default)]
    pub metadata: Metadata,

    /// A brief description of the price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,

    /// The ID of the product this price is associated with
    pub product: Id,

    /// The recurring components of a price such as `interval` and `usage_type`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<Recurring>,

    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<TaxBehavior>,

    /// Each element represents a pricing tier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<PriceTier>>,

    /// Defines if the tiering price should be calculated as a flat amount or percentages of the total price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers_mode: Option<TiersMode>,

    /// The transformation method to use for the tiered pricing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_quantity: Option<TransformQuantity>,
}

/// The type of price
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PriceType {
    /// A one-time price
    OneTime,

    /// A recurring price
    Recurring,
}

/// The recurring components of a price such as `interval` and `usage_type`
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Recurring {
    /// Specifies billing frequency
    pub interval: PriceInterval,

    /// The number of intervals between subscription billings
    pub interval_count: u32,

    /// Configures how the quantity per period should be determined
    pub usage_type: UsageType,

    /// Default number of trial days when subscribing a customer to this price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,

    /// Configures if the price is metered or not
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_usage: Option<AggregateUsage>,
}

/// Specifies billing frequency
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PriceInterval {
    /// Daily billing
    Day,

    /// Weekly billing
    Week,

    /// Monthly billing
    Month,

    /// Yearly billing
    Year,
}

/// Configures how the quantity per period should be determined
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UsageType {
    /// Licensed pricing model
    Licensed,

    /// Metered pricing model
    Metered,
}

/// Configures if the price is metered or not
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AggregateUsage {
    /// Summing up all usage during a period
    Sum,

    /// Using the last usage record during a period
    LastDuring,

    /// Using the last usage record ever
    LastEver,

    /// Using the maximum usage during a period
    Max,
}

/// Specifies whether the price is considered inclusive of taxes or exclusive of taxes
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TaxBehavior {
    /// Price doesn't include tax
    Exclusive,

    /// Price includes tax
    Inclusive,

    /// Price isn't taxed
    Unspecified,
}

/// Pricing tier
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PriceTier {
    /// Price for the tier
    pub unit_amount: i64,

    /// Upper bound of this tier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub up_to: Option<i64>,

    /// Same as unit_amount, but with decimals
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,

    /// Per unit billing amount for the tier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat_amount: Option<i64>,

    /// Same as flat_amount, but with decimals
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flat_amount_decimal: Option<String>,
}

/// Defines if the tiering price should be calculated as a flat amount or percentages of the total price
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TiersMode {
    /// Apply tier pricing to the total quantity
    Graduated,

    /// Apply tier pricing independently per group of quantities
    Volume,
}

/// The transformation method to use for the tiered pricing
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TransformQuantity {
    /// Divide quantity by this number
    pub divide_by: i64,

    /// After division, either round the result up or down
    pub round: RoundingMode,
}

/// Rounding mode for transform quantity
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RoundingMode {
    /// Round up to the nearest integer
    Up,

    /// Round down to the nearest integer
    Down,
}

/// Parameters for creating a new price
#[derive(Debug, Serialize, Clone)]
pub struct CreatePrice {
    /// Three-letter ISO currency code
    pub currency: Currency,

    /// The ID of the product that this price will belong to
    pub product: String,

    /// A positive integer in the currency's smallest unit representing how much to charge
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    /// Same as unit_amount, but accepts a decimal string with at most 12 decimal places
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,

    /// Whether the price is currently active
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// A brief description of the price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,

    /// The recurring components of a price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<Recurring>,

    /// Set of key-value pairs attached to the object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<TaxBehavior>,

    /// Each element represents a pricing tier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<PriceTier>>,

    /// Defines if the tiering price should be calculated as a flat amount or percentages of the total price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers_mode: Option<TiersMode>,

    /// A lookup key used to retrieve prices dynamically
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_key: Option<String>,

    /// The transformation method to use for the tiered pricing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_quantity: Option<TransformQuantity>,
}

/// Parameters for updating a price
#[derive(Debug, Serialize, Default, Clone)]
pub struct UpdatePrice {
    /// Whether the price is currently active
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// A brief description of the price
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,

    /// Set of key-value pairs attached to the object
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Metadata>,

    /// A lookup key used to retrieve prices dynamically
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lookup_key: Option<String>,

    /// The transformation method to use for the tiered pricing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_quantity: Option<TransformQuantity>,

    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<TaxBehavior>,
}

/// Price API implementation
impl Client {
    /// Create a new price
    pub async fn create_price(&self, params: &CreatePrice) -> Result<Price> {
        let url = format!("{}/prices", self.base_url());
        let response = self.http_client().post(&url).json(params).send().await?;

        let status = response.status();
        if !status.is_success() {
            let error: crate::stripe::error::ApiError = response.json().await?;
            return Err(error.into());
        }

        let price: Price = response.json().await?;
        Ok(price)
    }

    /// Retrieve a price by ID
    pub async fn get_price(&self, price_id: &str) -> Result<Price> {
        let url = format!("{}/prices/{}", self.base_url(), price_id);
        let response = self.http_client().get(&url).send().await?;

        let status = response.status();
        if !status.is_success() {
            let error: crate::stripe::error::ApiError = response.json().await?;
            return Err(error.into());
        }

        let price: Price = response.json().await?;
        Ok(price)
    }

    /// Update a price by ID
    pub async fn update_price(&self, price_id: &str, params: &UpdatePrice) -> Result<Price> {
        let url = format!("{}/prices/{}", self.base_url(), price_id);
        let response = self.http_client().post(&url).json(params).send().await?;

        let status = response.status();
        if !status.is_success() {
            let error: crate::stripe::error::ApiError = response.json().await?;
            return Err(error.into());
        }

        let price: Price = response.json().await?;
        Ok(price)
    }

    /// List all prices
    pub async fn list_prices(
        &self,
        limit: Option<u32>,
        active: Option<bool>,
        product: Option<&str>,
    ) -> Result<List<Price>> {
        let mut url = format!("{}/prices", self.base_url());

        let mut query_params = Vec::new();

        if let Some(limit) = limit {
            query_params.push(format!("limit={}", limit));
        }

        if let Some(active) = active {
            query_params.push(format!("active={}", active));
        }

        if let Some(product_id) = product {
            query_params.push(format!("product={}", product_id));
        }

        if !query_params.is_empty() {
            url = format!("{}?{}", url, query_params.join("&"));
        }

        let response = self.http_client().get(&url).send().await?;

        let status = response.status();
        if !status.is_success() {
            let error: crate::stripe::error::ApiError = response.json().await?;
            return Err(error.into());
        }

        let prices: List<Price> = response.json().await?;
        Ok(prices)
    }
}
