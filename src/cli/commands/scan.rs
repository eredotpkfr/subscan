use crate::{
    constants::{
        ASTERISK, DEFAULT_HTTP_TIMEOUT, DEFAULT_MODULE_CONCURRENCY, DEFAULT_RESOLVER_CONCURRENCY,
        DEFAULT_RESOLVER_TIMEOUT, DEFAULT_USER_AGENT,
    },
    enums::{cache::CacheFilter, output::OutputFormat},
};
use clap::Args;

/// Scan command arguments
#[derive(Args, Clone, Debug, Default)]
pub struct ScanCommandArgs {
    /// Target domain address to be enumerated
    #[arg(short, long)]
    pub domain: String,
    /// User-Agent header value for HTTP requests
    #[arg(short, long, default_value = DEFAULT_USER_AGENT)]
    pub user_agent: String,
    /// HTTP timeout value as a seconds
    #[arg(short = 't', long, default_value_t = DEFAULT_HTTP_TIMEOUT.as_secs())]
    pub http_timeout: u64,
    /// HTTP proxy
    #[arg(short, long, default_value = None)]
    pub proxy: Option<String>,
    /// Output format
    #[arg(value_enum, short, long, default_value_t = OutputFormat::JSON)]
    pub output: OutputFormat,
    /// Module runner concurrency level, thread counts of runner instances
    #[arg(short = 'c', long, default_value_t = DEFAULT_MODULE_CONCURRENCY)]
    pub module_concurrency: u64,
    /// IP resolver timeout value as a seconds
    #[arg(long, default_value_t = DEFAULT_RESOLVER_TIMEOUT.as_secs())]
    pub resolver_timeout: u64,
    /// IP resolver concurrency level, thread counts of resolver instances
    #[arg(long, default_value_t = DEFAULT_RESOLVER_CONCURRENCY)]
    pub resolver_concurrency: u64,
    /// Disable IP address resolve step
    #[arg(long = "disable-ip-resolve", default_value_t = false)]
    pub resolver_disabled: bool,
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
