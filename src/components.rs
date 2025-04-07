use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::str;

/// Component file mapping for both extension and generated files
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ComponentFiles {
    #[serde(default)]
    pub extension_file: Option<String>,
    #[serde(default)]
    pub generated_files: Vec<String>,
}

/// JSON structure for components configuration
#[derive(Debug, Serialize, Deserialize)]
struct ComponentsConfig {
    components: HashMap<String, ComponentFiles>,
}

/// Load components configuration from JSON file
fn load_components_config() -> Result<ComponentsConfig> {
    // Try to find the components.json file in various locations
    let possible_paths = vec![
        "components.json".to_string(),
        "./components.json".to_string(),
        "../components.json".to_string(),
        concat!(env!("CARGO_MANIFEST_DIR"), "/components.json").to_string(),
    ];

    for path in possible_paths {
        if Path::new(&path).exists() {
            let content = fs::read_to_string(&path)?;
            let config: ComponentsConfig = serde_json::from_str(&content)?;
            return Ok(config);
        }
    }

    // Fallback to a minimal default configuration if file can't be found
    Ok(ComponentsConfig {
        components: HashMap::new(),
    })
}

/// List of supported resource components
pub fn supported_components() -> HashSet<String> {
    let mut components = HashSet::new();

    // Add components from the configuration
    if let Ok(config) = load_components_config() {
        for key in config.components.keys() {
            components.insert(key.clone());
        }
    }

    // Always include "all" option
    components.insert("all".to_string());

    components
}

/// Check if a component is valid
pub fn is_valid_component(component: &str) -> bool {
    supported_components().contains(component)
}

/// Get file mapping for a specific component
pub fn get_component_file_mapping(component: &str) -> Result<ComponentFiles> {
    if component == "all" {
        return Err(anyhow!("The 'all' component should be handled separately"));
    }

    let config = load_components_config()?;

    if let Some(mapping) = config.components.get(component) {
        return Ok(mapping.clone());
    }

    // Default fallback if component isn't in the config but is valid (shouldn't happen)
    let ext_file = format!("{}_ext", component);
    let base_file = component.to_string();

    Ok(ComponentFiles {
        extension_file: Some(ext_file),
        generated_files: vec![base_file],
    })
}

/// Generate the content for a specific extension file
pub fn generate_extension_file(component: &str, filename: &str) -> Result<String> {
    // First check if the file exists in the templates directory
    let template_path = format!("src/templates/resources/{}.rs", filename);

    if Path::new(&template_path).exists() {
        let content = fs::read_to_string(&template_path)?;
        return Ok(content);
    }

    // If no template exists, generate a default one
    Ok(format!(
        "//! Extension methods for the Stripe {} resource\n\nuse crate::stripe::resources::generated::{};\n\n// Extension methods would be defined here\n",
        component, component
    ))
}

/// Generate the content for a specific generated file
pub fn generate_generated_file(filename: &str) -> Result<String> {
    // First check if the file exists in the templates directory
    let template_path = format!("src/templates/resources/generated/{}.rs", filename);

    if Path::new(&template_path).exists() {
        let content = fs::read_to_string(&template_path)?;
        return Ok(content);
    }

    // If no template exists, generate a default one
    let resource_name = filename.replace('_', " ");

    Ok(format!(
        "//! Generated code for Stripe {} resource\n\n// Resource definition would be here\n",
        resource_name
    ))
}

/// Generate the content for resources/types.rs
pub fn generate_resource_types_file() -> Result<String> {
    let template_path = "src/templates/resources/types.rs";

    if Path::new(&template_path).exists() {
        let content = fs::read_to_string(template_path)?;
        return Ok(content);
    }

    Ok(
        "//! Common types used in Stripe API resources\n\n// Type definitions would be here\n"
            .to_string(),
    )
}

/// Generate the content for resources/generated.rs
pub fn generate_resource_generated_file() -> Result<String> {
    let template_path = "src/templates/resources/generated.rs";

    if Path::new(&template_path).exists() {
        let content = fs::read_to_string(template_path)?;
        return Ok(content);
    }

    Ok(
        "//! Re-exports all generated resource definitions\n\npub use super::generated::*;\n"
            .to_string(),
    )
}

/// Get a list of all available component templates
pub fn get_all_component_templates() -> Vec<String> {
    let mut templates = supported_components()
        .into_iter()
        .filter(|c| c != "all")
        .collect::<Vec<String>>();
    templates.sort();
    templates
}
