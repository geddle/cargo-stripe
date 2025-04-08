use std::future::{self, Future};
use std::pin::Pin;

use crate::stripe::error::StripeError;

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
