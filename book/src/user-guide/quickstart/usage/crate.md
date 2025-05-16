# Crate Usage

You can easily add `Subscan` to your code and use its results in your application. Since `Subscan` works asynchronously, you need to use it in `async` code blocks. We recommend using [Tokio](https://tokio.rs/) as the async runtime

This chapter provides step-by-step guidance on how to integrate `Subscan` into your code. For more detailed usage and additional code examples, visit the project's [docs.rs](https://docs.rs/subscan/latest/subscan/) page or check the [examples/](https://github.com/eredotpkfr/subscan/tree/main/examples) folder in the repository

1. Add `subscan` crate into your project dependencies

   ```bash
   ~$ cargo add subscan
   ```

2. Create a new instance and start to use it

   ```rust,ignore
    #[tokio::main]
    async fn main() {
        // set module conccurrency to 1
        // set HTTP timeout to 120
        let config = SubscanConfig {
            concurrency: 1,
            filter: CacheFilter::FilterByName(ModuleNameFilter {
                modules: vec!["alienvault".into()],
                skips: vec![],
            }),
            requester: RequesterConfig {
                timeout: Duration::from_secs(120),
                ..Default::default()
            },
            ..Default::default()
        };

        let subscan = Subscan::from(config);
        let result = subscan.scan("domain.com").await;

        for item in result.items {
            // do something with item
        }
    }
   ```
