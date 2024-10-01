use crate::{
    enums::{APIAuthMethod, RequesterDispatcher},
    extractors::json::JSONExtractor,
    modules::generics::api_integration::GenericAPIIntegrationModule,
    requesters::client::HTTPClient,
    types::core::Subdomain,
};
use reqwest::Url;
use serde_json::Value;
use std::collections::BTreeSet;

/// Builtwith API integration module
///
/// It uses [`GenericAPIIntegrationModule`] its own inner
/// here are the configurations
pub struct Builtwith {}

pub const BUILTWITH_MODULE_NAME: &str = "Builtwith";
pub const BUILTWITH_URL: &str = "https://api.builtwith.com/v21/api.json";

impl Builtwith {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> GenericAPIIntegrationModule {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        GenericAPIIntegrationModule {
            name: BUILTWITH_MODULE_NAME.into(),
            url: Box::new(Self::get_query_url),
            next: Box::new(move |_, _| None),
            auth: APIAuthMethod::APIKeyAsQueryParam("KEY".into()),
            requester: requester.into(),
            extractor: extractor.into(),
        }
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

    pub fn extract(content: Value, domain: String) -> BTreeSet<Subdomain> {
        if let Some(results) = content["Results"].as_array() {
            let mut subs = BTreeSet::new();

            for result in results {
                if let Some(paths) = result["Result"]["Paths"].as_array() {
                    let filter = |item: &Value| {
                        let sub = item["SubDomain"].as_str()?.to_string();

                        Some(format!("{}.{}", sub, domain))
                    };

                    subs.extend(paths.iter().filter_map(filter));
                }
            }
            subs
        } else {
            BTreeSet::new()
        }
    }
}
