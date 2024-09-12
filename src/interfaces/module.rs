use async_trait::async_trait;
use enum_dispatch::enum_dispatch;

#[async_trait(?Send)]
#[enum_dispatch]
pub trait SubscanModuleInterface: Sync + Send {
    async fn name(&self) -> String;
    async fn run(&mut self, domain: String);
}
