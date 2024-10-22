/// Get command to fetch any module by name
pub mod get;
/// List command to list modules with details
pub mod list;
/// Run command to start any module
pub mod run;

use crate::cli::commands::module::{
    get::ModuleGetSubCommandArgs, list::ModuleListSubCommandArgs, run::ModuleRunSubCommandArgs,
};
use clap::{Args, Subcommand};

/// List of subcommands on module command
#[derive(Debug, Clone, Subcommand)]
pub enum ModuleSubCommands {
    /// Run a single module by name
    Run(ModuleRunSubCommandArgs),
    /// List all registered modules with their details
    List(ModuleListSubCommandArgs),
    /// Get a single module details
    Get(ModuleGetSubCommandArgs),
}

/// Module subcommand container
#[derive(Args, Clone, Debug)]
pub struct ModuleCommandArgs {
    /// Container for subcommands
    #[command(subcommand)]
    pub command: ModuleSubCommands,
}
