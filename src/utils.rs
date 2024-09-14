pub mod regex {
    use core::result::Result;
    use regex::{Error, Regex};

    /// Helper function that generates dynamically regex statement
    /// by given domain address to parse subdomain addresses
    /// according to any target domain address
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
