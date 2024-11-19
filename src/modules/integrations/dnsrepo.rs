use reqwest::Url;

use crate::{
    enums::{
        auth::AuthenticationMethod,
        content::Content,
        dispatchers::{RequesterDispatcher, SubscanModuleDispatcher},
    },
    extractors::html::HTMLExtractor,
    modules::generics::integration::GenericIntegrationModule,
    requesters::client::HTTPClient,
    types::{core::SubscanModuleCoreComponents, func::GenericIntegrationCoreFuncs},
};

pub const DNSREPO_MODULE_NAME: &str = "dnsrepo";
pub const DNSREPO_URL: &str = "https://dnsrepo.noc.org";
pub const DNSREPO_SUBDOMAIN_TAG: &str = "table > tbody > tr > td:first-child > a:first-child";

/// `DnsRepo` HTML crawler integration
///
/// It uses [`GenericIntegrationModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                                                 |
/// |:------------------:|:-----------------------------------------------------:|
/// | Module Name        | `dnsrepo`                                             |
/// | Doc URL            | <https://dnsrepo.noc.org>                             |
/// | Subdomain Selector | `table > tbody > tr > td:first-child > a:first-child` |
/// | Authentication     | [`AuthenticationMethod::NoAuthentication`]            |
/// | Requester          | [`HTTPClient`]                                        |
/// | Extractor          | [`HTMLExtractor`]                                     |
/// | Generic            | [`GenericIntegrationModule`]                          |
pub struct DnsRepo {}

impl DnsRepo {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let selector: String = DNSREPO_SUBDOMAIN_TAG.into();
        let removes: Vec<String> = vec!["<b>".into(), "</b>".into()];

        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: HTMLExtractor = HTMLExtractor::new(selector, removes);

        let generic = GenericIntegrationModule {
            name: DNSREPO_MODULE_NAME.into(),
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
        format!("{DNSREPO_URL}/?search={domain}")
    }

    pub fn get_next_url(_url: Url, _content: Content) -> Option<Url> {
        None
    }
}
