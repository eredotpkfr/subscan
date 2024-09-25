use crate::{
    interfaces::extractor::SubdomainExtractorInterface, types::core::Subdomain,
    utils::regex::generate_subdomain_regex,
};
use async_trait::async_trait;
use regex::Match;
use std::collections::BTreeSet;

/// Regex extractor component generates subdomain pattern by
/// given domain address and extracts subdomains via this pattern.
/// Also this object compatible with [`SubdomainExtractorInterface`]
/// and it uses `extract` method
#[derive(Default)]
pub struct RegexExtractor {}

impl RegexExtractor {
    /// Extract one subdomain from given [`String`] content
    ///
    /// # Panics
    ///
    /// When the regex pattern did not compile
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::extractors::regex::RegexExtractor;
    ///
    /// let domain = String::from("foo.com");
    /// let extractor = RegexExtractor::default();
    ///
    /// let match_content = String::from("bar.foo.com");
    /// let no_match_content = String::from("foobarbaz");
    ///
    /// assert!(extractor.extract_one(match_content, domain.clone()).is_some());
    /// assert!(extractor.extract_one(no_match_content, domain).is_none());
    /// ```
    pub fn extract_one(&self, content: String, domain: String) -> Option<Subdomain> {
        let pattern = generate_subdomain_regex(domain).unwrap();
        let to_string = |matches: Match| matches.as_str().to_string();

        pattern.find(&content).map(to_string)
    }
}

#[async_trait]
impl SubdomainExtractorInterface for RegexExtractor {
    /// Extract many subdomains from given [`String`] content
    ///
    /// # Panics
    ///
    /// When the regex pattern did not compile
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeSet;
    /// use subscan::extractors::regex::RegexExtractor;
    /// use subscan::interfaces::extractor::SubdomainExtractorInterface;
    /// use subscan::types::core::Subdomain;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let domain = String::from("foo.com");
    ///     let content = String::from("bar.foo.com\nbaz.foo.com");
    ///
    ///     let extractor = RegexExtractor::default();
    ///     let result = extractor.extract(content, domain).await;
    ///
    ///     assert_eq!(result, [
    ///         Subdomain::from("bar.foo.com"),
    ///         Subdomain::from("baz.foo.com"),
    ///     ].into());
    ///     assert_eq!(result.len(), 2);
    /// }
    /// ```
    async fn extract(&self, content: String, domain: String) -> BTreeSet<Subdomain> {
        let pattern = generate_subdomain_regex(domain).unwrap();
        let to_string = |item: Match| item.as_str().parse().ok();

        pattern.find_iter(&content).filter_map(to_string).collect()
    }
}
