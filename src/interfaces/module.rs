use crate::interfaces::requester::RequesterInterface;
use async_trait::async_trait;

#[async_trait(?Send)]
pub trait SubscanModuleInterface {
    fn name(&self) -> String;
    async fn run(&mut self, domain: String, requester: Box<dyn RequesterInterface>);
}
