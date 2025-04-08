use std::time::Duration;

use reqwest::StatusCode;

/// Defines different strategies for making API requests with retry logic
#[derive(Clone, Debug)]
pub enum RequestStrategy {
    /// Execute the request once with no retries
    Once,
    
    /// Execute the request once with a specified idempotency key
    Idempotent(String),
    
    /// Retry the request up to n times using a random idempotency key
    Retry(u32),
    
    /// Retry with exponential backoff up to n times using a random idempotency key
    ExponentialBackoff(u32),
}

impl RequestStrategy {
    /// Test whether to continue or stop retrying based on the current state
    pub fn test(
        &self,
        status: Option<StatusCode>,
        stripe_should_retry: Option<bool>,
        retry_count: u32,
    ) -> Outcome {
        // If Stripe explicitly says not to retry, then don't
        if !stripe_should_retry.unwrap_or(true) {
            return Outcome::Stop;
        }

        match (self, status, retry_count) {
            // A strategy of once or idempotent should run once
            (RequestStrategy::Once | RequestStrategy::Idempotent(_), _, 0) => Outcome::Continue(None),

            // Requests with client errors usually cannot be solved with retries
            (_, Some(c), _) if c.is_client_error() => Outcome::Stop,

            // Retry strategies should retry up to their max number of times
            (RequestStrategy::Retry(n), _, x) if x < *n => Outcome::Continue(None),
            (RequestStrategy::ExponentialBackoff(n), _, x) if x < *n => {
                Outcome::Continue(Some(calculate_backoff(x)))
            }

            // Unknown cases should be stopped to prevent infinite loops
            _ => Outcome::Stop,
        }
    }

    /// Get an idempotency key for this strategy, if applicable
    pub fn get_key(&self) -> Option<String> {
        match self {
            RequestStrategy::Once => None,
            RequestStrategy::Idempotent(key) => Some(key.clone()),
            #[cfg(feature = "uuid")]
            RequestStrategy::Retry(_) | RequestStrategy::ExponentialBackoff(_) => {
                Some(uuid::Uuid::new_v4().to_string())
            }
            #[cfg(not(feature = "uuid"))]
            RequestStrategy::Retry(_) | RequestStrategy::ExponentialBackoff(_) => None,
        }
    }

    /// Create a new idempotent strategy with a random UUID
    #[cfg(feature = "uuid")]
    pub fn idempotent_with_uuid() -> Self {
        use uuid::Uuid;
        Self::Idempotent(Uuid::new_v4().to_string())
    }
}

/// Calculate exponential backoff duration
fn calculate_backoff(retry_count: u32) -> Duration {
    Duration::from_secs(2_u64.saturating_pow(retry_count))
}

/// The outcome of testing a request strategy
#[derive(PartialEq, Eq, Debug)]
pub enum Outcome {
    /// Stop retrying
    Stop,
    
    /// Continue with optional delay
    Continue(Option<Duration>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_once_strategy() {
        let strategy = RequestStrategy::Once;
        assert_eq!(strategy.get_key(), None);
        assert_eq!(strategy.test(None, None, 0), Outcome::Continue(None));
        assert_eq!(strategy.test(None, None, 1), Outcome::Stop);
    }

    #[test]
    fn test_idempotent_strategy() {
        let strategy = RequestStrategy::Idempotent("key".to_string());
        assert_eq!(strategy.get_key(), Some("key".to_string()));
        assert_eq!(strategy.test(None, None, 0), Outcome::Continue(None));
        assert_eq!(strategy.test(None, None, 1), Outcome::Stop);
    }

    #[test]
    fn test_retry_strategy() {
        let strategy = RequestStrategy::Retry(3);
        assert_eq!(strategy.test(None, None, 0), Outcome::Continue(None));
        assert_eq!(strategy.test(None, None, 1), Outcome::Continue(None));
        assert_eq!(strategy.test(None, None, 2), Outcome::Continue(None));
        assert_eq!(strategy.test(None, None, 3), Outcome::Stop);
    }

    #[test]
    fn test_backoff_strategy() {
        let strategy = RequestStrategy::ExponentialBackoff(3);
        assert_eq!(strategy.test(None, None, 0), Outcome::Continue(Some(Duration::from_secs(1))));
        assert_eq!(strategy.test(None, None, 1), Outcome::Continue(Some(Duration::from_secs(2))));
        assert_eq!(strategy.test(None, None, 2), Outcome::Continue(Some(Duration::from_secs(4))));
        assert_eq!(strategy.test(None, None, 3), Outcome::Stop);
    }

    #[test]
    fn test_retry_header() {
        let strategy = RequestStrategy::Retry(3);
        assert_eq!(strategy.test(None, Some(false), 0), Outcome::Stop);
    }
}
