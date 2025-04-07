mod request_strategy;
mod stripe;

mod base {
    pub mod tokio;
}

pub(crate) mod config {
    pub use super::base::tokio::{Response, TokioClient as BaseClient};
    pub(crate) use super::base::tokio::{err, ok};
}

pub use config::BaseClient;
/// An alias for `Result`.
///
/// If `blocking` is enabled, defined as:
///
/// ```rust,ignore
/// type Response<T> = Result<T, Error>;
/// ```
///
/// If the `async` feature is enabled, this type is defined as:
///
/// ```rust,ignore
/// type Response<T> = Box<dyn Future<Result<T, Error>>>;
/// ```
pub use config::Response;
pub use request_strategy::RequestStrategy;

pub use self::stripe::Client;
