use core::result::Result;
use regex::{Error, Regex};

/// Helper function that generates dynamically regex statement
/// by given domain address to parse subdomains
///
/// # Examples
///
/// ```
/// use subscan::utilities::regex::generate_subdomain_regex;
///
/// let regex_stmt = generate_subdomain_regex("foo.com").unwrap();
///
/// assert_eq!(regex_stmt.as_str(), "(?:[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?\\.)+(foo\\.com)");
///
/// assert!(regex_stmt.find("bar.foo.com").is_some());
/// assert!(regex_stmt.find("foo").is_none());
/// ```
pub fn generate_subdomain_regex(domain: &str) -> Result<Regex, Error> {
    let formatted = format!(
        r"(?:[a-z0-9](?:[a-z0-9-]{{0,61}}[a-z0-9])?\.)+({domain})",
        domain = domain.replace(".", r"\.")
    );

    Regex::new(&formatted)
}
