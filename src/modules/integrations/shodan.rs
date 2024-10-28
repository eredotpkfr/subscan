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
use std::collections::BTreeSet;

pub const SHODAN_MODULE_NAME: &str = "shodan";
pub const SHODAN_URL: &str = "https://api.shodan.io";

/// `Shodan` API integration module
///
/// It uses [`GenericIntegrationModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                                        |
/// |:------------------:|:--------------------------------------------:|
/// | Module Name        | `shodan`                                     |
/// | Doc URL            | <https://shodan.io>                          |
/// | Authentication     | [`AuthenticationMethod::APIKeyAsQueryParam`] |
/// | Requester          | [`HTTPClient`]                               |
/// | Extractor          | [`JSONExtractor`]                            |
/// | Generic            | [`GenericIntegrationModule`]                 |
pub struct Shodan {}

impl Shodan {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        let generic = GenericIntegrationModule {
            name: SHODAN_MODULE_NAME.into(),
            auth: AuthenticationMethod::APIKeyAsQueryParam("key".into()),
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
        format!("{SHODAN_URL}/dns/domain/{domain}")
    }

    pub fn get_next_url(mut url: Url, content: Content) -> Option<Url> {
        if let Some(more) = content.as_json()["more"].as_bool() {
            if !more {
                None
            } else {
                let page_param = url.query_pairs().find(|item| item.0 == "page");

                if let Some(page) = page_param {
                    let new_page = (page.1.parse::<usize>().unwrap() + 1).to_string();

                    http::update_url_query(&mut url, "page", &new_page);
                } else {
                    http::update_url_query(&mut url, "page", "2");
                }

                Some(url)
            }
        } else {
            None
        }
    }

    pub fn extract(content: Value, domain: &str) -> BTreeSet<Subdomain> {
        if let Some(subs) = content["subdomains"].as_array() {
            let filter = |item: &Value| Some(format!("{}.{domain}", item.as_str()?));

            return subs.iter().filter_map(filter).collect();
        }

        [].into()
    }
}
