use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_BAZ_SUBDOMAIN, TEST_DOMAIN},
    funcs::read_testdata,
};
use serde_json::Value;
use std::collections::BTreeSet;
use subscan::{
    enums::Content, extractors::json::JSONExtractor,
    interfaces::extractor::SubdomainExtractorInterface,
};

#[tokio::test]
async fn extract_test() {
    let json = read_testdata("json/subdomains.json");

    let inner_parser = |json: Value, _domain: &str| {
        if let Some(subs) = json["data"]["subdomains"].as_array() {
            let filter = |json: &Value| Some(json["subdomain"].as_str().unwrap().to_string());

            BTreeSet::from_iter(subs.iter().filter_map(filter))
        } else {
            BTreeSet::new()
        }
    };

    let extractor = JSONExtractor::new(Box::new(inner_parser));

    let result = extractor.extract(json, TEST_DOMAIN).await;
    let no_result = extractor.extract(Content::default(), TEST_DOMAIN).await;

    let expected = BTreeSet::from([
        TEST_BAR_SUBDOMAIN.to_string(),
        TEST_BAZ_SUBDOMAIN.to_string(),
    ]);

    assert_eq!(result, expected);
    assert_eq!(no_result, BTreeSet::new());
}
