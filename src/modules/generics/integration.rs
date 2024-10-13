use crate::{
    enums::{AuthenticationMethod, RequesterDispatcher, SubdomainExtractorDispatcher},
    interfaces::{
        extractor::SubdomainExtractorInterface, module::SubscanModuleInterface,
        requester::RequesterInterface,
    },
    types::{
        core::SubscanModuleCoreComponents,
        env::{Credentials, Env},
        func::GenericIntegrationCoreFuncs,
    },
    utils::http,
};
use async_trait::async_trait;
use reqwest::{
    header::{HeaderName, HeaderValue},
    Url,
};
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
    /// Set authentication method, see [`AuthenticationMethod`] enum for details
    pub auth: AuthenticationMethod,
    /// Core functions
    pub funcs: GenericIntegrationCoreFuncs,
    /// Core components
    pub components: SubscanModuleCoreComponents,
}

impl GenericIntegrationModule {
    pub async fn authenticate(&self, url: &mut Url) -> bool {
        let envs = self.envs().await;

        match &self.auth {
            AuthenticationMethod::APIKeyAsHeader(name) => {
                self.set_apikey_header(name, envs.apikey).await
            }
            AuthenticationMethod::APIKeyAsQueryParam(param) => {
                self.set_apikey_param(url, param, envs.apikey).await
            }
            AuthenticationMethod::BasicHTTPAuthentication(credentials) => {
                if credentials.is_ok() {
                    self.set_credentials(credentials.clone()).await
                } else {
                    // Try to fetch credentials from envs if not provided on startup
                    self.set_credentials(envs.credentials).await
                }
            }
            AuthenticationMethod::NoAuthentication => true,
        }
    }

    async fn set_credentials(&self, credentials: Credentials) -> bool {
        if credentials.is_ok() {
            let mut requester = self.components.requester.lock().await;

            requester.config().await.set_credentials(credentials);

            return true;
        }
        false
    }

    async fn set_apikey_param(&self, url: &mut Url, param: &str, apikey: Env) -> bool {
        if let Some(apikey) = &apikey.value {
            http::update_url_query(url, param, apikey);
            return true;
        }
        false
    }

    async fn set_apikey_header(&self, name: &str, apikey: Env) -> bool {
        if let Some(apikey) = &apikey.value {
            let mut requester = self.components.requester.lock().await;

            let name = HeaderName::from_str(name).unwrap();
            let value = HeaderValue::from_str(apikey).unwrap();

            requester.config().await.add_header(name, value);
            return true;
        }
        false
    }
}

#[async_trait(?Send)]
impl SubscanModuleInterface for GenericIntegrationModule {
    async fn name(&self) -> &str {
        &self.name
    }

    async fn requester(&self) -> Option<&Mutex<RequesterDispatcher>> {
        Some(&self.components.requester)
    }

    async fn extractor(&self) -> Option<&SubdomainExtractorDispatcher> {
        Some(&self.components.extractor)
    }

    async fn run(&mut self, domain: &str) -> BTreeSet<String> {
        let mut url: Url = (self.funcs.url)(domain).parse().unwrap();
        let mut all_results = BTreeSet::new();

        if self.auth.is_set() && !self.authenticate(&mut url).await {
            return all_results;
        }

        let requester = self.components.requester.lock().await;
        let extractor = &self.components.extractor;

        loop {
            let content = requester.get_content(url.clone()).await;
            let news = extractor.extract(content.clone(), domain).await;

            if news.is_empty() {
                break;
            }

            all_results.extend(news);

            if let Some(next_url) = (self.funcs.next)(url.clone(), content.as_json()) {
                url = next_url;
            } else {
                break;
            }
        }

        all_results
    }
}
