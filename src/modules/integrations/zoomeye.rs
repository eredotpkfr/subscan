use std::collections::BTreeSet;

use crate::{
    enums::{
        auth::AuthenticationMethod,
        content::Content,
        dispatchers::{RequesterDispatcher, SubscanModuleDispatcher},
    },
    extractors::json::JSONExtractor,
    modules::generics::integration::GenericIntegrationModule,
    requesters::client::HTTPClient,
    types::{
        core::{Subdomain, SubscanModuleCoreComponents},
        func::GenericIntegrationCoreFuncs,
    },
    utils::http,
};
use reqwest::Url;
use serde_json::Value;

pub const ZOOMEYE_MODULE_NAME: &str = "zoomeye";
pub const ZOOMEYE_URL: &str = "https://api.zoomeye.hk/domain/search";

/// `ZoomEye` API integration module
///
/// It uses [`GenericIntegrationModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                                    |
/// |:------------------:|:----------------------------------------:|
/// | Module Name        | `zoomeye`                                |
/// | Doc URL            | <https://www.zoomeye.hk>                 |
/// | Authentication     | [`AuthenticationMethod::APIKeyAsHeader`] |
/// | Requester          | [`HTTPClient`]                           |
/// | Extractor          | [`JSONExtractor`]                        |
/// | Generic            | [`GenericIntegrationModule`]             |
pub struct ZoomEye {}

impl ZoomEye {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        let generic = GenericIntegrationModule {
            name: ZOOMEYE_MODULE_NAME.into(),
            auth: AuthenticationMethod::APIKeyAsHeader("API-Key".into()),
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
        let params = &[("q", domain), ("s", "250"), ("type", "1")];
        let url = Url::parse_with_params(ZOOMEYE_URL, params);

        url.unwrap().to_string()
    }

    pub fn get_next_url(mut url: Url, _content: Content) -> Option<Url> {
        let page_param = url.query_pairs().find(|item| item.0 == "page");

        if let Some(page) = page_param {
            let new_page = (page.1.parse::<usize>().unwrap() + 1).to_string();

            http::update_url_query(&mut url, "page", &new_page);
        } else {
            http::update_url_query(&mut url, "page", "2");
        }

        Some(url)
    }

    pub fn extract(content: Value, _domain: &str) -> BTreeSet<Subdomain> {
        if let Some(passives) = content["list"].as_array() {
            let filter = |item: &Value| Some(item["name"].as_str()?.to_string());

            return passives.iter().filter_map(filter).collect();
        }

        [].into()
    }
}
