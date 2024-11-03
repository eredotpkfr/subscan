use crate::{
    constants::{ASTERISK, DEFAULT_CONCURRENCY, DEFAULT_HTTP_TIMEOUT, DEFAULT_USER_AGENT},
    enums::{cache::CacheFilter, output::OutputFormat},
};
use clap::Args;

/// Scan command arguments
#[derive(Args, Clone, Debug, Default)]
pub struct ScanCommandArgs {
    /// Target domain address to be enumerated
    #[arg(short, long)]
    pub domain: String,
    /// Concurrency level, count of threads
    #[arg(short, long, default_value_t = DEFAULT_CONCURRENCY)]
    pub concurrency: u64,
    /// User-Agent header value for HTTP requests
    #[arg(short, long, default_value = DEFAULT_USER_AGENT)]
    pub user_agent: String,
    /// HTTP timeout value as a seconds
    #[arg(short, long, default_value_t = DEFAULT_HTTP_TIMEOUT.as_secs())]
    pub timeout: u64,
    /// HTTP proxy
    #[arg(short, long, default_value = None)]
    pub proxy: Option<String>,
    /// Output format
    #[arg(value_enum, short, long, default_value_t = OutputFormat::TXT)]
    pub output: OutputFormat,
    /// Comma separated list of modules to run
    #[arg(short, long, default_value = ASTERISK)]
    pub modules: String,
    /// Comma separated list of modules to skip
    #[arg(short, long, default_value = "")]
    pub skips: String,
}

impl ScanCommandArgs {
    pub fn filter(&self) -> CacheFilter {
        let filter_empty = |module: &str| {
            if !module.trim().is_empty() {
                Some(module.trim().to_lowercase())
            } else {
                None
            }
        };

        let split = self.modules.trim().split(",");
        let valids = split.filter_map(filter_empty).collect();

        let split = self.skips.trim().split(",");
        let invalids = split.filter_map(filter_empty).collect();

        if self.modules == ASTERISK && self.skips.is_empty() {
            CacheFilter::NoFilter
        } else if self.modules == ASTERISK && !self.skips.is_empty() {
            CacheFilter::FilterByName((vec![], invalids).into())
        } else {
            CacheFilter::FilterByName((valids, invalids).into())
        }
    }
}
