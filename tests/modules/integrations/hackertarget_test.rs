use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    mocks,
};
use subscan::{
    interfaces::module::SubscanModuleInterface,
    modules::integrations::hackertarget::{self, HACKERTARGET_MODULE_NAME, HACKERTARGET_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/hackertarget.json")]
async fn hackertarget_run_test() {
    let mut hackertarget = hackertarget::HackerTarget::dispatcher();

    mocks::wrap_module_dispatcher_url_field(&mut hackertarget, &stubr.path("/hackertarget"));

    let result = hackertarget.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(hackertarget.name().await, HACKERTARGET_MODULE_NAME);
    assert_eq!(result, [TEST_BAR_SUBDOMAIN.into()].into());
}

#[tokio::test]
async fn get_query_url_test() {
    let url = hackertarget::HackerTarget::get_query_url(TEST_DOMAIN);

    assert_eq!(url, format!("{HACKERTARGET_URL}/?q={TEST_DOMAIN}"));
}
