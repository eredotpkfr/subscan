use crate::{
    enums::{APIAuthMethod, RequesterDispatcher, SubdomainExtractorDispatcher},
    interfaces::{
        extractor::SubdomainExtractorInterface, module::SubscanModuleInterface,
        requester::RequesterInterface,
    },
    types::core::{GetNextUrlFunc, GetQueryUrlFunc},
    utils::http,
};
use async_trait::async_trait;
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::Url;
use std::{collections::BTreeSet, str::FromStr};
use tokio::sync::Mutex;

/// Generic integration module
///
/// Several modules uses this generic module to make API calls and parsing JSON response
/// It takes a extractor that compatible with [`SubdomainExtractorInterface`], mostly
/// [`JSONExtractor`](crate::extractors::json::JSONExtractor) extractor is used with this
/// module to parse JSON contents
pub struct GenericIntegrationModule {
    /// Module name
    pub name: String,
    /// Simple function field that gets query URL by given domain address
    pub url: GetQueryUrlFunc,
    /// Function definition that gets next URL to ensure fully fetch data with pagination
    /// from API endpoint
    pub next: GetNextUrlFunc,
    /// Set authentication method, see [`APIAuthMethod`] enum for details
    pub auth: APIAuthMethod,
    /// Requester object instance for HTTP requests
    pub requester: Mutex<RequesterDispatcher>,
    /// Any extractor object to extract subdomain from content
    pub extractor: SubdomainExtractorDispatcher,
}

impl GenericIntegrationModule {
    pub async fn authenticate(&self, url: &mut Url, apikey: String) {
        match &self.auth {
            APIAuthMethod::APIKeyAsHeader(name) => self.set_apikey_header(name, &apikey).await,
            APIAuthMethod::APIKeyAsQueryParam(param) => {
                self.set_apikey_param(url, param, &apikey).await
            }
            APIAuthMethod::NoAuth => {}
        }
    }

    async fn set_apikey_param(&self, url: &mut Url, param: &str, apikey: &str) {
        http::update_url_query(url, param, apikey);
    }

    async fn set_apikey_header(&self, name: &str, apikey: &str) {
        let mut requester = self.requester.lock().await;

        let name = HeaderName::from_str(name);
        let value = HeaderValue::from_str(apikey);

        if let (Ok(name), Ok(value)) = (name, value) {
            requester.config().await.add_header(name, value);
        }
    }
}

#[async_trait(?Send)]
impl SubscanModuleInterface for GenericIntegrationModule {
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
        let mut url: Url = (self.url)(&domain).parse().unwrap();
        let mut all_results = BTreeSet::new();

        if self.auth.is_set() {
            let apienv = self.fetch_apikey().await;

            if let Ok(apikey) = apienv.1 {
                self.authenticate(&mut url, apikey).await;
            } else {
                return all_results;
            }
        }

        let requester = self.requester.lock().await;

        loop {
            let content = requester.get_content(url.clone()).await;
            let (parsing, domain) = (content.clone(), domain.clone());

            let news = self.extractor.extract(parsing, domain).await;

            if news.is_empty() {
                break;
            }

            all_results.extend(news);

            if let Some(next_url) = (self.next)(url.clone(), content.as_json()) {
                url = next_url;
            } else {
                break;
            }
        }

        all_results
    }
}
