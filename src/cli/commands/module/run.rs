use crate::constants::{
    DEFAULT_HTTP_TIMEOUT, DEFAULT_RESOLVER_CONCURRENCY, DEFAULT_RESOLVER_TIMEOUT,
    DEFAULT_USER_AGENT,
};
use clap::Args;

/// Run command to start any module
#[derive(Args, Clone, Debug, Default)]
pub struct ModuleRunSubCommandArgs {
    /// Module name to be run
    pub name: String,
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
    /// IP resolver concurrency level, thread counts of resolver instances
    #[arg(long, default_value_t = DEFAULT_RESOLVER_TIMEOUT.as_secs())]
    pub resolver_timeout: u64,
    /// Thread count of IP resolver instances
    #[arg(long, default_value_t = DEFAULT_RESOLVER_CONCURRENCY)]
    pub resolver_concurrency: u64,
    /// Disable IP address resolve step
    #[arg(long = "disable-ip-resolve", default_value_t = false)]
    pub resolver_disabled: bool,
}
