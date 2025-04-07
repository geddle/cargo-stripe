use anyhow::{Context, Result, anyhow};
use colored::Colorize;
use std::path::{Path, PathBuf};

use crate::components;
use crate::utils::fs as fs_utils;

/// Run the add command to add a Stripe API component
pub fn run(component: &str, target_dir: Option<&PathBuf>, force: bool) -> Result<String> {
    // Find the target project's src directory
    let src_dir = fs_utils::find_src_directory(target_dir.map(Path::new))
        .context("Could not find the src directory. Are you in a Rust project?")?;

    // Ensure the stripe directory exists
    let stripe_dir = src_dir.join("stripe");
    if !stripe_dir.exists() {
        return Err(anyhow!(
            "Stripe SDK not initialized. Run 'cargo stripe init' first."
        ));
    }

    // Validate component name
    if !components::is_valid_component(component) {
        return Err(anyhow!(
            "Invalid component: '{}'. Use 'customer', 'charge', etc.",
            component
        ));
    }

    // Generate and write the component file
    let component_path = stripe_dir.join(format!("{}.rs", component));
    let component_content = components::generate_component(component)?;

    fs_utils::write_file(
        &component_path,
        &component_content,
        force,
        &format!("stripe/{}.rs", component),
    )?;

    // Update mod.rs to include the new component
    update_mod_rs(&stripe_dir, component)?;

    Ok(format!("Successfully added {} component", component))
}

// The update_mod_rs function remains unchanged

/// Update the mod.rs file to include the new component
fn update_mod_rs(stripe_dir: &Path, component: &str) -> Result<()> {
    let mod_path = stripe_dir.join("mod.rs");

    if !mod_path.exists() {
        return Err(anyhow!(
            "mod.rs not found. Run 'cargo stripe init' to create core files."
        ));
    }

    let mod_content = std::fs::read_to_string(&mod_path).context("Failed to read mod.rs")?;

    // Check if component is already included
    let component_mod_line = format!("pub mod {};", component);
    if mod_content.contains(&component_mod_line) {
        return Ok(());
    }

    // Find the best place to insert the new module declaration
    let updated_content = if let Some(pos) = mod_content.rfind("pub mod ") {
        if let Some(end_line) = mod_content[pos..].find(';') {
            let insert_pos = pos + end_line + 1;
            let (before, after) = mod_content.split_at(insert_pos);
            format!("{}\n{}{}", before, component_mod_line, after)
        } else {
            format!("{}\n{}", mod_content, component_mod_line)
        }
    } else {
        format!("{}\n{}", mod_content, component_mod_line)
    };

    std::fs::write(&mod_path, updated_content).context("Failed to update mod.rs")?;

    println!("{} Updated: {}", "✓".green(), mod_path.display());
    Ok(())
}
