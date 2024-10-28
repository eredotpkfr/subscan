use crate::config::SUBSCAN_ENV_NAMESPACE;

/// Formats given module name and environment variable name with [`SUBSCAN_ENV_NAMESPACE`]
/// prefix, returns fully generated environment variable name
///
/// # Examples
///
/// ```
/// use subscan::utils::env::format_env;
///
/// #[tokio::main]
/// async fn main() {
///     assert_eq!(format_env("foo", "apikey"), "SUBSCAN_FOO_APIKEY");
///     assert_eq!(format_env("foo", "username"), "SUBSCAN_FOO_USERNAME");
///     assert_eq!(format_env("bar", "password"), "SUBSCAN_BAR_PASSWORD");
/// }
/// ```
pub fn format_env(name: &str, env: &str) -> String {
    format!(
        "{}_{}_{}",
        SUBSCAN_ENV_NAMESPACE,
        name.to_uppercase(),
        env.to_uppercase(),
    )
}
