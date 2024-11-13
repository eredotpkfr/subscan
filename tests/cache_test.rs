use std::time::Duration;
use subscan::{
    cache::CacheManager,
    interfaces::{module::SubscanModuleInterface, requester::RequesterInterface},
    types::config::requester::RequesterConfig,
};

#[tokio::test]
async fn configure_test() {
    let manager = CacheManager::default();

    let old_config = RequesterConfig::default();
    let new_config = RequesterConfig {
        timeout: Duration::from_secs(120),
        ..Default::default()
    };

    for module in manager.modules().await.iter() {
        if let Some(requester) = module.lock().await.requester().await {
            assert_eq!(requester.lock().await.config().await, &old_config);
        }
    }

    manager.configure(new_config.clone()).await;

    for module in manager.modules().await.iter() {
        if let Some(requester) = module.lock().await.requester().await {
            assert_eq!(requester.lock().await.config().await, &new_config);
        }
    }
}

#[tokio::test]
async fn module_test() {
    let manager = CacheManager::default();

    assert!(manager.module("foo").await.is_none());
    assert!(manager.module("google").await.is_some());
}
