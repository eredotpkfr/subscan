mod common;

use common::constants::{TEST_BAR_SUBDOMAIN, TEST_BAZ_SUBDOMAIN, TEST_DOMAIN};
use std::collections::BTreeSet;
use subscan::{interfaces::extractor::SubdomainExtractorInterface, types::core::Subdomain};
#[cfg(test)]
mod html {
    use super::*;
    use subscan::extractors::html::HTMLExtractor;

    #[tokio::test]
    async fn extract_without_removes() {
        let content = "
            <article>
                <div>
                    <a>
                        <span>bar.foo.com</span>
                        <span>baz.foo.com</span>
                    </a>
                </div>
            </article>
        "
        .to_string();

        let selector = String::from("article > div > a > span:first-child");
        let extractor = HTMLExtractor::new(selector, vec![]);
        let result = extractor.extract(content, TEST_DOMAIN.to_string()).await;

        assert_eq!(
            result,
            BTreeSet::from([Subdomain::from(TEST_BAR_SUBDOMAIN)])
        );
    }

    #[tokio::test]
    async fn extract_with_removes() {
        let content = "
            <article>
                <div>
                    <a>
                        <span>bar<br>.foo.com</span>
                        <span>baz.foo.<br>com</span>
                    </a>
                </div>
            </article>
        "
        .to_string();

        let selector = String::from("article > div > a > span");
        let extractor = HTMLExtractor::new(selector, vec!["<br>".to_string()]);
        let result = extractor.extract(content, TEST_DOMAIN.to_string()).await;

        assert_eq!(result.len(), 2);
        assert_eq!(
            result,
            BTreeSet::from([
                Subdomain::from(TEST_BAR_SUBDOMAIN),
                Subdomain::from(TEST_BAZ_SUBDOMAIN)
            ])
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
            BTreeSet::from([
                Subdomain::from(TEST_BAR_SUBDOMAIN),
                Subdomain::from(TEST_BAZ_SUBDOMAIN),
            ])
        );
    }
}
