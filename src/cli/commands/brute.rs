use crate::{constants::DEFAULT_CONCURRENCY, enums::output::OutputFormat};
use clap::Args;
use std::path::PathBuf;

/// Brute force attack command arguments
#[derive(Args, Clone, Debug, Default)]
pub struct BruteCommandArgs {
    /// Target domain address to be enumerated
    #[arg(short, long)]
    pub domain: String,
    /// Concurrency level, count of threads
    #[arg(short, long, default_value_t = DEFAULT_CONCURRENCY)]
    pub concurrency: u64,
    /// Output format
    #[arg(value_enum, short, long, default_value_t = OutputFormat::JSON)]
    pub output: OutputFormat,
    /// Wordlist file path
    #[arg(short, long)]
    pub wordlist: PathBuf,
}
