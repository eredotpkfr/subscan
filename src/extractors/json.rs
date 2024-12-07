use std::collections::BTreeSet;

use async_trait::async_trait;

use crate::{
    enums::content::Content,
    interfaces::extractor::SubdomainExtractorInterface,
    types::{
        core::{Result, Subdomain},
        func::InnerExtractFunc,
    },
};

/// JSON content parser wrapper struct
///
/// This object compatible with [`SubdomainExtractorInterface`] and it uses `extract` method
/// to extract subdomain addresses from JSON content. JSON parsing function must be given
/// for this extractor. Please follow up examples to learn usage techniques
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
    /// use subscan::enums::content::Content;
    /// use std::collections::BTreeSet;
    /// use serde_json::{Value, json};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let inner = |content: Value, _domain: &str| {
    ///         let bar = content["foo"].as_str().unwrap().to_string();
    ///
    ///         Ok([bar].into())
    ///     };
    ///
    ///     let json = Content::from(json!({"foo": "bar"}));
    ///     let extractor = JSONExtractor::new(Box::new(inner));
    ///     let expected = BTreeSet::from(["bar".to_string()]);
    ///     let result = extractor.extract(json, "foo.com").await.unwrap();
    ///
    ///     assert_eq!(result, expected);
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
    /// use subscan::enums::content::Content;
    /// use std::collections::BTreeSet;
    /// use serde_json::{Value, json};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let content = Content::from(json!({"foo": "bar"}));
    ///
    ///     let inner = |item: Value, _domain: &str| {
    ///         Ok(
    ///             [
    ///                 Subdomain::from(item["foo"].as_str().unwrap())
    ///             ].into()
    ///         )
    ///     };
    ///     let extractor = JSONExtractor::new(Box::new(inner));
    ///
    ///     let result = extractor.extract(content, "foo.com").await.unwrap();
    ///
    ///     assert_eq!(result, [Subdomain::from("bar")].into());
    /// }
    /// ```
    async fn extract(&self, content: Content, domain: &str) -> Result<BTreeSet<Subdomain>> {
        (self.inner)(content.as_json(), domain)
    }
}
