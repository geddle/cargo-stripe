use anyhow::Result;
use std::str;

// Main files

/// Generate the content for lib.rs
pub fn generate_lib_rs() -> Result<&'static str> {
    Ok(include_str!("templates/mod.rs"))
}

/// Generate the content for error.rs
pub fn generate_error_rs() -> Result<&'static str> {
    Ok(include_str!("templates/error.rs"))
}

/// Generate the content for ids.rs
pub fn generate_ids_rs() -> Result<&'static str> {
    Ok(include_str!("templates/ids.rs"))
}

/// Generate the content for params.rs
pub fn generate_params_rs() -> Result<&'static str> {
    Ok(include_str!("templates/params.rs"))
}

/// Generate the content for resources.rs
pub fn generate_resources_rs() -> Result<&'static str> {
    Ok(include_str!("templates/resources.rs"))
}

// Client files

/// Generate the content for client/mod.rs
pub fn generate_client_mod_rs() -> Result<&'static str> {
    Ok(include_str!("templates/client/mod.rs"))
}

/// Generate the content for client/request_strategy.rs
pub fn generate_client_request_strategy_rs() -> Result<&'static str> {
    Ok(include_str!("templates/client/request_strategy.rs"))
}

/// Generate the content for client/stripe.rs
pub fn generate_client_stripe_rs() -> Result<&'static str> {
    Ok(include_str!("templates/client/stripe.rs"))
}

/// Generate the content for client/tokio.rs
pub fn generate_client_tokio_rs() -> Result<&'static str> {
    Ok(include_str!("templates/client/tokio.rs"))
}
