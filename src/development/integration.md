# Integrate Your Module Step by Step

This chapter provides a step-by-step guide on how to convert your custom subdomain discovery module into a `SubscanModule` component and integrate it with `Subscan`

Follow the steps below to integrate your module with `Subscan`

## 1. Create Your Custom Module

At first, you need to implement a module that follows the [`SubscanModuleInterface`](https://docs.rs/subscan/latest/subscan/interfaces/module/trait.SubscanModuleInterface.html) so that `Subscan` can run your module. If you're unsure how to implement it, you can follow the [Create Your Own Module](components/module.md#create-your-own-module) title. Also, if your module will use a generic implementation, refer to the [Generic Modules](generics/index.html) chapter

> The file should be created in the [`src/modules`](https://github.com/eredotpkfr/subscan/tree/main/src/modules) directory, where the modules are organized by their functionality: generics are stored in [`generics/`](https://github.com/eredotpkfr/subscan/tree/main/src/modules/generics), integrations in [`integrations/`](https://github.com/eredotpkfr/subscan/tree/main/src/modules/integrations), and search engines in [`engines/`](https://github.com/eredotpkfr/subscan/tree/main/src/modules/engines). Since we are integrating a custom module, we can create our file as `src/modules/example.rs`

Here is an example module that is compatible with the [`SubscanModuleInterface`](https://docs.rs/subscan/latest/subscan/interfaces/module/trait.SubscanModuleInterface.html). Let's integrate it into `Subscan`

```rust,ignore
pub struct ExampleModule {
    pub name: String,
}

#[async_trait]
impl SubscanModuleInterface for ExampleModule {
    async fn name(&self) -> &str {
        &self.name
    }

    async fn requester(&self) -> Option<&Mutex<RequesterDispatcher>> {
        None
    }

    async fn extractor(&self) -> Option<&SubdomainExtractorDispatcher> {
        None
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

## 2. Define Your Module as `SubscanModule`

To define this module as a `SubscanModule`, we need to wrap it with a [`SubscanModuleDispatcher`](https://docs.rs/subscan/latest/subscan/enums/dispatchers/enum.SubscanModuleDispatcher.html), as shown in the implementation of the `SubscanModule` type below

```rust,ignore
// `SubscanModule` type wrapper
pub type SubscanModule = Arc<Mutex<SubscanModuleDispatcher>>;

impl From<SubscanModuleDispatcher> for SubscanModule {
    fn from(module: SubscanModuleDispatcher) -> Self {
        Self::new(Mutex::new(module))
    }
}
```

### 2.1. Add a New Dispatcher Variant

Dispatchers are enumeration structures defined in the [`src/enums/dispatchers.rs`](https://docs.rs/subscan/latest/subscan/enums/dispatchers/index.html) file. Instead of using `Box` for dynamic dispatching when running modules, we can store the module variants within an enum. This allows the compiler to know the types based on the dispatcher during module execution and enables static dispatching. For more detailed technical information, you can refer to the [`enum_dispatch`](https://gitlab.com/antonok/enum_dispatch) crate

Now, let's add a dispatcher variant for our module. If we are using a generic implementation, we don't need to do this, as a variant has already been created for generic implementations, as shown below

```rust,ignore
#[enum_dispatch(SubscanModuleInterface)]
pub enum SubscanModuleDispatcher {
    // Enum variant of generic API integrations. It can be used for all generic API modules
    // at the same time, for this only requirement is the module should be implemented as
    // a GenericIntegrationModule
    GenericIntegrationModule(GenericIntegrationModule),
    // Also another generic variant for search engines, It can be used for all generic search
    // engine modules at the same time. Just modules should be implemented as
    // a GenericSearchEngineModule
    GenericSearchEngineModule(GenericSearchEngineModule),
    // Non-generic `ExampleModule` module variant
    ExampleModule(ExampleModule), // Add this line
}
```

### 2.2. Implement the `dispatcher()` Method for Your Module

After adding the dispatcher variant, we can add a method named `dispatcher(` to our module that will return it as a dispatcher variant

```rust,ignore
impl ExampleModule {
    pub fn dispatcher() -> SubscanModuleDispatcher {
        let example = Self {
            name: "example".into(),
        };

        example.into()
    }
}
```

## 3. Add Your Module to the In-Memory Cache

The only thing left to do is add our module to the in-memory cache as a `SubscanModule` so that the [CacheManager](https://docs.rs/subscan/latest/subscan/cache/struct.CacheManager.html) component can use it. To do this, let's add our module to the in-memory cache called `MODULE_CACHE` in [`cache.rs`](https://docs.rs/subscan/latest/subscan/cache/index.html) file

```rust,ignore
lazy_static! {
    static ref MODULE_CACHE: Vec<SubscanModule> = vec![
        // Search engines
        SubscanModule::from(bing::Bing::dispatcher()),
        SubscanModule::from(duckduckgo::DuckDuckGo::dispatcher()),
        // Integrations
        SubscanModule::from(alienvault::AlienVault::dispatcher()),
        SubscanModule::from(example::ExampleModule::dispatcher()), // Add this line
    ]
}
```

## 4. Run Your Module

```bash
~$ cargo build && target/debug/subscan module run example -d example.com
```

## 5. Write Tests for Your Module

Please write unit tests for your module. You can use the [`tests/`](https://github.com/eredotpkfr/subscan/tree/main/tests) folder as a reference
