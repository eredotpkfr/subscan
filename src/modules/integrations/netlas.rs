use crate::{
    enums::{
        RequesterDispatcher, SkipReason::NotAuthenticated, SubdomainExtractorDispatcher,
        SubscanModuleDispatcher, SubscanModuleStatus::Finished,
    },
    extractors::json::JSONExtractor,
    interfaces::{
        extractor::SubdomainExtractorInterface, module::SubscanModuleInterface,
        requester::RequesterInterface,
    },
    requesters::client::HTTPClient,
    types::core::{Subdomain, SubscanModuleCoreComponents, SubscanModuleResult},
};
use async_trait::async_trait;
use reqwest::{
    header::{HeaderName, HeaderValue},
    Url,
};
use serde_json::{json, Value};
use std::collections::BTreeSet;
use tokio::sync::Mutex;

pub const NETLAS_MODULE_NAME: &str = "netlas";
pub const NETLAS_URL: &str = "https://app.netlas.io";

/// `Netlas` non-generic integration module
///
/// | Property           | Value                |
/// |:------------------:|:--------------------:|
/// | Module Name        | `netlas`             |
/// | Doc URL            | <https://netlas.io>  |
/// | Requester          | [`HTTPClient`]       |
/// | Extractor          | [`JSONExtractor`]    |
/// | Generic            | [`None`]             |
pub struct Netlas {
    /// Module name
    pub name: String,
    /// Base index URL
    pub url: Url,
    /// Core components
    pub components: SubscanModuleCoreComponents,
}

impl Netlas {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let url = Url::parse(NETLAS_URL);

        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        let netlas = Self {
            name: NETLAS_MODULE_NAME.into(),
            url: url.unwrap(),
            components: SubscanModuleCoreComponents {
                requester: requester.into(),
                extractor: extractor.into(),
            },
        };

        netlas.into()
    }

    pub fn extract(content: Value, _domain: &str) -> BTreeSet<Subdomain> {
        if let Some(items) = content.as_array() {
            let filter = |item: &Value| Some(item["data"]["domain"].as_str()?.to_string());

            return items.iter().filter_map(filter).collect();
        }

        [].into()
    }
}

#[async_trait]
impl SubscanModuleInterface for Netlas {
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

        let requester = &mut *self.components.requester.lock().await;
        let extractor = &self.components.extractor;

        let apikey = self.envs().await.apikey.value.unwrap_or_default();
        let query = format!("domain:*.{domain} AND NOT domain:{domain}");

        requester.config().await.add_header(
            HeaderName::from_static("x-api-key"),
            HeaderValue::from_str(&apikey).unwrap(),
        );

        self.url.set_path("api/domains_count/");
        self.url.set_query(Some(&format!("q={query}")));

        let json = requester.get_content(self.url.clone()).await.as_json();
        let count = json["count"].as_i64();

        if let (Some(count), RequesterDispatcher::HTTPClient(requester)) = (count, requester) {
            self.url.set_query(None);
            self.url.set_path("api/domains/download/");

            let body = json!({
                "q": format!("domain:(domain:*.{domain} AND NOT domain:{domain})"),
                "fields": ["*"],
                "source_type": "include",
                "size": count
            });

            let request = requester
                .client
                .post(self.url.clone())
                .json(&body)
                .timeout(requester.config.timeout)
                .headers(requester.config.headers.clone())
                .build()
                .unwrap();

            if let Ok(response) = requester.client.execute(request).await {
                if let Ok(content) = response.text().await {
                    result.extend(extractor.extract(content.into(), domain).await);

                    return result.with_status(Finished).await;
                }
            }
        }

        result.with_status(NotAuthenticated.into()).await
    }
}
