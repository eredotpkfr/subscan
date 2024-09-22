use crate::{
    enums::{AuthMethod, RequesterDispatcher},
    extractors::json::JSONExtractor,
    modules::generics::api_integration::GenericAPIIntegrationModule,
    requesters::client::HTTPClient,
    types::core::Subdomain,
};
use serde_json::Value;
use std::collections::BTreeSet;

/// Bevigil API integration module
///
/// It uses [`GenericAPIIntegrationModule`] its own inner
/// here are the configurations
pub struct Bevigil {}

pub const BEVIGIL_MODULE_NAME: &str = "Bevigil";
pub const BEVIGIL_URL: &str = "https://osint.bevigil.com/api";

impl Bevigil {
    /// Create a new [`Bevigil`] module instance
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::modules::integrations::bevigil;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let bevigil = bevigil::Bevigil::new();
    ///
    ///     // do something with bevigil instance
    /// }
    /// ```
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> GenericAPIIntegrationModule {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        GenericAPIIntegrationModule {
            name: BEVIGIL_MODULE_NAME.into(),
            url: Box::new(Self::get_query_url),
            auth: AuthMethod::APIKeyInHeader("X-Access-Token".into()),
            requester: requester.into(),
            extractor: extractor.into(),
        }
    }

    /// Get Bevigil query URL from given domain address
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::modules::integrations::bevigil::{self, BEVIGIL_URL};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let domain = "foo.com";
    ///     let url = bevigil::Bevigil::get_query_url(&domain);
    ///
    ///     assert_eq!(url, format!("{BEVIGIL_URL}/{domain}/subdomains"));
    /// }
    /// ```
    pub fn get_query_url(domain: &str) -> String {
        format!("{BEVIGIL_URL}/{domain}/subdomains")
    }

    /// JSON parse method to extract subdomains
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::modules::integrations::bevigil;
    /// use std::collections::BTreeSet;
    /// use serde_json::Value;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let result = bevigil::Bevigil::extract(Value::default());
    ///
    ///     assert_eq!(result, BTreeSet::new());
    /// }
    /// ```
    pub fn extract(content: Value) -> BTreeSet<Subdomain> {
        if let Some(subs) = content["subdomains"].as_array() {
            let filter = |item: &Value| Some(item.as_str()?.to_string());

            BTreeSet::from_iter(subs.iter().filter_map(filter))
        } else {
            BTreeSet::new()
        }
    }
}
