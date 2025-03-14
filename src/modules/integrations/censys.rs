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
    utilities::{http, regex::generate_subdomain_regex},
};

pub const CENSYS_MODULE_NAME: &str = "censys";
pub const CENSYS_URL: &str = "https://search.censys.io/api/v2/certificates/search";

/// `Censys` API integration module
///
/// It uses [`GenericIntegrationModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                                    |
/// |:------------------:|:----------------------------------------:|
/// | Module Name        | `censys`                                 |
/// | Doc URL            | <https://search.censys.io>               |
/// | Authentication     | [`AuthenticationMethod::APIKeyAsHeader`] |
/// | Requester          | [`HTTPClient`]                           |
/// | Extractor          | [`JSONExtractor`]                        |
/// | Generic            | [`GenericIntegrationModule`]             |
pub struct Censys {}

impl Censys {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        let generic = GenericIntegrationModule {
            name: CENSYS_MODULE_NAME.into(),
            auth: AuthenticationMethod::APIKeyAsHeader("Authorization".into()),
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
        format!("{CENSYS_URL}?q={domain}")
    }

    pub fn get_next_url(mut url: Url, content: Content) -> Option<Url> {
        if let Some(cursor) = content.as_json()["result"]["links"]["next"].as_str() {
            http::update_url_query(&mut url, "cursor", cursor);
            Some(url)
        } else {
            None
        }
    }

    pub fn extract(content: Value, domain: &str) -> Result<BTreeSet<Subdomain>> {
        let mut subdomains = BTreeSet::new();

        let pattern = generate_subdomain_regex(domain)?;
        let matches = |item: &Value| {
            let to_string = |matched: Match| matched.as_str().to_string();

            pattern.find(item.as_str()?).map(to_string)
        };

        let hits = content["result"]["hits"].as_array().ok_or(JSONExtract)?;

        for result in hits {
            if let Some(names) = result["names"].as_array() {
                subdomains.extend(names.iter().filter_map(matches));
            }
        }

        Ok(subdomains)
    }
}
