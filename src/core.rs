use anyhow::Result;

// Main files

/// Generate the content for lib.rs
pub fn generate_mod_rs() -> Result<&'static str> {
    Ok("//! Stripe API SDK for Rust\n\n\
        //! This module contains automatically generated Stripe API bindings.\n\n\
        pub mod client;
        pub mod error;
        pub mod ids;
        pub mod params;
        pub mod resources;

        pub use client::*;
        pub use error::*;
        pub use ids::*;
        pub use params::*;
        pub use resources::*;
")
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

/// Generate the content for resources/types.rs
pub fn generate_resource_types_file() -> Result<String> {
    let template_path = "src/templates/resources/types.rs";

    if std::path::Path::new(&template_path).exists() {
        let content = std::fs::read_to_string(template_path)?;
        return Ok(content);
    }

    Ok(
        "//! Common types used in Stripe API resources\n\n// Type definitions would be here\n"
            .to_string(),
    )
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

/// Generate the content for client/stripe_client.rs
pub fn generate_client_stripe_client_rs() -> Result<&'static str> {
    Ok(include_str!("templates/client/stripe_client.rs"))
}

/// Generate the content for client/http_client.rs
pub fn generate_client_http_client_rs() -> Result<&'static str> {
    Ok(include_str!("templates/client/http_client.rs"))
}
