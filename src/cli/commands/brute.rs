use std::path::PathBuf;

use clap::Args;

use crate::{
    constants::{DEFAULT_RESOLVER_CONCURRENCY, DEFAULT_RESOLVER_TIMEOUT},
    enums::output::OutputFormat,
};

/// Brute force attack command arguments
#[derive(Args, Clone, Debug, Default)]
pub struct BruteCommandArgs {
    /// Target domain address for brute force attack
    #[arg(short, long)]
    pub domain: String,
    /// Wordlist file to be used during attack
    #[arg(short, long)]
    pub wordlist: PathBuf,
    /// If sets, output will be logged on stdout
    #[arg(short, long, default_value_t = false)]
    pub print: bool,
    /// Set output format
    #[arg(value_enum, short, long, default_value_t = OutputFormat::JSON)]
    pub output: OutputFormat,
    /// IP resolver timeout value as a milliseconds
    #[arg(long, default_value_t = DEFAULT_RESOLVER_TIMEOUT.as_millis() as u64)]
    pub resolver_timeout: u64,
    /// IP resolver concurrency level, thread counts of resolver instances
    #[arg(long, default_value_t = DEFAULT_RESOLVER_CONCURRENCY)]
    pub resolver_concurrency: u64,
}
