use subscan::{interfaces::module::SubscanModuleInterface, modules::engines::bing::Bing};

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mock::funcs,
};

#[tokio::test]
#[stubr::mock("module/engines/bing.json")]
async fn run_test() {
    let mut bing = Bing::dispatcher();

    funcs::wrap_module_url(&mut bing, &stubr.path("/search"));

    let result = bing.run(TEST_DOMAIN).await.unwrap();

    assert_eq!(result.subdomains, [TEST_BAR_SUBDOMAIN.into()].into());
}
