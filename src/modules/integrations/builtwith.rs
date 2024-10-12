use crate::{
    enums::{AuthenticationMethod, RequesterDispatcher, SubscanModuleDispatcher},
    extractors::json::JSONExtractor,
    modules::generics::integration::GenericIntegrationModule,
    requesters::client::HTTPClient,
    types::core::Subdomain,
};
use reqwest::Url;
use serde_json::Value;
use std::collections::BTreeSet;

pub const BUILTWITH_MODULE_NAME: &str = "builtwith";
pub const BUILTWITH_URL: &str = "https://api.builtwith.com/v21/api.json";

/// `BuiltWith` API integration module
///
/// It uses [`GenericIntegrationModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                                        |
/// |:------------------:|:--------------------------------------------:|
/// | Module Name        | `builtwith`                                  |
/// | Doc URL            | <https://api.builtwith.com>                  |
/// | Authentication     | [`AuthenticationMethod::APIKeyAsQueryParam`] |
/// | Requester          | [`HTTPClient`]                               |
/// | Extractor          | [`JSONExtractor`]                            |
pub struct BuiltWith {}

impl BuiltWith {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        let generic = GenericIntegrationModule {
            name: BUILTWITH_MODULE_NAME.into(),
            url: Box::new(Self::get_query_url),
            next: Box::new(Self::get_next_url),
            auth: AuthenticationMethod::APIKeyAsQueryParam("KEY".into()),
            requester: requester.into(),
            extractor: extractor.into(),
        };

        generic.into()
    }

    pub fn get_query_url(domain: &str) -> String {
        let params = &[
            ("HIDETEXT", "yes"),
            ("HIDEDL", "yes"),
            ("NOLIVE", "yes"),
            ("NOMETA", "yes"),
            ("NOPII", "yes"),
            ("NOATTR", "yes"),
            ("LOOKUP", domain),
        ];

        let url = Url::parse_with_params(BUILTWITH_URL, params);

        url.unwrap().to_string()
    }

    pub fn get_next_url(_url: Url, _content: Value) -> Option<Url> {
        None
    }

    pub fn extract(content: Value, domain: String) -> BTreeSet<Subdomain> {
        let mut subs = BTreeSet::new();

        if let Some(results) = content["Results"].as_array() {
            for result in results {
                if let Some(paths) = result["Result"]["Paths"].as_array() {
                    let filter = |item: &Value| {
                        let sub = item["SubDomain"].as_str()?.to_string();

                        Some(format!("{}.{}", sub, domain))
                    };

                    subs.extend(paths.iter().filter_map(filter));
                }
            }
        }

        subs
    }
}
