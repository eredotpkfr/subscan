use super::funcs::wrap_url_with_mock_func;
use crate::common::utils::md5_hex;
use reqwest::Url;
use serde_json::Value;
use std::{collections::BTreeSet, thread};
use subscan::{
    enums::{auth::AuthenticationMethod, dispatchers::RequesterDispatcher},
    extractors::{json::JSONExtractor, regex::RegexExtractor},
    modules::generics::{engine::GenericSearchEngineModule, integration::GenericIntegrationModule},
    requesters::client::HTTPClient,
    types::{
        core::SubscanModuleCoreComponents, func::GenericIntegrationCoreFuncs,
        query::SearchQueryParam,
    },
};

pub fn generic_search_engine(url: &str) -> GenericSearchEngineModule {
    let requester = RequesterDispatcher::HTTPClient(HTTPClient::default());
    let extractor = RegexExtractor::default();

    let url = Url::parse(url);
    let thread_name = thread::current().name().unwrap().to_uppercase();

    GenericSearchEngineModule {
        name: md5_hex(thread_name),
        url: url.unwrap(),
        param: SearchQueryParam::from("q"),
        components: SubscanModuleCoreComponents {
            requester: requester.into(),
            extractor: extractor.into(),
        },
    }
}

pub fn generic_integration(url: &str, auth: AuthenticationMethod) -> GenericIntegrationModule {
    let parse = |json: Value, _domain: &str| {
        if let Some(subs) = json["subdomains"].as_array() {
            let filter = |item: &Value| Some(item.as_str()?.to_string());

            BTreeSet::from_iter(subs.iter().filter_map(filter))
        } else {
            BTreeSet::new()
        }
    };

    let requester = RequesterDispatcher::HTTPClient(HTTPClient::default());
    let extractor = JSONExtractor::new(Box::new(parse));
    let thread_name = thread::current().name().unwrap().to_uppercase();

    GenericIntegrationModule {
        name: md5_hex(thread_name),
        auth,
        funcs: GenericIntegrationCoreFuncs {
            url: wrap_url_with_mock_func(url),
            next: Box::new(|_, _| None),
        },
        components: SubscanModuleCoreComponents {
            requester: requester.into(),
            extractor: extractor.into(),
        },
    }
}
