mod common;

#[cfg(test)]
mod chrome {
    use super::common;
    use reqwest::header::{HeaderMap, HeaderValue};
    use reqwest::header::{CONTENT_LENGTH, USER_AGENT};
    use reqwest::Url;
    use std::time::Duration;
    use subscan::types::config::{RequesterConfig, DEFAULT_HTTP_TIMEOUT};
    use subscan::{interfaces::requester::RequesterInterface, requesters::chrome::ChromeBrowser};
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

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
        let mock = Mock::given(method("GET")).and(path("/foo"));
        let response = ResponseTemplate::new(200).set_body_raw("foo", "text/html");

        let server = common::server::test_server_with_response(mock, response).await;
        let browser = ChromeBrowser::new();
        let url = Url::parse(format!("{}/foo", server.uri()).as_str()).unwrap();

        let content = browser.get_content(url).await.unwrap();

        assert_eq!(content, "<html><head></head><body>foo</body></html>");
    }

    #[tokio::test]
    #[should_panic]
    async fn chrome_get_content_timeout_test() {
        let mock = Mock::given(method("GET")).and(path("/foo"));
        let response = ResponseTemplate::new(200).set_delay(Duration::from_millis(100));

        let config = RequesterConfig {
            timeout: Duration::from_millis(50),
            headers: HeaderMap::default(),
            proxy: None,
        };

        let server = common::server::test_server_with_response(mock, response).await;
        let browser = ChromeBrowser::with_config(config);
        let url = Url::parse(format!("{}/foo", server.uri()).as_str()).unwrap();

        browser.get_content(url).await.unwrap();
    }
}
