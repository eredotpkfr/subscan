use crate::enums::{RequesterDispatcher, SubdomainExtractorDispatcher};
use crate::interfaces::extractor::SubdomainExtractorInterface;
use crate::interfaces::module::SubscanModuleInterface;
use crate::interfaces::requester::RequesterInterface;
use async_trait::async_trait;
use reqwest::Url;
use std::collections::BTreeSet;
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
        let requester = self.requester.lock().await;
        let url = Url::parse(&(self.url)(domain.clone())).unwrap();

        let content = requester.get_content(url).await.unwrap_or_default();

        self.extractor.extract(content, domain).await
    }
}
