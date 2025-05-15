use std::{
    collections::BTreeSet,
    net::{IpAddr, Ipv4Addr},
    str::FromStr,
};

use subscan::{
    enums::{
        auth::AuthenticationMethod::NoAuthentication, cache::CacheFilter,
        dispatchers::SubscanModuleDispatcher,
    },
    error::ModuleErrorKind::UrlParse,
    interfaces::module::SubscanModuleInterface,
    modules::{
        engines::google::{Google, GOOGLE_MODULE_NAME},
        integrations::alienvault::{AlienVault, ALIENVAULT_MODULE_NAME},
    },
    pools::module::SubscanModulePool,
    types::{
        config::pool::PoolConfig,
        core::SubscanModule,
        filters::ModuleNameFilter,
        result::{
            item::SubscanResultItem,
            status::{SkipReason::SkippedByUser, SubscanModuleStatus::Finished},
        },
    },
};

use crate::common::{
    constants::{LOCAL_HOST, TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mock::{funcs, modules, resolver::MockResolver},
};

#[tokio::test]
#[stubr::mock("module/engines/google.json")]
async fn submit_test() {
    let resolver = MockResolver::default_boxed();
    let config = PoolConfig {
        concurrency: 1,
        ..Default::default()
    };

    let mut dispatcher = Google::dispatcher();

    funcs::wrap_module_url(&mut dispatcher, &stubr.path("/search"));

    let google = SubscanModule::from(dispatcher);
    let pool = SubscanModulePool::new(config, resolver);

    let local = IpAddr::V4(Ipv4Addr::from_str(LOCAL_HOST).unwrap());
    let expecteds = [SubscanResultItem {
        subdomain: TEST_BAR_SUBDOMAIN.into(),
        ip: Some(local),
    }];

    assert!(pool.clone().is_empty().await);

    pool.clone().start(TEST_DOMAIN, &vec![google]).await;

    assert_eq!(pool.clone().len().await, 0);
    assert_eq!(pool.result().await.items, expecteds.into());
    assert_eq!(
        pool.result().await.statistics.get(GOOGLE_MODULE_NAME).unwrap().status,
        Finished
    );
}

#[tokio::test]
#[stubr::mock("module/engines/google.json")]
async fn result_test() {
    let resolver = MockResolver::default_boxed();
    let config = PoolConfig {
        concurrency: 1,
        ..Default::default()
    };

    let mut dispatcher = Google::dispatcher();

    funcs::wrap_module_url(&mut dispatcher, &stubr.path("/search"));

    let google = SubscanModule::from(dispatcher);
    let pool = SubscanModulePool::new(config, resolver);

    pool.clone().start(TEST_DOMAIN, &vec![google]).await;

    let result = pool.result().await;

    let local = IpAddr::V4(Ipv4Addr::from_str(LOCAL_HOST).unwrap());
    let expecteds = [SubscanResultItem {
        subdomain: TEST_BAR_SUBDOMAIN.into(),
        ip: Some(local),
    }];

    assert_eq!(result.items, expecteds.into());
    assert_eq!(
        result.statistics.get(GOOGLE_MODULE_NAME).unwrap().status,
        Finished
    );
}

#[tokio::test]
#[stubr::mock("module/engines/google.json")]
async fn result_test_with_filter() {
    let resolver = MockResolver::default_boxed();
    let filter = ModuleNameFilter {
        modules: vec!["google".to_string()],
        skips: vec!["alienvault".to_string()],
    };

    let config = PoolConfig {
        filter: CacheFilter::FilterByName(filter),
        concurrency: 1,
        ..Default::default()
    };

    let mut google_dispatcher = Google::dispatcher();
    let mut alienvault_dispatcher = AlienVault::dispatcher();

    funcs::wrap_module_url(&mut google_dispatcher, &stubr.path("/search"));
    funcs::wrap_module_url(&mut alienvault_dispatcher, &stubr.path("/alienvault"));

    let google = SubscanModule::from(google_dispatcher);
    let alienvault = SubscanModule::from(alienvault_dispatcher);
    let pool = SubscanModulePool::new(config, resolver);

    pool.clone().start(TEST_DOMAIN, &vec![google, alienvault]).await;

    let result = pool.result().await;

    let local = IpAddr::V4(Ipv4Addr::from_str(LOCAL_HOST).unwrap());
    let expecteds = [SubscanResultItem {
        subdomain: TEST_BAR_SUBDOMAIN.into(),
        ip: Some(local),
    }];

    assert_eq!(result.items, expecteds.into());
    assert_eq!(
        result.statistics.get(ALIENVAULT_MODULE_NAME).unwrap().status,
        SkippedByUser.into()
    );
    assert_eq!(
        result.statistics.get(GOOGLE_MODULE_NAME).unwrap().status,
        Finished
    );
}

#[tokio::test]
#[stubr::mock("module/generics/integration-with-invalid-data.json")]
async fn result_test_with_invalid_data() {
    let resolver = MockResolver::default_boxed();
    let config = PoolConfig {
        filter: CacheFilter::NoFilter,
        concurrency: 1,
        ..Default::default()
    };

    let generic = modules::generic_integration(&stubr.path("/subdomains"), NoAuthentication);
    let dispatcher = SubscanModuleDispatcher::from(generic);
    let module = SubscanModule::from(dispatcher);
    let pool = SubscanModulePool::new(config, resolver);

    pool.clone().start(TEST_DOMAIN, &vec![module.clone()]).await;

    let result = pool.result().await;

    let binding = module.lock().await;
    let name = binding.name().await;
    let local = IpAddr::V4(Ipv4Addr::from_str(LOCAL_HOST).unwrap());
    let expecteds = [SubscanResultItem {
        subdomain: TEST_BAR_SUBDOMAIN.into(),
        ip: Some(local),
    }];

    assert_eq!(result.items, expecteds.into());
    assert_eq!(result.statistics.get(name).unwrap().status, Finished);
}

#[tokio::test]
async fn result_test_with_error() {
    let resolver = MockResolver::default_boxed();
    let config = PoolConfig {
        concurrency: 1,
        ..Default::default()
    };

    let mut dispatcher = AlienVault::dispatcher();

    if let SubscanModuleDispatcher::GenericIntegrationModule(ref mut alienvault) = dispatcher {
        alienvault.funcs.url = Box::new(|_| "invalid-url".to_string());
    }

    let alienvault = SubscanModule::from(dispatcher);
    let pool = SubscanModulePool::new(config, resolver);

    pool.clone().start(TEST_DOMAIN, &vec![alienvault]).await;

    let result = pool.result().await;

    assert_eq!(result.items, BTreeSet::new());
    assert_eq!(
        result.statistics.get(ALIENVAULT_MODULE_NAME).unwrap().status,
        UrlParse.into()
    );
}
