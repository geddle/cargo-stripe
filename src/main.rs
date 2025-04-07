use clap::{CommandFactory, Parser, Subcommand};
use colored::Colorize;
use std::path::PathBuf;
use std::process;

mod commands;
mod components;
mod core;
mod utils;

#[derive(Parser)]
#[clap(
    name = "cargo-stripe",
    bin_name = "cargo-stripe",
    version,
    about = "A CLI tool for adding Stripe API components to Rust projects",
    after_help = "Note: This tool is designed to be used as a cargo subcommand.\n\
                   Use as: cargo stripe <command> [options]"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize the Stripe SDK in your project
    Init {
        /// Target directory (defaults to current directory)
        #[clap(value_name = "DIR")]
        dir: Option<PathBuf>,

        /// Force overwriting existing files
        #[clap(short, long)]
        force: bool,
    },

    /// Add a Stripe API component to your project
    Add {
        /// Name of the component to add (e.g., 'customer', 'payment_intent', or 'all' for all components)
        #[clap(value_name = "COMPONENT")]
        component: String,

        /// Target directory (defaults to current directory)
        #[clap(value_name = "DIR")]
        dir: Option<PathBuf>,

        /// Force overwriting existing files
        #[clap(short, long)]
        force: bool,
    },

    /// List all available Stripe API components
    List,

    /// Display usage examples for this tool
    Examples,
}

#[tokio::main]
async fn main() {
    // When the binary is invoked as a cargo subcommand, the first argument will be "stripe".
    // We need to handle that specially to make the CLI work correctly.
    let args: Vec<String> = std::env::args().collect();
    let is_cargo_subcommand = args.len() > 1 && args[1] == "stripe";

    let cli = if is_cargo_subcommand {
        // Skip the "stripe" argument when parsing
        let args = std::env::args().take(1).chain(args.iter().skip(2).cloned());
        Cli::parse_from(args)
    } else {
        Cli::parse()
    };

    let result = match cli.command {
        Some(Commands::Init { dir, force }) => commands::init::run(dir.as_ref(), force),
        Some(Commands::Add {
            component,
            dir,
            force,
        }) => commands::add::run(&component, dir.as_ref(), force),
        Some(Commands::List) => {
            // Display all available components
            let components = components::get_all_component_templates();
            println!("{}", "Available Stripe API components:".bold());
            println!("These components include both extension files and generated resource definitions.\n");
            
            for component in &components {
                println!("  • {}", component);
            }
            
            println!("\n{}", "Special options:".bold());
            println!("  • all - Add all components at once (generates complete API)");
            
            println!("\n{}", "Usage:".bold());
            println!("  cargo stripe add <component>");
            return;
        },
        Some(Commands::Examples) => {
            println!("{}", "Usage Examples:".bold());
            println!("\n{}", "1. Initialize the Stripe SDK:".bold());
            println!("   cargo stripe init");
            
            println!("\n{}", "2. Add a specific component:".bold());
            println!("   cargo stripe add payment_intent");
            
            println!("\n{}", "3. Add all components:".bold());
            println!("   cargo stripe add all");
            
            println!("\n{}", "4. List available components:".bold());
            println!("   cargo stripe list");
            
            println!("\n{}", "Common errors:".bold());
            println!("   Using 'cargo run add ...' instead of 'cargo stripe add ...'");
            println!("   This is incorrect because the tool is designed to be used as a cargo subcommand.");
            return;
        },
        None => {
            // If no subcommand is provided, show help
            Cli::command().print_help().unwrap();
            println!("\n\nRun 'cargo stripe examples' for usage examples.");
            return;
        }
    };

    match result {
        Ok(msg) => println!("{}", msg.green()),
        Err(err) => {
            eprintln!("{}: {}", "Error".red().bold(), err);
            
            // Add helpful hints for common errors
            if err.to_string().contains("Stripe SDK not initialized") {
                eprintln!("\n{}: Run 'cargo stripe init' first, or ensure you're in the correct directory.", "Hint".yellow().bold());
            } else if err.to_string().contains("Invalid component") {
                eprintln!("\n{}: Run 'cargo stripe list' to see available components.", "Hint".yellow().bold());
            } else if err.to_string().contains("Could not find the src directory") {
                eprintln!("\n{}: Make sure you're in a Rust project or specify the target directory.", "Hint".yellow().bold());
            }
            
            eprintln!("\n{}: Run 'cargo stripe examples' for usage examples.", "Help".yellow().bold());
            process::exit(1);
        }
    }
}
