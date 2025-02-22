use subscan::{
    enums::content::Content,
    interfaces::module::SubscanModuleInterface,
    modules::integrations::hackertarget::{HackerTarget, HACKERTARGET_URL},
};

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    mock::funcs,
};

#[tokio::test]
#[stubr::mock("module/integrations/hackertarget.json")]
async fn run_test() {
    let mut hackertarget = HackerTarget::dispatcher();

    funcs::wrap_module_url(&mut hackertarget, &stubr.path("/hackertarget"));

    let result = hackertarget.run(TEST_DOMAIN).await.unwrap();

    assert_eq!(result.subdomains, [TEST_BAR_SUBDOMAIN.into()].into());
}

#[tokio::test]
async fn get_query_url_test() {
    let url = HackerTarget::get_query_url(TEST_DOMAIN);
    let expected = format!("{HACKERTARGET_URL}/?q={TEST_DOMAIN}");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn get_next_url_test() {
    let url = TEST_URL.parse().unwrap();
    let next = HackerTarget::get_next_url(url, Content::Empty);

    assert!(next.is_none());
}
