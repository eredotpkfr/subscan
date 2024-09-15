mod common;

use common::matchers::header_with_panic;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::header::{CONTENT_LENGTH, USER_AGENT};
use reqwest::Url;
use std::time::Duration;
use subscan::interfaces::requester::RequesterInterface;
use subscan::types::config::{RequesterConfig, DEFAULT_HTTP_TIMEOUT};
use wiremock::http::HeaderName;
use wiremock::matchers::{method, path};
use wiremock::{Mock, ResponseTemplate};

#[cfg(test)]
mod chrome {
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
            proxy: Some(String::from("http://foo.bar")),
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
    async fn chrome_get_content_test() {
        let mock = Mock::given(method("GET")).and(path("/hello"));
        let response = ResponseTemplate::new(200).set_body_raw("hello", "text/html");
        let server = common::server::test_server_with_response(mock, response).await;

        let browser = ChromeBrowser::new();
        let url = Url::parse(format!("{}/hello", server.uri()).as_str()).unwrap();

        let content = browser.get_content(url).await.unwrap();

        assert_eq!(content, "<html><head></head><body>hello</body></html>");
    }

    #[tokio::test]
    #[should_panic]
    async fn chrome_get_content_timeout_test() {
        let mock = Mock::given(method("GET")).and(path("/hello"));
        let response = ResponseTemplate::new(200).set_delay(Duration::from_millis(100));
        let server = common::server::test_server_with_response(mock, response).await;

        let config = RequesterConfig {
            timeout: Duration::from_millis(50),
            headers: HeaderMap::default(),
            proxy: None,
        };

        let browser = ChromeBrowser::with_config(config);
        let url = Url::parse(format!("{}/hello", server.uri()).as_str()).unwrap();

        browser.get_content(url).await.unwrap();
    }

    #[tokio::test]
    async fn chrome_get_content_extra_header_test() {
        let mock = Mock::given(method("GET"))
            .and(path("/hello"))
            .and(header_with_panic("x-api-key"));
        let response = ResponseTemplate::new(200).set_body_raw("hello", "text/html");

        let mut config = RequesterConfig::default();

        config.add_header(
            HeaderName::from_static("x-api-key"),
            HeaderValue::from_static("foobarbaz"),
        );

        let server = common::server::test_server_with_response(mock, response).await;
        let browser = ChromeBrowser::with_config(config);
        let url = Url::parse(format!("{}/hello", server.uri()).as_str()).unwrap();

        let content = browser.get_content(url).await.unwrap();

        assert_eq!(content, "<html><head></head><body>hello</body></html>");
    }
}

#[cfg(test)]
mod client {
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
            proxy: Some(String::from("http://foo.bar")),
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
    }

    #[tokio::test]
    async fn client_get_content_test() {
        let mock = Mock::given(method("GET")).and(path("/hello"));
        let response = ResponseTemplate::new(200).set_body_raw("hello", "text/html");
        let server = common::server::test_server_with_response(mock, response).await;

        let client = HTTPClient::default();
        let url = Url::parse(format!("{}/hello", server.uri()).as_str()).unwrap();

        let content = client.get_content(url).await.unwrap();

        assert_eq!(content, "hello");
    }

    #[tokio::test]
    #[should_panic]
    async fn client_get_content_timeout_test() {
        let mock = Mock::given(method("GET")).and(path("/hello"));
        let response = ResponseTemplate::new(200).set_delay(Duration::from_millis(100));
        let server = common::server::test_server_with_response(mock, response).await;

        let config = RequesterConfig {
            timeout: Duration::from_millis(50),
            headers: HeaderMap::default(),
            proxy: None,
        };

        let client = HTTPClient::with_config(config);
        let url = Url::parse(format!("{}/hello", server.uri()).as_str()).unwrap();

        client.get_content(url).await.unwrap();
    }

    #[tokio::test]
    async fn client_get_content_extra_header_test() {
        let mock = Mock::given(method("GET"))
            .and(path("/hello"))
            .and(header_with_panic("x-api-key"));
        let response = ResponseTemplate::new(200).set_body_raw("hello", "text/html");

        let mut config = RequesterConfig::default();

        config.add_header(
            HeaderName::from_static("x-api-key"),
            HeaderValue::from_static("foobarbaz"),
        );

        let server = common::server::test_server_with_response(mock, response).await;
        let client = HTTPClient::with_config(config);
        let url = Url::parse(format!("{}/hello", server.uri()).as_str()).unwrap();

        let content = client.get_content(url).await.unwrap();

        assert_eq!(content, "hello");
    }
}
