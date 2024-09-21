use crate::interfaces::extractor::SubdomainExtractorInterface;
use crate::types::core::{InnerExtractMethod, Subdomain};
use async_trait::async_trait;
use serde_json;
use std::collections::BTreeSet;

/// JSON content parser wrapper struct. This object compatible
/// with [`SubdomainExtractorInterface`] and it uses `extract`
/// method to extract subdomain addresses from JSON content.
/// JSON parsing function must be given for this extractor. Please
/// follow up examples to learn usage techniques
pub struct JSONExtractor {
    inner: InnerExtractMethod,
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
    /// use crate::subscan::interfaces::extractor::SubdomainExtractorInterface;
    /// use std::collections::BTreeSet;
    /// use serde_json::Value;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let json = "{\"foo\": \"bar\"}".to_string();
    ///     let domain = "foo.com".to_string();
    ///
    ///     let func = |item: Value| {
    ///         BTreeSet::from([item["foo"].as_str().unwrap().into()])
    ///     };
    ///     let extractor = JSONExtractor::new(Box::new(func));
    ///
    ///     let result = extractor.extract(json, domain).await;
    ///
    ///     assert_eq!(result, BTreeSet::from(["bar".into()]));
    /// }
    /// ```
    async fn extract(&self, content: String, _domain: String) -> BTreeSet<Subdomain> {
        (self.inner)(serde_json::from_str(&content).unwrap_or_default())
    }
}

impl JSONExtractor {
    /// Creates a new [`JSONExtractor`] instance
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::extractors::json::JSONExtractor;
    /// use std::collections::BTreeSet;
    /// use serde_json::Value;
    ///
    /// let extractor = JSONExtractor::new(Box::new(move |_: Value| BTreeSet::default()));
    ///
    /// // do something with extractor instance
    /// ```
    pub fn new(inner: InnerExtractMethod) -> Self {
        Self { inner }
    }
}
