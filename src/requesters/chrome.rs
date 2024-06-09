use crate::types::config::RequesterConfig;
use crate::{enums::RequesterType, interfaces::requester::RequesterInterface};
use async_trait::async_trait;
use headless_chrome::{browser::default_executable, browser::LaunchOptions, Browser};
use reqwest::Url;

pub struct ChromeBrowser {
    config: RequesterConfig,
    browser: Browser,
}

impl ChromeBrowser {
    pub fn new() -> Self {
        let builder = LaunchOptions::default_builder()
            .path(Some(default_executable().unwrap()))
            .headless(true)
            .sandbox(false)
            .build()
            .unwrap();

        ChromeBrowser {
            config: RequesterConfig::new(),
            browser: Browser::new(builder).unwrap(),
        }
    }
}

#[async_trait(?Send)]
impl RequesterInterface for ChromeBrowser {
    async fn r#type(&self) -> RequesterType {
        RequesterType::ChromeBrowser
    }

    async fn config(&self) -> RequesterConfig {
        self.config.clone()
    }

    async fn configure(&mut self, config: RequesterConfig) {
        self.config = config
    }

    async fn get_content(&self, _url: Url) -> Option<String> {
        Some(String::new())
    }
}
