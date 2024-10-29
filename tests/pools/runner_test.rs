use subscan::{
    enums::dispatchers::SubscanModuleDispatcher, modules::engines::google::Google,
    pools::runner::SubscanModuleRunnerPool, types::core::SubscanModule,
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

    pool.clone().submit(google).await;
    pool.clone().spawn_runners(1).await;

    assert_eq!(pool.clone().len().await, 1);

    pool.clone().join().await;

    assert!(!pool.results().await.is_empty());
}

#[tokio::test]
#[stubr::mock("module/engines/google.json")]
async fn results_test() {
    let mut google_dispatcher = Google::dispatcher();

    if let SubscanModuleDispatcher::GenericSearchEngineModule(ref mut module) = google_dispatcher {
        module.url = stubr.path("/search").parse().unwrap();
    }

    let google = SubscanModule::from(google_dispatcher);
    let pool = SubscanModuleRunnerPool::new(TEST_DOMAIN.into());

    pool.clone().submit(google).await;
    pool.clone().spawn_runners(1).await;
    pool.clone().join().await;

    let binding = pool.results().await;
    let result = binding.first();

    assert!(result.is_some());

    assert_eq!(result.unwrap().module, "google");
    assert_eq!(
        result.unwrap().subdomains,
        [TEST_BAR_SUBDOMAIN.into()].into()
    );
}
