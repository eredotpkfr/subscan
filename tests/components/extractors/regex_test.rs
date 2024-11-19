use std::collections::BTreeSet;

use subscan::{
    enums::content::Content, extractors::regex::RegexExtractor,
    interfaces::extractor::SubdomainExtractorInterface,
};

use crate::common::constants::{TEST_BAR_SUBDOMAIN, TEST_BAZ_SUBDOMAIN, TEST_DOMAIN};

#[tokio::test]
async fn extract_one_test() {
    let extractor = RegexExtractor::default();

    let matches = String::from(TEST_BAR_SUBDOMAIN);
    let no_matches = String::from("foobarbaz");

    assert!(extractor.extract_one(matches, TEST_DOMAIN).is_some());
    assert!(extractor.extract_one(no_matches, TEST_DOMAIN).is_none());
}

#[tokio::test]
async fn extract_test() {
    let content = Content::from("bar.foo.com\nbaz.foo.com");

    let extractor = RegexExtractor::default();
    let result = extractor.extract(content, TEST_DOMAIN).await;

    let expected = BTreeSet::from([
        TEST_BAR_SUBDOMAIN.to_string(),
        TEST_BAZ_SUBDOMAIN.to_string(),
    ]);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), expected);
}
