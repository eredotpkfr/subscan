use crate::interfaces::requester::RequesterInterface;
use async_trait::async_trait;
use reqwest::{Client, Request, RequestBuilder};
use reqwest::{Method, Url};

#[async_trait(?Send)]
impl RequesterInterface for Client {
    async fn request(&self, method: Method, url: Url) -> RequestBuilder {
        self.request(method, url)
    }

    async fn get(&self, request: Request) -> Option<String> {
        if let Ok(response) = self.execute(request).await {
            if let Ok(content) = response.text().await {
                Some(content)
            } else {
                None
            }
        } else {
            None
        }
    }
}
