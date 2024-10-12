mod constants {
    use reqwest::header::HeaderValue;

    pub const TEST_URL: &str = "http://foo.com";
    pub const USER_AGENT_VALUE: HeaderValue = HeaderValue::from_static("x-api-key");
    pub const CONTENT_LENGTH_VALUE: HeaderValue = HeaderValue::from_static("10000");
}

#[cfg(test)]
mod modules {
    use super::constants::{CONTENT_LENGTH_VALUE, TEST_URL, USER_AGENT_VALUE};
    use reqwest::header::{HeaderMap, CONTENT_LENGTH, USER_AGENT};
    use std::time::Duration;
    use subscan::{
        cache::{self, modules},
        interfaces::{module::SubscanModuleInterface, requester::RequesterInterface},
        types::{
            config::RequesterConfig,
            env::{Credentials, Env},
        },
    };

    #[tokio::test]
    async fn configure_all_requesters_test() {
        let headers = HeaderMap::from_iter([
            (USER_AGENT, USER_AGENT_VALUE),
            (CONTENT_LENGTH, CONTENT_LENGTH_VALUE),
        ]);
        let credentials = Credentials {
            username: Env {
                name: "USERNAME".into(),
                value: Some("foo".to_string()),
            },
            password: Env {
                name: "PASSWORD".into(),
                value: None,
            },
        };

        let old_config = RequesterConfig::default();
        let new_config = RequesterConfig {
            timeout: Duration::from_secs(120),
            headers,
            proxy: Some(TEST_URL.to_string()),
            credentials,
        };

        for module in cache::ALL_MODULES.iter() {
            let module = module.lock().await;

            if let Some(requester) = module.requester().await {
                assert_eq!(requester.lock().await.config().await, &old_config);
            }
        }

        modules::configure_all_requesters(new_config.clone()).await;

        for module in cache::ALL_MODULES.iter() {
            let module = module.lock().await;

            if let Some(requester) = module.requester().await {
                assert_eq!(requester.lock().await.config().await, &new_config);
            }
        }
    }
}
