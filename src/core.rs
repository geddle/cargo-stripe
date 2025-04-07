use anyhow::Result;
use std::str;

// Main files

/// Generate the content for lib.rs
pub fn generate_lib_rs() -> Result<String> {
    Ok(str::from_utf8(include_bytes!("templates/lib.rs"))?.to_string())
}

/// Generate the content for error.rs
pub fn generate_error_rs() -> Result<String> {
    Ok(str::from_utf8(include_bytes!("templates/error.rs"))?.to_string())
}

/// Generate the content for ids.rs
pub fn generate_ids_rs() -> Result<String> {
    Ok(str::from_utf8(include_bytes!("templates/ids.rs"))?.to_string())
}

/// Generate the content for params.rs
pub fn generate_params_rs() -> Result<String> {
    Ok(str::from_utf8(include_bytes!("templates/params.rs"))?.to_string())
}

/// Generate the content for resources.rs
pub fn generate_resources_rs() -> Result<String> {
    Ok(str::from_utf8(include_bytes!("templates/resources.rs"))?.to_string())
}

// Client files

/// Generate the content for client/mod.rs
pub fn generate_client_mod_rs() -> Result<String> {
    Ok(str::from_utf8(include_bytes!("templates/client/mod.rs"))?.to_string())
}

/// Generate the content for client/request_strategy.rs
pub fn generate_client_request_strategy_rs() -> Result<String> {
    Ok(str::from_utf8(include_bytes!("templates/client/request_strategy.rs"))?.to_string())
}

/// Generate the content for client/stripe.rs
pub fn generate_client_stripe_rs() -> Result<String> {
    Ok(str::from_utf8(include_bytes!("templates/client/stripe.rs"))?.to_string())
}

/// Generate the content for client/tokio.rs
pub fn generate_client_tokio_rs() -> Result<String> {
    Ok(str::from_utf8(include_bytes!("templates/client/tokio.rs"))?.to_string())
}
