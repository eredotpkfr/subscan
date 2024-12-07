# Subscan Module

`SubscanModule` components are the core components that can be executed by `Subscan`. Each module capable of performing subdomain discovery is named a `SubscanModule`, and when the [`subscan scan`](/user-guide/commands/scan.md) command is run, these modules are read from an in-memory cache and executed asynchronously. This architecture makes `Subscan` extensible and modular

A `SubscanModule` may contain various components such as `Requester` and `Extractor`. Most modules implemented in `Subscan` use these components. You can list the implemented modules with their details using the [`subscan module list`](/user-guide/commands/module.md#list) command. If you'd like to view the in-memory cache, you can check the [CacheManager](https://docs.rs/subscan/latest/subscan/cache/struct.CacheManager.html) struct, which is another component designed for operations like filtering the cache or accessing a specific module

## Create Your Own Module

Each `SubscanModule` component should be implemented following the interface below. For a better understanding, you can explore the [docs.rs](https://docs.rs/subscan/latest/subscan/interfaces/module/index.html) page and review the crates listed below

- [`async_trait`](https://github.com/dtolnay/async-trait)
- [`enum_dispatch`](https://gitlab.com/antonok/enum_dispatch)

```rust,ignore
#[async_trait]
#[enum_dispatch]
pub trait SubscanModuleInterface: Sync + Send {
    // Returns module name, name should clarify what does module
    async fn name(&self) -> &str;
    // Loads `.env` file and fetches module environment variables with variable name.
    // If system environment variable set with same name, `.env` file will be overrode
    async fn envs(&self) -> SubscanModuleEnvs {
        self.name().await.into()
    }
    // Returns module requester address as a mutable reference if available
    async fn requester(&self) -> Option<&Mutex<RequesterDispatcher>>;
    // Returns module extractor reference if available
    async fn extractor(&self) -> Option<&SubdomainExtractorDispatcher>;
    // Just like a `main` method, when the module run this `run` method will be called.
    // So this method should do everything
    async fn run(&mut self, domain: &str) -> Result<SubscanModuleResult>;
}
```

Below is a simple example of a custom module. For more examples, you can check the [examples/](https://github.com/eredotpkfr/subscan/tree/main/examples) folder on the project's GitHub page. You can also refer to the source code of predefined requester implementations for a better understanding

```rust,ignore
pub struct CustomModule {
    pub requester: Mutex<RequesterDispatcher>,
    pub extractor: SubdomainExtractorDispatcher,
}

#[async_trait]
impl SubscanModuleInterface for CustomModule {
    async fn name(&self) -> &str {
        &"name"
    }

    async fn requester(&self) -> Option<&Mutex<RequesterDispatcher>> {
        Some(&self.requester)
    }

    async fn extractor(&self) -> Option<&SubdomainExtractorDispatcher> {
        Some(&self.extractor)
    }

    async fn run(&mut self, _domain: &str) -> Result<SubscanModuleResult> {
        let mut result: SubscanModuleResult = self.name().await.into();

        let subdomains = BTreeSet::from_iter([
            Subdomain::from("bar.foo.com"),
            Subdomain::from("baz.foo.com"),
        ]);

        result.extend(subdomains);

        Ok(result.with_finished().await)
    }
}
```
