use crate::{
    enums::RequesterType, interfaces::requester::RequesterInterface, types::config::RequesterConfig,
};
use async_trait::async_trait;
use reqwest::{Client, Proxy, Url};

const CLIENT_BUILD_ERR: &str = "Cannot create HTTP client!";
const REQUEST_BUILD_ERR: &str = "Cannot build request!";
const PROXY_PARSE_ERR: &str = "Cannot parse proxy!";

/// HTTP requester struct, send HTTP requests
/// via [`reqwest`] client. Also its compatible
/// with [`RequesterInterface`]
#[derive(Default)]
pub struct HTTPClient {
    config: RequesterConfig,
    client: Client,
}

impl HTTPClient {
    /// Returns a new [`HTTPClient`] instance from given [`RequesterConfig`]
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::time::Duration;
    /// use reqwest::header::HeaderMap;
    /// use subscan::requesters::client::HTTPClient;
    /// use subscan::types::config::RequesterConfig;
    ///
    /// let config = RequesterConfig {
    ///     proxy: None,
    ///     headers: HeaderMap::default(),
    ///     timeout: Duration::from_secs(60),
    /// };
    ///
    /// let client = HTTPClient::with_config(config);
    ///
    /// // do something with client
    /// ```
    pub fn with_config(config: RequesterConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }
}

#[async_trait(?Send)]
impl RequesterInterface for HTTPClient {
    async fn r#type(&self) -> RequesterType {
        RequesterType::HTTPClient
    }

    async fn config(&self) -> RequesterConfig {
        self.config.clone()
    }

    async fn configure(&mut self, config: RequesterConfig) {
        let mut builder = Client::builder().default_headers(config.headers.clone());

        if let Some(proxy) = &config.proxy {
            builder = builder.proxy(Proxy::all(proxy).expect(PROXY_PARSE_ERR));
        }

        self.config = config;
        self.client = builder.build().expect(CLIENT_BUILD_ERR);
    }

    async fn get_content(&self, url: Url) -> Option<String> {
        let request = self
            .client
            .get(url)
            .timeout(self.config.timeout)
            .headers(self.config.headers.clone())
            .build()
            .expect(REQUEST_BUILD_ERR);

        if let Ok(response) = self.client.execute(request).await {
            if let Ok(content) = response.text().await {
                Some(content)
            } else {
                None
            }
        } else {
            None
        }
    }
}
