use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mock::funcs,
};
use subscan::{interfaces::module::SubscanModuleInterface, modules::engines::bing::Bing};

#[tokio::test]
#[stubr::mock("module/engines/bing.json")]
async fn run_test() {
    let mut bing = Bing::dispatcher();

    funcs::wrap_module_dispatcher_url_field(&mut bing, &stubr.path("/search"));

    let result = bing.run(TEST_DOMAIN).await;

    assert_eq!(result.subdomains, [TEST_BAR_SUBDOMAIN.into()].into());
}
