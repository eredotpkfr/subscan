use std::{collections::BTreeSet, io::Error};

use async_trait::async_trait;
use chrono::Datelike;
use flume::Sender;
use futures::TryStreamExt;
use reqwest::Url;
use serde_json::Value;
use tokio::{io::AsyncBufReadExt, sync::Mutex};
use tokio_util::io::StreamReader;

use crate::{
    enums::{
        dispatchers::{RequesterDispatcher, SubdomainExtractorDispatcher, SubscanModuleDispatcher},
        result::OptionalSubscanModuleResult,
    },
    extractors::regex::RegexExtractor,
    interfaces::{
        extractor::SubdomainExtractorInterface, module::SubscanModuleInterface,
        requester::RequesterInterface,
    },
    requesters::client::HTTPClient,
    types::{core::SubscanModuleCoreComponents, result::status::SubscanModuleStatus::Finished},
};

pub const COMMONCRAWL_MODULE_NAME: &str = "commoncrawl";
pub const COMMONCRAWL_INDEX_URL: &str = "https://index.commoncrawl.org/collinfo.json";

/// `CommonCrawl` non-generic integration module
///
/// | Property           | Value                                      |
/// |:------------------:|:------------------------------------------:|
/// | Module Name        | `commoncrawl`                              |
/// | Doc URL            | <https://commoncrawl.org>                  |
/// | Requester          | [`HTTPClient`]                             |
/// | Extractor          | [`RegexExtractor`]                         |
/// | Generic            | [`None`]                                   |
pub struct CommonCrawl {
    /// Module name
    pub name: String,
    /// Base index URL
    pub url: Url,
    /// Core components
    pub components: SubscanModuleCoreComponents,
}

impl CommonCrawl {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let url = Url::parse(COMMONCRAWL_INDEX_URL);

        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: RegexExtractor = RegexExtractor::default();

        let commoncrawl = Self {
            name: COMMONCRAWL_MODULE_NAME.into(),
            url: url.unwrap(),
            components: SubscanModuleCoreComponents {
                requester: requester.into(),
                extractor: extractor.into(),
            },
        };

        commoncrawl.into()
    }

    pub fn extract_cdx_urls(&self, json: Value, year: &str) -> Option<BTreeSet<String>> {
        if let Some(indexes) = json.as_array() {
            let filter = |item: &Value| {
                if item["id"].as_str()?.contains(year) {
                    Some(item["cdx-api"].as_str()?.to_string())
                } else {
                    None
                }
            };

            return Some(indexes.iter().filter_map(filter).collect());
        }

        None
    }
}

#[async_trait]
impl SubscanModuleInterface for CommonCrawl {
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
        let requester = self.requester().await.unwrap().lock().await;
        let extractor = self.extractor().await.unwrap();

        if let RequesterDispatcher::HTTPClient(requester) = &*requester {
            let year = chrono::Utc::now().year().to_string();
            let query = format!("*.{domain}");
            let content = requester.get_content(self.url.clone()).await;

            match content {
                Ok(content) => match self.extract_cdx_urls(content.as_json(), &year) {
                    Some(cdxs) => {
                        for cdx in cdxs {
                            let parsed = Url::parse_with_params(&cdx, &[("url", &query)]);

                            if let Ok(cdx_url) = parsed {
                                let rbuilder = requester
                                    .client
                                    .get(cdx_url)
                                    .timeout(requester.config.timeout)
                                    .headers(requester.config.headers.clone());

                                if let Ok(request) = rbuilder.build() {
                                    if let Ok(response) = requester.client.execute(request).await {
                                        let stream = response.bytes_stream().map_err(Error::other);
                                        let reader = StreamReader::new(stream);
                                        let mut lines = reader.lines();

                                        while let Ok(next_line) = lines.next_line().await {
                                            if let Some(line) = next_line {
                                                let subdomains =
                                                    extractor.extract(line.into(), domain).await;

                                                for subdomain in &subdomains.unwrap_or_default() {
                                                    results
                                                        .send(self.item(subdomain).await)
                                                        .unwrap();
                                                }
                                            } else {
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        results.send(self.status(Finished).await).unwrap();
                    }
                    None => results.send(self.error("not get cdx URLs").await).unwrap(),
                },
                Err(err) => results.send(self.status(err.into()).await).unwrap(),
            }
        } else {
            results.send(self.error("misconfigured requester").await).unwrap();
        }
    }
}
