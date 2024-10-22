/// In-memory cache module to store all modules
pub mod cache;
/// Includes CLI components
pub mod cli;
/// Project configuration utils
pub mod config;
/// Enumerations and project type definitions
pub mod enums;
/// Data extractors like
/// [`extractors::regex`], [`extractors::html`], etc.
pub mod extractors;
/// Trait implementations
pub mod interfaces;
/// All modules listed under this module, core components for subscan
pub mod modules;
/// HTTP requesters listed under this module
/// like [`requesters::chrome`], [`requesters::client`], etc.
pub mod requesters;
/// Project core type definitions
pub mod types;
/// Utilities for the handle different stuff things
pub mod utils;

use crate::{cli::Cli, types::config::SubscanConfig};

/// Main `Subscan` object definition
#[derive(Default)]
pub struct Subscan {
    pub config: SubscanConfig,
}

impl From<Cli> for Subscan {
    fn from(cli: Cli) -> Self {
        Self { config: cli.into() }
    }
}

impl From<SubscanConfig> for Subscan {
    fn from(config: SubscanConfig) -> Self {
        Self { config }
    }
}

impl Subscan {
    pub fn new(config: SubscanConfig) -> Self {
        Self { config }
    }
}
