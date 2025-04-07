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

    // Generate and write core files
    write_core_files(&stripe_dir, force)?;

    Ok(format!(
        "Successfully initialized Stripe SDK in {}",
        stripe_dir.display()
    ))
}

/// Write all core SDK files to the project
fn write_core_files(stripe_dir: &Path, force: bool) -> Result<()> {
    // Create mod.rs - Main module file
    let mod_rs_content = core::generate_mod_rs()?;
    fs_utils::write_file(
        &stripe_dir.join("mod.rs"),
        &mod_rs_content,
        force,
        "stripe/mod.rs",
    )?;

    // Create client.rs - API client with authentication
    let client_rs_content = core::generate_client_rs()?;
    fs_utils::write_file(
        &stripe_dir.join("client.rs"),
        &client_rs_content,
        force,
        "stripe/client.rs",
    )?;

    // Create error.rs - Error handling
    let error_rs_content = core::generate_error_rs()?;
    fs_utils::write_file(
        &stripe_dir.join("error.rs"),
        &error_rs_content,
        force,
        "stripe/error.rs",
    )?;

    // Create types.rs - Common types
    let types_rs_content = core::generate_types_rs()?;
    fs_utils::write_file(
        &stripe_dir.join("types.rs"),
        &types_rs_content,
        force,
        "stripe/types.rs",
    )?;

    Ok(())
}
