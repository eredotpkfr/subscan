use std::collections::BTreeSet;

use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

use crate::{
    enums::{content::Content, dispatchers::SubdomainExtractorDispatcher},
    extractors::{html::HTMLExtractor, json::JSONExtractor, regex::RegexExtractor},
    types::core::{Result, Subdomain},
};

/// Extractor trait definition to implement subdomain extractors
///
/// All subdomain extractors that implemented in the future must be compatible with this
/// trait. Basically it has single `extract` method like a `main` method. It should
/// extract subdomain addresses and return them from given [`String`] content
#[async_trait]
#[enum_dispatch]
pub trait SubdomainExtractorInterface: Send + Sync {
    /// Generic extract method, it should extract subdomain addresses
    /// from given [`String`] content
    async fn extract(&self, content: Content, domain: &str) -> Result<BTreeSet<Subdomain>>;
}
