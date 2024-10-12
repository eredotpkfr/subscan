use crate::{
    enums::{AuthenticationMethod, RequesterDispatcher, SubscanModuleDispatcher},
    extractors::json::JSONExtractor,
    modules::generics::integration::GenericIntegrationModule,
    requesters::client::HTTPClient,
    types::{
        core::{Subdomain, SubscanModuleCoreComponents},
        func::GenericIntegrationCoreFuncs,
    },
    utils::{http, regex::generate_subdomain_regex},
};
use regex::Match;
use reqwest::Url;
use serde_json::Value;
use std::collections::BTreeSet;

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
pub struct Censys {}

impl Censys {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        let generic = GenericIntegrationModule {
            name: CENSYS_MODULE_NAME.into(),
            funcs: GenericIntegrationCoreFuncs {
                url: Box::new(Self::get_query_url),
                next: Box::new(Self::get_next_url),
                request: None,
            },
            auth: AuthenticationMethod::APIKeyAsHeader("Authorization".into()),
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

    pub fn get_next_url(mut url: Url, content: Value) -> Option<Url> {
        if let Some(cursor) = content["result"]["links"]["next"].as_str() {
            http::update_url_query(&mut url, "cursor", cursor);
            Some(url)
        } else {
            None
        }
    }

    pub fn extract(content: Value, domain: &str) -> BTreeSet<Subdomain> {
        let mut subs = BTreeSet::new();

        if let Some(hits) = content["result"]["hits"].as_array() {
            let pattern = generate_subdomain_regex(domain).unwrap();

            for result in hits {
                if let Some(names) = result["names"].as_array() {
                    let matches = |item: &Value| {
                        let to_string = |matched: Match| matched.as_str().to_string();

                        pattern.find(item.as_str()?).map(to_string)
                    };

                    subs.extend(names.iter().filter_map(matches));
                }
            }
        }

        subs
    }
}
