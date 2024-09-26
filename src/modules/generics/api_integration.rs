use crate::{
    enums::{APIAuthMethod, RequesterDispatcher, SubdomainExtractorDispatcher},
    interfaces::{
        extractor::SubdomainExtractorInterface, module::SubscanModuleInterface,
        requester::RequesterInterface,
    },
    types::core::APIKeyAsEnv,
};
use async_trait::async_trait;
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::Url;
use serde_json::Value;
use std::{collections::BTreeSet, str::FromStr};
use tokio::sync::Mutex;

/// Generic API integration module
///
/// Several modules uses this generic module
/// to make API calls and parsing JSON response
///
/// It takes a extractor that compatible with
/// [`SubdomainExtractorInterface`], mostly
/// [`JSONExtractor`](crate::extractors::json::JSONExtractor) extractor
/// is used with this module to parse JSON contents
pub struct GenericAPIIntegrationModule {
    /// Module name
    pub name: String,
    /// Simple function field that gets query URL
    /// by given domain address
    pub url: Box<dyn Fn(&str) -> String + Sync + Send>,
    /// Function definition that gets next URL to ensure
    /// fully fetch data with pagination from API endpoint
    pub next: Box<dyn Fn(Url, Value) -> Option<Url> + Sync + Send>,
    /// Set authentication method, see [`APIAuthMethod`] enum
    /// for details
    pub auth: APIAuthMethod,
    /// Requester object instance for HTTP requests
    pub requester: Mutex<RequesterDispatcher>,
    /// Any extractor object to extract subdomain from content
    pub extractor: SubdomainExtractorDispatcher,
}

impl GenericAPIIntegrationModule {
    pub async fn authenticate(&self, url: &mut Url, apienv: APIKeyAsEnv) {
        let apikey = apienv.1;

        if apikey.is_err() {
            return;
        }

        match &self.auth {
            APIAuthMethod::APIKeyAsHeader(name) => {
                self.set_apikey_header(name, &apikey.unwrap()).await
            }
            APIAuthMethod::APIKeyAsQueryParam(param) => {
                self.set_apikey_param(url, param, &apikey.unwrap()).await
            }
            APIAuthMethod::APIKeyAsURLSlug | APIAuthMethod::NoAuth => {}
        }
    }

    async fn set_apikey_param(&self, url: &mut Url, param: &str, apikey: &str) {
        url.set_query(Some(&format!("{param}={apikey}")));
    }

    async fn set_apikey_header(&self, name: &str, apikey: &str) {
        let mut requester = self.requester.lock().await;
        let (name, value) = (HeaderName::from_str(name), HeaderValue::from_str(apikey));

        if let (Ok(name), Ok(value)) = (name, value) {
            requester.config().await.add_header(name, value);
        }
    }
}

#[async_trait(?Send)]
impl SubscanModuleInterface for GenericAPIIntegrationModule {
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

        self.authenticate(&mut url, self.fetch_apikey().await).await;

        let requester = self.requester.lock().await;

        loop {
            let json = requester.get_json_content(url.clone()).await;
            let news = self
                .extractor
                .extract(json.to_string(), domain.clone())
                .await;

            if news.is_empty() {
                break;
            }

            all_results.extend(news);

            if let Some(next_url) = (self.next)(url.clone(), json) {
                url = next_url;
            } else {
                break;
            }
        }

        all_results
    }
}
