use std::path::PathBuf;

use clap::Args;

use crate::{
    constants::{
        DEFAULT_HTTP_TIMEOUT, DEFAULT_RESOLVER_CONCURRENCY, DEFAULT_RESOLVER_TIMEOUT,
        DEFAULT_USER_AGENT,
    },
    enums::output::OutputFormat,
};

/// Run command to start any module
#[derive(Args, Clone, Debug, Default)]
pub struct ModuleRunSubCommandArgs {
    /// Module name to be run
    pub name: String,
    /// Target domain address to be enumerated
    #[arg(short, long)]
    pub domain: String,
    /// Set output format
    #[arg(value_enum, short, long, default_value_t = OutputFormat::JSON)]
    pub output: OutputFormat,
    /// If sets, output will be logged on stdout
    #[arg(long, default_value_t = false)]
    pub print: bool,
    /// Set User-Agent header value for HTTP requests
    #[arg(short, long, default_value = DEFAULT_USER_AGENT)]
    pub user_agent: String,
    /// HTTP timeout value as a seconds
    #[arg(short = 't', long, default_value_t = DEFAULT_HTTP_TIMEOUT.as_secs())]
    pub http_timeout: u64,
    /// Set HTTP proxy
    #[arg(short, long, default_value = None)]
    pub proxy: Option<String>,
    /// IP resolver timeout value as a milliseconds
    #[arg(long, default_value_t = DEFAULT_RESOLVER_TIMEOUT.as_millis() as u64)]
    pub resolver_timeout: u64,
    /// IP resolver concurrency level, thread counts of resolver instances
    #[arg(long, default_value_t = DEFAULT_RESOLVER_CONCURRENCY)]
    pub resolver_concurrency: u64,
    /// Disable IP address resolve process
    #[arg(long = "disable-ip-resolve", default_value_t = false)]
    pub resolver_disabled: bool,
    /// A text file containing list of resolvers to use for enumeration.
    /// See `resolverlist.template`
    #[arg(long, default_value = None)]
    pub resolver_list: Option<PathBuf>,
}
