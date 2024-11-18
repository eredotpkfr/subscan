use crate::{
    enums::{
        auth::AuthenticationMethod,
        content::Content,
        dispatchers::{RequesterDispatcher, SubscanModuleDispatcher},
    },
    error::{ModuleErrorKind::JSONExtract, SubscanError},
    extractors::json::JSONExtractor,
    modules::generics::integration::GenericIntegrationModule,
    requesters::client::HTTPClient,
    types::{
        core::{Result, Subdomain, SubscanModuleCoreComponents},
        func::GenericIntegrationCoreFuncs,
    },
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
/// | Generic            | [`GenericIntegrationModule`]                 |
pub struct BuiltWith {}

impl BuiltWith {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        let generic = GenericIntegrationModule {
            name: BUILTWITH_MODULE_NAME.into(),
            auth: AuthenticationMethod::APIKeyAsQueryParam("KEY".into()),
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

    pub fn get_next_url(_url: Url, _content: Content) -> Option<Url> {
        None
    }

    pub fn extract(content: Value, domain: &str) -> Result<BTreeSet<Subdomain>> {
        let mut subs = BTreeSet::new();
        let filter = |item: &Value| Some(format!("{}.{}", item["SubDomain"].as_str()?, domain));
        let results = content["Results"]
            .as_array()
            .ok_or(SubscanError::from(JSONExtract))?;

        for result in results {
            if let Some(paths) = result["Result"]["Paths"].as_array() {
                subs.extend(paths.iter().filter_map(filter));
            }
        }

        Ok(subs)
    }
}
