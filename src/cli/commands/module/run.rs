use crate::{
    config::{DEFAULT_HTTP_TIMEOUT, DEFAULT_USER_AGENT},
    enums::output::OutputFormat,
};
use clap::Args;

/// Run command to start any module
#[derive(Args, Clone, Debug)]
pub struct ModuleRunSubCommandArgs {
    /// Module name to be run
    pub name: String,
    /// Target domain address to be enumerated
    #[arg(short, long)]
    pub domain: String,
    /// User-Agent header value for HTTP requests (default: [`DEFAULT_USER_AGENT`])
    #[arg(short, long, default_value = DEFAULT_USER_AGENT)]
    pub user_agent: String,
    /// HTTP timeout value as a seconds (default: [`DEFAULT_HTTP_TIMEOUT`])
    #[arg(short, long, default_value_t = DEFAULT_HTTP_TIMEOUT.as_secs())]
    pub timeout: u64,
    /// HTTP proxy
    #[arg(short, long, default_value = None)]
    pub proxy: Option<String>,
    /// Output format
    #[arg(value_enum, short, long, default_value_t = OutputFormat::TXT)]
    pub output: OutputFormat,
}
