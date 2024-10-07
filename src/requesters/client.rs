use crate::{
    enums::Content, interfaces::requester::RequesterInterface, types::config::RequesterConfig,
};
use async_trait::async_trait;
use reqwest::{Client, Proxy, Url};

const CLIENT_BUILD_ERR: &str = "Cannot create HTTP client!";
const REQUEST_BUILD_ERR: &str = "Cannot build request!";
const PROXY_PARSE_ERR: &str = "Cannot parse proxy!";

/// HTTP requester struct, send HTTP requests via [`reqwest`] client.
/// Also its compatible with [`RequesterInterface`]
#[derive(Default)]
pub struct HTTPClient {
    config: RequesterConfig,
    client: Client,
}

impl HTTPClient {
    /// Returns a new default [`HTTPClient`] instance
    pub fn new() -> Self {
        Self::default()
    }

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
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = RequesterConfig {
    ///         proxy: None,
    ///         headers: HeaderMap::default(),
    ///         timeout: Duration::from_secs(60),
    ///     };
    ///
    ///     let client = HTTPClient::with_config(config);
    ///
    ///     // do something with client
    /// }
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
    /// Get requester config object as a [`RequesterConfig`]
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::time::Duration;
    /// use subscan::requesters::client::HTTPClient;
    /// use subscan::interfaces::requester::RequesterInterface;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = HTTPClient::default();
    ///
    ///     assert_eq!(client.config().await.timeout, Duration::from_secs(10));
    /// }
    /// ```
    async fn config(&mut self) -> &mut RequesterConfig {
        &mut self.config
    }

    /// Configure requester with a new config object
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::time::Duration;
    /// use subscan::requesters::client::HTTPClient;
    /// use subscan::types::config::RequesterConfig;
    /// use subscan::interfaces::requester::RequesterInterface;
    /// use reqwest::header::HeaderMap;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = HTTPClient::default();
    ///
    ///     let new_config = RequesterConfig {
    ///         timeout: Duration::from_secs(120),
    ///         proxy: None,
    ///         headers: HeaderMap::default(),
    ///     };
    ///
    ///     client.configure(new_config.clone()).await;
    ///
    ///     assert_eq!(client.config().await.timeout, new_config.timeout);
    /// }
    /// ```
    async fn configure(&mut self, config: RequesterConfig) {
        let mut builder = Client::builder().default_headers(config.headers.clone());

        if let Some(proxy) = &config.proxy {
            builder = builder.proxy(Proxy::all(proxy).expect(PROXY_PARSE_ERR));
        }

        self.config = config;
        self.client = builder.build().expect(CLIENT_BUILD_ERR);
    }

    /// Get page source HTML from given [`reqwest::Url`]
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::requesters::client::HTTPClient;
    /// use subscan::interfaces::requester::RequesterInterface;
    /// use reqwest::Url;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut client = HTTPClient::default();
    ///     let url = Url::parse("https://foo.com").unwrap();
    ///
    ///     let content = client.get_content(url).await;
    ///
    ///     // do something with content
    /// }
    /// ```
    async fn get_content(&self, url: Url) -> Content {
        let rbuilder = self.client.get(url);
        let request = rbuilder
            .timeout(self.config.timeout)
            .headers(self.config.headers.clone())
            .build()
            .expect(REQUEST_BUILD_ERR);

        if let Ok(response) = self.client.execute(request).await {
            if let Ok(content) = response.text().await {
                Content::String(content)
            } else {
                Content::Empty
            }
        } else {
            Content::Empty
        }
    }
}
