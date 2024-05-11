use crate::extractors::regex::RegexExtractor;
use crate::interfaces::extractor::SubdomainExtractorInterface;
use crate::types::Subdomain;
use scraper::html::Select;
use scraper::{Html, Selector};
use std::collections::HashSet;

pub struct HTMLExtractor {
    selector: String,
    regextractor: RegexExtractor,
}

impl HTMLExtractor {
    pub fn new(selector: String) -> Self {
        HTMLExtractor {
            selector: selector,
            regextractor: RegexExtractor::new(),
        }
    }
}

impl SubdomainExtractorInterface for HTMLExtractor {
    fn extract(&self, content: String, domain: String) -> HashSet<Subdomain> {
        let document = Html::parse_document(&content);
        let selector = Selector::parse(&self.selector).unwrap();

        document
            .select(&selector)
            .filter_map(|item| {
                self.regextractor
                    .extract_one(item.inner_html(), domain.clone())
            })
            .collect()
    }
}
