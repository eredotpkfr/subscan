use crate::interfaces::extractor::SubdomainExtractorInterface;
use crate::types::core::Subdomain;
use crate::utils::regex::generate_domain_regex;
use async_trait::async_trait;
use core::result::Result;
use regex::{Error, Regex};
use std::collections::HashSet;

pub struct RegexExtractor {}

impl RegexExtractor {
    pub fn new() -> Self {
        Self {}
    }

    fn generate_domain_regex(&self, domain: String) -> Result<Regex, Error> {
        generate_domain_regex(domain)
    }

    pub fn extract_one(&self, content: String, domain: String) -> Option<Subdomain> {
        let pattern = self.generate_domain_regex(domain).unwrap();

        if let Some(matches) = pattern.find(&content) {
            Some(matches.as_str().to_string())
        } else {
            None
        }
    }
}

#[async_trait]
impl SubdomainExtractorInterface for RegexExtractor {
    async fn extract(&self, content: String, domain: String) -> HashSet<Subdomain> {
        let pattern = self.generate_domain_regex(domain).unwrap();

        pattern
            .find_iter(&content)
            .filter_map(|item| item.as_str().parse().ok())
            .collect()
    }
}
