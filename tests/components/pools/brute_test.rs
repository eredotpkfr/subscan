use std::{
    fs::{self, remove_file},
    net::{IpAddr, Ipv4Addr},
    str::FromStr,
};

use subscan::{
    pools::brute::SubscanBrutePool,
    types::{
        config::{pool::PoolConfig, resolver::ResolverConfig},
        result::item::SubscanResultItem,
    },
};

use crate::common::{
    constants::{
        LOCAL_HOST, TEST_BAR_SUBDOMAIN, TEST_BAZ_SUBDOMAIN, TEST_DOMAIN, TEST_FOO_SUBDOMAIN,
    },
    mock::resolver::MockResolver,
    utils::{self, testdata_path},
};

#[tokio::test]
async fn submit_test() {
    let rconfig = ResolverConfig {
        concurrency: 1,
        ..Default::default()
    };
    let resolver = MockResolver::boxed(rconfig);
    let config = PoolConfig {
        concurrency: 1,
        ..Default::default()
    };

    let pool = SubscanBrutePool::new(config, resolver);
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
    let config = PoolConfig {
        concurrency: 1,
        ..Default::default()
    };

    let pool = SubscanBrutePool::new(config, resolver);

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
    let config = PoolConfig {
        concurrency: 1,
        ..Default::default()
    };

    let pool = SubscanBrutePool::new(config, resolver);
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

#[tokio::test]
async fn start_with_stream_test() {
    let stream = utils::testdata_path().join("stream.txt");
    let resolver = MockResolver::default_boxed();

    let config = PoolConfig {
        concurrency: 1,
        stream: Some(stream.clone()),
        ..Default::default()
    };

    let pool = SubscanBrutePool::new(config.clone(), resolver);
    let wordlist = testdata_path().join("txt/wordlist.txt");

    pool.clone().start(TEST_DOMAIN, wordlist).await;

    let local = IpAddr::V4(Ipv4Addr::from_str(LOCAL_HOST).unwrap());
    let expecteds = vec![
        SubscanResultItem {
            subdomain: TEST_FOO_SUBDOMAIN.into(),
            ip: Some(local),
        },
        SubscanResultItem {
            subdomain: TEST_BAR_SUBDOMAIN.into(),
            ip: Some(local),
        },
        SubscanResultItem {
            subdomain: TEST_BAZ_SUBDOMAIN.into(),
            ip: Some(local),
        },
    ];

    let binding = fs::read_to_string(stream.clone()).unwrap();
    let lines: Vec<&str> = binding.lines().collect();

    assert_eq!(lines[0], expecteds[0].as_txt());
    assert_eq!(lines[1], expecteds[1].as_txt());
    assert_eq!(lines[2], expecteds[2].as_txt());

    remove_file(stream).unwrap();
}
