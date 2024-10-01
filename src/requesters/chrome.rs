use crate::{interfaces::requester::RequesterInterface, types::config::RequesterConfig};
use async_trait::async_trait;
use headless_chrome::{browser::LaunchOptions, Browser};
use reqwest::Url;
use serde_json::Value;

/// Chrome requester struct, send HTTP requests via Chrome browser.
/// Also its compatible with [`RequesterInterface`]
pub struct ChromeBrowser {
    config: RequesterConfig,
    browser: Browser,
}

impl Default for ChromeBrowser {
    fn default() -> Self {
        Self::new()
    }
}

impl ChromeBrowser {
    /// Returns a new [`ChromeBrowser`] instance
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::requesters::chrome::ChromeBrowser;
    ///
    /// let browser = ChromeBrowser::new();
    ///
    /// // do something with browser
    /// ```
    pub fn new() -> Self {
        Self {
            config: RequesterConfig::default(),
            browser: Browser::new(Self::default_options()).unwrap(),
        }
    }

    /// Returns a new [`ChromeBrowser`] instance from given [`RequesterConfig`] object
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::time::Duration;
    /// use reqwest::header::HeaderMap;
    /// use subscan::requesters::chrome::ChromeBrowser;
    /// use subscan::types::config::RequesterConfig;
    ///
    /// let config = RequesterConfig {
    ///     proxy: None,
    ///     headers: HeaderMap::default(),
    ///     timeout: Duration::from_secs(60),
    /// };
    ///
    /// let browser = ChromeBrowser::with_config(config);
    ///
    /// // do something with browser
    /// ```
    pub fn with_config(config: RequesterConfig) -> Self {
        Self {
            config,
            browser: Browser::new(Self::default_options()).unwrap(),
        }
    }

    /// Returns default launch options as a [`LaunchOptions`]
    /// instance, the default options are listed in the
    /// table below
    ///
    /// | Property   | Default Value |
    /// |:----------:|:-------------:|
    /// | headless   | `true`        |
    /// | sandbox    | `false`       |
    /// | enable_gpu | `false`       |
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::requesters::chrome::ChromeBrowser;
    /// use headless_chrome::Browser;
    ///
    /// let options = ChromeBrowser::default_options();
    /// let browser = Browser::new(options).unwrap();
    ///
    /// // do something with browser
    /// ```
    pub fn default_options<'a>() -> LaunchOptions<'a> {
        LaunchOptions::default_builder()
            .headless(true)
            .sandbox(false)
            .enable_gpu(false)
            .build()
            .unwrap()
    }
}

#[async_trait(?Send)]
impl RequesterInterface for ChromeBrowser {
    /// Get requester config object as a [`RequesterConfig`]
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use std::time::Duration;
    /// use subscan::requesters::chrome::ChromeBrowser;
    /// use subscan::interfaces::requester::RequesterInterface;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut browser = ChromeBrowser::default();
    ///
    ///     assert_eq!(browser.config().await.timeout, Duration::from_secs(10));
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
    /// use subscan::requesters::chrome::ChromeBrowser;
    /// use subscan::types::config::RequesterConfig;
    /// use subscan::interfaces::requester::RequesterInterface;
    /// use reqwest::header::HeaderMap;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut browser = ChromeBrowser::default();
    ///
    ///     let new_config = RequesterConfig {
    ///         timeout: Duration::from_secs(120),
    ///         proxy: None,
    ///         headers: HeaderMap::default(),
    ///     };
    ///
    ///     browser.configure(new_config.clone()).await;
    ///
    ///     assert_eq!(browser.config().await.timeout, new_config.timeout);
    /// }
    /// ```
    async fn configure(&mut self, config: RequesterConfig) {
        let mut options = Self::default_options();

        if let Some(proxy) = &config.proxy {
            options.proxy_server = Some(proxy.as_str())
        }

        self.browser = Browser::new(options).unwrap();
        self.config = config;
    }

    /// Get page source HTML from given [`reqwest::Url`]
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::requesters::chrome::ChromeBrowser;
    /// use subscan::interfaces::requester::RequesterInterface;
    /// use reqwest::Url;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut browser = ChromeBrowser::default();
    ///     let url = Url::parse("https://foo.com").unwrap();
    ///
    ///     let content = browser.get_content(url).await.unwrap();
    ///
    ///     // do something with content
    /// }
    /// ```
    async fn get_content(&self, url: Url) -> Option<String> {
        let tab = self.browser.new_tab().expect("Cannot create tab!");
        let headers = self.config.headers_as_hashmap();

        tab.set_default_timeout(self.config.timeout);
        tab.set_extra_http_headers(headers).unwrap();

        tab.navigate_to(url.to_string().as_str()).unwrap();
        tab.wait_until_navigated().unwrap();

        let content = tab.get_content().ok();

        tab.close(true).unwrap();

        content
    }

    async fn get_json_content(&self, url: Url) -> Value {
        let content = self.get_content(url).await.unwrap_or_default();

        serde_json::from_str(&content).unwrap_or_default()
    }
}
