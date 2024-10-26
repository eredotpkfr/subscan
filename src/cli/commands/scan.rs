use crate::config::{DEFAULT_CONCURRENCY, DEFAULT_HTTP_TIMEOUT, DEFAULT_USER_AGENT};
use clap::Args;

/// Scan command arguments
#[derive(Args, Clone, Debug)]
pub struct ScanCommandArgs {
    /// Target domain address to be enumerated
    #[arg(short, long)]
    pub domain: String,
    /// Concurrency level, count of threads (default: [`DEFAULT_CONCURRENCY`])
    #[arg(short, long, default_value_t = DEFAULT_CONCURRENCY)]
    pub concurrency: u64,
    /// User-Agent header value for HTTP requests (default: [`DEFAULT_USER_AGENT`])
    #[arg(short, long, default_value = DEFAULT_USER_AGENT)]
    pub user_agent: String,
    /// HTTP timeout value as a seconds (default: [`DEFAULT_HTTP_TIMEOUT`])
    #[arg(short, long, default_value_t = DEFAULT_HTTP_TIMEOUT.as_secs())]
    pub timeout: u64,
    /// HTTP proxy
    #[arg(short, long, default_value = None)]
    pub proxy: Option<String>,
}
