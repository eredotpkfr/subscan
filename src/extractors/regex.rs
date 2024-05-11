use crate::interfaces::extractor::SubdomainExtractorInterface;
use crate::types::Subdomain;
use crate::utils::regex::generate_domain_regex;
use core::result::Result;
use regex::{Error, Regex};
use std::collections::HashSet;

pub struct RegexExtractor {}

impl RegexExtractor {
    pub fn new() -> Self {
        RegexExtractor {}
    }

    fn generate_domain_regex(&self, domain: String) -> Result<Regex, Error> {
        generate_domain_regex(domain)
    }

    pub fn extract_one(&self, content: String, domain: String) -> Option<Subdomain> {
        let pattern = self.generate_domain_regex(domain).unwrap();

        if let Some(matches) = pattern.find(&content) {
            Some(Subdomain::from(matches.as_str()))
        } else {
            None
        }
    }
}

impl SubdomainExtractorInterface for RegexExtractor {
    fn extract(&self, content: String, domain: String) -> HashSet<Subdomain> {
        let pattern = self.generate_domain_regex(domain).unwrap();

        pattern
            .find_iter(&content)
            .filter_map(|item| item.as_str().parse().ok())
            .collect()
    }
}
