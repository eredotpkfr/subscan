#[cfg(test)]
mod requesters {
    use reqwest::header::HeaderMap;
    use strum::IntoEnumIterator;
    use std::time::Duration;
    use subscan::cache;
    use subscan::{
        enums::RequesterType, interfaces::requester::RequesterInterface,
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
            headers: HeaderMap::default(),
            proxy: None,
        };

        for requester in cache::ALL_REQUESTERS.values() {
            assert_eq!(requester.lock().await.config().await.timeout.as_secs(), 10);
        }

        cache::requesters::configure_all(new_config).await;

        for requester in cache::ALL_REQUESTERS.values() {
            assert_eq!(requester.lock().await.config().await.timeout.as_secs(), 120);
        }
    }
}
