use crate::types::core::Subdomain;
use async_trait::async_trait;
use std::collections::HashSet;

#[async_trait]
pub trait SubdomainExtractorInterface {
    async fn extract(&self, content: String, domain: String) -> HashSet<Subdomain>;
}
