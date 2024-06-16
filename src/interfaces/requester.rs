use crate::{
    enums::RequesterDispatcher,
    requesters::{chrome::ChromeBrowser, client::HTTPClient},
    types::config::RequesterConfig,
};
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use reqwest::Url;

#[async_trait(?Send)]
#[enum_dispatch]
pub trait RequesterInterface: Sync + Send {
    async fn config(&self) -> RequesterConfig;
    async fn configure(&mut self, config: RequesterConfig);
    async fn get_content(&self, url: Url) -> Option<String>;
}
