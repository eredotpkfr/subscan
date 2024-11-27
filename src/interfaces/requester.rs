use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use reqwest::Url;

use crate::{
    enums::{content::Content, dispatchers::RequesterDispatcher},
    requesters::{chrome::ChromeBrowser, client::HTTPClient},
    types::{config::requester::RequesterConfig, core::Result},
};

/// Generic HTTP client trait definition to implement different HTTP requester objects
/// with a single interface compatible
///
/// Other requesters that will be implemented in the future must conform to this interface.
/// Mostly uses to get string content from any URL with a single stupid `get_content` method
#[async_trait]
#[enum_dispatch]
pub trait RequesterInterface: Sync + Send {
    /// Returns requester configurations as a [`RequesterConfig`] object
    async fn config(&mut self) -> &mut RequesterConfig;
    /// Configure current requester object by using new [`RequesterConfig`] object
    async fn configure(&mut self, config: RequesterConfig);
    /// HTTP GET method implementation to fetch HTML content from given source [`Url`]
    async fn get_content(&self, url: Url) -> Result<Content>;
}
