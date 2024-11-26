use async_trait::async_trait;
use reqwest::Url;
use subscan::{
    enums::content::Content,
    interfaces::requester::RequesterInterface,
    types::{config::requester::RequesterConfig, core::Result},
};

pub struct CustomRequester {
    config: RequesterConfig,
}

#[async_trait]
impl RequesterInterface for CustomRequester {
    async fn config(&mut self) -> &mut RequesterConfig {
        &mut self.config
    }

    async fn configure(&mut self, config: RequesterConfig) {
        self.config = config;
    }

    async fn get_content(&self, _url: Url) -> Result<Content> {
        Ok(Content::Empty)
    }
}

#[tokio::main]
async fn main() {
    let url = Url::parse("https://example.com").unwrap();
    let requester = CustomRequester {
        config: RequesterConfig::default(),
    };

    let content = requester.get_content(url).await.unwrap();

    assert_eq!(content.as_string(), "");
}
