use std::collections::BTreeSet;

use subscan::{
    extractors::html::HTMLExtractor, interfaces::extractor::SubdomainExtractorInterface,
};

use crate::common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_BAZ_SUBDOMAIN, TEST_DOMAIN},
    utils::read_testdata,
};

#[tokio::test]
async fn extract_without_removes() {
    let html = read_testdata("html/subdomains.html");

    let selector = String::from("article > div > a > span:first-child");
    let extractor = HTMLExtractor::new(selector, vec![]);
    let result = extractor.extract(html, TEST_DOMAIN).await;

    assert_eq!(result.unwrap(), [TEST_BAR_SUBDOMAIN.into()].into());
}

#[tokio::test]
async fn extract_with_removes() {
    let html = read_testdata("html/subdomains-with-removes.html");

    let selector = String::from("article > div > a > span");
    let extractor = HTMLExtractor::new(selector, vec!["<br>".to_string()]);
    let result = extractor.extract(html, TEST_DOMAIN).await;

    let expected = BTreeSet::from([
        TEST_BAR_SUBDOMAIN.to_string(),
        TEST_BAZ_SUBDOMAIN.to_string(),
    ]);

    assert_eq!(result.unwrap(), expected);
}
