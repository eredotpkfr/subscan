# Extractors

Extractor components are responsible for parsing subdomain addresses from any `Content` object

The extractor components already implemented in `Subscan` are as follows

- [HTMLExtractor](https://docs.rs/subscan/latest/subscan/extractors/html/struct.HTMLExtractor.html)

    Extracts subdomain addresses from inner text by given `XPath` or `CSS` selector

- [JSONExtractor](https://docs.rs/subscan/latest/subscan/extractors/json/struct.JSONExtractor.html)

    Extracts subdomain addresses from `JSON` content. `JSON` parsing function must be given for this extractor

- [RegexExtractor](https://docs.rs/subscan/latest/subscan/extractors/regex/struct.RegexExtractor.html)

    Regex extractor component generates subdomain pattern by given domain address and extracts subdomains via this pattern

## Create Your Custom Extractor

Each extractor component should be implemented following the interface below. For a better understanding, you can explore the [docs.rs](https://docs.rs/subscan/latest/subscan/interfaces/extractor/index.html) page and review the crates listed below

- [async_trait](https://github.com/dtolnay/async-trait)
- [enum_dispatch](https://gitlab.com/antonok/enum_dispatch)

```rust,ignore
#[async_trait]
#[enum_dispatch]
pub trait SubdomainExtractorInterface: Send + Sync {
    // Generic extract method, it should extract subdomain addresses
    // from given Content
    async fn extract(&self, content: Content, domain: &str) -> Result<BTreeSet<Subdomain>>;
}
```

Below is a simple example of a custom extractor. For more examples, you can check the [examples/](https://github.com/eredotpkfr/subscan/tree/main/examples) folder on the project's GitHub page. You can also refer to the source code of predefined requester implementations for a better understanding

```rust,ignore
pub struct CustomExtractor {}

#[async_trait]
impl SubdomainExtractorInterface for CustomExtractor {
    async fn extract(&self, content: Content, _domain: &str) -> Result<BTreeSet<Subdomain>> {
        let subdomain = content.as_string().replace("-", "");

        Ok([subdomain].into())
    }
}
```
