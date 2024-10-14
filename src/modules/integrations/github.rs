use crate::{
    enums::{Content, RequesterDispatcher, SubdomainExtractorDispatcher, SubscanModuleDispatcher},
    extractors::regex::RegexExtractor,
    interfaces::{
        extractor::SubdomainExtractorInterface, module::SubscanModuleInterface,
        requester::RequesterInterface,
    },
    requesters::client::HTTPClient,
    types::core::SubscanModuleCoreComponents,
};
use async_trait::async_trait;
use reqwest::{
    header::{HeaderValue, AUTHORIZATION},
    Url,
};
use serde_json::Value;
use std::collections::BTreeSet;
use tokio::sync::Mutex;

pub const GITHUB_MODULE_NAME: &str = "github";
pub const GITHUB_URL: &str = "https://github.com/";
pub const GITHUB_API_URL: &str = "https://api.github.com/search/code";
pub const GITHUB_RAW_URL: &str = "https://raw.githubusercontent.com/";

/// `GitHub` non-generic integration module
///
/// | Property           | Value                |
/// |:------------------:|:--------------------:|
/// | Module Name        | `github`             |
/// | Doc URL            | <https://github.com> |
/// | Requester          | [`HTTPClient`]       |
/// | Extractor          | [`RegexExtractor`]   |
/// | Generic            | [`None`]             |
pub struct GitHub {
    /// Module name
    pub name: String,
    /// Base index URL
    pub url: Url,
    /// Core components
    pub components: SubscanModuleCoreComponents,
}

impl GitHub {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let url = Url::parse(GITHUB_API_URL);

        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: RegexExtractor = RegexExtractor::default();

        let github = Self {
            name: GITHUB_MODULE_NAME.into(),
            url: url.unwrap(),
            components: SubscanModuleCoreComponents {
                requester: requester.into(),
                extractor: extractor.into(),
            },
        };

        github.into()
    }

    pub fn get_raw_url(&self, item: &Value) -> Option<Url> {
        if let Some(html_url) = item["html_url"].as_str() {
            let raw = html_url.replace(GITHUB_URL, GITHUB_RAW_URL);

            return raw.replace("/blob/", "/").parse().ok();
        }

        None
    }

    pub async fn get_html_urls(&self, content: Content) -> BTreeSet<Url> {
        if let Some(items) = content.as_json()["items"].as_array() {
            let filter = |item: &Value| self.get_raw_url(item);

            return items.iter().filter_map(filter).collect();
        }

        [].into()
    }
}

#[async_trait(?Send)]
impl SubscanModuleInterface for GitHub {
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
        let mut all_results = BTreeSet::new();

        let envs = self.envs().await;

        let params = &[
            ("per_page", "100"),
            ("q", domain),
            ("sort", "created"),
            ("order", "asc"),
        ];

        let requester = &mut *self.components.requester.lock().await;
        let extractor = &self.components.extractor;

        let rconfig = requester.config().await;
        let auth = format!("token {}", envs.apikey.value.unwrap_or_default());

        rconfig.add_header(AUTHORIZATION, HeaderValue::from_str(&auth).unwrap());

        let url = Url::parse_with_params(GITHUB_API_URL, params).unwrap();
        let content = requester.get_content(url).await;

        for raw_url in self.get_html_urls(content).await {
            let raw_content = requester.get_content(raw_url.clone()).await;

            all_results.extend(extractor.extract(raw_content, domain).await);
        }

        all_results
    }
}
