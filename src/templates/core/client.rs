//! Stripe API client implementation

use reqwest::{header, Client as ReqwestClient};
use std::sync::Arc;
use std::time::Duration;

use crate::error::{Error, Result};
use crate::{API_BASE, API_VERSION};

/// Configuration for the Stripe client
#[derive(Debug, Clone)]
pub struct Config {
    /// The Stripe API key
    pub api_key: String,
    
    /// Base URL for the Stripe API
    pub api_base: String,
    
    /// Stripe API version
    pub api_version: String,
    
    /// Timeout for requests in seconds
    pub timeout: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            api_base: API_BASE.to_string(),
            api_version: API_VERSION.to_string(),
            timeout: 30,
        }
    }
}

/// Stripe API client
#[derive(Debug, Clone)]
pub struct Client {
    config: Config,
    http_client: Arc<ReqwestClient>,
}

impl Client {
    /// Create a new Stripe client with the given API key
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        let config = Config {
            api_key: api_key.into(),
            ..Default::default()
        };
        
        Self::new_with_config(config)
    }
    
    /// Create a new Stripe client with a custom configuration
    pub fn new_with_config(config: Config) -> Result<Self> {
        if config.api_key.is_empty() {
            return Err(Error::Configuration("API key is required".into()));
        }
        
        let mut headers = header::HeaderMap::new();
        let auth_value = format!("Bearer {}", config.api_key);
        let auth_header = header::HeaderValue::from_str(&auth_value)
            .map_err(|e| Error::Configuration(format!("Invalid API key: {}", e)))?;
        
        headers.insert(header::AUTHORIZATION, auth_header);
        
        let version_header = header::HeaderValue::from_str(&config.api_version)
            .map_err(|e| Error::Configuration(format!("Invalid API version: {}", e)))?;
        
        headers.insert("Stripe-Version", version_header);
        
        let http_client = ReqwestClient::builder()
            .timeout(Duration::from_secs(config.timeout))
            .default_headers(headers)
            .build()
            .map_err(|e| Error::Configuration(format!("Failed to create HTTP client: {}", e)))?;
        
        Ok(Self {
            config,
            http_client: Arc::new(http_client),
        })
    }
    
    /// Get the client configuration
    pub fn config(&self) -> &Config {
        &self.config
    }
    
    /// Get the underlying HTTP client
    pub fn http_client(&self) -> &ReqwestClient {
        &self.http_client
    }
    
    /// Get the base URL for API requests
    pub fn base_url(&self) -> &str {
        &self.config.api_base
    }
}