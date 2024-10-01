use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_BAZ_SUBDOMAIN, TEST_DOMAIN},
    funcs::read_stub,
    mocks::wrap_url_with_mock_func,
};
use reqwest::Url;
use serde_json::{self, Value};
use std::{collections::BTreeSet, env};
use subscan::{
    interfaces::module::SubscanModuleInterface,
    modules::integrations::builtwith::{self, BUILTWITH_MODULE_NAME, BUILTWITH_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/builtwith.json")]
async fn builtwith_run_test() {
    let mut builtwith = builtwith::Builtwith::new();
    let (env_name, _) = builtwith.fetch_apikey().await;

    env::set_var(&env_name, "builtwith-api-key");

    builtwith.url = wrap_url_with_mock_func(stubr.path("/builtwith").as_str());

    let result = builtwith.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(builtwith.name().await, BUILTWITH_MODULE_NAME);
    assert_eq!(
        result,
        [
            TEST_BAR_SUBDOMAIN.to_string(),
            TEST_BAZ_SUBDOMAIN.to_string(),
        ]
        .into()
    );

    env::remove_var(env_name);
}

#[tokio::test]
async fn get_query_url_test() {
    let params = &[
        ("HIDETEXT", "yes"),
        ("HIDEDL", "yes"),
        ("NOLIVE", "yes"),
        ("NOMETA", "yes"),
        ("NOPII", "yes"),
        ("NOATTR", "yes"),
    ];

    let expected = Url::parse_with_params(BUILTWITH_URL, params).unwrap();
    let url = builtwith::Builtwith::get_query_url(TEST_DOMAIN);

    assert_eq!(url, format!("{expected}&LOOKUP={TEST_DOMAIN}"));
}

#[tokio::test]
async fn extract_test() {
    let json = read_stub("module/integrations/builtwith.json")["response"]["jsonBody"].clone();
    let extracted = builtwith::Builtwith::extract(json, TEST_DOMAIN.to_string());
    let not_extracted = builtwith::Builtwith::extract(Value::Null, TEST_DOMAIN.to_string());

    assert_eq!(
        extracted,
        [
            TEST_BAR_SUBDOMAIN.to_string(),
            TEST_BAZ_SUBDOMAIN.to_string()
        ]
        .into()
    );
    assert_eq!(not_extracted, BTreeSet::new());
}
