//! Common types used across the Stripe API

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Representation of a Stripe resource ID
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Id(String);

impl Id {
    /// Create a new ID from a string
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
    
    /// Get the inner string value
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for Id {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for Id {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A list of objects returned by the Stripe API
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct List<T> {
    /// The URL for accessing this list
    pub url: String,
    
    /// True if this list has more items
    pub has_more: bool,
    
    /// Array containing the actual response elements, paginated by any request parameters
    pub data: Vec<T>,
    
    /// The total count of all objects available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<u64>,
}

impl<T> List<T> {
    /// Create a new empty list
    pub fn new() -> Self {
        Self {
            url: String::new(),
            has_more: false,
            data: Vec::new(),
            total_count: None,
        }
    }
    
    /// Get the number of items in this page
    pub fn len(&self) -> usize {
        self.data.len()
    }
    
    /// Check if this page is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// A timestamp representing a specific date and time
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp(i64);

impl Timestamp {
    /// Create a new timestamp from a Unix timestamp (seconds since epoch)
    pub fn new(timestamp: i64) -> Self {
        Self(timestamp)
    }
    
    /// Get the inner timestamp value (seconds since epoch)
    pub fn as_i64(&self) -> i64 {
        self.0
    }
    
    /// Convert to a DateTime<Utc>
    #[cfg(feature = "chrono")]
    pub fn to_datetime(&self) -> chrono::DateTime<chrono::Utc> {
        use chrono::{DateTime, TimeZone, Utc};
        Utc.timestamp_opt(self.0, 0).unwrap()
    }
}

impl From<i64> for Timestamp {
    fn from(timestamp: i64) -> Self {
        Self(timestamp)
    }
}

/// Currencies supported by Stripe
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Currency {
    /// United States Dollar
    Usd,
    /// Euro
    Eur,
    /// British Pound
    Gbp,
    /// Japanese Yen
    Jpy,
    /// Canadian Dollar
    Cad,
    /// Australian Dollar
    Aud,
    /// Swiss Franc
    Chf,
    /// Chinese Yuan
    Cny,
    /// Hong Kong Dollar
    Hkd,
    /// New Zealand Dollar
    Nzd,
    /// Mexican Peso
    Mxn,
    /// Singapore Dollar
    Sgd,
    /// Swedish Krona
    Sek,
    /// Danish Krone
    Dkk,
    /// Norwegian Krone
    Nok,
    // Add more currencies as needed
    /// Other currency (represented as a lowercase three-letter ISO code)
    #[serde(other)]
    Other,
}

/// Metadata attached to Stripe objects
pub type Metadata = HashMap<String, String>;