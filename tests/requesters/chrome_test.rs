use crate::common::constants::TEST_URL;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::header::{CONTENT_LENGTH, USER_AGENT};
use reqwest::Url;
use std::time::Duration;
use subscan::{
    interfaces::requester::RequesterInterface,
    requesters::chrome::ChromeBrowser,
    types::config::{RequesterConfig, DEFAULT_HTTP_TIMEOUT},
};

#[tokio::test]
async fn chrome_configure_test() {
    let mut browser = ChromeBrowser::default();
    let mut config = browser.config().await;

    let new_headers = HeaderMap::from_iter([
        (USER_AGENT, HeaderValue::from_static("foo")),
        (CONTENT_LENGTH, HeaderValue::from_static("20")),
    ]);

    let new_config = RequesterConfig {
        headers: new_headers.clone(),
        timeout: Duration::from_secs(120),
        proxy: Some(TEST_URL.to_string()),
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
}

#[tokio::test]
#[stubr::mock("hello/hello.json")]
async fn chrome_get_content_test() {
    let browser = ChromeBrowser::default();
    let url: Url = stubr.path("/hello").parse().unwrap();

    let content = browser.get_content(url).await;

    assert!(content.to_string().contains("hello"));
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
    let url: Url = stubr.path("/hello-delayed").parse().unwrap();

    browser.get_content(url).await;
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

    let content = browser.get_content(url).await;

    assert!(content.to_string().contains("hello"));
}
