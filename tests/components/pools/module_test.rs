use std::{
    net::{IpAddr, Ipv4Addr},
    str::FromStr,
};

use subscan::{
    enums::{cache::CacheFilter, dispatchers::SubscanModuleDispatcher},
    modules::engines::google::Google,
    pools::module::SubscanModulePool,
    types::{core::SubscanModule, result::item::ScanResultItem},
};

use crate::common::{
    constants::{LOCAL_HOST, TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mock::funcs::spawn_mock_dns_server,
};

#[tokio::test]
#[stubr::mock("module/engines/google.json")]
async fn submit_test() {
    let server = spawn_mock_dns_server().await;
    let resolver = server.get_resolver().await;
    let mut dispatcher = Google::dispatcher();

    if let SubscanModuleDispatcher::GenericSearchEngineModule(ref mut module) = dispatcher {
        module.url = stubr.path("/search").parse().unwrap();
    }

    let google = SubscanModule::from(dispatcher);
    let pool = SubscanModulePool::new(TEST_DOMAIN.into(), resolver, CacheFilter::default());
    let item = ScanResultItem {
        subdomain: TEST_BAR_SUBDOMAIN.into(),
        ip: Some(IpAddr::V4(Ipv4Addr::from_str(LOCAL_HOST).unwrap())),
    };

    assert!(pool.clone().is_empty().await);

    pool.clone().submit(google).await;
    pool.clone().spawn_runners(1).await;

    assert_eq!(pool.clone().len().await, 1);
    pool.clone().join().await;

    pool.clone().spawn_resolvers(1).await;
    pool.clone().join().await;

    assert_eq!(pool.result().await.results, [item].into());
}

#[tokio::test]
#[stubr::mock("module/engines/google.json")]
async fn result_test() {
    let server = spawn_mock_dns_server().await;
    let resolver = server.get_resolver().await;
    let mut google_dispatcher = Google::dispatcher();

    if let SubscanModuleDispatcher::GenericSearchEngineModule(ref mut module) = google_dispatcher {
        module.url = stubr.path("/search").parse().unwrap();
    }

    let google = SubscanModule::from(google_dispatcher);
    let pool = SubscanModulePool::new(TEST_DOMAIN.into(), resolver, CacheFilter::default());

    pool.clone().submit(google).await;
    pool.clone().start(1).await;

    let binding = pool.result().await;
    let result = binding.results.first();

    assert!(result.is_some());
    assert!(result.unwrap().ip.is_some());

    assert_eq!(result.unwrap().subdomain, TEST_BAR_SUBDOMAIN);
    assert_eq!(result.unwrap().ip.unwrap().to_string(), LOCAL_HOST);
}
