use crate::{
    enums::dispatchers::{
        RequesterDispatcher, SubdomainExtractorDispatcher, SubscanModuleDispatcher,
    },
    error::{ModuleErrorKind::GetContentError, SubscanError},
    extractors::regex::RegexExtractor,
    interfaces::{extractor::SubdomainExtractorInterface, module::SubscanModuleInterface},
    requesters::client::HTTPClient,
    types::{
        core::{Result, SubscanModuleCoreComponents},
        result::module::SubscanModuleResult,
    },
};
use async_trait::async_trait;
use futures::TryStreamExt;
use reqwest::Url;
use std::io::Error;
use tokio::{io::AsyncBufReadExt, sync::Mutex};
use tokio_util::io::StreamReader;

pub const WAYBACKARCHIVE_MODULE_NAME: &str = "waybackarchive";
pub const WAYBACKARCHIVE_URL: &str = "http://web.archive.org/cdx/search/cdx";

/// `WaybackArchive` non-generic integration module
///
/// | Property           | Value                                      |
/// |:------------------:|:------------------------------------------:|
/// | Module Name        | `waybackarchive`                           |
/// | Doc URL            | <https://web.archive.org>                  |
/// | Requester          | [`HTTPClient`]                             |
/// | Extractor          | [`RegexExtractor`]                         |
/// | Generic            | [`None`]                                   |
pub struct WaybackArchive {
    /// Module name
    pub name: String,
    /// Base index URL
    pub url: Url,
    /// Core components
    pub components: SubscanModuleCoreComponents,
}

impl WaybackArchive {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let url = Url::parse(WAYBACKARCHIVE_URL);

        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: RegexExtractor = RegexExtractor::default();

        let waybackarchive = Self {
            name: WAYBACKARCHIVE_MODULE_NAME.into(),
            url: url.unwrap(),
            components: SubscanModuleCoreComponents {
                requester: requester.into(),
                extractor: extractor.into(),
            },
        };

        waybackarchive.into()
    }
}

#[async_trait]
impl SubscanModuleInterface for WaybackArchive {
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
        let mut url = self.url.clone();
        let mut result: SubscanModuleResult = self.name().await.into();

        let requester = &*self.requester().await.unwrap().lock().await;
        let extractor = self.extractor().await.unwrap();

        if let RequesterDispatcher::HTTPClient(requester) = requester {
            let query = format!("url=*.{domain}/*&output=txt&fl=original&collapse=urlkey");

            url.set_query(Some(&query));

            let request = requester
                .client
                .get(url)
                .timeout(requester.config.timeout)
                .headers(requester.config.headers.clone())
                .build()
                .unwrap();

            let response = requester
                .client
                .execute(request)
                .await
                .map_err(|_| SubscanError::from(GetContentError))?;

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
        }

        Ok(result.with_finished().await)
    }
}
