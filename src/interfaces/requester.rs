use crate::{
    enums::RequesterDispatcher,
    requesters::{chrome::ChromeBrowser, client::HTTPClient},
    types::config::RequesterConfig,
};
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use reqwest::Url;
use serde_json::Value;

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
/// use reqwest::Url;
/// use async_trait::async_trait;
/// use serde_json::Value;
///
/// pub struct CustomRequester {
///     config: RequesterConfig
/// }
///
/// #[async_trait(?Send)]
/// impl RequesterInterface for CustomRequester {
///     async fn config(&mut self) -> &mut RequesterConfig {
///         &mut self.config
///     }
///
///     async fn configure(&mut self, config: RequesterConfig) {
///         self.config = config;
///     }
///
///     async fn get_content(&self, url: Url) -> Option<String> {
///         Some(String::from("foo"))
///     }
///
///     async fn get_json_content(&self, url: Url) -> Value {
///         Value::Bool(false)
///     }
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let url = Url::parse("https://foo.com").unwrap();
///
///     let mut requester = CustomRequester {
///         config: RequesterConfig::default(),
///     };
///
///     let config = requester.config().await.clone();
///
///     assert_eq!(requester.get_json_content(url.clone()).await, false);
///     assert_eq!(requester.get_content(url).await.unwrap(), "foo");
///     assert_eq!(config.proxy, None);
///     assert_eq!(config.timeout, Duration::from_secs(10));
///     assert_eq!(config.headers.len(), 0);
/// }
/// ```
#[async_trait(?Send)]
#[enum_dispatch]
pub trait RequesterInterface: Sync + Send {
    /// Returns requester configurations as a [`RequesterConfig`] object
    async fn config(&mut self) -> &mut RequesterConfig;
    /// Configure current requester object by using new [`RequesterConfig`] object
    async fn configure(&mut self, config: RequesterConfig);
    /// Get HTML source of page from given [`reqwest::Url`] object
    async fn get_content(&self, url: Url) -> Option<String>;
    /// Get JSON content from any URL
    async fn get_json_content(&self, url: Url) -> Value;
}
