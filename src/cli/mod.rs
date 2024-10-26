/// CLI banner
pub mod banner;
/// List of CLI commands
pub mod commands;

use crate::{cli::commands::Commands, config::SUBSCAN_BANNER_LOG_TARGET, logger};
use banner::banner;
use clap::Parser;
use clap_verbosity_flag::{InfoLevel, Verbosity};

/// Data structure for CLI, stores configurations to be used on run-time
#[derive(Clone, Debug, Parser)]
#[command(version, about = banner(), long_about = banner())]
pub struct Cli {
    /// Container for subcommands
    #[command(subcommand)]
    pub command: Commands,
    #[command(flatten)]
    verbose: Verbosity<InfoLevel>,
}

impl Cli {
    pub async fn init(&self) {
        logger::init(Some(self.verbose.log_level_filter())).await;
    }

    pub async fn banner(&self) {
        log::info!(target: SUBSCAN_BANNER_LOG_TARGET, "{}", banner());
    }
}
