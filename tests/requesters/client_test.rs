use crate::common::constants::TEST_URL;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_LENGTH, USER_AGENT},
    Url,
};
use std::time::Duration;
use subscan::{
    interfaces::requester::RequesterInterface,
    requesters::client::HTTPClient,
    types::{
        config::requester::RequesterConfig,
        env::{Credentials, Env},
    },
};

#[tokio::test]
async fn client_configure_test() {
    let mut client = HTTPClient::default();

    let headers = HeaderMap::from_iter([
        (USER_AGENT, HeaderValue::from_static("foo")),
        (CONTENT_LENGTH, HeaderValue::from_static("20")),
    ]);
    let credentials = Credentials {
        username: Env {
            name: "USERNAME".into(),
            value: Some("foo".to_string()),
        },
        password: Env {
            name: "PASSWORD".into(),
            value: Some("bar".to_string()),
        },
    };

    let new_config = RequesterConfig {
        headers,
        timeout: Duration::from_secs(120),
        proxy: Some(TEST_URL.to_string()),
        credentials,
    };

    client.configure(new_config.clone()).await;

    assert_eq!(client.config().await.clone(), new_config);
}

#[tokio::test]
#[stubr::mock("hello/hello.json")]
async fn client_get_content_test() {
    let client = HTTPClient::default();
    let url = Url::parse(&stubr.path("/hello")).unwrap();

    let content = client.get_content(url).await.as_string();

    assert_eq!(content, "hello");
}

#[tokio::test]
#[stubr::mock("hello/hello-delayed.json")]
#[should_panic]
async fn client_get_content_timeout_test() {
    let config = RequesterConfig {
        timeout: Duration::from_millis(500),
        ..Default::default()
    };

    let client = HTTPClient::with_config(config);
    let url = Url::parse(&stubr.path("/hello-delayed")).unwrap();

    client.get_content(url).await.as_json().as_str().unwrap();
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

    let content = client.get_content(url).await.as_string();

    assert_eq!(content, "hello");
}

#[tokio::test]
#[stubr::mock("hello/hello-with-basic-http-auth.json")]
async fn client_get_content_basic_http_auth_test() {
    let mut config = RequesterConfig::default();

    let credentials = Credentials {
        username: Env {
            name: "USERNAME".into(),
            value: Some("foo".to_string()),
        },
        password: Env {
            name: "PASSWORD".into(),
            value: Some("bar".to_string()),
        },
    };

    config.set_credentials(credentials);

    let client = HTTPClient::with_config(config);
    let url = Url::parse(&stubr.path("/hello-with-basic-http-auth")).unwrap();
    let content = client.get_content(url).await.as_string();

    assert_eq!(content, "hello");
}
