use crate::{
    enums::{
        auth::AuthenticationMethod,
        content::Content,
        dispatchers::{RequesterDispatcher, SubscanModuleDispatcher},
    },
    error::{ModuleErrorKind::JSONExtractError, SubscanError},
    extractors::json::JSONExtractor,
    modules::generics::integration::GenericIntegrationModule,
    requesters::client::HTTPClient,
    types::{
        core::{Result, Subdomain, SubscanModuleCoreComponents},
        func::GenericIntegrationCoreFuncs,
    },
    utilities::regex::generate_subdomain_regex,
};
use regex::Match;
use reqwest::Url;
use serde_json::Value;
use std::collections::BTreeSet;

pub const CERTSPOTTER_MODULE_NAME: &str = "certspotter";
pub const CERTSPOTTER_URL: &str = "https://api.certspotter.com/v1/issuances";

/// `CertSpotter` API integration module
///
/// It uses [`GenericIntegrationModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                                    |
/// |:------------------:|:----------------------------------------:|
/// | Module Name        | `certspotter`                            |
/// | Doc URL            | <https://sslmate.com/certspotter>        |
/// | Authentication     | [`AuthenticationMethod::APIKeyAsHeader`] |
/// | Requester          | [`HTTPClient`]                           |
/// | Extractor          | [`JSONExtractor`]                        |
/// | Generic            | [`GenericIntegrationModule`]             |
pub struct CertSpotter {}

impl CertSpotter {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        let generic = GenericIntegrationModule {
            name: CERTSPOTTER_MODULE_NAME.into(),
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
        let params = &[
            ("domain", domain),
            ("include_subdomains", "true"),
            ("expand", "dns_names"),
        ];

        let url = Url::parse_with_params(CERTSPOTTER_URL, params);

        url.unwrap().to_string()
    }

    pub fn get_next_url(_url: Url, _content: Content) -> Option<Url> {
        None
    }

    pub fn extract(content: Value, domain: &str) -> Result<BTreeSet<Subdomain>> {
        let mut subs = BTreeSet::new();

        let pattern = generate_subdomain_regex(domain).unwrap();
        let matches = |item: &Value| {
            let to_string = |matched: Match| matched.as_str().to_string();

            pattern.find(item.as_str()?).map(to_string)
        };

        let results = content
            .as_array()
            .ok_or(SubscanError::from(JSONExtractError))?;

        for result in results {
            if let Some(names) = result["dns_names"].as_array() {
                subs.extend(names.iter().filter_map(matches));
            }
        }

        Ok(subs)
    }
}
