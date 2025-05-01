use std::collections::BTreeSet;

use async_trait::async_trait;
use flume::Sender;
use reqwest::{
    header::{HeaderName, HeaderValue},
    Response, Url,
};
use serde_json::{json, Value};
use tokio::sync::Mutex;

use crate::{
    enums::{
        dispatchers::{RequesterDispatcher, SubdomainExtractorDispatcher, SubscanModuleDispatcher},
        result::SubscanModuleResult,
    },
    error::ModuleErrorKind::JSONExtract,
    extractors::json::JSONExtractor,
    interfaces::{
        extractor::SubdomainExtractorInterface, module::SubscanModuleInterface,
        requester::RequesterInterface,
    },
    requesters::client::HTTPClient,
    types::{
        core::{Result, Subdomain, SubscanModuleCoreComponents},
        result::status::{SkipReason::AuthenticationNotProvided, SubscanModuleStatus::Finished},
    },
};

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

    pub fn extract(content: Value, _domain: &str) -> Result<BTreeSet<Subdomain>> {
        if let Some(items) = content.as_array() {
            let filter = |item: &Value| Some(item["data"]["domain"].as_str()?.to_string());

            return Ok(items.iter().filter_map(filter).collect());
        }

        Err(JSONExtract.into())
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

    async fn run(&mut self, domain: &str, results: Sender<Option<SubscanModuleResult>>) {
        let mut url = self.url.clone();

        let requester = &mut *self.requester().await.unwrap().lock().await;
        let extractor = self.extractor().await.unwrap();

        let apikey = self.envs().await.apikey.value.unwrap_or_default();
        let query = format!("domain:*.{domain} AND NOT domain:{domain}");

        requester.config().await.add_header(
            HeaderName::from_static("x-api-key"),
            HeaderValue::from_str(&apikey).unwrap(),
        );

        url.set_path("api/domains_count/");
        url.set_query(Some(&format!("q={query}")));

        let json = requester
            .get_content(url.clone())
            .await
            .unwrap_or_default()
            .as_json();
        let count = json["count"].as_i64();

        if let (Some(count), RequesterDispatcher::HTTPClient(requester)) = (count, requester) {
            url.set_query(None);
            url.set_path("api/domains/download/");

            let body = json!({
                "q": format!("domain:(domain:*.{domain} AND NOT domain:{domain})"),
                "fields": ["*"],
                "source_type": "include",
                "size": count
            });

            let rbuilder = requester
                .client
                .post(url)
                .json(&body)
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

        results.send(AuthenticationNotProvided.into()).unwrap();
    }
}
