use crate::{
    enums::RequesterType, interfaces::requester::RequesterInterface, types::config::RequesterConfig,
};
use async_trait::async_trait;
use headless_chrome::{browser::LaunchOptions, Browser};
use reqwest::Url;

/// Chrome requester struct, send HTTP requests
/// via Chrome browser. Also its compatible
/// with [`RequesterInterface`]
pub struct ChromeBrowser {
    config: RequesterConfig,
    browser: Browser,
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
    async fn r#type(&self) -> RequesterType {
        RequesterType::ChromeBrowser
    }

    async fn config(&self) -> RequesterConfig {
        self.config.clone()
    }

    async fn configure(&mut self, config: RequesterConfig) {
        let mut options = Self::default_options();

        if let Some(proxy) = &config.proxy {
            options.proxy_server = Some(proxy.as_str())
        }

        self.browser = Browser::new(options).unwrap();
        self.config = config
    }

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
}
