use crate::interfaces::extractor::SubdomainExtractorInterface;
use crate::interfaces::module::SubscanModuleInterface;
use crate::interfaces::requester::RequesterInterface;
use crate::types::Subdomain;
use async_trait::async_trait;
use reqwest::{Method, Request, Url};
use std::collections::HashSet;

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

pub struct GenericSearchEngineModule {
    pub name: String,
    pub url: Url,
    pub query_param: String,
    pub extractor: Box<dyn SubdomainExtractorInterface>,
    pub query: String,
    pub query_state: HashSet<Subdomain>,
    pub all_results: HashSet<Subdomain>,
}

impl GenericSearchEngineModule {
    pub fn new(
        name: String,
        url: Url,
        query_param: String,
        extractor: Box<dyn SubdomainExtractorInterface>,
    ) -> Box<dyn SubscanModuleInterface> {
        Box::new(Self {
            name: name,
            url: url,
            query_param: query_param,
            extractor: extractor,
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

    pub async fn build_request(&self, requester: &Box<dyn RequesterInterface>) -> Request {
        let builder = requester.request(Method::GET, self.url.clone());

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
    fn name(&self) -> String {
        self.name.clone()
    }

    async fn run(&mut self, domain: String, requester: Box<dyn RequesterInterface>) {
        self.query = self.get_start_query(domain.clone()).await;

        while let Some(response) = requester.get(self.build_request(&requester).await).await {
            let results = self.extractor.extract(response, domain.clone());

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
