use crate::{
    enums::{RequesterDispatcher, RequesterType},
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
/// use subscan::enums::RequesterType;
/// use reqwest::Url;
/// use async_trait::async_trait;
///
/// pub struct CustomRequester {}
///
/// #[async_trait(?Send)]
/// impl RequesterInterface for CustomRequester {
///     async fn r#type(&self) -> RequesterType {
///         RequesterType::HTTPClient
///     }
///     async fn config(&self) -> RequesterConfig {
///         RequesterConfig::default()
///     }
///     async fn configure(&mut self, config: RequesterConfig) {}
///     async fn get_content(&self, url: Url) -> Option<String> {
///         Some(String::from("foo"))
///     }
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let url = Url::parse("https://foo.com").expect("URL parse error!");
///     let requester = CustomRequester {};
///     let config = requester.config().await;
///
///     assert_eq!(requester.r#type().await, RequesterType::HTTPClient);
///     assert_eq!(requester.get_content(url).await.unwrap(), "foo");
///     assert_eq!(config.proxy, None);
///     assert_eq!(config.timeout, Duration::from_secs(10));
///     assert_eq!(config.headers.len(), 0);
/// }
/// ```
#[async_trait(?Send)]
#[enum_dispatch]
pub trait RequesterInterface: Sync + Send {
    /// Requester type method returns requester's type.
    /// All requester types defined under the [`RequesterType`]
    /// enum
    async fn r#type(&self) -> RequesterType;
    /// Returns requester configurations as a [`RequesterConfig`] object
    async fn config(&self) -> RequesterConfig;
    /// Configure current requester object by using new [`RequesterConfig`] object
    async fn configure(&mut self, config: RequesterConfig);
    /// Get HTML source of page from given [`reqwest::Url`] object
    async fn get_content(&self, url: Url) -> Option<String>;
}
