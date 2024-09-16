mod common;

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::header::{CONTENT_LENGTH, USER_AGENT};
use reqwest::Url;
use std::time::Duration;
use subscan::{
    enums::RequesterType,
    interfaces::requester::RequesterInterface,
    types::config::{RequesterConfig, DEFAULT_HTTP_TIMEOUT},
};

#[cfg(test)]
mod chrome {
    use super::common::constants::TEST_URL;
    use super::*;
    use subscan::requesters::chrome::ChromeBrowser;

    #[tokio::test]
    async fn chrome_configure_test() {
        let mut browser = ChromeBrowser::new();
        let mut config = browser.config().await;

        let new_headers = HeaderMap::from_iter([
            (USER_AGENT, HeaderValue::from_static("foo")),
            (CONTENT_LENGTH, HeaderValue::from_static("20")),
        ]);
        let new_config = RequesterConfig {
            headers: new_headers.clone(),
            timeout: Duration::from_secs(120),
            proxy: Some(String::from(TEST_URL)),
        };

        assert_eq!(config.timeout, DEFAULT_HTTP_TIMEOUT);
        assert_eq!(config.headers.len(), 0);
        assert_eq!(config.proxy, None);

        browser.configure(new_config.clone()).await;
        config = browser.config().await;

        assert_eq!(config.timeout, new_config.timeout);
        assert_eq!(config.headers, new_config.headers);
        assert_eq!(config.headers.len(), new_headers.len());
        assert_eq!(config.proxy, new_config.proxy);

        assert_eq!(browser.r#type().await, RequesterType::ChromeBrowser);
    }

    #[tokio::test]
    #[stubr::mock("hello/hello.json")]
    async fn chrome_get_content_test() {
        let browser = ChromeBrowser::new();
        let url = Url::parse(&stubr.path("/hello")).unwrap();

        let content = browser.get_content(url).await.unwrap();

        assert!(content.contains("hello"));
    }

    #[tokio::test]
    #[stubr::mock("hello/hello-delayed.json")]
    #[should_panic]
    async fn chrome_get_content_timeout_test() {
        let config = RequesterConfig {
            timeout: Duration::from_millis(500),
            headers: HeaderMap::default(),
            proxy: None,
        };

        let browser = ChromeBrowser::with_config(config);
        let url = Url::parse(&stubr.path("/hello-delayed")).unwrap();

        browser.get_content(url).await.unwrap();
    }

    #[tokio::test]
    #[stubr::mock("hello/hello-with-headers.json")]
    async fn chrome_get_content_extra_header_test() {
        let mut config = RequesterConfig::default();

        config.add_header(
            HeaderName::from_static("x-api-key"),
            HeaderValue::from_static("hello-api"),
        );

        let browser = ChromeBrowser::with_config(config);
        let url = Url::parse_with_params(
            &stubr.path("/hello-with-headers"),
            &[("search", "site:foo.com")],
        )
        .unwrap();

        let content = browser.get_content(url).await.unwrap();

        assert!(content.contains("hello"));
    }
}

#[cfg(test)]
mod client {
    use super::common::constants::TEST_URL;
    use super::*;
    use subscan::requesters::client::HTTPClient;

    #[tokio::test]
    async fn client_configure_test() {
        let mut client = HTTPClient::default();
        let mut config = client.config().await;

        let new_headers = HeaderMap::from_iter([
            (USER_AGENT, HeaderValue::from_static("foo")),
            (CONTENT_LENGTH, HeaderValue::from_static("20")),
        ]);
        let new_config = RequesterConfig {
            headers: new_headers.clone(),
            timeout: Duration::from_secs(120),
            proxy: Some(String::from(TEST_URL)),
        };

        assert_eq!(config.timeout, DEFAULT_HTTP_TIMEOUT);
        assert_eq!(config.headers.len(), 0);
        assert_eq!(config.proxy, None);

        client.configure(new_config.clone()).await;
        config = client.config().await;

        assert_eq!(config.timeout, new_config.timeout);
        assert_eq!(config.headers, new_config.headers);
        assert_eq!(config.headers.len(), new_headers.len());
        assert_eq!(config.proxy, new_config.proxy);

        assert_eq!(client.r#type().await, RequesterType::HTTPClient);
    }

    #[tokio::test]
    #[stubr::mock("hello/hello.json")]
    async fn client_get_content_test() {
        let client = HTTPClient::default();
        let url = Url::parse(&stubr.path("/hello")).unwrap();

        let content = client.get_content(url).await.unwrap();

        assert_eq!(content, "hello");
    }

    #[tokio::test]
    #[stubr::mock("hello/hello-delayed.json")]
    #[should_panic]
    async fn client_get_content_timeout_test() {
        let config = RequesterConfig {
            timeout: Duration::from_millis(500),
            headers: HeaderMap::default(),
            proxy: None,
        };

        let client = HTTPClient::with_config(config);
        let url = Url::parse(&stubr.path("/hello-delayed")).unwrap();

        client.get_content(url).await.unwrap();
    }

    #[tokio::test]
    #[stubr::mock("hello/hello-with-headers.json")]
    async fn client_get_content_extra_header_test() {
        let mut config = RequesterConfig::default();

        config.add_header(
            HeaderName::from_static("x-api-key"),
            HeaderValue::from_static("hello-api"),
        );

        let client = HTTPClient::with_config(config);
        let url = Url::parse_with_params(
            &stubr.path("/hello-with-headers"),
            &[("search", "site:foo.com")],
        )
        .unwrap();

        let content = client.get_content(url).await.unwrap();

        assert_eq!(content, "hello");
    }
}
