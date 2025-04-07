//! Stripe API client for Rust
//!
//! This module provides a Rust-idiomatic interface to the Stripe API.

pub mod client;
pub mod error;
pub mod types;

// Re-exports for convenience
pub use client::Client;
pub use error::{Error, Result};
pub use types::*;

/// The current version of the Stripe API used by this crate
pub const API_VERSION: &str = "2023-10-16";

/// The base URL for the Stripe API
pub const API_BASE: &str = "https://api.stripe.com/v1";