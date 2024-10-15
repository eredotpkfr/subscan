use crate::{
    enums::{Content, RequesterDispatcher, SubdomainExtractorDispatcher, SubscanModuleDispatcher},
    extractors::html::HTMLExtractor,
    interfaces::{
        extractor::SubdomainExtractorInterface, module::SubscanModuleInterface,
        requester::RequesterInterface,
    },
    requesters::client::HTTPClient,
    types::core::SubscanModuleCoreComponents,
};
use async_trait::async_trait;
use regex::Regex;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Url,
};
use std::collections::BTreeSet;
use tokio::sync::Mutex;

pub const DNSDUMPSTER_MODULE_NAME: &str = "dnsdumpster";
pub const DNSDUMPSTER_URL: &str = "https://dnsdumpster.com/";
pub const DNSDUMPSTER_SUBDOMAIN_TAG: &str = "table > tbody > tr > td:first-child";
pub const DNSDUMPSTER_CSRF_PATTERN: &str =
    r#"<input type="hidden" name="csrfmiddlewaretoken" value="(?<token>.*)">"#;

/// `DnsDumpster` non-generic integration module
///
/// | Property           | Value                                                           |
/// |:------------------:|:---------------------------------------------------------------:|
/// | Module Name        | `dnsdumpster`                                                   |
/// | Doc URL            | <https://dnsdumpster.com>                                       |
/// | Subdomain Selector | `table > tbody > tr > td:first-child`                           |
/// | CSRF Token Pattern | `<input type="hidden" name="csrfmiddlewaretoken" value="(.*)">` |
/// | Requester          | [`HTTPClient`]                                                  |
/// | Extractor          | [`HTMLExtractor`]                                               |
/// | Generic            | [`None`]                                                        |
pub struct DnsDumpster {
    /// Module name
    pub name: String,
    /// Base index URL
    pub url: Url,
    /// Core components
    pub components: SubscanModuleCoreComponents,
}

impl DnsDumpster {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let url = Url::parse(DNSDUMPSTER_URL);

        let selector: String = DNSDUMPSTER_SUBDOMAIN_TAG.into();
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: HTMLExtractor = HTMLExtractor::new(selector, vec![]);

        let dnsdumpster = Self {
            name: DNSDUMPSTER_MODULE_NAME.into(),
            url: url.unwrap(),
            components: SubscanModuleCoreComponents {
                requester: requester.into(),
                extractor: extractor.into(),
            },
        };

        dnsdumpster.into()
    }

    pub async fn get_csrf_token(&self, content: Content) -> Option<String> {
        let pattern = Regex::new(DNSDUMPSTER_CSRF_PATTERN).unwrap();

        if let Some(caps) = pattern.captures(&content.as_string()) {
            return Some(caps["token"].to_string());
        }

        None
    }
}

#[async_trait(?Send)]
impl SubscanModuleInterface for DnsDumpster {
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
        let requester = &mut *self.components.requester.lock().await;
        let extractor = &self.components.extractor;

        let content = requester.get_content(self.url.clone()).await;
        let token = self.get_csrf_token(content).await;

        if let (Some(token), RequesterDispatcher::HTTPClient(requester)) = (token, requester) {
            let cookie = format!("csrftoken={token}; Domain=dnsdumpster.com");
            let headers = HeaderMap::from_iter([
                (
                    HeaderName::from_static("referer"),
                    HeaderValue::from_static(DNSDUMPSTER_URL),
                ),
                (
                    HeaderName::from_static("x-csrf-token"),
                    HeaderValue::from_str(&token).unwrap(),
                ),
                (
                    HeaderName::from_static("cookie"),
                    HeaderValue::from_str(&cookie).unwrap(),
                ),
            ]);
            let params = &[
                ("csrfmiddlewaretoken", token.as_str()),
                ("targetip", domain),
                ("user", "free"),
            ];

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
                    return extractor.extract(content.into(), domain).await;
                }
            }
        }

        [].into()
    }
}
