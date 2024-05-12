use async_trait::async_trait;

#[async_trait(?Send)]
pub trait SubscanModuleInterface {
    async fn name(&self) -> String;
    async fn run(&mut self, domain: String);
}
