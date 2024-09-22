use crate::{
    enums::{AuthMethod, RequesterDispatcher, SubdomainExtractorDispatcher},
    interfaces::{
        extractor::SubdomainExtractorInterface, module::SubscanModuleInterface,
        requester::RequesterInterface,
    },
};
use async_trait::async_trait;
use reqwest::{
    header::{HeaderName, HeaderValue},
    Url,
};
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
    pub url: Box<dyn Fn(String) -> String + Sync + Send>,
    /// Set authentication method, see [`AuthMethod`] enum
    /// for details
    pub auth: AuthMethod,
    /// Requester object instance for HTTP requests
    pub requester: Mutex<RequesterDispatcher>,
    /// Any extractor object to extract subdomain from content
    pub extractor: SubdomainExtractorDispatcher,
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
        let mut requester = self.requester.lock().await;
        let url = Url::parse(&(self.url)(domain.clone())).unwrap();

        match &self.auth {
            AuthMethod::APIKeyInHeader(key) => {
                if let Ok(apikey) = self.fetch_apikey().await {
                    let name = HeaderName::from_str(key.as_str()).unwrap();
                    let value = HeaderValue::from_str(apikey.as_str()).unwrap();

                    requester.config().await.add_header(name, value);
                }
            }
            AuthMethod::APIKeyInURL => {}
            AuthMethod::NoAuth => {}
        }

        let content = requester.get_content(url).await.unwrap_or_default();

        self.extractor.extract(content, domain).await
    }
}
