use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
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

    pub fn add_header(&mut self, name: HeaderName, value: HeaderValue) {
        self.headers.insert(name, value);
    }
}
