use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_BAZ_SUBDOMAIN, TEST_DOMAIN},
    utils::read_testdata,
};
use serde_json::Value;
use std::collections::BTreeSet;
use subscan::{
    enums::content::Content,
    error::{ModuleErrorKind::JSONExtract, SubscanError},
    extractors::json::JSONExtractor,
    interfaces::extractor::SubdomainExtractorInterface,
};

#[tokio::test]
async fn extract_test() {
    let json = read_testdata("json/subdomains.json");

    let inner_parser = |json: Value, _domain: &str| {
        if let Some(subs) = json["data"]["subdomains"].as_array() {
            let filter = |json: &Value| Some(json["subdomain"].as_str().unwrap().to_string());

            return Ok(BTreeSet::from_iter(subs.iter().filter_map(filter)));
        }

        Err(SubscanError::from(JSONExtract))
    };

    let extractor = JSONExtractor::new(Box::new(inner_parser));

    let result = extractor.extract(json, TEST_DOMAIN).await;
    let no_result = extractor.extract(Content::default(), TEST_DOMAIN).await;

    let expected = BTreeSet::from([
        TEST_BAR_SUBDOMAIN.to_string(),
        TEST_BAZ_SUBDOMAIN.to_string(),
    ]);

    assert!(result.is_ok());
    assert!(no_result.is_err());

    assert_eq!(no_result.err().unwrap(), SubscanError::from(JSONExtract));
    assert_eq!(result.unwrap(), expected);
}
