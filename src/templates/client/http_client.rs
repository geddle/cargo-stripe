use std::future::{self, Future};
use std::pin::Pin;
use std::time::Duration;

use http::{Request as HttpRequest, StatusCode};
use http_types::{Request, Url};
use hyper::body::{Body as HyperBody, Bytes, Incoming};
use hyper_util::client::legacy::Client as HyperClient;
use hyper_util::client::legacy::connect::HttpConnector;
use hyper_util::rt::TokioExecutor;
use serde::de::DeserializeOwned;
use tokio::time::sleep;

use crate::stripe::error::{ErrorResponse, StripeError};

use super::request_strategy::{Outcome, RequestStrategy};

/// A response future for async operations
pub type Response<T> = Pin<Box<dyn Future<Output = Result<T, StripeError>> + Send>>;

/// Helper to create successful responses
#[inline(always)]
pub(crate) fn ok<T: Send + 'static>(ok: T) -> Response<T> {
    Box::pin(future::ready(Ok(ok)))
}

/// Helper to create error responses
#[inline(always)]
pub(crate) fn err<T: Send + 'static>(err: StripeError) -> Response<T> {
    Box::pin(future::ready(Err(err)))
}

/// TLS connector for HTTPS requests
mod connector {
    pub use hyper_rustls::HttpsConnector;
    pub use hyper_util::client::legacy::connect::HttpConnector;

    pub fn create() -> HttpsConnector<HttpConnector> {
        let mut http = HttpConnector::new();
        http.enforce_http(false);
        
        HttpsConnector::from((
            http, 
            hyper_rustls::TlsConfigBuilder::native()
                .https_only()
                .build()
                .unwrap()
        ))
    }
}

// Implement conversion from hyper_util error to StripeError
impl From<hyper_util::client::legacy::Error> for StripeError {
    fn from(err: hyper_util::client::legacy::Error) -> Self {
        StripeError::ClientError(format!("HTTP client error: {}", err))
    }
}

// Implement conversion from hyper error to StripeError
impl From<hyper::Error> for StripeError {
    fn from(err: hyper::Error) -> Self {
        StripeError::ClientError(format!("Hyper error: {}", err))
    }
}

type HttpClient = HyperClient<connector::HttpsConnector<HttpConnector>, Incoming>;

/// Modern HTTP client built on hyper v1 and tokio
#[derive(Clone)]
pub struct StripeHttpClient {
    client: HttpClient,
}

impl Default for StripeHttpClient {
    fn default() -> Self {
        Self::new()
    }
}

impl StripeHttpClient {
    /// Create a new HTTP client
    pub fn new() -> Self {
        Self {
            client: HyperClient::builder(TokioExecutor::new())
                .pool_max_idle_per_host(0)
                .build(connector::create()),
        }
    }

    /// Execute a request with the specified strategy
    pub fn execute<T: DeserializeOwned + Send + 'static>(
        &self,
        request: Request,
        strategy: &RequestStrategy,
    ) -> Response<T> {
        let client = self.client.clone();
        let strategy = strategy.clone();

        Box::pin(async move {
            let bytes = send_request(&client, request, &strategy).await?;
            let json_deserializer = &mut serde_json::Deserializer::from_slice(&bytes);
            serde_path_to_error::deserialize(json_deserializer).map_err(StripeError::from)
        })
    }
}

/// Send a request to the Stripe API with retry handling
async fn send_request(
    client: &HttpClient,
    mut request: Request,
    strategy: &RequestStrategy,
) -> Result<Bytes, StripeError> {
    let mut tries = 0;
    let mut last_status: Option<StatusCode> = None;
    let mut last_retry_header: Option<bool> = None;
    let mut last_error = StripeError::ClientError("Invalid strategy".to_string());

    // Set idempotency key if provided by strategy
    if let Some(key) = strategy.get_key() {
        request.insert_header("Idempotency-Key", key);
    }

    let body = request.body_bytes().await?;

    loop {
        match strategy.test(last_status, last_retry_header, tries) {
            Outcome::Stop => return Err(last_error),
            Outcome::Continue(duration) => {
                if let Some(duration) = duration {
                    sleep(duration).await;
                }

                // Clone and prepare the request for this attempt
                let mut req = request.clone();
                req.set_body(body.clone());

                // Convert and send the request
                let hyper_req = convert_request(req).await?;
                let response = match client.request(hyper_req).await {
                    Ok(response) => response,
                    Err(err) => {
                        last_error = StripeError::from(err);
                        tries += 1;
                        continue;
                    }
                };

                let status = response.status();
                let retry = response
                    .headers()
                    .get("Stripe-Should-Retry")
                    .and_then(|s| s.to_str().ok())
                    .and_then(|s| s.parse::<bool>().ok());

                // Get response body
                let body_bytes = hyper::body::to_bytes(response.into_body()).await?;

                // Handle error responses
                if !status.is_success() {
                    tries += 1;
                    let json_deserializer = &mut serde_json::Deserializer::from_slice(&body_bytes);
                    last_error = serde_path_to_error::deserialize(json_deserializer)
                        .map(|mut e: ErrorResponse| {
                            e.error.http_status = status.as_u16();
                            StripeError::from(e.error)
                        })
                        .unwrap_or_else(|_| {
                            StripeError::ClientError(format!("HTTP error: {}", status))
                        });
                    
                    last_status = Some(status);
                    last_retry_header = retry;
                    continue;
                }

                return Ok(body_bytes);
            }
        }
    }
}

/// Convert an http_types::Request to a hyper v1 compatible request
async fn convert_request(mut request: Request) -> Result<HttpRequest<Incoming>, StripeError> {
    let body_bytes = request
        .body_bytes()
        .await?;
    
    // Manually convert from http_types::Request to http::Request
    let method = http::Method::from_bytes(request.method().to_string().as_bytes())
        .map_err(|e| StripeError::ClientError(format!("Invalid method: {}", e)))?;
    
    let uri = http::Uri::try_from(request.url().as_str())
        .map_err(|e| StripeError::ClientError(format!("Invalid URI: {}", e)))?;
    
    let mut builder = HttpRequest::builder()
        .method(method)
        .uri(uri);
    
    // Copy headers
    for (name, values) in request.iter() {
        for value in values {
            let header_name = http::HeaderName::from_bytes(name.as_str().as_bytes())
                .map_err(|e| StripeError::ClientError(format!("Invalid header name: {}", e)))?;
            
            let header_value = http::HeaderValue::from_str(value.as_str())
                .map_err(|e| StripeError::ClientError(format!("Invalid header value: {}", e)))?;
            
            builder = builder.header(header_name, header_value);
        }
    }
    
    // Create request with properly converted body
    let request = builder
        .body(Incoming::from(body_bytes))
        .map_err(|e| StripeError::ClientError(format!("Failed to build request: {}", e)))?;
    
    Ok(request)
}

#[cfg(test)]
mod tests {
    use super::*;
    use http_types::{Method, Url};

    #[tokio::test]
    async fn test_client_creation() {
        let client = StripeHttpClient::new();
        assert!(client.client.pool_max_idle_per_host().unwrap() == 0);
    }

    #[tokio::test]
    async fn test_convert_request() {
        // Create a simple request
        let request = Request::get(Url::parse("https://example.com").unwrap());
        
        // Test that the conversion works without error
        let result = convert_request(request).await;
        assert!(result.is_ok());
        
        let hyper_req = result.unwrap();
        assert_eq!(hyper_req.method(), "GET");
        assert_eq!(hyper_req.uri().to_string(), "https://example.com/");
    }
}
