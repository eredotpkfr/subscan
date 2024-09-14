use crate::{
    enums::{RequesterDispatcher, RequesterType},
    requesters::{chrome::ChromeBrowser, client::HTTPClient},
    types::config::RequesterConfig,
};
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use reqwest::Url;

/// HTTP requester interface definiton, other requesters
/// that will be implemented in the future
/// must conform to this interface
///
/// # Examples
///
/// ```
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
///         RequesterConfig::new()
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
///
///     assert_eq!(requester.r#type().await, RequesterType::HTTPClient);
///     assert_eq!(requester.get_content(url).await.unwrap(), "foo");
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
    /// Get page HTML source from given [`reqwest::Url`] object
    async fn get_content(&self, url: Url) -> Option<String>;
}
