use crate::interfaces::extractor::SubdomainExtractorInterface;
use crate::interfaces::module::SubscanModuleInterface;
use crate::interfaces::requester::RequesterInterface;
use async_trait::async_trait;
use reqwest::{Method, Request, Url};

const USER_AGENT: &str = "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36";

pub struct GenericSearchEngineModule {
    pub name: String,
    pub url: Url,
    pub query_param: String,
    pub extractor: Box<dyn SubdomainExtractorInterface>,
}

impl GenericSearchEngineModule {
    pub async fn get_start_query(&self, domain: String) -> String {
        format!("site:{}", domain).to_string()
    }
    pub async fn build_request(
        &self,
        requester: &Box<dyn RequesterInterface>,
        query: String,
    ) -> Request {
        requester
            .request(Method::GET, self.url.clone())
            .header("User-Agent", USER_AGENT)
            .query(&[(self.query_param.clone(), query)])
            .build()
            .unwrap()
    }
}

#[async_trait(?Send)]
impl SubscanModuleInterface for GenericSearchEngineModule {
    fn name(&self) -> String {
        self.name.clone()
    }

    async fn run(&self, domain: String, requester: Box<dyn RequesterInterface>) {
        let query = self.get_start_query(domain.clone()).await;

        loop {
            let request = self.build_request(&requester, query.clone()).await;
            let response = requester.get(request).await;

            let results = self.extractor.extract(response, domain.clone());
            println!("{:#?}", results);

            break;
        }
    }
}
