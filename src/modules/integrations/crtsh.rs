use crate::{
    enums::{AuthenticationMethod, Content, RequesterDispatcher, SubscanModuleDispatcher},
    extractors::json::JSONExtractor,
    modules::generics::integration::GenericIntegrationModule,
    requesters::client::HTTPClient,
    types::{
        core::{Subdomain, SubscanModuleCoreComponents},
        func::GenericIntegrationCoreFuncs,
    },
    utils::regex::generate_subdomain_regex,
};
use regex::Match;
use reqwest::Url;
use serde_json::Value;
use std::collections::BTreeSet;

pub const CRTSH_MODULE_NAME: &str = "crtsh";
pub const CRTSH_URL: &str = "https://crt.sh";

/// `Crt.sh` API integration module
///
/// It uses [`GenericIntegrationModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                                      |
/// |:------------------:|:------------------------------------------:|
/// | Module Name        | `crtsh`                                    |
/// | Doc URL            | <https://crt.sh>                           |
/// | Authentication     | [`AuthenticationMethod::NoAuthentication`] |
/// | Requester          | [`HTTPClient`]                             |
/// | Extractor          | [`JSONExtractor`]                          |
/// | Is Generic?        | [`GenericIntegrationModule`]               |
pub struct Crtsh {}

impl Crtsh {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        let generic = GenericIntegrationModule {
            name: CRTSH_MODULE_NAME.into(),
            auth: AuthenticationMethod::NoAuthentication,
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
        let params = &[("q", domain), ("output", "json")];
        let url = Url::parse_with_params(CRTSH_URL, params);

        url.unwrap().to_string()
    }

    pub fn get_next_url(_url: Url, _content: Content) -> Option<Url> {
        None
    }

    pub fn extract(content: Value, domain: &str) -> BTreeSet<Subdomain> {
        let mut subs = BTreeSet::new();
        let pattern = generate_subdomain_regex(domain).unwrap();

        if let Some(results) = content.as_array() {
            let matches = |item: &Value| {
                let to_string = |matched: Match| matched.as_str().to_string();

                pattern.find(item["name_value"].as_str()?).map(to_string)
            };

            subs.extend(results.iter().filter_map(matches));
        }

        subs
    }
}
