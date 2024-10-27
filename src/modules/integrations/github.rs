use crate::{
    enums::{
        Content, RequesterDispatcher,
        SkipReason::NotAuthenticated,
        SubdomainExtractorDispatcher, SubscanModuleDispatcher,
        SubscanModuleStatus::{Failed, Finished},
    },
    extractors::regex::RegexExtractor,
    interfaces::{
        extractor::SubdomainExtractorInterface, module::SubscanModuleInterface,
        requester::RequesterInterface,
    },
    requesters::client::HTTPClient,
    types::core::{SubscanModuleCoreComponents, SubscanModuleResult},
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

    pub async fn get_html_urls(&self, content: Content) -> Option<BTreeSet<Url>> {
        if let Some(items) = content.as_json()["items"].as_array() {
            let filter = |item: &Value| self.get_raw_url(item);

            return Some(items.iter().filter_map(filter).collect());
        }

        None
    }
}

#[async_trait]
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

    async fn run(&mut self, domain: &str) -> SubscanModuleResult {
        let mut result: SubscanModuleResult = self.name().await.into();

        let envs = self.envs().await;

        if let Some(apikey) = envs.apikey.value {
            let query = format!("per_page=100&q={domain}&sort=created&order=asc");

            let requester = &mut *self.components.requester.lock().await;
            let extractor = &self.components.extractor;

            let rconfig = requester.config().await;
            let auth = HeaderValue::from_str(&format!("token {}", apikey));

            rconfig.add_header(AUTHORIZATION, auth.unwrap());
            self.url.set_query(Some(&query));

            let content = requester.get_content(self.url.clone()).await;

            if let Some(raws) = self.get_html_urls(content).await {
                for raw_url in raws {
                    let raw_content = requester.get_content(raw_url.clone()).await;

                    result.extend(extractor.extract(raw_content, domain).await);
                }

                return result.with_status(Finished).await;
            }

            return result.with_status(Failed("not get raw URLs".into())).await;
        }

        result.with_status(NotAuthenticated.into()).await
    }
}
