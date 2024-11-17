use crate::{
    enums::content::Content,
    error::{ModuleErrorKind::GetContentError, SubscanError},
    interfaces::requester::RequesterInterface,
    types::{config::requester::RequesterConfig, core::Result},
};
use async_trait::async_trait;
use headless_chrome::{browser::LaunchOptions, Browser};
use reqwest::Url;

/// Chrome requester struct, send HTTP requests via Chrome browser.
/// Also its compatible with [`RequesterInterface`]
pub struct ChromeBrowser {
    pub config: RequesterConfig,
    pub browser: Browser,
}

impl Default for ChromeBrowser {
    fn default() -> Self {
        Self::new()
    }
}

impl ChromeBrowser {
    /// Returns a new [`ChromeBrowser`] instance
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
    /// use subscan::types::config::requester::RequesterConfig;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = RequesterConfig {
    ///         timeout: Duration::from_secs(60),
    ///         ..Default::default()
    ///     };
    ///
    ///     let browser = ChromeBrowser::with_config(config);
    ///
    ///     // do something with browser
    /// }
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
    /// | Property   | Default Value   |
    /// |:----------:|:---------------:|
    /// | headless   | [`true`]        |
    /// | sandbox    | [`false`]       |
    /// | enable_gpu | [`false`]       |
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::requesters::chrome::ChromeBrowser;
    /// use headless_chrome::Browser;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let options = ChromeBrowser::default_options();
    ///     let browser = Browser::new(options).unwrap();
    ///
    ///     // do something with browser
    /// }
    /// ```
    pub fn default_options<'a>() -> LaunchOptions<'a> {
        LaunchOptions {
            headless: true,
            sandbox: false,
            enable_gpu: false,
            ..Default::default()
        }
    }
}

#[async_trait]
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
    /// use subscan::types::config::requester::RequesterConfig;
    /// use subscan::interfaces::requester::RequesterInterface;
    /// use reqwest::header::HeaderMap;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut browser = ChromeBrowser::default();
    ///
    ///     let new_config = RequesterConfig {
    ///         timeout: Duration::from_secs(120),
    ///         ..Default::default()
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
            options.proxy_server = Some(proxy.as_str());
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
    ///     let content = browser.get_content(url).await;
    ///
    ///     // do something with content
    /// }
    /// ```
    async fn get_content(&self, url: Url) -> Result<Content> {
        let tab = self
            .browser
            .new_tab()
            .map_err(|_| SubscanError::from(GetContentError))?;
        let headers = self.config.headers_as_hashmap();

        // Set basic configurations
        tab.set_default_timeout(self.config.timeout);
        tab.set_extra_http_headers(headers)
            .map_err(|_| SubscanError::from(GetContentError))?;

        // Set basic HTTP authentication if credentials provided
        if self.config.credentials.is_ok() {
            let username = self.config.credentials.username.value.clone();
            let password = self.config.credentials.password.value.clone();

            tab.authenticate(username, password)
                .map_err(|_| SubscanError::from(GetContentError))?;
        }

        tab.navigate_to(url.to_string().as_str())
            .map_err(|_| SubscanError::from(GetContentError))?;

        if let Ok(tab) = tab.wait_until_navigated() {
            let content = tab.get_content();

            tab.close(true)
                .map_err(|_| SubscanError::from(GetContentError))?;
            Ok(Content::String(
                content.map_err(|_| SubscanError::from(GetContentError))?,
            ))
        } else {
            tab.close(true)
                .map_err(|_| SubscanError::from(GetContentError))?;
            Ok(Content::Empty)
        }
    }
}
