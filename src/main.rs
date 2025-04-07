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
    about = "A CLI tool for adding Stripe API components to Rust projects"
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
        /// Name of the component to add
        component: String,

        /// Target directory (defaults to current directory)
        #[clap(value_name = "DIR")]
        dir: Option<PathBuf>,

        /// Force overwriting existing files
        #[clap(short, long)]
        force: bool,
    },
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
        None => {
            // If no subcommand is provided, show help
            Cli::command().print_help().unwrap();
            return;
        }
    };

    match result {
        Ok(msg) => println!("{}", msg.green()),
        Err(err) => {
            eprintln!("{}: {}", "Error".red().bold(), err);
            process::exit(1);
        }
    }
}
