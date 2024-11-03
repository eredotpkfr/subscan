use crate::{
    enums::{content::Content, dispatchers::RequesterDispatcher},
    requesters::{chrome::ChromeBrowser, client::HTTPClient},
    types::config::RequesterConfig,
};
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use reqwest::Url;

/// Generic HTTP client trait definition to implement different
/// HTTP requester objects with a single interface compatible
///
/// Other requesters that will be implemented in the future
/// must conform to this interface. Mostly use to get
/// string content from any URL with a single stupid `get_content`
/// method
///
/// # Examples
///
/// ```
/// use std::time::Duration;
/// use subscan::interfaces::requester::RequesterInterface;
/// use subscan::types::config::RequesterConfig;
/// use subscan::enums::content::Content;
/// use subscan::constants::DEFAULT_HTTP_TIMEOUT;
/// use reqwest::Url;
/// use async_trait::async_trait;
/// use serde_json::Value;
///
/// pub struct CustomRequester {
///     config: RequesterConfig
/// }
///
/// #[async_trait]
/// impl RequesterInterface for CustomRequester {
///     async fn config(&mut self) -> &mut RequesterConfig {
///         &mut self.config
///     }
///
///     async fn configure(&mut self, config: RequesterConfig) {
///         self.config = config;
///     }
///
///     async fn get_content(&self, url: Url) -> Content {
///         Content::from("foo")
///     }
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let url: Url = "https://foo.com".parse().unwrap();
///
///     let mut requester = CustomRequester {
///         config: RequesterConfig::default(),
///     };
///
///     let config = requester.config().await.clone();
///     let content = requester.get_content(url).await;
///
///     assert_eq!(content.as_string(), "foo");
///     assert_eq!(config.proxy, None);
///     assert_eq!(config.timeout, DEFAULT_HTTP_TIMEOUT);
///     assert_eq!(config.headers.len(), 0);
///
///     // Configure with new config instance
///     let new_config = RequesterConfig {
///         timeout: Duration::from_secs(120),
///         ..Default::default()
///     };
///
///     requester.configure(new_config).await;
///
///     assert_eq!(requester.config().await.timeout.as_secs(), 120);
/// }
/// ```
#[async_trait]
#[enum_dispatch]
pub trait RequesterInterface: Sync + Send {
    /// Returns requester configurations as a [`RequesterConfig`] object
    async fn config(&mut self) -> &mut RequesterConfig;
    /// Configure current requester object by using new [`RequesterConfig`] object
    async fn configure(&mut self, config: RequesterConfig);
    /// HTTP GET method implementation to fetch HTML content from given source [`Url`]
    async fn get_content(&self, url: Url) -> Content;
}
