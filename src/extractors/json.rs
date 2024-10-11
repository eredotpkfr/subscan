use crate::{
    enums::Content,
    interfaces::extractor::SubdomainExtractorInterface,
    types::{core::Subdomain, func::InnerExtractFunc},
};
use async_trait::async_trait;
use std::collections::BTreeSet;

/// JSON content parser wrapper struct
///
/// This object compatible with [`SubdomainExtractorInterface`] and it uses `extract`
/// method to extract subdomain addresses from JSON content.
/// JSON parsing function must be given for this extractor. Please
/// follow up examples to learn usage techniques
pub struct JSONExtractor {
    inner: InnerExtractFunc,
}

impl JSONExtractor {
    /// Creates a new [`JSONExtractor`] instance
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::extractors::json::JSONExtractor;
    /// use subscan::interfaces::extractor::SubdomainExtractorInterface;
    /// use subscan::enums::Content;
    /// use std::collections::BTreeSet;
    /// use serde_json::{Value, json};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let domain = String::from("foo.com");
    ///     let inner = |content: Value, _domain: String| {
    ///         let bar = content["foo"].as_str().unwrap().to_string();
    ///
    ///         [bar].into()
    ///     };
    ///
    ///     let json = Content::from(json!({"foo": "bar"}));
    ///     let extractor = JSONExtractor::new(Box::new(inner));
    ///     let expected = BTreeSet::from(["bar".to_string()]);
    ///
    ///     assert_eq!(extractor.extract(json, domain).await, expected);
    /// }
    /// ```
    pub fn new(inner: InnerExtractFunc) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl SubdomainExtractorInterface for JSONExtractor {
    /// Main extraction method to extract subdomains from
    /// given JSON content
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::extractors::json::JSONExtractor;
    /// use subscan::interfaces::extractor::SubdomainExtractorInterface;
    /// use subscan::types::core::Subdomain;
    /// use subscan::enums::Content;
    /// use std::collections::BTreeSet;
    /// use serde_json::{Value, json};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let content = Content::from(json!({"foo": "bar"}));
    ///     let domain = "foo.com".to_string();
    ///
    ///     let func = |item: Value, _domain: String| {
    ///         [
    ///             Subdomain::from(item["foo"].as_str().unwrap())
    ///         ].into()
    ///     };
    ///     let extractor = JSONExtractor::new(Box::new(func));
    ///
    ///     let result = extractor.extract(content, domain).await;
    ///
    ///     assert_eq!(result, [Subdomain::from("bar")].into());
    /// }
    /// ```
    async fn extract(&self, content: Content, domain: String) -> BTreeSet<Subdomain> {
        (self.inner)(content.as_json(), domain)
    }
}
