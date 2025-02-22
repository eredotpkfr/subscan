/// Brute command to start brute force attack on any domain address
pub mod brute;
/// Module command to manage implemented modules
pub mod module;
/// Scan command to start scan on any domain address
pub mod scan;

use clap::Subcommand;

use crate::cli::commands::{
    brute::BruteCommandArgs, module::ModuleCommandArgs, scan::ScanCommandArgs,
};

/// List of CLI commands
#[derive(Clone, Debug, Subcommand)]
pub enum Commands {
    /// Start scan on any domain address
    Scan(ScanCommandArgs),
    /// Start brute force attack with a given wordlist
    Brute(BruteCommandArgs),
    /// Subcommand to manage implemented modules
    Module(ModuleCommandArgs),
}
