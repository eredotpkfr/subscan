use crate::{
    enums::dispatchers::{
        RequesterDispatcher, SubdomainExtractorDispatcher, SubscanModuleDispatcher,
    },
    error::ModuleErrorKind::CustomError,
    extractors::regex::RegexExtractor,
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
use async_trait::async_trait;
use chrono::Datelike;
use futures::TryStreamExt;
use reqwest::Url;
use serde_json::Value;
use std::{collections::BTreeSet, io::Error};
use tokio::{io::AsyncBufReadExt, sync::Mutex};
use tokio_util::io::StreamReader;

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

    async fn run(&mut self, domain: &str) -> Result<SubscanModuleResult> {
        let mut result: SubscanModuleResult = self.name().await.into();

        let requester = self.requester().await.unwrap().lock().await;
        let extractor = self.extractor().await.unwrap();

        if let RequesterDispatcher::HTTPClient(requester) = &*requester {
            let year = chrono::Utc::now().year().to_string();
            let query = format!("*.{}", domain);
            let content = requester.get_content(self.url.clone()).await?;

            if let Some(cdxs) = self.extract_cdx_urls(content.as_json(), &year) {
                for cdx in cdxs {
                    let cdx_url = Url::parse_with_params(&cdx, &[("url", &query)]);
                    let request = requester
                        .client
                        .get(cdx_url.unwrap())
                        .timeout(requester.config.timeout)
                        .headers(requester.config.headers.clone())
                        .build()
                        .unwrap();

                    if let Ok(response) = requester.client.execute(request).await {
                        let stream = response.bytes_stream().map_err(Error::other);
                        let reader = StreamReader::new(stream);
                        let mut lines = reader.lines();

                        while let Ok(next_line) = lines.next_line().await {
                            if let Some(line) = next_line {
                                result.extend(extractor.extract(line.into(), domain).await?);
                            } else {
                                break;
                            }
                        }
                    } else {
                        continue;
                    }
                }
            } else {
                return Err(CustomError("not get cdx URLs".into()).into());
            }
        }

        Ok(result.with_finished().await)
    }
}
