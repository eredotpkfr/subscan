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
pub const DEFAULT_RESOLVER_TIMEOUT: Duration = Duration::from_millis(1000);
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
/// Port number pattern to parse resolver ports from file
pub const RL_PORT_PATTERN: &str = r#"(?<port>([1-9][0-9]{0,3}|[1-5][0-9]{4}|6[0-4][0-9]{3}|65[0-4][0-9]{2}|655[0-2][0-9]|6553[0-5]))"#;
/// IPv4 pattern to find IPv4 addresses from resolver list file. This regex
/// pattern is used to parse IPv4 addresses and perform basic validations.
/// It is used to make sure that IPv4 addresses are correctly written to
/// the resolver list file in valid format
pub const RL_IPV4_PATTERN: &str = r#"(?<ip>(\b25[0-5]|\b2[0-4][0-9]|\b[01]?[0-9][0-9]?)(\.(25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)){3})"#;
/// IPv6 pattern to find IPv6 addresses from resolver list file. This regex
/// pattern does not validate the IPv6 address, it only checks if it is
/// written to the file in the valid format and is used to parse the IPv6 address
pub const RL_IPV6_PATTERN: &str = r#"\[(?<ip>.+)\]"#;
