use std::collections::BTreeSet;

use async_trait::async_trait;
use regex::Match;

use crate::{
    enums::content::Content,
    interfaces::extractor::SubdomainExtractorInterface,
    types::core::{Result, Subdomain},
    utilities::regex::generate_subdomain_regex,
};

/// Regex extractor component generates subdomain pattern by given domain address and
/// extracts subdomains via this pattern. Also this object compatible with
/// [`SubdomainExtractorInterface`] and it uses `extract` method
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
    /// let extractor = RegexExtractor::default();
    ///
    /// let match_content = String::from("bar.foo.com\nbaz.foo.com");
    /// let no_match_content = String::from("foobarbaz");
    ///
    /// let matched = extractor.extract_one(match_content, "foo.com");
    /// let not_matched = extractor.extract_one(no_match_content, "foo.com");
    ///
    /// assert!(matched.is_some());
    /// assert!(not_matched.is_none());
    ///
    /// assert_eq!(matched.unwrap(), "bar.foo.com");
    /// ```
    pub fn extract_one(&self, content: String, domain: &str) -> Option<Subdomain> {
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
    /// use subscan::enums::content::Content;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let content = Content::from("bar.foo.com\nbaz.foo.com");
    ///
    ///     let extractor = RegexExtractor::default();
    ///     let result = extractor.extract(content.clone(), "foo.com").await.unwrap();
    ///
    ///     assert_eq!(result.len(), 2);
    ///     assert_eq!(result, [
    ///         Subdomain::from("bar.foo.com"),
    ///         Subdomain::from("baz.foo.com"),
    ///     ].into());
    ///
    ///     let result = extractor.extract(content, "{}}").await;
    ///
    ///     assert!(result.is_err());
    /// }
    /// ```
    async fn extract(&self, content: Content, domain: &str) -> Result<BTreeSet<Subdomain>> {
        let pattern = generate_subdomain_regex(domain)?;
        let to_string = |item: Match| item.as_str().parse().ok();
        let content = content.as_string();

        Ok(pattern.find_iter(&content).filter_map(to_string).collect())
    }
}
