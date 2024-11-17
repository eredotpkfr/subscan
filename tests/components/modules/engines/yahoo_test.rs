use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mock::funcs,
};
use subscan::{interfaces::module::SubscanModuleInterface, modules::engines::yahoo::Yahoo};

#[tokio::test]
#[stubr::mock("module/engines/yahoo.json")]
async fn run_test() {
    let mut yahoo = Yahoo::dispatcher();

    funcs::wrap_module_url(&mut yahoo, &stubr.path("/search"));

    let result = yahoo.run(TEST_DOMAIN).await.unwrap();

    assert_eq!(result.subdomains, [TEST_BAR_SUBDOMAIN.into()].into());
}
