use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

/// Generic Subscan module trait definition each module
/// that will be implemented in the future
/// must conform to this interface
///
/// # Examples
///
/// ```
/// use subscan::interfaces::module::SubscanModuleInterface;
/// use async_trait::async_trait;
///
/// pub struct FooModule {}
///
/// #[async_trait(?Send)]
/// impl SubscanModuleInterface for FooModule {
///     async fn name(&self) -> String {
///         String::from("Foo")
///     }
///     async fn run(&mut self, domain: String) {
///         // do something in `run` method
///     }
/// }
///
/// #[tokio::main]
/// async fn main() {
///     let foo = FooModule {};
///
///     assert_eq!(foo.name().await, "Foo");
/// }
/// ```
#[async_trait(?Send)]
#[enum_dispatch]
pub trait SubscanModuleInterface: Sync + Send {
    /// Returns module name, name should clarify what does module
    async fn name(&self) -> String;
    /// Just like a `main` method, when the module
    /// calls this `run` method will be called, so this method
    /// should does everything
    async fn run(&mut self, domain: String);
}
