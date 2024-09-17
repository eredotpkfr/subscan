use reqwest::header::{HeaderMap, CONTENT_LENGTH, USER_AGENT};
use std::time::Duration;
use strum::IntoEnumIterator;

mod constants {
    use reqwest::header::HeaderValue;

    pub const TEST_URL: &str = "http://foo.com";
    pub const USER_AGENT_VALUE: HeaderValue = HeaderValue::from_static("x-api-key");
    pub const CONTENT_LENGTH_VALUE: HeaderValue = HeaderValue::from_static("10000");
}

#[cfg(test)]
mod requesters {
    use super::constants::{CONTENT_LENGTH_VALUE, TEST_URL, USER_AGENT_VALUE};
    use super::*;
    use subscan::{
        cache, enums::RequesterType, interfaces::requester::RequesterInterface,
        types::config::RequesterConfig,
    };

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
                (USER_AGENT, USER_AGENT_VALUE),
                (CONTENT_LENGTH, CONTENT_LENGTH_VALUE),
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
