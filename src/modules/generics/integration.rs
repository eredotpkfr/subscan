use std::str::FromStr;

use async_trait::async_trait;
use flume::Sender;
use reqwest::{
    header::{HeaderName, HeaderValue},
    Url,
};
use tokio::sync::Mutex;

use crate::{
    enums::{
        auth::AuthenticationMethod,
        dispatchers::{RequesterDispatcher, SubdomainExtractorDispatcher},
        result::OptionalSubscanModuleResult,
    },
    error::SubscanError,
    interfaces::{
        extractor::SubdomainExtractorInterface, module::SubscanModuleInterface,
        requester::RequesterInterface,
    },
    types::{
        core::SubscanModuleCoreComponents,
        env::{Credentials, Env},
        func::GenericIntegrationCoreFuncs,
        result::status::{SkipReason::AuthenticationNotProvided, SubscanModuleStatus::Finished},
    },
    utilities::http,
};

/// Generic integration module
///
/// Several modules uses this generic module to make API calls and parsing JSON response
/// It takes a extractor that compatible with [`SubdomainExtractorInterface`], mostly
/// [`JSONExtractor`](crate::extractors::json::JSONExtractor) extractor is used with this
/// module to parse JSON contents
pub struct GenericIntegrationModule {
    pub name: String,
    pub auth: AuthenticationMethod,
    pub funcs: GenericIntegrationCoreFuncs,
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
            let requester = &mut *self.components.requester.lock().await;
            let rconfig = requester.config().await;

            rconfig.set_credentials(credentials);

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
            let requester = &mut *self.components.requester.lock().await;
            let rconfig = requester.config().await;

            let name = HeaderName::from_str(name).unwrap();
            let value = HeaderValue::from_str(apikey).unwrap();

            rconfig.add_header(name, value);

            return true;
        }
        false
    }
}

#[async_trait]
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

    async fn run(&mut self, domain: &str, results: Sender<OptionalSubscanModuleResult>) {
        let url = (self.funcs.url)(domain).parse();

        match url.clone().map_err(|err| SubscanError::from(err)) {
            Ok(mut url) => {
                if self.auth.is_set() && !self.authenticate(&mut url).await {
                    results.send(self.status(AuthenticationNotProvided.into()).await).unwrap();
                } else {
                    let requester = self.components.requester.lock().await;
                    let extractor = &self.components.extractor;

                    loop {
                        let content = requester.get_content(url.clone()).await;

                        match content {
                            Ok(content) => match extractor.extract(content.clone(), domain).await {
                                Ok(subdomains) => {
                                    for subdomain in &subdomains {
                                        results.send(self.item(subdomain).await).unwrap();
                                    }

                                    if let Some(next) = (self.funcs.next)(url, content) {
                                        url = next;
                                    } else {
                                        results.send(self.status(Finished).await).unwrap();
                                        break;
                                    }
                                }
                                Err(err) => {
                                    results.send(self.status(err.into()).await).unwrap();
                                    break;
                                }
                            },
                            Err(err) => {
                                results.send(self.status(err.into()).await).unwrap();
                                break;
                            }
                        }
                    }
                }
            }
            Err(err) => results.send(self.status(err.into()).await).unwrap(),
        };
    }
}
