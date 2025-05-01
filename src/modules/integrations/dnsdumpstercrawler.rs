use async_trait::async_trait;
use flume::Sender;
use regex::Regex;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Response, Url,
};
use tokio::sync::Mutex;

use crate::{
    enums::{
        content::Content,
        dispatchers::{RequesterDispatcher, SubdomainExtractorDispatcher, SubscanModuleDispatcher},
        result::SubscanModuleResult,
    },
    error::ModuleErrorKind::Custom,
    extractors::html::HTMLExtractor,
    interfaces::{
        extractor::SubdomainExtractorInterface, module::SubscanModuleInterface,
        requester::RequesterInterface,
    },
    requesters::client::HTTPClient,
    types::{
        core::{Result, SubscanModuleCoreComponents},
        result::status::SubscanModuleStatus::Finished,
    },
};

pub const DNSDUMPSTERCRAWLER_MODULE_NAME: &str = "dnsdumpstercrawler";
pub const DNSDUMPSTERCRAWLER_URL: &str = "https://api.dnsdumpster.com/htmld/";
pub const DNSDUMPSTERCRAWLER_BASE_URL: &str = "https://dnsdumpster.com";
pub const DNSDUMPSTERCRAWLER_SUBDOMAIN_TAG: &str = "table > tbody > tr > td:first-child";
pub const DNSDUMPSTERCRAWLER_AUTH_TOKEN_PATTERN: &str = r#"Authorization": "(?<token>[^"]+)"#;

/// `DNSDumpsterCrawler` non-generic integration module
///
/// | Property           | Value                                                           |
/// |:------------------:|:---------------------------------------------------------------:|
/// | Module Name        | `dnsdumpstercrawler`                                            |
/// | Doc URL            | <https://dnsdumpster.com>                                       |
/// | Subdomain Selector | `table > tbody > tr > td:first-child`                           |
/// | Auth Token Pattern | `"Authorization": "(?<token>[^"]+)"`                            |
/// | Requester          | [`HTTPClient`]                                                  |
/// | Extractor          | [`HTMLExtractor`]                                               |
/// | Generic            | [`None`]                                                        |
pub struct DNSDumpsterCrawler {
    /// Module name
    pub name: String,
    /// API URL
    pub url: Url,
    /// Base index URL
    pub base_url: Url,
    /// Core components
    pub components: SubscanModuleCoreComponents,
}

impl DNSDumpsterCrawler {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let url = Url::parse(DNSDUMPSTERCRAWLER_URL);
        let base_url = Url::parse(DNSDUMPSTERCRAWLER_BASE_URL);

        let selector: String = DNSDUMPSTERCRAWLER_SUBDOMAIN_TAG.into();
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: HTMLExtractor = HTMLExtractor::new(selector, vec![]);

        let dnsdumpster = Self {
            name: DNSDUMPSTERCRAWLER_MODULE_NAME.into(),
            url: url.unwrap(),
            base_url: base_url.unwrap(),
            components: SubscanModuleCoreComponents {
                requester: requester.into(),
                extractor: extractor.into(),
            },
        };

        dnsdumpster.into()
    }

    pub async fn get_auth_token(&self, content: Content) -> Option<String> {
        let pattern = Regex::new(DNSDUMPSTERCRAWLER_AUTH_TOKEN_PATTERN).unwrap();

        if let Some(caps) = pattern.captures(&content.as_string()) {
            return Some(caps["token"].to_string());
        }

        None
    }
}

#[async_trait]
impl SubscanModuleInterface for DNSDumpsterCrawler {
    async fn name(&self) -> &str {
        &self.name
    }

    async fn requester(&self) -> Option<&Mutex<RequesterDispatcher>> {
        Some(&self.components.requester)
    }

    async fn extractor(&self) -> Option<&SubdomainExtractorDispatcher> {
        Some(&self.components.extractor)
    }

    async fn run(&mut self, domain: &str, results: Sender<Option<SubscanModuleResult>>) {
        let requester = &mut *self.requester().await.unwrap().lock().await;
        let extractor = self.extractor().await.unwrap();

        let content = requester
            .get_content(self.base_url.clone())
            .await
            .unwrap_or_default();
        let token = self.get_auth_token(content).await;

        if let (Some(token), RequesterDispatcher::HTTPClient(requester)) = (token, requester) {
            let headers = HeaderMap::from_iter([(
                HeaderName::from_static("authorization"),
                HeaderValue::from_str(&token).unwrap(),
            )]);
            let params = &[("target", domain)];

            requester.config.headers.extend(headers);

            let rbuilder = requester
                .client
                .post(self.url.clone())
                .form(params)
                .timeout(requester.config.timeout)
                .headers(requester.config.headers.clone());

            if let Ok(request) = rbuilder.build() {
                let response: Result<Response> = requester
                    .client
                    .execute(request)
                    .await
                    .map_err(|err| err.into());

                match response {
                    Ok(response) => {
                        let content = response.text().await.unwrap_or_default();
                        let subdomains = extractor
                            .extract(content.into(), domain)
                            .await
                            .unwrap_or_default();

                        for subdomain in &subdomains {
                            results
                                .send(Some((self.name().await, subdomain).into()))
                                .unwrap();
                        }

                        results.send(Finished.into()).unwrap();
                    }
                    Err(err) => {
                        results.send(err.status().into()).unwrap();
                    }
                }
            }
            results.send(Finished.into()).unwrap();
        }

        results.send(Custom("not get token".into()).into()).unwrap();
    }
}
