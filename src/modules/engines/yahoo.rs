use crate::{
    enums::{RequesterDispatcher, SubscanModuleDispatcher},
    extractors::html::HTMLExtractor,
    modules::generics::search_engine::GenericSearchEngineModule,
    requesters::client::HTTPClient,
};
use reqwest::Url;

pub const YAHOO_MODULE_NAME: &str = "Yahoo";
pub const YAHOO_SEARCH_URL: &str = "https://search.yahoo.com/search";
pub const YAHOO_SEARCH_PARAM: &str = "p";
pub const YAHOO_CITE_TAG: &str = "ol > li > div > div > h3 > a > span";

/// Yahoo search engine enumerator
///
/// It uses [`GenericSearchEngineModule`] its own inner
/// here are the configurations
///
/// | Property           | Value                                 |
/// |:------------------:|:-------------------------------------:|
/// | Module Name        | `Yahoo`                               |
/// | Search URL         | <https://search.yahoo.com/search>     |
/// | Search Param       | `p`                                   |
/// | Subdomain Selector | `ol > li > div > div > h3 > a > span` |
pub struct Yahoo {}

impl Yahoo {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let url = Url::parse(YAHOO_SEARCH_URL);
        let removes = vec![String::from("<b>"), String::from("</b>")];

        let extractor: HTMLExtractor = HTMLExtractor::new(YAHOO_CITE_TAG.into(), removes);
        let requester: RequesterDispatcher = HTTPClient::default().into();

        let generic = GenericSearchEngineModule {
            name: YAHOO_MODULE_NAME.into(),
            param: YAHOO_SEARCH_PARAM.into(),
            url: url.unwrap(),
            requester: requester.into(),
            extractor: extractor.into(),
        };

        generic.into()
    }
}
