# Generic Search Engine Module

The [`GenericSearchEngineModule`](https://docs.rs/subscan/latest/subscan/modules/generics/engine/struct.GenericSearchEngineModule.html) is primarily used for search engine integrations. It performs subdomain discovery by conducting dork searches on search engines and provides a generic implementation for search engines that use the same dork structure. To understand how it works, review the source code on the [docs.rs](https://docs.rs/subscan/latest/subscan/modules/generics/engine/struct.GenericSearchEngineModule.html) page. Additionally, the source code of other module implementations that use this implementation can help guide you in its usage

A search engine module that uses this internally would look like the example below

```rust,ignore
pub const EXAMPLE_MODULE_NAME: &str = "example";
pub const EXAMPLE_SEARCH_URL: &str = "https://www.example.com/search";
pub const EXAMPLE_SEARCH_PARAM: &str = "q";
pub const EXAMPLE_CITE_TAG: &str = "cite";

pub struct ExampleModule {}

impl ExampleModule {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let url = Url::parse(EXAMPLE_SEARCH_URL);

        let extractor: HTMLExtractor = HTMLExtractor::new(EXAMPLE_CITE_TAG.into(), vec![]);
        let requester: RequesterDispatcher = HTTPClient::default().into();

        let generic = GenericSearchEngineModule {
            name: EXAMPLE_MODULE_NAME.into(),
            param: EXAMPLE_SEARCH_PARAM.into(),
            url: url.unwrap(),
            components: SubscanModuleCoreComponents {
                requester: requester.into(),
                extractor: extractor.into(),
            },
        };

        generic.into()
    }
}
```
