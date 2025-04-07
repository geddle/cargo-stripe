use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};

use crate::core;
use crate::utils::fs as fs_utils;

/// Run the init command to set up the base Stripe SDK files
pub fn run(target_dir: Option<&PathBuf>, force: bool) -> Result<String> {
    // If target directory is provided, ensure it exists and contains a Rust project
    if let Some(dir) = target_dir {
        fs_utils::ensure_project_exists(dir)?;
    }

    // Find the target project's src directory
    let src_dir = fs_utils::find_src_directory(target_dir.map(Path::new))
        .context("Could not find the src directory. Are you in a Rust project?")?;

    // Create the stripe directory if it doesn't exist
    let stripe_dir = src_dir.join("stripe");
    if !stripe_dir.exists() {
        fs::create_dir_all(&stripe_dir).context("Failed to create stripe directory")?;
        println!(
            "{} Created directory: {}",
            "✓".green(),
            stripe_dir.display()
        );
    }

    // Create the client directory
    let client_dir = stripe_dir.join("client");
    if !client_dir.exists() {
        fs::create_dir_all(&client_dir).context("Failed to create client directory")?;
        println!(
            "{} Created directory: {}",
            "✓".green(),
            client_dir.display()
        );
    }

    // Generate and write core files
    write_core_files(&stripe_dir, &client_dir, force)?;

    Ok(format!(
        "Successfully initialized Stripe SDK in {}",
        stripe_dir.display()
    ))
}

/// Write all core SDK files to the project
fn write_core_files(stripe_dir: &Path, client_dir: &Path, force: bool) -> Result<()> {
    // Create main files
    
    // Create lib.rs - Main library file
    let lib_rs_content = core::generate_lib_rs()?;
    fs_utils::write_file(
        &stripe_dir.join("lib.rs"),
        &lib_rs_content,
        force,
        "stripe/lib.rs",
    )?;
    
    // Create error.rs - Error handling
    let error_rs_content = core::generate_error_rs()?;
    fs_utils::write_file(
        &stripe_dir.join("error.rs"),
        &error_rs_content,
        force,
        "stripe/error.rs",
    )?;
    
    // Create ids.rs - ID types
    let ids_rs_content = core::generate_ids_rs()?;
    fs_utils::write_file(
        &stripe_dir.join("ids.rs"),
        &ids_rs_content,
        force,
        "stripe/ids.rs",
    )?;
    
    // Create params.rs - Parameter types
    let params_rs_content = core::generate_params_rs()?;
    fs_utils::write_file(
        &stripe_dir.join("params.rs"),
        &params_rs_content,
        force,
        "stripe/params.rs",
    )?;
    
    // Create resources.rs - Resource definitions
    let resources_rs_content = core::generate_resources_rs()?;
    fs_utils::write_file(
        &stripe_dir.join("resources.rs"),
        &resources_rs_content,
        force,
        "stripe/resources.rs",
    )?;
    
    // Create client files
    
    // Create client/mod.rs - Client module
    let client_mod_rs_content = core::generate_client_mod_rs()?;
    fs_utils::write_file(
        &client_dir.join("mod.rs"),
        &client_mod_rs_content,
        force,
        "stripe/client/mod.rs",
    )?;
    
    // Create client/request_strategy.rs - Request strategy
    let request_strategy_rs_content = core::generate_client_request_strategy_rs()?;
    fs_utils::write_file(
        &client_dir.join("request_strategy.rs"),
        &request_strategy_rs_content,
        force,
        "stripe/client/request_strategy.rs",
    )?;
    
    // Create client/stripe.rs - Stripe client
    let stripe_rs_content = core::generate_client_stripe_rs()?;
    fs_utils::write_file(
        &client_dir.join("stripe.rs"),
        &stripe_rs_content,
        force,
        "stripe/client/stripe.rs",
    )?;
    
    // Create client/tokio.rs - Tokio client
    let tokio_rs_content = core::generate_client_tokio_rs()?;
    fs_utils::write_file(
        &client_dir.join("tokio.rs"),
        &tokio_rs_content,
        force,
        "stripe/client/tokio.rs",
    )?;

    Ok(())
}
