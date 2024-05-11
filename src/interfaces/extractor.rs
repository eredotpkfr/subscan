use crate::types::Subdomain;
use std::collections::HashSet;

pub trait SubdomainExtractorInterface {
    fn extract(&self, content: String, domain: String) -> HashSet<Subdomain>;
}
