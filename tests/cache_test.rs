use reqwest::header::{HeaderMap, HeaderValue, CONTENT_LENGTH, USER_AGENT};
use std::time::Duration;
use strum::IntoEnumIterator;

#[cfg(test)]
mod requesters {
    use super::*;
    use subscan::{
        cache, enums::RequesterType, interfaces::requester::RequesterInterface,
        types::config::RequesterConfig,
    };

    const TEST_URL: &str = "http://foo.com";

    #[tokio::test]
    async fn get_by_type_test() {
        for rtype in RequesterType::iter() {
            let requester = cache::requesters::get_by_type(&rtype).lock().await;

            assert_eq!(requester.r#type().await, rtype);
        }
    }

    #[tokio::test]
    async fn configure_all_test() {
        let new_config = RequesterConfig {
            timeout: Duration::from_secs(120),
            headers: HeaderMap::from_iter([
                (USER_AGENT, HeaderValue::from_static("x-api-key")),
                (CONTENT_LENGTH, HeaderValue::from_static("10000")),
            ]),
            proxy: Some(TEST_URL.to_string()),
        };

        for requester in cache::ALL_REQUESTERS.values() {
            let requester = requester.lock().await;

            assert_eq!(requester.config().await, RequesterConfig::default());
        }

        cache::requesters::configure_all(new_config.clone()).await;

        for requester in cache::ALL_REQUESTERS.values() {
            assert_eq!(requester.lock().await.config().await, new_config);
        }
    }
}
