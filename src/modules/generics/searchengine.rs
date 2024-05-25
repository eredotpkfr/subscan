use crate::interfaces::extractor::SubdomainExtractorInterface;
use crate::interfaces::module::SubscanModuleInterface;
use crate::interfaces::requester::RequesterInterface;

use crate::types::query::{SearchQuery, SearchQueryParam};
use async_trait::async_trait;
use reqwest::Url;
use std::collections::BTreeSet;

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

pub struct GenericSearchEngineModule {
    pub name: String,
    pub url: Url,
    pub param: SearchQueryParam,
    pub requester: Box<dyn RequesterInterface>,
    pub extractor: Box<dyn SubdomainExtractorInterface>,
}

impl GenericSearchEngineModule {
    pub async fn get_search_query(&self, domain: String) -> SearchQuery {
        self.param.to_search_query(domain, "site:".to_string())
    }
}

#[async_trait(?Send)]
impl SubscanModuleInterface for GenericSearchEngineModule {
    async fn name(&self) -> String {
        self.name.clone()
    }

    async fn run(&mut self, domain: String) {
        let mut all_results = BTreeSet::new();
        let mut query = self.get_search_query(domain.clone()).await;
        let extra_params = [("num".to_string(), 100.to_string())];

        loop {
            let url = query.as_url(self.url.clone(), &extra_params);
            let response = self.requester.get_content(url).await.unwrap_or_default();
            let results = self.extractor.extract(response, domain.clone()).await;

            all_results.extend(results.clone());

            if !query.update_many(results.clone()) {
                break;
            }
        }
        
        println!("{:#?}\nTotal: {}", all_results, all_results.len());
    }
}
