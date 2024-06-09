use crate::Cli;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, USER_AGENT};
use std::time::Duration;
#[derive(Debug, Clone)]
pub struct RequesterConfig {
    pub headers: HeaderMap,
    pub timeout: Duration,
    pub proxy: Option<String>,
}

impl RequesterConfig {
    pub fn new() -> Self {
        Self {
            headers: HeaderMap::new(),
            timeout: Duration::from_secs(10),
            proxy: None,
        }
    }

    pub fn from_cli(cli: &Cli) -> Self {
        Self {
            headers: HeaderMap::from_iter([(
                USER_AGENT,
                HeaderValue::from_str(&cli.user_agent).unwrap(),
            )]),
            timeout: Duration::from_secs(cli.timeout),
            proxy: cli.proxy.clone(),
        }
    }

    pub fn add_header(&mut self, name: HeaderName, value: HeaderValue) {
        self.headers.insert(name, value);
    }
}
