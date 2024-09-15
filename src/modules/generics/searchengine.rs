use crate::{
    enums::{RequesterDispatcher, SubdomainExtractorDispatcher},
    interfaces::{
        extractor::SubdomainExtractorInterface, module::SubscanModuleInterface,
        requester::RequesterInterface,
    },
    types::query::{SearchQuery, SearchQueryParam},
};
use async_trait::async_trait;
use reqwest::Url;
use std::collections::BTreeSet;
use tokio::sync::Mutex;

pub struct GenericSearchEngineModule<'a> {
    pub name: String,
    pub url: Url,
    pub param: SearchQueryParam,
    pub requester: &'a Mutex<RequesterDispatcher>,
    pub extractor: SubdomainExtractorDispatcher,
}

impl<'a> GenericSearchEngineModule<'a> {
    pub async fn get_search_query(&self, domain: String) -> SearchQuery {
        self.param.to_search_query(domain, "site:".to_string())
    }
}

#[async_trait(?Send)]
impl<'a> SubscanModuleInterface for GenericSearchEngineModule<'a> {
    async fn name(&self) -> String {
        self.name.clone()
    }

    async fn run(&mut self, domain: String) {
        let requester = self.requester.lock().await;
        let extra_params = [("num".to_string(), 100.to_string())];

        let mut query = self.get_search_query(domain.clone()).await;
        let mut all_results = BTreeSet::new();

        loop {
            let url = query.as_url(self.url.clone(), &extra_params);
            let response = requester.get_content(url).await.unwrap_or_default();
            let results = self.extractor.extract(response, domain.clone()).await;

            all_results.extend(results.clone());

            if !query.update_many(results.clone()) {
                break;
            }
        }

        println!("{:#?}\nTotal: {}", all_results, all_results.len());
    }
}
