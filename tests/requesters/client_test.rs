use crate::common::constants::TEST_URL;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::header::{CONTENT_LENGTH, USER_AGENT};
use reqwest::Url;
use std::time::Duration;
use subscan::requesters::client::HTTPClient;
use subscan::{
    enums::RequesterType,
    interfaces::requester::RequesterInterface,
    types::config::{RequesterConfig, DEFAULT_HTTP_TIMEOUT},
};

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
        proxy: Some(TEST_URL.to_string()),
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
