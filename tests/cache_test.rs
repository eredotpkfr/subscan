use reqwest::header::HeaderMap;
use std::time::Duration;
use strum::IntoEnumIterator;

#[cfg(test)]
mod requesters {
    use super::*;
    use subscan::{
        cache,
        enums::RequesterType,
        interfaces::requester::RequesterInterface,
        types::config::{RequesterConfig, DEFAULT_HTTP_TIMEOUT},
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
            headers: HeaderMap::default(),
            proxy: None,
        };

        for requester in cache::ALL_REQUESTERS.values() {
            assert_eq!(
                requester.lock().await.config().await.timeout,
                DEFAULT_HTTP_TIMEOUT
            );
        }

        cache::requesters::configure_all(new_config.clone()).await;

        for requester in cache::ALL_REQUESTERS.values() {
            assert_eq!(
                requester.lock().await.config().await.timeout,
                new_config.timeout
            );
        }
    }
}
