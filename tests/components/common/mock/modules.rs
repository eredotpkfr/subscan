use std::collections::BTreeSet;

use reqwest::Url;
use serde_json::Value;
use subscan::{
    enums::{auth::AuthenticationMethod, dispatchers::RequesterDispatcher},
    error::ModuleErrorKind::JSONExtract,
    extractors::{json::JSONExtractor, regex::RegexExtractor},
    modules::generics::{engine::GenericSearchEngineModule, integration::GenericIntegrationModule},
    requesters::client::HTTPClient,
    types::{
        core::SubscanModuleCoreComponents, func::GenericIntegrationCoreFuncs,
        query::SearchQueryParam,
    },
};

use super::funcs::wrap_url_with_mock_func;
use crate::common::utils::current_thread_hex;

pub fn generic_search_engine(url: &str) -> GenericSearchEngineModule {
    let requester = RequesterDispatcher::HTTPClient(HTTPClient::default());
    let extractor = RegexExtractor::default();
    let url = Url::parse(url);

    GenericSearchEngineModule {
        name: current_thread_hex(),
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
        if let Some(subdomains) = json["subdomains"].as_array() {
            let filter = |item: &Value| Some(item.as_str()?.to_string());

            return Ok(BTreeSet::from_iter(subdomains.iter().filter_map(filter)));
        }

        Err(JSONExtract.into())
    };

    let requester = RequesterDispatcher::HTTPClient(HTTPClient::default());
    let extractor = JSONExtractor::new(Box::new(parse));

    GenericIntegrationModule {
        name: current_thread_hex(),
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
