use std::collections::BTreeSet;

use async_trait::async_trait;
use subscan::{
    enums::content::Content,
    interfaces::extractor::SubdomainExtractorInterface,
    types::core::{Result, Subdomain},
};

pub struct CustomExtractor {}

#[async_trait]
impl SubdomainExtractorInterface for CustomExtractor {
    async fn extract(&self, content: Content, _domain: &str) -> Result<BTreeSet<Subdomain>> {
        let subdomain = content.as_string().replace("-", "");

        Ok([subdomain].into())
    }
}

#[tokio::main]
async fn main() {
    let content = Content::from("--foo.com--");
    let extractor = CustomExtractor {};
    let result = extractor.extract(content, "foo.com").await.unwrap();

    assert_eq!(result, ["foo.com".into()].into());
}
