use crate::constants::{DEFAULT_HTTP_TIMEOUT, DEFAULT_USER_AGENT};
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
    #[arg(short, long, default_value_t = DEFAULT_HTTP_TIMEOUT.as_secs())]
    pub timeout: u64,
    /// HTTP proxy
    #[arg(short, long, default_value = None)]
    pub proxy: Option<String>,
}
