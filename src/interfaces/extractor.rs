use crate::types::core::Subdomain;
use async_trait::async_trait;
use std::collections::BTreeSet;

#[async_trait]
pub trait SubdomainExtractorInterface: Send + Sync {
    async fn extract(&self, content: String, domain: String) -> BTreeSet<Subdomain>;
}
