use std::time::Duration;

use reqwest::{Client as ReqwestClient, Method, StatusCode, RequestBuilder, Url};
use serde::{de::DeserializeOwned, Serialize};
use tokio::time::sleep;

use crate::stripe::{
    error::{ErrorResponse, StripeError},
    params::AppInfo,
    AccountId, ApplicationId, Headers,
    resources::ApiVersion,
};

use super::{
    request_strategy::{Outcome, RequestStrategy},
    http_client::{Response, err, ok},
};

/// Client agent identifier
static USER_AGENT: &str = concat!("Stripe/v1 RustBindings/", env!("CARGO_PKG_VERSION"));

/// Main client for interacting with the Stripe API
#[derive(Clone)]
pub struct StripeClient {
    client: ReqwestClient,
    secret_key: String,
    headers: Headers,
    strategy: RequestStrategy,
    app_info: Option<AppInfo>,
    api_base: Url,
    api_root: String,
}

impl StripeClient {
    /// Create a new client with the given secret key
    pub fn new(secret_key: impl Into<String>) -> Result<Self, StripeError> {
        Self::from_url("https://api.stripe.com/", secret_key)
    }

    /// Create a new client pointed at a specific URL (useful for testing)
    pub fn from_url<'a>(url: impl Into<&'a str>, secret_key: impl Into<String>) -> Result<Self, StripeError> {
        let client = ReqwestClient::builder()
            .timeout(Duration::from_secs(30))
            .connect_timeout(Duration::from_secs(10))
            .pool_idle_timeout(Some(Duration::from_secs(60)))
            .user_agent(USER_AGENT)
            .build()
            .map_err(|e| StripeError::ClientError(format!("Failed to create HTTP client: {}", e)))?;

        let api_base = Url::parse(url.into())
            .map_err(|e| StripeError::ClientError(format!("Invalid URL: {}", e)))?;

        Ok(Self {
            client,
            secret_key: secret_key.into(),
            headers: Headers {
                stripe_version: ApiVersion::default(),
                user_agent: USER_AGENT.to_string(),
                client_id: None,
                stripe_account: None,
            },
            strategy: RequestStrategy::Once,
            app_info: None,
            api_base,
            api_root: "v1".to_string(),
        })
    }

    /// Set the client id for the client
    pub fn with_client_id(mut self, id: ApplicationId) -> Self {
        self.headers.client_id = Some(id);
        self
    }

    /// Set the stripe account for the client
    pub fn with_stripe_account(mut self, id: AccountId) -> Self {
        self.headers.stripe_account = Some(id);
        self
    }

    /// Set the request strategy for the client
    pub fn with_strategy(mut self, strategy: RequestStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    /// Set the application info for the client
    pub fn with_app_info(
        mut self,
        name: String,
        version: Option<String>,
        url: Option<String>,
    ) -> Self {
        let app_info = AppInfo { name, version, url };
        self.headers.user_agent = format!("{} {}", USER_AGENT, app_info);
        self.app_info = Some(app_info);
        self
    }

    /// Make a `GET` http request with just a path
    pub fn get<T: DeserializeOwned + Send + 'static>(&self, path: &str) -> Response<T> {
        let url = self.url(path);
        self.execute(self.create_request(Method::GET, url, None::<&()>))
    }

    /// Make a `GET` http request with url query parameters
    pub fn get_query<T: DeserializeOwned + Send + 'static, P: Serialize + Send + 'static>(
        &self,
        path: &str,
        params: &P,
    ) -> Response<T> {
        let request = match self.create_query_request(Method::GET, path, params) {
            Ok(req) => req,
            Err(e) => return super::http_client::err(e),
        };
        self.execute(request)
    }

    /// Make a `DELETE` http request with just a path
    pub fn delete<T: DeserializeOwned + Send + 'static>(&self, path: &str) -> Response<T> {
        let url = self.url(path);
        self.execute(self.create_request(Method::DELETE, url, None::<&()>))
    }

    /// Make a `DELETE` http request with url query parameters
    pub fn delete_query<T: DeserializeOwned + Send + 'static, P: Serialize + Send + 'static>(
        &self,
        path: &str,
        params: &P,
    ) -> Response<T> {
        let request = match self.create_query_request(Method::DELETE, path, params) {
            Ok(req) => req,
            Err(e) => return super::http_client::err(e),
        };
        self.execute(request)
    }

    /// Make a `POST` http request with just a path
    pub fn post<T: DeserializeOwned + Send + 'static>(&self, path: &str) -> Response<T> {
        let url = self.url(path);
        self.execute(self.create_request(Method::POST, url, None::<&()>))
    }

    /// Make a `POST` http request with urlencoded body
    pub fn post_form<T: DeserializeOwned + Send + 'static, F: Serialize + Send + 'static>(
        &self,
        path: &str,
        form: &F,
    ) -> Response<T> {
        let url = self.url(path);
        let request = self.create_request(Method::POST, url, Some(form))
            .header("content-type", "application/x-www-form-urlencoded");
        self.execute(request)
    }

    /// Create a URL for the given path
    fn url(&self, path: &str) -> Url {
        let mut url = self.api_base.clone();
        url.set_path(&format!("{}/{}", self.api_root, path.trim_start_matches('/')));
        url
    }

    /// Create a request builder with the appropriate headers and parameters
    fn create_request<P: Serialize + ?Sized>(
        &self,
        method: Method,
        url: Url,
        params: Option<&P>,
    ) -> RequestBuilder {
        let mut builder = self.client.request(method, url)
            .header("authorization", format!("Bearer {}", self.secret_key))
            .header("stripe-version", self.headers.stripe_version.as_str())
            .header("user-agent", &self.headers.user_agent);

        // Set optional headers
        if let Some(client_id) = &self.headers.client_id {
            builder = builder.header("client-id", client_id.as_str());
        }
        if let Some(account) = &self.headers.stripe_account {
            builder = builder.header("stripe-account", account.as_str());
        }

        // If idempotency key is set in the request strategy, add it
        if let Some(key) = self.strategy.get_key() {
            builder = builder.header("idempotency-key", key);
        }

        // Add parameters if provided
        if let Some(params) = params {
            builder = builder.form(params);
        }

        builder
    }

    /// Create a request with query parameters
    fn create_query_request<P: Serialize>(
        &self,
        method: Method,
        path: &str,
        params: &P,
    ) -> Result<RequestBuilder, StripeError> {
        let url = self.url(path);
        let request = self.create_request(method, url, None::<&()>);
        
        Ok(request.query(params))
    }

    /// Execute a request with the configured strategy
    fn execute<T: DeserializeOwned + Send + 'static>(
        &self,
        request: RequestBuilder,
    ) -> Response<T> {
        let strategy = self.strategy.clone();

        Box::pin(async move {
            let mut tries = 0;
            let mut last_status: Option<StatusCode> = None;
            let mut last_retry_header: Option<bool> = None;
            let mut last_error = StripeError::ClientError("Invalid strategy".to_string());

            loop {
                match strategy.test(last_status, last_retry_header, tries) {
                    Outcome::Stop => return Err(last_error),
                    Outcome::Continue(duration) => {
                        if let Some(duration) = duration {
                            sleep(duration).await;
                        }

                        // Clone the request for this attempt
                        // We need a new clone for each iteration since send() consumes the builder
                        let request_clone = request.try_clone()
                            .ok_or_else(|| StripeError::ClientError("Failed to clone request".to_string()))?;

                        // Send the request
                        let response = match request_clone.send().await {
                            Ok(response) => response,
                            Err(err) => {
                                last_error = if err.is_timeout() {
                                    StripeError::Timeout
                                } else {
                                    StripeError::ClientError(format!("HTTP request error: {}", err))
                                };
                                tries += 1;
                                continue;
                            }
                        };

                        let status = response.status();
                        let retry = response
                            .headers()
                            .get("stripe-should-retry")
                            .and_then(|s| s.to_str().ok())
                            .and_then(|s| s.parse::<bool>().ok());

                        // Check for error responses
                        if !status.is_success() {
                            tries += 1;
                            
                            // Attempt to parse the error response
                            let bytes = match response.bytes().await {
                                Ok(bytes) => bytes,
                                Err(e) => {
                                    last_error = StripeError::ClientError(format!(
                                        "HTTP error {} and failed to read body: {}", status, e
                                    ));
                                    last_status = Some(status);
                                    last_retry_header = retry;
                                    continue;
                                }
                            };
                            
                            // Use serde_path_to_error for better error messages
                            let json_deserializer = &mut serde_json::Deserializer::from_slice(&bytes);
                            match serde_path_to_error::deserialize::<_, ErrorResponse>(json_deserializer) {
                                Ok(mut err_response) => {
                                    err_response.error.http_status = status.as_u16();
                                    last_error = StripeError::Stripe(err_response.error);
                                }
                                Err(_) => {
                                    // Failed to parse the response as JSON
                                    let text = String::from_utf8_lossy(&bytes);
                                    last_error = StripeError::ClientError(format!(
                                        "HTTP error {}: {}", status, text
                                    ));
                                }
                            }
                            
                            last_status = Some(status);
                            last_retry_header = retry;
                            continue;
                        }

                        // Successfully received response
                        let bytes = response.bytes().await
                            .map_err(|e| StripeError::ClientError(format!("Failed to get response body: {}", e)))?;
                            
                        // Use serde_path_to_error to get better error messages with paths
                        let json_deserializer = &mut serde_json::Deserializer::from_slice(&bytes);
                        return serde_path_to_error::deserialize(json_deserializer)
                            .map_err(StripeError::JSONSerialize);
                    }
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stripe::AccountId;

    #[test]
    fn test_user_agent() {
        let client = StripeClient::new("sk_test_12345").unwrap();
        assert_eq!(
            client.headers.user_agent,
            format!("Stripe/v1 RustBindings/{}", env!("CARGO_PKG_VERSION"))
        );
    }

    #[test]
    fn test_user_agent_with_app_info() {
        let client = StripeClient::new("sk_test_12345").unwrap().with_app_info(
            "test-app".to_string(),
            Some("1.0.0".to_string()),
            Some("https://example.com".to_string()),
        );
        
        assert_eq!(
            client.headers.user_agent,
            format!(
                "Stripe/v1 RustBindings/{} test-app/1.0.0 (https://example.com)",
                env!("CARGO_PKG_VERSION")
            )
        );
    }

    #[test]
    fn test_url_creation() {
        let client = StripeClient::new("sk_test_12345").unwrap();
        let url = client.url("customers");
        assert_eq!(url.as_str(), "https://api.stripe.com/v1/customers");
    }

    #[test]
    fn test_url_with_leading_slash() {
        let client = StripeClient::new("sk_test_12345").unwrap();
        let url = client.url("/customers");
        assert_eq!(url.as_str(), "https://api.stripe.com/v1/customers");
    }

    #[test]
    fn test_stripe_account_header() {
        let account_id = "acct_12345".parse::<AccountId>().unwrap();
        let client = StripeClient::new("sk_test_12345").unwrap()
            .with_stripe_account(account_id);
        
        assert_eq!(client.headers.stripe_account, Some(account_id));
    }
}