use std::{
    net::{IpAddr, Ipv4Addr},
    str::FromStr,
    sync::Arc,
};

use subscan::{
    enums::dispatchers::SubscanModuleDispatcher,
    modules::engines::google::Google,
    pools::module::SubscanModulePool,
    types::{core::SubscanModule, result::item::ScanResultItem},
};
use tokio::sync::Notify;

use crate::common::{MockDNSServer, LOCAL_HOST};

const TEST_DOMAIN: &str = "foo.com";
const TEST_BAR_SUBDOMAIN: &str = "bar.foo.com";

#[tokio::test]
#[stubr::mock("module/engines/google.json")]
async fn submit_test() {
    let notify_one = Arc::new(Notify::new());
    let notift_two = notify_one.clone();

    let server = MockDNSServer::new(TEST_DOMAIN);
    let rconfig = server.get_resolver_config().await;

    tokio::spawn(async move {
        notift_two.notify_one();
        server.start().await;
    });

    notify_one.notified().await;

    let mut dispatcher = Google::dispatcher();

    if let SubscanModuleDispatcher::GenericSearchEngineModule(ref mut module) = dispatcher {
        module.url = stubr.path("/search").parse().unwrap();
    }

    let google = SubscanModule::from(dispatcher);
    let pool = SubscanModulePool::new(TEST_DOMAIN.into(), rconfig.into());

    assert!(pool.clone().is_empty().await);

    pool.clone().submit(google).await;
    pool.clone().spawn_runners(1).await;

    assert_eq!(pool.clone().len().await, 1);
    pool.clone().join().await;

    pool.clone().spawn_resolvers(1).await;
    pool.clone().join().await;

    assert_eq!(
        pool.result().await.results,
        [ScanResultItem {
            subdomain: TEST_BAR_SUBDOMAIN.into(),
            ip: Some(IpAddr::V4(Ipv4Addr::from_str(LOCAL_HOST).unwrap()))
        }]
        .into()
    );
}

#[tokio::test]
#[stubr::mock("module/engines/google.json")]
async fn results_test() {
    let notify_one = Arc::new(Notify::new());
    let notift_two = notify_one.clone();

    let server = MockDNSServer::new(TEST_DOMAIN);
    let rconfig = server.get_resolver_config().await;

    tokio::spawn(async move {
        notift_two.notify_one();
        server.start().await;
    });

    notify_one.notified().await;

    let mut google_dispatcher = Google::dispatcher();

    if let SubscanModuleDispatcher::GenericSearchEngineModule(ref mut module) = google_dispatcher {
        module.url = stubr.path("/search").parse().unwrap();
    }

    let google = SubscanModule::from(google_dispatcher);
    let pool = SubscanModulePool::new(TEST_DOMAIN.into(), rconfig.into());

    pool.clone().submit(google).await;
    pool.clone().start(1).await;

    let binding = pool.result().await;
    let result = binding.results.first();

    assert!(result.is_some());
    assert!(result.unwrap().ip.is_some());

    assert_eq!(result.unwrap().subdomain, TEST_BAR_SUBDOMAIN);
    assert_eq!(result.unwrap().ip.unwrap().to_string(), LOCAL_HOST);
}
