use std::env;

use reqwest::Url;
use serde_json::Value;
use subscan::{
    enums::content::Content,
    error::{
        ModuleErrorKind::JSONExtract,
        SubscanError::{self, ModuleErrorWithResult},
    },
    interfaces::module::SubscanModuleInterface,
    modules::integrations::binaryedge::{BinaryEdge, BINARYEDGE_URL},
};

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN, TEST_URL},
    mock::funcs,
    utils::read_stub,
};

#[tokio::test]
#[stubr::mock("module/integrations/binaryedge.json")]
async fn run_test() {
    let mut binaryedge = BinaryEdge::dispatcher();
    let env_name = binaryedge.envs().await.apikey.name;

    env::set_var(&env_name, "binaryedge-api-key");
    funcs::wrap_module_url(&mut binaryedge, &stubr.path("/binaryedge"));

    let result = binaryedge.run(TEST_DOMAIN).await;

    assert!(result.is_err());

    if let ModuleErrorWithResult(result) = result.err().unwrap() {
        assert_eq!(result.subdomains, [TEST_BAR_SUBDOMAIN.into()].into());
    }

    env::remove_var(env_name);
}

#[tokio::test]
async fn get_query_url_test() {
    let url = BinaryEdge::get_query_url(TEST_DOMAIN);
    let expected = format!("{BINARYEDGE_URL}/{TEST_DOMAIN}");

    assert_eq!(url, expected);
}

#[tokio::test]
async fn get_next_url_test() {
    let url = Url::parse(TEST_URL).unwrap();

    let mut next = BinaryEdge::get_next_url(url.clone(), Content::Empty).unwrap();
    let mut expected = Url::parse(&format!("{TEST_URL}/?page=2")).unwrap();

    assert_eq!(next, expected);

    next = BinaryEdge::get_next_url(next, Content::Empty).unwrap();
    expected = Url::parse(&format!("{TEST_URL}/?page=3")).unwrap();

    assert_eq!(next, expected);
}

#[tokio::test]
async fn extract_test() {
    let json = read_stub("module/integrations/binaryedge.json")["response"]["jsonBody"].clone();

    let extracted = BinaryEdge::extract(json, TEST_DOMAIN);
    let not_extracted = BinaryEdge::extract(Value::Null, TEST_DOMAIN);

    assert!(extracted.is_ok());
    assert!(not_extracted.is_err());

    assert_eq!(extracted.unwrap(), [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(
        not_extracted.err().unwrap(),
        SubscanError::from(JSONExtract)
    );
}
