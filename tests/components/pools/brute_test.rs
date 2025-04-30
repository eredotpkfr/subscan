use std::{
    net::{IpAddr, Ipv4Addr},
    str::FromStr,
};

use subscan::{pools::brute::SubscanBrutePool, types::result::item::SubscanResultItem};

use crate::common::{
    constants::{LOCAL_HOST, TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mock::resolver::MockResolver,
    utils::testdata_path,
};

#[tokio::test]
async fn submit_test() {
    let resolver = MockResolver::default_boxed();

    let pool = SubscanBrutePool::new(1, resolver);
    let item = SubscanResultItem {
        subdomain: TEST_BAR_SUBDOMAIN.into(),
        ip: Some(IpAddr::V4(Ipv4Addr::from_str(LOCAL_HOST).unwrap())),
    };

    assert!(pool.clone().is_empty().await);

    pool.clone().submit("bar".into()).await;
    pool.clone().spawn_bruters(TEST_DOMAIN).await;

    assert_eq!(pool.clone().len().await, 1);

    pool.clone().kill_bruters().await;
    pool.clone().join().await;

    assert_eq!(pool.result().await.items, [item].into());
}

#[tokio::test]
async fn result_test() {
    let resolver = MockResolver::default_boxed();

    let pool = SubscanBrutePool::new(1, resolver);

    pool.clone().submit("bar".into()).await;
    pool.clone().spawn_bruters(TEST_DOMAIN).await;
    pool.clone().kill_bruters().await;
    pool.clone().join().await;

    let binding = pool.result().await;
    let result = binding.items.first();

    assert!(result.is_some());
    assert!(result.unwrap().ip.is_some());

    assert_eq!(result.unwrap().subdomain, TEST_BAR_SUBDOMAIN);
    assert_eq!(result.unwrap().ip.unwrap().to_string(), LOCAL_HOST);
}

#[tokio::test]
async fn start_test() {
    let resolver = MockResolver::default_boxed();

    let pool = SubscanBrutePool::new(1, resolver);
    let wordlist = testdata_path().join("txt/wordlist.txt");

    pool.clone().start(TEST_DOMAIN, wordlist).await;

    let binding = pool.result().await;
    let result = binding.items.first();

    assert!(result.is_some());
    assert!(result.unwrap().ip.is_some());

    assert_eq!(binding.items.len(), 3);
    assert_eq!(result.unwrap().subdomain, TEST_BAR_SUBDOMAIN);
    assert_eq!(result.unwrap().ip.unwrap().to_string(), LOCAL_HOST);
}
