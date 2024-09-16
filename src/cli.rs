use clap::Parser;

/// Data structure for CLI, stores configurations to be
/// used on run-time
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Target domain address to be enumerated
    #[arg(short, long)]
    pub domain: String,
    /// User-Agent header value for HTTP requests
    #[arg(
        short,
        long,
        default_value = "Mozilla/5.0 (Macintosh; \
            Intel Mac OS X 10_15_7) AppleWebKit/537.36  \
            (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36"
    )]
    pub user_agent: String,
    /// HTTP timeout value as a seconds
    #[arg(short, long, default_value_t = 10)]
    pub timeout: u64,
    /// HTTP proxy
    #[arg(short, long, default_value = None)]
    pub proxy: Option<String>,
}
