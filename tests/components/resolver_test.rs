use subscan::{resolver::Resolver, types::config::resolver::ResolverConfig};

use crate::common::{
    constants::{LOCAL_HOST, TEST_DOMAIN},
    mock::funcs::spawn_mock_dns_server,
};

#[tokio::test]
async fn lookup_ip_future_test_with_returns_none() {
    let rconfig = ResolverConfig {
        disabled: true,
        ..Default::default()
    };
    let resolver = Resolver::from(rconfig);
    let lookup_ip = resolver.lookup_ip_future().await;

    assert!(lookup_ip(&resolver, TEST_DOMAIN.into()).await.is_none());
}

#[tokio::test]
async fn lookup_ip_future_test_with_returns_ip() {
    let server = spawn_mock_dns_server().await;
    let resolver = server.get_resolver().await;

    let lookup_ip = resolver.lookup_ip_future().await;
    let ip = lookup_ip(&resolver, TEST_DOMAIN.into()).await.unwrap();

    assert_eq!(ip.to_string(), LOCAL_HOST);
}
