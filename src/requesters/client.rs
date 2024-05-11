use crate::interfaces::requester::RequesterInterface;
use async_trait::async_trait;
use reqwest::{Client, Request, RequestBuilder};
use reqwest::{Method, Url};

#[async_trait(?Send)]
impl RequesterInterface for Client {
    fn request(&self, method: Method, url: Url) -> RequestBuilder {
        self.request(method, url)
    }

    async fn get(&self, request: Request) -> String {
        self.execute(request)
            .await
            .unwrap()
            .text()
            .await
            .unwrap_or_default()
    }
}
