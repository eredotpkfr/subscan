/// List of CLI commands
pub mod commands;

use crate::cli::commands::Commands;
use clap::Parser;

/// Data structure for CLI, stores configurations to be used on run-time
#[derive(Clone, Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Container for subcommands
    #[command(subcommand)]
    pub command: Commands,
}
