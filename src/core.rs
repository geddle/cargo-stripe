use anyhow::Result;
use std::str;

/// Generate the content for mod.rs
pub fn generate_mod_rs() -> Result<String> {
    Ok(str::from_utf8(include_bytes!("templates/core/mod.rs"))?.to_string())
}

/// Generate the content for client.rs
pub fn generate_client_rs() -> Result<String> {
    Ok(str::from_utf8(include_bytes!("templates/core/client.rs"))?.to_string())
}

/// Generate the content for error.rs
pub fn generate_error_rs() -> Result<String> {
    Ok(str::from_utf8(include_bytes!("templates/core/error.rs"))?.to_string())
}

/// Generate the content for types.rs
pub fn generate_types_rs() -> Result<String> {
    Ok(str::from_utf8(include_bytes!("templates/core/types.rs"))?.to_string())
}
