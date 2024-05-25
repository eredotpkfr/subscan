use crate::types::config::RequesterConfig;
use async_trait::async_trait;
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::Url;

const HEADER_ADD_ERR: &str = "Cannot add header!";

#[async_trait(?Send)]
pub trait RequesterInterface: Sync + Send {
    async fn config(&self) -> RequesterConfig;
    async fn configure(&mut self, config: RequesterConfig);
    async fn add_header(&mut self, name: String, value: String) {
        self.config().await.add_header(
            HeaderName::from_bytes(name.as_bytes()).expect(HEADER_ADD_ERR),
            HeaderValue::from_bytes(value.as_bytes()).expect(HEADER_ADD_ERR),
        );
    }
    async fn get_content(&self, url: Url) -> Option<String>;
}
