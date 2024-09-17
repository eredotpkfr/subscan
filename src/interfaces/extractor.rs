use crate::enums::SubdomainExtractorDispatcher;
use crate::extractors::{html::HTMLExtractor, regex::RegexExtractor};
use crate::types::core::Subdomain;
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
/// use async_trait::async_trait;
///
/// pub struct CustomExtractor {}
///
/// #[async_trait]
/// impl SubdomainExtractorInterface for CustomExtractor {
///     async fn extract(&self, content: String, domain: String) -> BTreeSet<Subdomain> {
///         BTreeSet::from([
///             Subdomain::from(content.replace("-", ""))
///         ])
///     }
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let content = String::from("--foo.com--");
///     let domain = String::from("foo.com");
///
///     let extractor = CustomExtractor {};
///
///     let result = extractor.extract(content, domain).await;
///
///     assert_eq!(result, BTreeSet::from([Subdomain::from("foo.com")]));
/// }
/// ```
#[async_trait]
#[enum_dispatch]
pub trait SubdomainExtractorInterface: Send + Sync {
    /// Generic extract method, it should extract subdomain addresses
    /// from given [`String`] content
    async fn extract(&self, content: String, domain: String) -> BTreeSet<Subdomain>;
}
