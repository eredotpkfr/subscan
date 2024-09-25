use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_BAZ_SUBDOMAIN, TEST_DOMAIN},
    funcs::read_testdata,
};
use serde_json::Value;
use std::collections::BTreeSet;
use subscan::extractors::json::JSONExtractor;
use subscan::interfaces::extractor::SubdomainExtractorInterface;

#[tokio::test]
async fn extract_test() {
    let json = read_testdata("json/subdomains.json");

    let inner_parser = |item: Value| {
        if let Some(subs) = item["data"]["subdomains"].as_array() {
            let filter = |item: &Value| Some(item["subdomain"].as_str().unwrap().to_string());

            BTreeSet::from_iter(subs.iter().filter_map(filter))
        } else {
            BTreeSet::new()
        }
    };

    let domain = TEST_DOMAIN.to_string();
    let extractor = JSONExtractor::new(Box::new(inner_parser));

    let result = extractor.extract(json, domain.clone()).await;
    let no_result = extractor.extract(String::new(), domain).await;

    let expected = BTreeSet::from([
        TEST_BAR_SUBDOMAIN.to_string(),
        TEST_BAZ_SUBDOMAIN.to_string(),
    ]);

    assert_eq!(result, expected);
    assert_eq!(no_result, BTreeSet::new());
}
