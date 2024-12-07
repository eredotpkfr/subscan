# Generic Integration Module

The [`GenericIntegrationModule`](https://docs.rs/subscan/latest/subscan/modules/generics/integration/struct.GenericIntegrationModule.html) is primarily used for simple `API` integrations. To understand how it works, check the source code on the [docs.rs](https://docs.rs/subscan/latest/subscan/modules/generics/integration/struct.GenericIntegrationModule.html) page. Additionally, looking at the source code of other modules that use this implementation will help you understand how to utilize it

A module that uses this one internally would look like the following

```rust,ignore
pub const EXAMPLE_MODULE_NAME: &str = "example";
pub const EXAMPLE_URL: &str = "https://api.example.com/api/v1";

pub struct ExampleModule {}

impl ExampleModule {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let requester: RequesterDispatcher = HTTPClient::default().into();
        let extractor: JSONExtractor = JSONExtractor::new(Box::new(Self::extract));

        let generic = GenericIntegrationModule {
            name: EXAMPLE_MODULE_NAME.into(),
            auth: AuthenticationMethod::NoAuthentication,
            funcs: GenericIntegrationCoreFuncs {
                url: Box::new(Self::get_query_url),
                next: Box::new(Self::get_next_url),
            },
            components: SubscanModuleCoreComponents {
                requester: requester.into(),
                extractor: extractor.into(),
            },
        };

        generic.into()
    }

    pub fn get_query_url(domain: &str) -> String {
        format!("{EXAMPLE_URL}/{domain}/subdomains")
    }

    pub fn get_next_url(_url: Url, _content: Content) -> Option<Url> {
        None
    }

    pub fn extract(content: Value, _domain: &str) -> Result<BTreeSet<Subdomain>> {
        if let Some(items) = content["items"].as_array() {
            let filter = |item: &Value| Some(item["hostname"].as_str()?.to_string());

            return Ok(items.iter().filter_map(filter).collect());
        }

        Err(JSONExtract.into())
    }
}
```
