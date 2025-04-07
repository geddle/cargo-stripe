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
                let hyper_req = convert_request(req).await;
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
                let bytes = hyper::body::to_bytes(response.into_body()).await?;

                // Handle error responses
                if !status.is_success() {
                    tries += 1;
                    let json_deserializer = &mut serde_json::Deserializer::from_slice(&bytes);
                    last_error = serde_path_to_error::deserialize(json_deserializer)
                        .map(|mut e: ErrorResponse| {
                            e.error.http_status = status.as_u16();
                            StripeError::from(e.error)
                        })
                        .unwrap_or_else(|_| {
                            StripeError::Http(format!("HTTP error: {}", status))
                        });
                    
                    last_status = Some(status);
                    last_retry_header = retry;
                    continue;
                }

                return Ok(bytes);
            }
        }
    }
}

/// Convert an http_types::Request to a hyper v1 compatible request
async fn convert_request(mut request: Request) -> HttpRequest<Incoming> {
    let body = request
        .body_bytes()
        .await
        .expect("We know the data is a valid bytes object.");
    
    // First convert to an http::Request with a dummy body
    let req: HttpRequest<()> = request.into();
    
    // Extract parts and rebuild with correct body
    let (parts, _) = req.into_parts();
    let bytes = Bytes::from(body);
    
    // Create the incoming body
    let body = Incoming::from(bytes);
    
    HttpRequest::from_parts(parts, body)
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
    async fn test_client_request() {
        // This test would normally use httpmock, but is simplified here
        let client = StripeHttpClient::new();
        let request = Request::get(Url::parse("https://example.com").unwrap());
        
        // Just test that the conversion works without error
        let hyper_req = convert_request(request).await;
        assert_eq!(hyper_req.method(), "GET");
    }
}
