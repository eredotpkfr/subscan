use crate::interfaces::extractor::SubdomainExtractorInterface;
use crate::interfaces::module::SubscanModuleInterface;
use crate::interfaces::requester::RequesterInterface;
use crate::types::core::{QueryParam, SearchQuery, Subdomain};
use async_trait::async_trait;
use reqwest::{Method, Request, Url};
use std::collections::BTreeSet;

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

pub struct GenericSearchEngineModule {
    pub name: String,
    pub url: Url,
    pub param: QueryParam,
    pub requester: Box<dyn RequesterInterface>,
    pub extractor: Box<dyn SubdomainExtractorInterface>,
    pub all_results: BTreeSet<Subdomain>,
}

impl GenericSearchEngineModule {
    pub fn new(
        name: String,
        url: Url,
        param: QueryParam,
        requester: Box<dyn RequesterInterface>,
        extractor: Box<dyn SubdomainExtractorInterface>,
    ) -> Self {
        Self {
            name: name,
            url: url,
            param: param,
            requester: requester,
            extractor: extractor,
            all_results: BTreeSet::new(),
        }
    }

    pub async fn get_start_query(&self, domain: String) -> SearchQuery {
        self.param.to_search_query(domain, "site:".to_string())
    }

    pub async fn build_request(&self, query: &mut SearchQuery) -> Request {
        let builder = self.requester.request(Method::GET, self.url.clone()).await;

        builder
            .header("User-Agent", USER_AGENT)
            .query(&[
                (self.param.as_string(), query.as_search_str()),
                ("num".to_string(), 100.to_string()),
            ])
            .build()
            .unwrap()
    }
}

#[async_trait(?Send)]
impl SubscanModuleInterface for GenericSearchEngineModule {
    async fn name(&self) -> String {
        self.name.clone()
    }

    async fn run(&mut self, domain: String) {
        let mut query = self.get_start_query(domain.clone()).await;

        loop {
            let request = self.build_request(&mut query).await;
            let response = self.requester.get(request).await.unwrap();
            let results = self.extractor.extract(response, domain.clone()).await;

            self.all_results.extend(results.clone());

            if !query.update_many(results.clone()) {
                break;
            }
        }

        println!("{:#?}\nTotal: {}", self.all_results, self.all_results.len());
    }
}
