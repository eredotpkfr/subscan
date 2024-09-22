/// Helpful regex utilities listed in this module
pub mod regex {
    use core::result::Result;
    use regex::{Error, Regex};

    /// Helper function that generates dynamically regex statement
    /// by given domain address to parse subdomains
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::utils::regex::generate_subdomain_regex;
    ///
    /// let domain = String::from("foo.com");
    /// let subdomain = String::from("bar.foo.com");
    /// let no_match = String::from("foo");
    ///
    /// let regex_stmt = generate_subdomain_regex(domain).unwrap();
    ///
    /// assert_eq!(regex_stmt.as_str(), "(?:[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?\\.)+(foo\\.com)");
    ///
    /// assert!(regex_stmt.find(&subdomain).is_some());
    /// assert!(regex_stmt.find(&no_match).is_none());
    /// ```
    pub fn generate_subdomain_regex(domain: String) -> Result<Regex, Error> {
        let formatted = format!(
            r"(?:[a-z0-9](?:[a-z0-9-]{{0,61}}[a-z0-9])?\.)+({domain})",
            domain = domain.replace(".", r"\.")
        );

        Regex::new(&formatted)
    }
}

pub mod env {
    use crate::config::SUBSCAN_ENV_NAMESPACE;

    /// Fetches API key from system environment variables
    /// if available. Module environment variables uses [`SUBSCAN_ENV_NAMESPACE`]
    /// namespace with `SUBSCAN_<module_name>_APIKEY` format
    ///
    /// # Examples
    ///
    /// ```
    /// use std::env;
    /// use subscan::utils::env::get_subscan_module_apikey;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let env_key = "SUBSCAN_FOO_APIKEY";
    ///
    ///     env::remove_var(env_key);
    ///
    ///     assert_eq!(get_subscan_module_apikey("FOO").is_ok(), false);
    ///
    ///     env::set_var(env_key, "foo");
    ///
    ///     assert_eq!(get_subscan_module_apikey("FOO").unwrap(), "foo");
    /// }
    /// ```
    pub fn get_subscan_module_apikey(name: &str) -> Result<String, dotenvy::Error> {
        let key = format!("{}_{}_APIKEY", SUBSCAN_ENV_NAMESPACE, name);

        dotenvy::var(key)
    }
}
