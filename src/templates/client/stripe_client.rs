use http_types::{Body, Method, Request, Url};
use serde::{de::DeserializeOwned, Serialize};

use crate::stripe::{
    error::StripeError,
    params::AppInfo,
    AccountId, ApplicationId, Headers,
};
use crate::stripe::generated::core::version::VERSION;

use super::{
    http_client::{Response, StripeHttpClient},
    request_strategy::RequestStrategy,
    http_client as client
};

static USER_AGENT: &str = concat!("Stripe/v1 RustBindings/", env!("CARGO_PKG_VERSION"));

/// Main client for interacting with the Stripe API
#[derive(Clone)]
pub struct StripeClient {
    client: StripeHttpClient,
    secret_key: String,
    headers: Headers,
    strategy: RequestStrategy,
    app_info: Option<AppInfo>,
    api_base: Url,
    api_root: String,
}

impl StripeClient {
    /// Create a new client with the given secret key
    pub fn new(secret_key: impl Into<String>) -> Self {
        Self::from_url("https://api.stripe.com/", secret_key)
    }

    /// Create a new client pointed at a specific URL (useful for testing)
    pub fn from_url<'a>(url: impl Into<&'a str>, secret_key: impl Into<String>) -> Self {
        Self {
            client: StripeHttpClient::new(),
            secret_key: secret_key.into(),
            headers: Headers {
                stripe_version: VERSION,
                user_agent: USER_AGENT.to_string(),
                client_id: None,
                stripe_account: None,
            },
            strategy: RequestStrategy::Once,
            app_info: None,
            api_base: Url::parse(url.into()).expect("invalid url"),
            api_root: "v1".to_string(),
        }
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
        self.client.execute::<T>(self.create_request(Method::Get, url), &self.strategy)
    }

    /// Make a `GET` http request with url query parameters
    pub fn get_query<T: DeserializeOwned + Send + 'static, P: Serialize>(
        &self,
        path: &str,
        params: P,
    ) -> Response<T> {
        let url = match self.url_with_params(path, params) {
            Err(e) => return client::err(e),
            Ok(ok) => ok,
        };
        self.client.execute::<T>(self.create_request(Method::Get, url), &self.strategy)
    }

    /// Make a `DELETE` http request with just a path
    pub fn delete<T: DeserializeOwned + Send + 'static>(&self, path: &str) -> Response<T> {
        let url = self.url(path);
        self.client.execute::<T>(self.create_request(Method::Delete, url), &self.strategy)
    }

    /// Make a `DELETE` http request with url query parameters
    pub fn delete_query<T: DeserializeOwned + Send + 'static, P: Serialize>(
        &self,
        path: &str,
        params: P,
    ) -> Response<T> {
        let url = match self.url_with_params(path, params) {
            Err(e) => return client::err(e),
            Ok(ok) => ok,
        };
        self.client.execute::<T>(self.create_request(Method::Delete, url), &self.strategy)
    }

    /// Make a `POST` http request with just a path
    pub fn post<T: DeserializeOwned + Send + 'static>(&self, path: &str) -> Response<T> {
        let url = self.url(path);
        self.client.execute::<T>(self.create_request(Method::Post, url), &self.strategy)
    }

    /// Make a `POST` http request with urlencoded body
    pub fn post_form<T: DeserializeOwned + Send + 'static, F: Serialize>(
        &self,
        path: &str,
        form: F,
    ) -> Response<T> {
        let url = self.url(path);
        let mut req = self.create_request(Method::Post, url);

        let mut params_buffer = Vec::new();
        let qs_ser = &mut serde_qs::Serializer::new(&mut params_buffer);
        if let Err(qs_ser_err) = serde_path_to_error::serialize(&form, qs_ser) {
            return client::err(StripeError::QueryStringSerialize(qs_ser_err));
        }

        let body = std::str::from_utf8(params_buffer.as_slice())
            .expect("Unable to extract string from params_buffer")
            .to_string();

        req.set_body(Body::from_string(body));
        req.insert_header("content-type", "application/x-www-form-urlencoded");
        
        self.client.execute::<T>(req, &self.strategy)
    }

    /// Create a URL for the given path
    fn url(&self, path: &str) -> Url {
        let mut url = self.api_base.clone();
        url.set_path(&format!("{}/{}", self.api_root, path.trim_start_matches('/')));
        url
    }

    /// Create a URL with query parameters
    fn url_with_params<P: Serialize>(&self, path: &str, params: P) -> Result<Url, StripeError> {
        let mut url = self.url(path);

        let mut params_buffer = Vec::new();
        let qs_ser = &mut serde_qs::Serializer::new(&mut params_buffer);
        serde_path_to_error::serialize(&params, qs_ser).map_err(StripeError::QueryStringSerialize)?;

        let params = std::str::from_utf8(params_buffer.as_slice())
            .expect("Unable to extract string from params_buffer")
            .to_string();

        url.set_query(Some(&params));
        Ok(url)
    }

    /// Create an HTTP request with the appropriate headers
    fn create_request(&self, method: Method, url: Url) -> Request {
        let mut req = Request::new(method, url);
        req.insert_header("authorization", format!("Bearer {}", self.secret_key));

        for (key, value) in self.headers.to_array().iter().filter_map(|(k, v)| v.map(|v| (*k, v))) {
            req.insert_header(key, value);
        }

        req
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_agent() {
        let client = StripeClient::new("sk_test_12345");
        assert_eq!(
            client.headers.user_agent,
            format!("Stripe/v1 RustBindings/{}", env!("CARGO_PKG_VERSION"))
        );
    }

    #[test]
    fn test_user_agent_with_app_info() {
        let client = StripeClient::new("sk_test_12345").with_app_info(
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
        let client = StripeClient::new("sk_test_12345");
        let url = client.url("customers");
        assert_eq!(url.as_str(), "https://api.stripe.com/v1/customers");
    }

    #[test]
    fn test_url_with_leading_slash() {
        let client = StripeClient::new("sk_test_12345");
        let url = client.url("/customers");
        assert_eq!(url.as_str(), "https://api.stripe.com/v1/customers");
    }
}
