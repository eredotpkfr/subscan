use std::time::Duration;

/// `Subscan` banner module path
pub const SUBSCAN_BANNER_LOG_TARGET: &str = "subscan::banner";
/// `Subscan` environment variable namespace
pub const SUBSCAN_ENV_NAMESPACE: &str = "SUBSCAN";
/// `Subscan` Chrome browser executable path env
pub const SUBSCAN_CHROME_PATH_ENV: &str = "SUBSCAN_CHROME_PATH";
/// Concurrency level of module runner instances, count of threads
pub const DEFAULT_MODULE_CONCURRENCY: u64 = 5;
/// Concurrency level of resolver instances, count of threads
pub const DEFAULT_RESOLVER_CONCURRENCY: u64 = 100;
/// Default HTTP timeout as a [`Duration`]
pub const DEFAULT_HTTP_TIMEOUT: Duration = Duration::from_secs(30);
/// Default DNS resolver timeout as a [`Duration`]
pub const DEFAULT_RESOLVER_TIMEOUT: Duration = Duration::from_millis(10);
/// Default User-Agent headers for HTTP requests
pub const DEFAULT_USER_AGENT: &str = "\
    Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) \
    AppleWebKit/537.36 (KHTML, like Gecko) \
    Chrome/135.0.0.0 Safari/537.36\
";
/// Asterisk character to indicate all modules
pub const ASTERISK: &str = "*";
/// Time logging format
pub const LOG_TIME_FORMAT: &str = "%H:%M:%S %Z";
