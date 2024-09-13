#[cfg(test)]
mod requesters {
    use reqwest::header::HeaderMap;
    use std::time::Duration;
    use subscan::cache;
    use subscan::{
        enums::RequesterType, interfaces::requester::RequesterInterface,
        types::config::RequesterConfig,
    };

    #[tokio::test]
    async fn get_by_type_test() {
        let types = vec![RequesterType::ChromeBrowser, RequesterType::HTTPClient];

        for rtype in types {
            let requester = cache::requesters::get_by_type(&rtype).lock();

            assert_eq!(requester.await.r#type().await, rtype);
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
            let rconfig = requester.lock().await.config().await;

            assert_eq!(rconfig.timeout.as_secs(), 10);
        }

        cache::requesters::configure_all(new_config).await;

        for requester in cache::ALL_REQUESTERS.values() {
            let rconfig = requester.lock().await.config().await;

            assert_eq!(rconfig.timeout.as_secs(), 120);
        }
    }
}
