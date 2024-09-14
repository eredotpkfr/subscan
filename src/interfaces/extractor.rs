use crate::types::core::Subdomain;
use async_trait::async_trait;
use std::collections::BTreeSet;

/// Extractor interface definiton, subscan extractors
/// that implemented in the future must be compatible
/// with this trait
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
///     assert_eq!(result.len(), 1);
///     assert_eq!(result, BTreeSet::from([Subdomain::from("foo.com")]));
/// }
/// ```
#[async_trait]
pub trait SubdomainExtractorInterface: Send + Sync {
    /// Generic extract method, it should extract subdomain addresess
    /// from given [`String`] content
    async fn extract(&self, content: String, domain: String) -> BTreeSet<Subdomain>;
}
