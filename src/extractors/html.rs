use crate::extractors::regex::RegexExtractor;
use crate::interfaces::extractor::SubdomainExtractorInterface;
use crate::types::core::Subdomain;
use async_trait::async_trait;
use scraper::{Html, Selector};
use std::collections::HashSet;

pub struct HTMLExtractor {
    selector: String,
    removes: Vec<String>,
    regextractor: RegexExtractor,
}

impl HTMLExtractor {
    pub fn new(selector: String, removes: Vec<String>) -> Self {
        Self {
            selector: selector,
            removes: removes,
            regextractor: RegexExtractor::new(),
        }
    }
}

#[async_trait]
impl SubdomainExtractorInterface for HTMLExtractor {
    async fn extract(&self, content: String, domain: String) -> HashSet<Subdomain> {
        let document = Html::parse_document(&content);
        let selector = Selector::parse(&self.selector).unwrap();

        document
            .select(&selector)
            .map(|item| {
                let mut text = item.inner_html();

                self.removes.iter().for_each(|element| {
                    text = text.replace(element, "");
                });

                text
            })
            .filter_map(|item| self.regextractor.extract_one(item, domain.clone()))
            .collect()
    }
}
