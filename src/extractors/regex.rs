use crate::interfaces::extractor::SubdomainExtractorInterface;
use crate::types::core::Subdomain;
use crate::utils::regex::generate_subdomain_regex;
use async_trait::async_trait;
use regex::Match;
use std::collections::BTreeSet;

pub struct RegexExtractor {}

impl RegexExtractor {
    pub fn new() -> Self {
        Self {}
    }

    pub fn extract_one(&self, content: String, domain: String) -> Option<Subdomain> {
        let pattern = generate_subdomain_regex(domain).unwrap();

        if let Some(matches) = pattern.find(&content) {
            Some(matches.as_str().to_string())
        } else {
            None
        }
    }
}

#[async_trait]
impl SubdomainExtractorInterface for RegexExtractor {
    async fn extract(&self, content: String, domain: String) -> BTreeSet<Subdomain> {
        let pattern = generate_subdomain_regex(domain).unwrap();
        let to_string = |item: Match| item.as_str().parse().ok();

        pattern.find_iter(&content).filter_map(to_string).collect()
    }
}
