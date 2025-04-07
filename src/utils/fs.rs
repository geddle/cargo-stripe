use anyhow::{Context, Result, anyhow};
use colored::Colorize;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Find the project root directory (where Cargo.toml is located)
pub fn find_project_root(target_dir: Option<&Path>) -> Result<PathBuf> {
    let current_dir = match target_dir {
        Some(dir) => dir.to_path_buf(),
        None => env::current_dir()?,
    };

    // Check if we're in a Rust project root (contains Cargo.toml)
    let cargo_path = current_dir.join("Cargo.toml");
    if cargo_path.exists() {
        return Ok(current_dir);
    }

    // Check if we're in a src directory
    if current_dir.ends_with("src") {
        if let Some(parent) = current_dir.parent() {
            if parent.join("Cargo.toml").exists() {
                return Ok(parent.to_path_buf());
            }
        }
    }

    // Check if we're in a subdirectory of a Rust project
    let mut path = current_dir.clone();
    while let Some(parent) = path.parent() {
        if parent.join("Cargo.toml").exists() {
            return Ok(parent.to_path_buf());
        }
        path = parent.to_path_buf();
    }

    Err(anyhow!(
        "Not in a Rust project. Make sure you're in a directory with a Cargo.toml file."
    ))
}

/// Find the src directory of the target project
pub fn find_src_directory(target_dir: Option<&Path>) -> Result<PathBuf> {
    let project_root = find_project_root(target_dir)?;
    let src_path = project_root.join("src");
    
    if src_path.exists() && src_path.is_dir() {
        return Ok(src_path);
    }
    
    Err(anyhow!(
        "The project doesn't have a src directory. Make sure it's a valid Rust project."
    ))
}

/// Write content to a file, asking for confirmation if the file exists and force is false
pub fn write_file<T: AsRef<[u8]>>(path: &Path, content: T, force: bool, relative_path: &str) -> Result<()> {
    if path.exists() && !force {
        let response = prompt_yes_no(&format!(
            "The file {} already exists. Overwrite?",
            relative_path
        ))?;

        if !response {
            println!("{} Skipped {}", "→".yellow(), relative_path);
            return Ok(());
        }
    }

    fs::write(path, content)
        .with_context(|| format!("Failed to write file: {}", path.display()))?;

    println!("{} Written: {}", "✓".green(), relative_path);
    Ok(())
}

/// Prompt the user for a yes/no response
pub fn prompt_yes_no(question: &str) -> Result<bool> {
    print!("{} {} [y/N] ", "?".blue(), question);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input.trim().to_lowercase() == "y")
}

/// Create a new Rust project in the target directory if it doesn't exist
pub fn ensure_project_exists(target_dir: &Path) -> Result<()> {
    // If the directory doesn't exist, create it
    if !target_dir.exists() {
        fs::create_dir_all(target_dir)?;
        println!(
            "{} Created directory: {}",
            "✓".green(),
            target_dir.display()
        );

        // Run cargo init to create a new Rust project
        let status = std::process::Command::new("cargo")
            .arg("init")
            .arg(target_dir)
            .status()
            .context(
                "Failed to run 'cargo init'. Make sure cargo is installed and in your PATH.",
            )?;

        if !status.success() {
            return Err(anyhow!(
                "Failed to initialize Rust project in {}",
                target_dir.display()
            ));
        }

        println!("{} Initialized new Rust project", "✓".green());
    }

    // Ensure Cargo.toml exists
    if !target_dir.join("Cargo.toml").exists() {
        return Err(anyhow!(
            "Target directory is not a Rust project. Missing Cargo.toml."
        ));
    }

    Ok(())
}
