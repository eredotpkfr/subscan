use crate::{config::DEFAULT_CONCURRENCY, enums::output::OutputFormat};
use clap::Args;

/// Brute force attack command arguments
#[derive(Args, Clone, Debug)]
pub struct BruteCommandArgs {
    /// Target domain address to be enumerated
    #[arg(short, long)]
    pub domain: String,
    /// Concurrency level, count of threads (default: [`DEFAULT_CONCURRENCY`])
    #[arg(short, long, default_value_t = DEFAULT_CONCURRENCY)]
    pub concurrency: u64,
    /// Output format
    #[arg(value_enum, short, long, default_value_t = OutputFormat::TXT)]
    pub output: OutputFormat,
}
