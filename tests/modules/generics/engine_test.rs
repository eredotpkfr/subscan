use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    mocks::generic_search_engine,
};
use subscan::interfaces::module::SubscanModuleInterface;

#[tokio::test]
async fn get_search_query_test() {
    let module = generic_search_engine(TEST_URL);

    let mut query = module.get_search_query(TEST_DOMAIN.to_string()).await;

    assert_eq!(query.as_search_str(), "site:foo.com");
    assert_eq!(module.name().await, module.name);
}

#[tokio::test]
#[stubr::mock("module/generics/search-engine.json")]
async fn run_test() {
    let mut module = generic_search_engine(&stubr.path("/search"));

    let result = module.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(module.name().await, module.name);
    assert_eq!(result, [TEST_BAR_SUBDOMAIN.into()].into());
}
