/// CLI banner
pub mod banner;
/// List of CLI commands
pub mod commands;

use banner::banner;
use clap::Parser;
use clap_verbosity_flag::{DebugLevel, Verbosity};

use crate::{cli::commands::Commands, constants::SUBSCAN_BANNER_LOG_TARGET, logger};

/// Data structure for CLI, stores configurations to be used on run-time
#[derive(Clone, Debug, Parser)]
#[command(version, about = banner(), long_about = banner())]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    #[command(flatten)]
    pub verbose: Verbosity<DebugLevel>,
}

impl Cli {
    pub async fn init(&self) {
        logger::init(Some(self.verbose.log_level_filter())).await;
    }

    pub async fn banner(&self) {
        log::debug!(target: SUBSCAN_BANNER_LOG_TARGET, "{}", banner());
    }
}
