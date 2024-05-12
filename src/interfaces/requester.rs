use async_trait::async_trait;

use reqwest::{Method, Url};
use reqwest::{Request, RequestBuilder};

#[async_trait(?Send)]
pub trait RequesterInterface {
    async fn request(&self, method: Method, url: Url) -> RequestBuilder;
    async fn get(&self, request: Request) -> Option<String>;
}
