use std::time::Duration;

/// `Subscan` environment variable namespace
pub const SUBSCAN_ENV_NAMESPACE: &str = "SUBSCAN";
/// Default HTTP timeout as a [`Duration`]
pub const DEFAULT_HTTP_TIMEOUT: Duration = Duration::from_secs(30);
