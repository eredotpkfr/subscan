use reqwest::Url;
use scraper::{Html, Selector};

use crate::{
    enums::{
        auth::AuthenticationMethod,
        content::Content,
        dispatchers::{RequesterDispatcher, SubscanModuleDispatcher},
    },
    extractors::html::HTMLExtractor,
    modules::generics::integration::GenericIntegrationModule,
    requesters::chrome::ChromeBrowser,
    types::{core::SubscanModuleCoreComponents, func::GenericIntegrationCoreFuncs},
};

pub const NETCRAFT_MODULE_NAME: &str = "netcraft";
pub const NETCRAFT_URL: &str = "https://searchdns.netcraft.com";
pub const NETCRAFT_SUBDOMAIN_TAG: &str = "table > tbody > tr > td:nth-child(2) > a";
pub const NETCRAFT_NEXT_URL_TAG: &str = "table + p > a";

/// `Netcraft` API integration module
///
/// It uses [`GenericIntegrationModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                                      |
/// |:------------------:|:------------------------------------------:|
/// | Module Name        | `netcraft`                                 |
/// | Doc URL            | <https://searchdns.netcraft.com>           |
/// | Subdomain Selector | `table > tbody > tr > td:nth-child(2) > a` |
/// | Next URL Selector  | `table + p > a`                            |
/// | Authentication     | [`AuthenticationMethod::NoAuthentication`] |
/// | Requester          | [`ChromeBrowser`]                          |
/// | Extractor          | [`HTMLExtractor`]                          |
/// | Generic            | [`GenericIntegrationModule`]               |
pub struct Netcraft {}

impl Netcraft {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let selector: String = NETCRAFT_SUBDOMAIN_TAG.into();
        let removes: Vec<String> = vec!["<b>".into(), "</b>".into()];

        let requester: RequesterDispatcher = ChromeBrowser::new().into();
        let extractor: HTMLExtractor = HTMLExtractor::new(selector, removes);

        let generic = GenericIntegrationModule {
            name: NETCRAFT_MODULE_NAME.into(),
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
        let params = &[("restriction", "site+ends+with"), ("host", domain)];
        let url = Url::parse_with_params(NETCRAFT_URL, params);

        url.unwrap().to_string()
    }

    pub fn get_next_url(_url: Url, content: Content) -> Option<Url> {
        let document = Html::parse_document(&content.as_string());
        let selector = Selector::parse(NETCRAFT_NEXT_URL_TAG).ok()?;

        let mut selected = document.select(&selector);

        if let Some(href) = selected.next()?.attr("href") {
            Some(format!("{NETCRAFT_URL}{href}").parse().ok()?)
        } else {
            None
        }
    }
}
