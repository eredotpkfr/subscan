use subscan::{interfaces::lookup::LookUpHostFuture, types::config::resolver::ResolverConfig};

use crate::common::{
    constants::{LOCAL_HOST, TEST_DOMAIN},
    mock::resolver::MockResolver,
};

#[tokio::test]
async fn lookup_host_future_test_with_returns_none() {
    let rconfig = ResolverConfig {
        disabled: true,
        ..Default::default()
    };
    let resolver = MockResolver::boxed(rconfig);
    let lookup_host = resolver.lookup_host_future().await;

    assert!(lookup_host(TEST_DOMAIN.into()).await.is_none());
}

#[tokio::test]
async fn lookup_host_future_test_with_returns_ip() {
    let resolver = MockResolver::default_boxed();
    let lookup_host = resolver.lookup_host_future().await;
    let ip = lookup_host(TEST_DOMAIN.into()).await.unwrap();

    assert_eq!(ip.to_string(), LOCAL_HOST);
}
