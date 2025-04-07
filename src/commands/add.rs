use anyhow::{Context, Result, anyhow};
use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};

use crate::components;
use crate::utils::fs as fs_utils;

/// Run the add command to add a Stripe API component
pub fn run(component: &str, target_dir: Option<&PathBuf>, force: bool) -> Result<String> {
    // Check for common misuse: using "generated" as component name
    if component == "generated" {
        return Err(anyhow!(
            "Cannot use 'generated' as a component name. If you're trying to add a component to a project, use: cargo stripe add <component_name>\nRun 'cargo stripe list' to see available components."
        ));
    }

    // Find the target project's src directory
    let src_dir = fs_utils::find_src_directory(target_dir.map(Path::new))
        .context("Could not find the src directory. Are you in a Rust project?")?;

    // Ensure the stripe directory exists
    let stripe_dir = src_dir.join("stripe");
    if !stripe_dir.exists() {
        println!("Stripe SDK not initialized. Initializing first...");
        // Create the basic structure required for add command to work
        fs::create_dir_all(&stripe_dir).context("Failed to create stripe directory")?;
        println!(
            "{} Created directory: {}",
            "✓".green(),
            stripe_dir.display()
        );

        // Create a basic lib.rs file if it doesn't exist
        let lib_path = stripe_dir.join("lib.rs");
        if !lib_path.exists() {
            fs::write(
                &lib_path,
                "//! Stripe API SDK for Rust\n\n\
                 //! This module contains automatically generated Stripe API bindings.\n\n",
            )
            .context("Failed to create lib.rs")?;
            println!("{} Created file: {}", "✓".green(), lib_path.display());
        }
    }

    // Create resources directory and its subdirectories if they don't exist
    let resources_dir = stripe_dir.join("resources");
    if !resources_dir.exists() {
        fs::create_dir_all(&resources_dir).context("Failed to create resources directory")?;
        println!(
            "{} Created directory: {}",
            "✓".green(),
            resources_dir.display()
        );
    }

    // Create the generated subdirectory for base resource definitions
    let generated_dir = resources_dir.join("generated");
    if !generated_dir.exists() {
        fs::create_dir_all(&generated_dir)
            .context("Failed to create resources/generated directory")?;
        println!(
            "{} Created directory: {}",
            "✓".green(),
            generated_dir.display()
        );
    }

    // Handle "all" component option
    if component == "all" {
        return add_all_components(&stripe_dir, &resources_dir, &generated_dir, force);
    }

    // Validate component name
    if !components::is_valid_component(component) {
        return Err(anyhow!(
            "Invalid component: '{}'. Run 'cargo stripe list' to see available components.",
            component
        ));
    }

    // Generate and write the component file(s)
    add_single_component(
        &stripe_dir,
        &resources_dir,
        &generated_dir,
        component,
        force,
    )?;

    Ok(format!("Successfully added {} component", component))
}

/// Add a single component including both the extension and generated files
fn add_single_component(
    stripe_dir: &Path,
    resources_dir: &Path,
    generated_dir: &Path,
    component: &str,
    force: bool,
) -> Result<()> {
    // Get the component mapping to determine which files to add
    let component_mapping = components::get_component_file_mapping(component)?;

    // Add the extension file if it exists
    if let Some(ext_file) = &component_mapping.extension_file {
        let ext_content = components::generate_extension_file(component, ext_file)?;
        let ext_path = resources_dir.join(ext_file);

        fs_utils::write_file(
            &ext_path,
            &ext_content,
            force,
            &format!("stripe/resources/{}.rs", ext_file),
        )?;

        println!("{} Added extension file: {}", "✓".green(), ext_file);
    }

    // Add all the generated files
    for gen_file in &component_mapping.generated_files {
        let gen_content = components::generate_generated_file(gen_file)?;
        let gen_path = generated_dir.join(gen_file);

        fs_utils::write_file(
            &gen_path,
            &gen_content,
            force,
            &format!("stripe/resources/generated/{}.rs", gen_file),
        )?;

        println!("{} Added generated file: {}", "✓".green(), gen_file);
    }

    // Make sure the resources directory is included in lib.rs
    update_mod_rs(stripe_dir, "resources")?;

    // Update the resources/mod.rs file to include this component
    update_resources_mod_rs(resources_dir, component, "generated")?;

    // Update the resources/generated/mod.rs file to include all generated files
    update_generated_mod_rs(generated_dir, &component_mapping.generated_files)?;

    Ok(())
}

/// Add all components
fn add_all_components(
    stripe_dir: &Path,
    resources_dir: &Path,
    generated_dir: &Path,
    force: bool,
) -> Result<String> {
    let templates = components::get_all_component_templates();
    let mut added_count = 0;

    println!("Adding all Stripe API components...");

    for component in &templates {
        match add_single_component(stripe_dir, resources_dir, generated_dir, component, force) {
            Ok(_) => {
                println!("{} Added component: {}", "✓".green(), component);
                added_count += 1;
            }
            Err(e) => {
                println!("{} Failed to add component {}: {}", "✗".red(), component, e);
            }
        }
    }

    // Also add the types.rs and generated.rs files
    if let Ok(types_content) = components::generate_resource_types_file() {
        let types_path = resources_dir.join("types.rs");
        fs_utils::write_file(
            &types_path,
            &types_content,
            force,
            "stripe/resources/types.rs",
        )?;
        println!("{} Added: types.rs", "✓".green());
    }

    if let Ok(gen_content) = components::generate_resource_generated_file() {
        let gen_path = resources_dir.join("generated.rs");
        fs_utils::write_file(
            &gen_path,
            &gen_content,
            force,
            "stripe/resources/generated.rs",
        )?;
        println!("{} Added: generated.rs", "✓".green());
    }

    Ok(format!(
        "Successfully added {} Stripe API components",
        added_count
    ))
}

/// Update the main mod.rs file to include the new module
fn update_mod_rs(stripe_dir: &Path, module: &str) -> Result<()> {
    let mod_path = stripe_dir.join("mod.rs");

    if !mod_path.exists() {
        return Err(anyhow!(
            "mod.rs not found. Run 'cargo stripe init' to create core files."
        ));
    }

    let mod_content = std::fs::read_to_string(&mod_path).context("Failed to read lib.rs")?;

    // Check if module is already included
    let module_mod_line = format!("pub mod {};", module);
    if mod_content.contains(&module_mod_line) {
        return Ok(());
    }

    // Find the best place to insert the new module declaration
    let updated_content = if let Some(pos) = mod_content.rfind("pub mod ") {
        if let Some(end_line) = mod_content[pos..].find(';') {
            let insert_pos = pos + end_line + 1;
            let (before, after) = mod_content.split_at(insert_pos);
            format!("{}\n{}{}", before, module_mod_line, after)
        } else {
            format!("{}\n{}", mod_content, module_mod_line)
        }
    } else {
        format!("{}\n{}", mod_content, module_mod_line)
    };

    std::fs::write(&mod_path, updated_content).context("Failed to update lib.rs")?;

    println!("{} Updated: {}", "✓".green(), mod_path.display());
    Ok(())
}

/// Update or create the resources/mod.rs file to include the component and generated module
fn update_resources_mod_rs(resources_dir: &Path, component: &str, submodule: &str) -> Result<()> {
    let mod_path = resources_dir.join("mod.rs");

    // If mod.rs doesn't exist, create it
    let mod_content = if mod_path.exists() {
        std::fs::read_to_string(&mod_path).context("Failed to read resources/mod.rs")?
    } else {
        "//! Stripe API resources\n\npub mod types;\npub mod generated;\n".to_string()
    };

    // Format component name for module declaration
    let component_module = if component.ends_with("_ext") {
        component.trim_end_matches("_ext")
    } else {
        component
    };

    // Check and add the component module
    let module_mod_line = format!("pub mod {};", component_module);
    let updated_content = if !mod_content.contains(&module_mod_line) {
        add_module_to_content(&mod_content, &module_mod_line)
    } else {
        mod_content.clone()
    };

    // Check and add the submodule if needed
    let submodule_mod_line = format!("pub mod {};", submodule);
    let final_content = if !updated_content.contains(&submodule_mod_line) {
        add_module_to_content(&updated_content, &submodule_mod_line)
    } else {
        updated_content
    };

    if final_content != mod_content {
        std::fs::write(&mod_path, final_content).context("Failed to update resources/mod.rs")?;
        println!("{} Updated: {}", "✓".green(), mod_path.display());
    }

    Ok(())
}

/// Update or create the resources/generated/mod.rs file to include all generated files
fn update_generated_mod_rs(generated_dir: &Path, generated_files: &[String]) -> Result<()> {
    let mod_path = generated_dir.join("mod.rs");

    // If mod.rs doesn't exist, create it
    let mod_content = if mod_path.exists() {
        std::fs::read_to_string(&mod_path).context("Failed to read resources/generated/mod.rs")?
    } else {
        "//! Generated Stripe API resource definitions\n\n".to_string()
    };

    let mut updated_content = mod_content.clone();

    // Add each generated file as a module
    for file in generated_files {
        let module_name = file.trim_end_matches(".rs");
        let module_mod_line = format!("pub mod {};", module_name);

        if !updated_content.contains(&module_mod_line) {
            updated_content = add_module_to_content(&updated_content, &module_mod_line);
        }
    }

    if updated_content != mod_content {
        std::fs::write(&mod_path, updated_content)
            .context("Failed to update resources/generated/mod.rs")?;
        println!("{} Updated: {}", "✓".green(), mod_path.display());
    }

    Ok(())
}

/// Helper function to add a module declaration to content
fn add_module_to_content(content: &str, module_line: &str) -> String {
    if let Some(pos) = content.rfind("pub mod ") {
        if let Some(end_line) = content[pos..].find(';') {
            let insert_pos = pos + end_line + 1;
            let (before, after) = content.split_at(insert_pos);
            format!("{}\n{}{}", before, module_line, after)
        } else {
            format!("{}\n{}", content, module_line)
        }
    } else {
        format!("{}\n{}", content, module_line)
    }
}
