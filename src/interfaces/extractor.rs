use crate::{
    enums::{content::Content, dispatchers::SubdomainExtractorDispatcher},
    extractors::{html::HTMLExtractor, json::JSONExtractor, regex::RegexExtractor},
    types::core::Subdomain,
};
use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use std::collections::BTreeSet;

/// Extractor trait definition to implement subdomain extractors
///
/// All subdomain extractors that implemented in the future
/// must be compatible with this trait. Basically it has single
/// `extract` method like a `main` method. It should extract
/// subdomain addresses and return them from given [`String`]
/// content
///
/// # Examples
///
/// ```
/// use std::collections::BTreeSet;
/// use subscan::interfaces::extractor::SubdomainExtractorInterface;
/// use subscan::types::core::Subdomain;
/// use subscan::enums::content::Content;
/// use async_trait::async_trait;
///
/// pub struct CustomExtractor {}
///
/// #[async_trait]
/// impl SubdomainExtractorInterface for CustomExtractor {
///     async fn extract(&self, content: Content, domain: &str) -> BTreeSet<Subdomain> {
///         let sub = content.as_string().replace("-", "");
///
///         [Subdomain::from(sub)].into()
///     }
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let content = Content::from("--foo.com--");
///     let extractor = CustomExtractor {};
///
///     let result = extractor.extract(content, "foo.com").await;
///
///     assert_eq!(result, [Subdomain::from("foo.com")].into());
/// }
/// ```
#[async_trait]
#[enum_dispatch]
pub trait SubdomainExtractorInterface: Send + Sync {
    /// Generic extract method, it should extract subdomain addresses
    /// from given [`String`] content
    async fn extract(&self, content: Content, domain: &str) -> BTreeSet<Subdomain>;
}
