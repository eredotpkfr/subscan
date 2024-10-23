/// CLI banner
pub mod banner;
/// List of CLI commands
pub mod commands;

use crate::cli::commands::Commands;
use banner::banner;
use clap::Parser;

/// Data structure for CLI, stores configurations to be used on run-time
#[derive(Clone, Debug, Parser)]
#[command(version, about = banner(), long_about = banner())]
pub struct Cli {
    /// Container for subcommands
    #[command(subcommand)]
    pub command: Commands,
}

impl Cli {
    pub async fn banner(&self) {
        println!("{}", banner());
    }
}
