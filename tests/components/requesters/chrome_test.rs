use crate::common::constants::TEST_URL;
use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_LENGTH, USER_AGENT},
    Url,
};
use std::time::Duration;
use subscan::{
    interfaces::requester::RequesterInterface,
    requesters::chrome::ChromeBrowser,
    types::{
        config::requester::RequesterConfig,
        env::{Credentials, Env},
    },
};

#[tokio::test]
async fn chrome_configure_test() {
    let mut browser = ChromeBrowser::default();

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

    browser.configure(new_config.clone()).await;

    assert_eq!(browser.config().await.clone(), new_config);
}

#[tokio::test]
#[stubr::mock("hello/hello.json")]
async fn chrome_get_content_test() {
    let browser = ChromeBrowser::default();
    let url = Url::parse(&stubr.path("/hello")).unwrap();

    let content = browser.get_content(url).await.unwrap().as_string();

    assert!(content.contains("hello"));
}

#[tokio::test]
#[stubr::mock("hello/hello-delayed.json")]
#[should_panic]
async fn chrome_get_content_timeout_test() {
    let config = RequesterConfig {
        timeout: Duration::from_millis(500),
        ..Default::default()
    };

    let browser = ChromeBrowser::with_config(config);
    let url = Url::parse(&stubr.path("/hello-delayed")).unwrap();

    browser
        .get_content(url)
        .await
        .unwrap()
        .as_json()
        .as_str()
        .unwrap();
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

    let content = browser.get_content(url).await.unwrap().as_string();

    assert!(content.contains("hello"));
}
