use crate::interfaces::extractor::SubdomainExtractorInterface;
use crate::interfaces::module::SubscanModuleInterface;
use crate::interfaces::requester::RequesterInterface;
use crate::types::core::Subdomain;
use async_trait::async_trait;
use reqwest::{Method, Request, Url};
use std::collections::HashSet;

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

pub struct GenericSearchEngineModule {
    pub name: String,
    pub requester: Box<dyn RequesterInterface>,
    pub extractor: Box<dyn SubdomainExtractorInterface>,
    pub url: Url,
    pub query_param: String,
    pub query: String,
    pub query_state: HashSet<String>,
    pub all_results: HashSet<Subdomain>,
}

impl GenericSearchEngineModule {
    pub fn new(
        name: String,
        requester: Box<dyn RequesterInterface>,
        extractor: Box<dyn SubdomainExtractorInterface>,
        url: Url,
        query_param: String,
    ) -> Box<dyn SubscanModuleInterface> {
        Box::new(Self {
            name: name,
            requester: requester,
            extractor: extractor,
            url: url,
            query_param: query_param,
            query: String::new(),
            query_state: HashSet::new(),
            all_results: HashSet::new(),
        })
    }

    pub async fn get_start_query(&self, domain: String) -> String {
        format!("site:{}", domain).to_string()
    }

    pub fn format_for_query(&mut self, item: &Subdomain, domain: String) -> Option<String> {
        let formatted = format!(".{}", domain);

        if let Some(sub) = item.strip_suffix(&formatted) {
            if self.query_state.insert(sub.to_string()) {
                Some(format!("-{}", sub.to_string()))
            } else {
                None
            }
        } else {
            None
        }
    }

    pub async fn get_next_query(&mut self, domain: String, results: HashSet<Subdomain>) -> String {
        let news = results
            .iter()
            .filter_map(|item| self.format_for_query(&item, domain.clone()))
            .collect::<Vec<Subdomain>>()
            .join(" ");

        format!("{} {}", self.query, news).trim().to_string()
    }

    pub async fn build_request(&self) -> Request {
        let builder = self.requester.request(Method::GET, self.url.clone()).await;

        builder
            .header("User-Agent", USER_AGENT)
            .query(&[
                (self.query_param.clone(), self.query.clone()),
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
        self.query = self.get_start_query(domain.clone()).await;

        while let Some(response) = self.requester.get(self.build_request().await).await {
            let results = self.extractor.extract(response, domain.clone()).await;

            self.all_results.extend(results.clone());

            let next_query = self.get_next_query(domain.clone(), results).await;

            if self.query == next_query {
                break;
            }

            self.query = next_query;
        }
        println!("{:#?}\nTotal: {}", self.all_results, self.all_results.len());
    }
}
