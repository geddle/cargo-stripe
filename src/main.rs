use clap::{Parser, Subcommand};
use colored::Colorize;
use std::process;

mod commands;
mod components;
mod core;
mod utils;

#[derive(Parser)]
#[clap(bin_name = "cargo")]
#[clap(
    version,
    about = "A CLI tool for adding Stripe API components to Rust projects"
)]
enum Cargo {
    #[clap(subcommand)]
    Stripe(StripeCommands),
}

#[derive(Subcommand)]
enum StripeCommands {
    /// Initialize the Stripe SDK in your project
    Init {
        /// Force overwriting existing files
        #[clap(short, long)]
        force: bool,
    },

    /// Add a Stripe API component to your project
    Add {
        /// Name of the component to add
        component: String,

        /// Force overwriting existing files
        #[clap(short, long)]
        force: bool,
    },
}

#[tokio::main]
async fn main() {
    let Cargo::Stripe(cmd) = Cargo::parse();

    let result = match cmd {
        StripeCommands::Init { force } => commands::init::run(force),
        StripeCommands::Add { component, force } => commands::add::run(&component, force),
    };

    match result {
        Ok(msg) => println!("{}", msg.green()),
        Err(err) => {
            eprintln!("{}: {}", "Error".red().bold(), err);
            process::exit(1);
        }
    }
}
