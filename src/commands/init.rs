use anyhow::{Context, Result};
use colored::Colorize;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::core;
use crate::utils::fs as fs_utils;

/// Run the init command to set up the base Stripe SDK files
pub fn run(target_dir: Option<&PathBuf>, force: bool) -> Result<String> {
    // If target directory is provided, ensure it exists and contains a Rust project
    if let Some(dir) = target_dir {
        fs_utils::ensure_project_exists(dir)?;
    }

    // Find the target project's root directory
    let root_dir = fs_utils::find_project_root(target_dir.map(Path::new))
        .context("Could not find the project root. Are you in a Rust project?")?;

    // Find the target project's src directory
    let src_dir = root_dir.join("src");
    if !src_dir.exists() {
        return Err(anyhow::anyhow!(
            "Could not find the src directory. Are you in a Rust project?"
        ));
    }

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

    // Create the resources directory
    let resources_dir = stripe_dir.join("resources");
    if !resources_dir.exists() {
        fs::create_dir_all(&resources_dir).context("Failed to create resources directory")?;
        println!(
            "{} Created directory: {}",
            "✓".green(),
            resources_dir.display()
        );
    }

    // Generate and write core files
    write_core_files(&stripe_dir, &client_dir, &resources_dir, force)?;

    // Add the required dependencies to Cargo.toml
    add_dependencies(&root_dir)?;

    Ok(format!(
        "Successfully initialized Stripe SDK in {}",
        stripe_dir.display()
    ))
}

/// Write all core SDK files to the project
fn write_core_files(
    stripe_dir: &Path,
    client_dir: &Path,
    resources_dir: &Path,
    force: bool,
) -> Result<()> {
    // Create main files
    let lib_rs_content = core::generate_mod_rs()?;
    fs_utils::write_file(
        &stripe_dir.join("mod.rs"),
        lib_rs_content,
        force,
        "stripe/mod.rs",
    )?;

    // Create error.rs - Error handling
    let error_rs_content = core::generate_error_rs()?;
    fs_utils::write_file(
        &stripe_dir.join("error.rs"),
        error_rs_content,
        force,
        "stripe/error.rs",
    )?;

    // Create ids.rs - ID types
    let ids_rs_content = core::generate_ids_rs()?;
    fs_utils::write_file(
        &stripe_dir.join("ids.rs"),
        ids_rs_content,
        force,
        "stripe/ids.rs",
    )?;

    // Create params.rs - Parameter types
    let params_rs_content = core::generate_params_rs()?;
    fs_utils::write_file(
        &stripe_dir.join("params.rs"),
        params_rs_content,
        force,
        "stripe/params.rs",
    )?;

    // Create resources/types.rs - Common types
    if let Ok(types_content) = core::generate_resource_types_file() {
        fs_utils::write_file(
            &resources_dir.join("types.rs"),
            &types_content,
            force,
            "stripe/resources/types.rs",
        )?;
    }

    // Create resources/mod.rs - Initial module declarations
    let resources_mod_content = "//! Stripe API resources\n\npub mod types;\npub use types::*;";
    fs_utils::write_file(
        &resources_dir.join("mod.rs"),
        resources_mod_content,
        force,
        "stripe/resources/mod.rs",
    )?;

    // Create client files

    // Create client/mod.rs - Client module
    let client_mod_rs_content = core::generate_client_mod_rs()?;
    fs_utils::write_file(
        &client_dir.join("mod.rs"),
        client_mod_rs_content,
        force,
        "stripe/client/mod.rs",
    )?;

    // Create client/request_strategy.rs - Request strategy
    let request_strategy_rs_content = core::generate_client_request_strategy_rs()?;
    fs_utils::write_file(
        &client_dir.join("request_strategy.rs"),
        request_strategy_rs_content,
        force,
        "stripe/client/request_strategy.rs",
    )?;

    // Create client/stripe_client.rs - Stripe client
    let stripe_rs_content = core::generate_client_stripe_client_rs()?;
    fs_utils::write_file(
        &client_dir.join("stripe_client.rs"),
        stripe_rs_content,
        force,
        "stripe/client/stripe_client.rs",
    )?;

    // Create client/http_client.rs - Http client
    let httpclient_rs_content = core::generate_client_http_client_rs()?;
    fs_utils::write_file(
        &client_dir.join("http_client.rs"),
        httpclient_rs_content,
        force,
        "stripe/client/http_client.rs",
    )?;

    Ok(())
}

/// Add the required dependencies to the project's Cargo.toml
fn add_dependencies(root_dir: &Path) -> Result<()> {
    let cargo_toml_path = root_dir.join("Cargo.toml");
    if !cargo_toml_path.exists() {
        return Err(anyhow::anyhow!(
            "Could not find Cargo.toml. Are you in a Rust project?"
        ));
    }

    // Read the current Cargo.toml
    let mut cargo_toml_content = String::new();
    fs::File::open(&cargo_toml_path)?.read_to_string(&mut cargo_toml_content)?;

    // Parse the current Cargo.toml
    let mut cargo_toml: toml::Value =
        toml::from_str(&cargo_toml_content).context("Failed to parse Cargo.toml")?;

    // Define the required dependencies
    let dependencies = vec![
        ("tokio", "1.28", Some(vec!["rt-multi-thread", "macros"])),
        ("reqwest", "0.11", Some(vec!["json", "rustls-tls"])),
        ("serde", "1.0", Some(vec!["derive"])),
        ("serde_json", "1.0", None),
        ("thiserror", "1.0", None),
        ("smart-default", "0.7", None),
        ("http-types", "2.12", None),
        ("http", "1.3", None),
        ("hyper", "1.6", None),
        ("hyper-util", "0.1", None),
        ("smol_str", "0.3", None),
        ("futures-util", "0.3", None),
        ("hyper-rustls", "0.27", None),
        ("serde_path_to_error", "0.1", None),
        ("serde_qs", "0.14", None),
    ];

    // Get or create dependencies table
    let dependencies_table = cargo_toml
        .as_table_mut()
        .and_then(|t| {
            if !t.contains_key("dependencies") {
                t.insert(
                    "dependencies".to_string(),
                    toml::Value::Table(toml::value::Table::new()),
                );
            }
            t.get_mut("dependencies")
        })
        .and_then(toml::Value::as_table_mut)
        .ok_or_else(|| anyhow::anyhow!("Failed to access dependencies in Cargo.toml"))?;

    // Add dependencies if they don't exist or update them
    let mut added_count = 0;
    for (name, version, features) in dependencies {
        if !dependencies_table.contains_key(name) {
            // Add the dependency
            if let Some(feature_list) = features {
                let mut dep_table = toml::value::Table::new();
                dep_table.insert(
                    "version".to_string(),
                    toml::Value::String(version.to_string()),
                );

                // Create features array
                let features_array = feature_list
                    .into_iter()
                    .map(|f| toml::Value::String(f.to_string()))
                    .collect::<Vec<_>>();

                dep_table.insert("features".to_string(), toml::Value::Array(features_array));

                dependencies_table.insert(name.to_string(), toml::Value::Table(dep_table));
            } else {
                // Simple version dependency
                dependencies_table
                    .insert(name.to_string(), toml::Value::String(version.to_string()));
            }

            added_count += 1;
            println!("{} Added dependency: {}", "✓".green(), name);
        }
    }

    // Write the updated Cargo.toml
    if added_count > 0 {
        let updated_content =
            toml::to_string(&cargo_toml).context("Failed to serialize updated Cargo.toml")?;

        fs::write(&cargo_toml_path, updated_content)
            .context("Failed to write updated Cargo.toml")?;

        println!(
            "{} Updated Cargo.toml with required dependencies",
            "✓".green()
        );
    } else {
        println!(
            "{} All required dependencies already present in Cargo.toml",
            "✓".green()
        );
    }

    Ok(())
}
