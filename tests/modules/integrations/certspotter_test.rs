use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_DOMAIN},
    funcs::read_stub,
    mocks,
};
use reqwest::Url;
use serde_json::{self, Value};
use std::{collections::BTreeSet, env};
use subscan::{
    interfaces::module::SubscanModuleInterface,
    modules::integrations::certspotter::{self, CERTSPOTTER_MODULE_NAME, CERTSPOTTER_URL},
};

#[tokio::test]
#[stubr::mock("module/integrations/certspotter.json")]
async fn certspotter_run_test() {
    let mut certspotter = certspotter::CertSpotter::dispatcher();
    let (env_name, _) = certspotter.fetch_apikey().await;

    env::set_var(&env_name, "certspotter-api-key");
    mocks::wrap_module_dispatcher_url_field(&mut certspotter, &stubr.path("/certspotter"));

    let result = certspotter.run(TEST_DOMAIN.to_string()).await;

    assert_eq!(certspotter.name().await, CERTSPOTTER_MODULE_NAME);
    assert_eq!(result, [TEST_BAR_SUBDOMAIN.into()].into());

    env::remove_var(env_name);
}

#[tokio::test]
async fn get_query_url_test() {
    let url = certspotter::CertSpotter::get_query_url(TEST_DOMAIN);

    let params = &[
        ("domain", TEST_DOMAIN),
        ("include_subdomains", "true"),
        ("expand", "dns_names"),
    ];

    let expected = Url::parse_with_params(CERTSPOTTER_URL, params);

    assert_eq!(url, expected.unwrap().to_string());
}

#[tokio::test]
async fn extract_test() {
    let json = read_stub("module/integrations/certspotter.json")["response"]["jsonBody"].clone();
    let extracted = certspotter::CertSpotter::extract(json, TEST_DOMAIN.to_string());
    let not_extracted = certspotter::CertSpotter::extract(Value::Null, TEST_DOMAIN.to_string());

    assert_eq!(extracted, [TEST_BAR_SUBDOMAIN.into()].into());
    assert_eq!(not_extracted, BTreeSet::new());
}
