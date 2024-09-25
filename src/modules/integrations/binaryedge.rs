use crate::{
    enums::{RequesterDispatcher, SubdomainExtractorDispatcher},
    extractors::json::JSONExtractor,
    interfaces::{
        extractor::SubdomainExtractorInterface, module::SubscanModuleInterface,
        requester::RequesterInterface,
    },
    requesters::client::HTTPClient,
    types::core::Subdomain,
};
use async_trait::async_trait;
use reqwest::{
    header::{HeaderName, HeaderValue},
    Url,
};
use serde_json::Value;
use std::{collections::BTreeSet, str::FromStr};
use tokio::sync::Mutex;

/// Binaryedge API integration module
///
/// It uses [`GenericAPIIntegrationModule`] its own inner
/// here are the configurations
pub struct Binaryedge {
    /// Module name
    pub name: String,
    /// API search URL
    pub url: Url,
    /// Requester object instance for HTTP requests
    pub requester: Mutex<RequesterDispatcher>,
    /// Any extractor object to extract subdomain from content
    pub extractor: SubdomainExtractorDispatcher,
}

pub const BINARYEDGE_MODULE_NAME: &str = "Binaryedge";
pub const BINARYEDGE_URL: &str = "https://api.binaryedge.io/v2/query/domains/subdomain";

impl Default for Binaryedge {
    fn default() -> Self {
        Self::new()
    }
}

impl Binaryedge {
    pub fn new() -> Self {
        let url = Url::parse(BINARYEDGE_URL);
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        Self {
            name: BINARYEDGE_MODULE_NAME.into(),
            url: url.unwrap(),
            requester: requester.into(),
            extractor: extractor.into(),
        }
    }

    pub fn extract(content: Value) -> BTreeSet<Subdomain> {
        if let Some(subs) = content["events"].as_array() {
            let filter = |item: &Value| Some(item.as_str()?.to_string());

            BTreeSet::from_iter(subs.iter().filter_map(filter))
        } else {
            BTreeSet::new()
        }
    }

    pub async fn get_query_url(&self, domain: &str) -> Url {
        format!("{BINARYEDGE_URL}/{domain}").parse().unwrap()
    }
}

#[async_trait(?Send)]
impl SubscanModuleInterface for Binaryedge {
    async fn name(&self) -> &str {
        &self.name
    }

    async fn requester(&self) -> Option<&Mutex<RequesterDispatcher>> {
        Some(&self.requester)
    }

    async fn extractor(&self) -> Option<&SubdomainExtractorDispatcher> {
        Some(&self.extractor)
    }

    async fn run(&mut self, domain: String) -> BTreeSet<String> {
        let (_, apikey) = self.fetch_apikey().await;

        if apikey.is_err() {
            return BTreeSet::new();
        }

        let mut requester = self.requester.lock().await;
        let (name, value) = (
            HeaderName::from_str("X-Key"),
            HeaderValue::from_str(&apikey.unwrap()),
        );

        if let (Ok(name), Ok(value)) = (name, value) {
            requester.config().await.add_header(name, value);
        }

        let mut all_results = BTreeSet::new();

        let mut url = self.get_query_url(&domain).await;
        let mut page = 1;

        loop {
            let content = requester.get_content(url.clone()).await.unwrap_or_default();
            let news = self.extractor.extract(content, domain.clone()).await;

            if !news.is_empty() {
                page += 1;
                url.set_query(Some(&format!("page={}", page)));
                all_results.extend(news);
            } else {
                break;
            }
            println!("url: {}", url);
        }

        all_results
    }
}
