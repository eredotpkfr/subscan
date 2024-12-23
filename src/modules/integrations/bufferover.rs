use std::collections::BTreeSet;

use regex::Match;
use reqwest::Url;
use serde_json::Value;

use crate::{
    enums::{
        auth::AuthenticationMethod,
        content::Content,
        dispatchers::{RequesterDispatcher, SubscanModuleDispatcher},
    },
    error::ModuleErrorKind::JSONExtract,
    extractors::json::JSONExtractor,
    modules::generics::integration::GenericIntegrationModule,
    requesters::client::HTTPClient,
    types::{
        core::{Result, Subdomain, SubscanModuleCoreComponents},
        func::GenericIntegrationCoreFuncs,
    },
    utilities::regex::generate_subdomain_regex,
};

pub const BUFFEROVER_MODULE_NAME: &str = "bufferover";
pub const BUFFEROVER_URL: &str = "https://tls.bufferover.run";

/// `BufferOver` API integration module
///
/// It uses [`GenericIntegrationModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                                    |
/// |:------------------:|:----------------------------------------:|
/// | Module Name        | `bufferover`                             |
/// | Doc URL            | <https://tls.bufferover.run>             |
/// | Authentication     | [`AuthenticationMethod::APIKeyAsHeader`] |
/// | Requester          | [`HTTPClient`]                           |
/// | Extractor          | [`JSONExtractor`]                        |
/// | Generic            | [`GenericIntegrationModule`]             |
pub struct BufferOver {}

impl BufferOver {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        let generic = GenericIntegrationModule {
            name: BUFFEROVER_MODULE_NAME.into(),
            auth: AuthenticationMethod::APIKeyAsHeader("X-API-Key".into()),
            funcs: GenericIntegrationCoreFuncs {
                url: Box::new(Self::get_query_url),
                next: Box::new(Self::get_next_url),
            },
            components: SubscanModuleCoreComponents {
                requester: requester.into(),
                extractor: extractor.into(),
            },
        };

        generic.into()
    }

    pub fn get_query_url(domain: &str) -> String {
        format!("{BUFFEROVER_URL}/dns?q={domain}")
    }

    pub fn get_next_url(_url: Url, _content: Content) -> Option<Url> {
        None
    }

    pub fn extract(content: Value, domain: &str) -> Result<BTreeSet<Subdomain>> {
        if let Some(results) = content["Results"].as_array() {
            let pattern = generate_subdomain_regex(domain)?;
            let filter = |item: &Value| {
                let to_string = |matches: Match| matches.as_str().to_string();

                pattern.find(item.as_str()?).map(to_string)
            };

            return Ok(results.iter().filter_map(filter).collect());
        }

        Err(JSONExtract.into())
    }
}
