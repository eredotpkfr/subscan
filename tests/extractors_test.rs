#[cfg(test)]
mod html {
    use std::collections::BTreeSet;
    use subscan::{
        extractors::html::HTMLExtractor, interfaces::extractor::SubdomainExtractorInterface,
        types::core::Subdomain,
    };

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
        let domain = String::from("foo.com");
        let selector = String::from("article > div > a > span:first-child");

        let extractor = HTMLExtractor::new(selector, vec![]);
        let result = extractor.extract(content, domain).await;

        assert_eq!(result.len(), 1);
        assert_eq!(result, BTreeSet::from([Subdomain::from("bar.foo.com")]));
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
        let domain = String::from("foo.com");
        let selector = String::from("article > div > a > span");

        let extractor = HTMLExtractor::new(selector, vec!["<br>".to_string()]);
        let result = extractor.extract(content, domain).await;

        assert_eq!(result.len(), 2);
        assert_eq!(
            result,
            BTreeSet::from([
                Subdomain::from("bar.foo.com"),
                Subdomain::from("baz.foo.com")
            ])
        );
    }
}

#[cfg(test)]
mod regex {
    use std::collections::BTreeSet;
    use subscan::{
        extractors::regex::RegexExtractor, interfaces::extractor::SubdomainExtractorInterface,
        types::core::Subdomain,
    };

    #[tokio::test]
    async fn extract_one_test() {
        let domain = String::from("foo.com");
        let extractor = RegexExtractor::new();

        let match_content = String::from("bar.foo.com");
        let no_match_content = String::from("foobarbaz");

        assert!(extractor
            .extract_one(match_content, domain.clone())
            .is_some());
        assert!(extractor.extract_one(no_match_content, domain).is_none());
    }

    #[tokio::test]
    async fn extract_test() {
        let domain = String::from("foo.com");
        let content = String::from("bar.foo.com\nbaz.foo.com");

        let extractor = RegexExtractor::new();
        let result = extractor.extract(content, domain).await;

        assert_eq!(
            result,
            BTreeSet::from([
                Subdomain::from("bar.foo.com"),
                Subdomain::from("baz.foo.com"),
            ])
        );
        assert_eq!(result.len(), 2);
    }
}
