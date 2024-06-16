use crate::{
    extractors::regex::RegexExtractor, interfaces::extractor::SubdomainExtractorInterface,
    types::core::Subdomain,
};
use async_trait::async_trait;
use scraper::{ElementRef, Html, Selector};
use std::collections::BTreeSet;

pub struct HTMLExtractor {
    selector: String,
    removes: Vec<String>,
    regextractor: RegexExtractor,
}

impl HTMLExtractor {
    pub fn new(selector: String, removes: Vec<String>) -> Self {
        Self {
            selector,
            removes,
            regextractor: RegexExtractor::new(),
        }
    }
}

#[async_trait]
impl SubdomainExtractorInterface for HTMLExtractor {
    async fn extract(&self, content: String, domain: String) -> BTreeSet<Subdomain> {
        let document = Html::parse_document(&content);
        let selector = Selector::parse(&self.selector).unwrap();
        let selected = document.select(&selector);

        let remove = |item: ElementRef| {
            let mut text = item.inner_html();

            self.removes.iter().for_each(|element| {
                text = text.replace(element, "");
            });

            text
        };

        let extract = |item| self.regextractor.extract_one(item, domain.clone());

        selected.map(remove).filter_map(extract).collect()
    }
}
