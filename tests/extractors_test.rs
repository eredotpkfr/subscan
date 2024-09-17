mod common;

use common::{
    constants::{TEST_BAR_SUBDOMAIN, TEST_BAZ_SUBDOMAIN, TEST_DOMAIN},
    funcs::read_testdata,
};
use subscan::interfaces::extractor::SubdomainExtractorInterface;

#[cfg(test)]
mod html {
    use super::*;
    use subscan::extractors::html::HTMLExtractor;

    #[tokio::test]
    async fn extract_without_removes() {
        let html = read_testdata("html/subdomains.html");

        let selector = String::from("article > div > a > span:first-child");
        let extractor = HTMLExtractor::new(selector, vec![]);
        let result = extractor.extract(html, TEST_DOMAIN.to_string()).await;

        assert_eq!(result, [TEST_BAR_SUBDOMAIN.to_string()].into());
    }

    #[tokio::test]
    async fn extract_with_removes() {
        let html = read_testdata("html/subdomains-with-removes.html");

        let selector = String::from("article > div > a > span");
        let extractor = HTMLExtractor::new(selector, vec!["<br>".to_string()]);
        let result = extractor.extract(html, TEST_DOMAIN.to_string()).await;

        assert_eq!(result.len(), 2);
        assert_eq!(
            result,
            [
                TEST_BAR_SUBDOMAIN.to_string(),
                TEST_BAZ_SUBDOMAIN.to_string()
            ]
            .into()
        );
    }
}

#[cfg(test)]
mod regex {
    use super::*;
    use subscan::extractors::regex::RegexExtractor;

    #[tokio::test]
    async fn extract_one_test() {
        let extractor = RegexExtractor::default();

        let match_content = String::from(TEST_BAR_SUBDOMAIN);
        let no_match_content = String::from("foobarbaz");

        assert!(extractor
            .extract_one(match_content, TEST_DOMAIN.to_string())
            .is_some());
        assert!(extractor
            .extract_one(no_match_content, TEST_DOMAIN.to_string())
            .is_none());
    }

    #[tokio::test]
    async fn extract_test() {
        let content = String::from("bar.foo.com\nbaz.foo.com");

        let extractor = RegexExtractor::default();
        let result = extractor.extract(content, TEST_DOMAIN.to_string()).await;

        assert_eq!(result.len(), 2);
        assert_eq!(
            result,
            [
                TEST_BAR_SUBDOMAIN.to_string(),
                TEST_BAZ_SUBDOMAIN.to_string(),
            ]
            .into()
        );
    }
}
