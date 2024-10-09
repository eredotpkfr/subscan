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

/// Utilities about project environments
pub mod env {
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
}

/// Helpful HTTP utilities
pub mod http {
    use reqwest::Url;

    /// Update query params without remove old query params. If the
    /// given parameter name non-exists it will append end of the
    /// query otherwise it's value will be updated
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::utils::http::update_url_query;
    /// use reqwest::Url;
    ///
    /// let mut url: Url = "https://foo.com".parse().unwrap();
    ///
    /// update_url_query(&mut url, "a".into(), "b".into());
    /// assert_eq!(url.to_string(), "https://foo.com/?a=b");
    ///
    /// // does not override old `a` parameter
    /// update_url_query(&mut url, "x".into(), "y".into());
    /// assert_eq!(url.to_string(), "https://foo.com/?a=b&x=y");
    ///
    /// update_url_query(&mut url, "a".into(), "c".into());
    /// assert_eq!(url.to_string(), "https://foo.com/?x=y&a=c");
    /// ```
    pub fn update_url_query(url: &mut Url, name: &str, value: &str) {
        let binding = url.clone();
        let pairs = binding.query_pairs();
        let filtered = pairs.filter(|item| item.0.to_lowercase() != name.to_lowercase());

        url.query_pairs_mut()
            .clear()
            .extend_pairs(filtered)
            .append_pair(name, value)
            .finish();
    }
}
