use crate::extractors::{html::HTMLExtractor, json::JSONExtractor, regex::RegexExtractor};
use crate::requesters::{chrome::ChromeBrowser, client::HTTPClient};
use enum_dispatch::enum_dispatch;

/// Dispatcher enumeration to decide extractor types
///
/// It allows to made static type dispatching instead of
/// dynamic dispatch and speed up performance. For more
/// technical details please follow up `enum_dispatch` package
#[enum_dispatch(SubdomainExtractorInterface)]
pub enum SubdomainExtractorDispatcher {
    /// HTML extractor type to extract subdomain addresses
    /// from any HTML content. See the [`HTMLExtractor`]
    /// struct definition for examples and technical details
    HTMLExtractor(HTMLExtractor),
    /// Regex extractor type allows to extract subdomain
    /// addresses from string content with a regex pattern
    /// by given domain address. See the [`RegexExtractor`]
    /// for technical details and examples usages
    RegexExtractor(RegexExtractor),
    /// JSON extractor type can extract subdomains from
    /// JSON content. In this type head up point
    /// is to know that created as a wrapper
    /// struct to be compatible with
    /// [`SubdomainExtractorInterface`](crate::interfaces::extractor::SubdomainExtractorInterface)
    /// so parser method must be implemented and gave
    /// this wrapper struct. See the [`JSONExtractor`] struct
    /// and [`InnerExtractMethod`](crate::types::core::InnerExtractMethod)
    /// type for examples and technical details
    JSONExtractor(JSONExtractor),
}

/// Dispatcher enumeration to decide requester types
///
/// It allows to made static type dispatching instead of
/// dynamic dispatch and speed up performance. For more
/// technical details please follow up `enum_dispatch` package
#[enum_dispatch(RequesterInterface)]
pub enum RequesterDispatcher {
    /// Chrome browser struct definition as a enum value.
    /// On this requester type, Chrome browser will run and
    /// all HTTP requests made with browser. It has pros according
    /// to [`HTTPClient`] requester like running Js, rendering
    /// pages, etc. See the [`ChromeBrowser`] definition to learn
    /// usage
    ChromeBrowser(ChromeBrowser),
    /// Simple HTTP client interface to make requesters, it does
    /// not allows to run Js, rendering pages or user interface.
    /// Just send HTTP requests via [`reqwest`]. See the [`HTTPClient`]
    /// struct definition for examples and technical details
    HTTPClient(HTTPClient),
}

/// Authentication methods for API calls.
/// [`GenericAPIIntegrationModule`](crate::modules::generics::api_integration::GenericAPIIntegrationModule)
/// uses them to apply correct auth method. See the
/// method descriptions to learn how it works
pub enum AuthMethod {
    /// Some APIs uses request headers to get
    /// API key. If this auth type selected API key
    /// will add in request headers with a given header key
    APIKeyInHeader(String),
    /// This auth type uses when API require API
    /// key in URL
    APIKeyInURL,
    /// This auth type does nothing for auth
    NoAuth,
}
