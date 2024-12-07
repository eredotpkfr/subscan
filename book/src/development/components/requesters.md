# Requesters

Requesters are components designed to manage HTTP requests through a unified interface. Each requester offers unique features, and when HTTP requests are needed during subdomain discovery, the appropriate requester can be selected based on the requirements.

`Subscan` includes predefined requesters like

- [ChromeBrowser](https://docs.rs/subscan/latest/subscan/requesters/chrome/struct.ChromeBrowser.html)

    This requester component runs a `Chrome` process in the background, allowing HTTP requests through the browser. It has advantages such as rendering `JavaScript`, bypassing anti-bot systems, etc.

- [HTTPClient](https://docs.rs/subscan/latest/subscan/requesters/client/struct.HTTPClient.html)

    The HTTP client requester component is identical to the standard HTTP client, using the [reqwest](https://docs.rs/reqwest/latest/reqwest/) crate's client as its implementation

## Create Your Custom Requester

Each requester component should be implemented following the interface below. For a better understanding, you can explore the [docs.rs](https://docs.rs/subscan/latest/subscan/interfaces/requester/index.html) page and review the crates listed below

- [`async_trait`](https://github.com/dtolnay/async-trait)
- [`enum_dispatch`](https://gitlab.com/antonok/enum_dispatch)

```rust,ignore
#[async_trait]
#[enum_dispatch]
pub trait RequesterInterface: Sync + Send {
    // Returns requester configurations as a RequesterConfig object
    async fn config(&mut self) -> &mut RequesterConfig;
    // Configure current requester object by using new RequesterConfig object
    async fn configure(&mut self, config: RequesterConfig);
    // HTTP GET method implementation to fetch HTML content from given source URL
    async fn get_content(&self, url: Url) -> Result<Content>;
}
```

Below is a simple example of a custom requester. For more examples, you can check the [examples/](https://github.com/eredotpkfr/subscan/tree/main/examples) folder on the project's GitHub page. You can also refer to the source code of predefined requester implementations for a better understanding

```rust,ignore
pub struct CustomRequester {
    config: RequesterConfig,
}

#[async_trait]
impl RequesterInterface for CustomRequester {
    async fn config(&mut self) -> &mut RequesterConfig {
        &mut self.config
    }

    async fn configure(&mut self, config: RequesterConfig) {
        self.config = config;
    }

    async fn get_content(&self, _url: Url) -> Result<Content> {
        Ok(Content::Empty)
    }
}
```
