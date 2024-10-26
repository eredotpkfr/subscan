use subscan::{
    enums::SubscanModuleDispatcher, modules::engines::google::Google,
    pool::SubscanModuleRunnerPool, types::core::SubscanModule,
};

const TEST_DOMAIN: &str = "foo.com";
const TEST_BAR_SUBDOMAIN: &str = "bar.foo.com";

#[tokio::test]
#[stubr::mock("module/engines/google.json")]
async fn submit_test() {
    let mut dispatcher = Google::dispatcher();

    if let SubscanModuleDispatcher::GenericSearchEngineModule(ref mut module) = dispatcher {
        module.url = stubr.path("/search").parse().unwrap();
    }

    let google = SubscanModule::from(dispatcher);
    let pool = SubscanModuleRunnerPool::new(TEST_DOMAIN.into());

    assert!(pool.clone().is_empty().await);

    pool.clone().spawn_runners(1).await;

    assert_eq!(pool.clone().len().await, 1);

    pool.clone().submit(google).await;
    pool.clone().join().await;

    assert_eq!(pool.results().await, [TEST_BAR_SUBDOMAIN.into()].into());
}
