use crate::{
    enums::{AuthMethod, RequesterDispatcher, SubdomainExtractorDispatcher},
    interfaces::{
        extractor::SubdomainExtractorInterface, module::SubscanModuleInterface,
        requester::RequesterInterface,
    },
};
use async_trait::async_trait;
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::Url;
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
    /// Set authentication method, see [`AuthMethod`] enum
    /// for details
    pub auth: AuthMethod,
    /// Requester object instance for HTTP requests
    pub requester: Mutex<RequesterDispatcher>,
    /// Any extractor object to extract subdomain from content
    pub extractor: SubdomainExtractorDispatcher,
}

impl GenericAPIIntegrationModule {
    async fn authenticate(&self, domain: &str) -> Url {
        let url: Url = (self.url)(domain).parse().unwrap();
        let apikey = self.fetch_apikey().await;

        match &self.auth {
            AuthMethod::APIKeyInHeader(key) => {
                if let Ok(apikey) = apikey {
                    let mut requester = self.requester.lock().await;

                    let (name, value) = (HeaderName::from_str(key), HeaderValue::from_str(&apikey));

                    if let (Ok(name), Ok(value)) = (name, value) {
                        requester.config().await.add_header(name, value);
                    }
                }
            }
            AuthMethod::APIKeyInURL | AuthMethod::NoAuth => {}
        }

        url
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
        let url = self.authenticate(&domain).await;

        let requester = self.requester.lock().await;
        let content = requester.get_content(url).await.unwrap_or_default();

        self.extractor.extract(content, domain).await
    }
}
