use async_trait::async_trait;
use regex::Regex;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Url,
};
use tokio::sync::Mutex;

use crate::{
    enums::{
        content::Content,
        dispatchers::{RequesterDispatcher, SubdomainExtractorDispatcher, SubscanModuleDispatcher},
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
        result::module::SubscanModuleResult,
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

    async fn run(&mut self, domain: &str) -> Result<SubscanModuleResult> {
        let mut result: SubscanModuleResult = self.name().await.into();

        let requester = &mut *self.requester().await.unwrap().lock().await;
        let extractor = self.extractor().await.unwrap();

        let content = requester.get_content(self.base_url.clone()).await?;
        let token = self.get_auth_token(content).await;

        if let (Some(token), RequesterDispatcher::HTTPClient(requester)) = (token, requester) {
            let headers =
                HeaderMap::from_iter([(AUTHORIZATION, HeaderValue::from_str(&token).unwrap())]);
            let params = &[("target", domain)];

            requester.config.headers.extend(headers);

            let request = requester
                .client
                .post(self.url.clone())
                .form(params)
                .timeout(requester.config.timeout)
                .headers(requester.config.headers.clone())
                .build()
                .unwrap();

            if let Ok(response) = requester.client.execute(request).await {
                if let Ok(content) = response.text().await {
                    result.extend(extractor.extract(content.into(), domain).await?);

                    return Ok(result.with_finished().await);
                }
            }
        }

        Err(Custom("not get token".into()).into())
    }
}
