use crate::{
    enums::{
        dispatchers::{RequesterDispatcher, SubdomainExtractorDispatcher},
        module::SubscanModuleStatus::Finished,
    },
    interfaces::{
        extractor::SubdomainExtractorInterface, module::SubscanModuleInterface,
        requester::RequesterInterface,
    },
    types::{
        core::SubscanModuleCoreComponents, query::SearchQueryParam, result::SubscanModuleResult,
    },
};
use async_trait::async_trait;
use reqwest::Url;
use tokio::sync::Mutex;

/// Generic search engine module that enumerates subdomain addresses by using dorking technique
///
/// Most search engines has advanced search techniques to filter results, these techniques are
/// called dorking. To be example `inurl:foo` dork filters only URLs that includes `foo`
/// keyword, there is a so many dorking keywords like `intext`, `filetype`, `intitle`, `ext`, etc.
/// You can guess what each dork does from its name
///
/// To enumerate already indexed subdomains by search engine `site:foo.com` dork can be used.
/// Site dork filters only `foo.com` domains and all the indexed subdomains are listed
/// in the results
///
/// Also you can exclude any search result into results with using dash (`-`) character. To be
/// example `site:foo.com -www` dork gets `foo.com` related domain addresses but
/// excludes `www.foo.com`
///
/// According to above information, we can concat this two technique. If we search subdomains by
/// using `site:foo.com` dork and then in the next search if we exclude all previous search results
/// we can get new subdomain addresses. After then the only thing to do just repeat this logic as
/// long as there are new subdomains
///
/// To understand better you can follow up below table
///
/// | Search | Dork                          | Results      | URL                                                   |
/// |:------:|:-----------------------------:|:------------:|:-----------------------------------------------------:|
/// | 1      | `site:foo.com`                | `www`, `api` | <https://foo.com?q=site:foo.com>                      |
/// | 2      | `site:foo.com -www -api`      | `app`        | <https://foo.com?q=site:foo.com%20-www%20-api>        |
/// | 3      | `site:foo.com -www -api -app` |              | <https://foo.com?q=site:foo.com%20-www%20-api%20-app> |
pub struct GenericSearchEngineModule {
    /// Module name
    pub name: String,
    /// Target search URL to be visited while enumerate subdomains
    pub url: Url,
    /// Search engine search query parameter
    pub param: SearchQueryParam,
    /// Core components
    pub components: SubscanModuleCoreComponents,
}

#[async_trait]
impl SubscanModuleInterface for GenericSearchEngineModule {
    async fn name(&self) -> &str {
        &self.name
    }

    async fn requester(&self) -> Option<&Mutex<RequesterDispatcher>> {
        Some(&self.components.requester)
    }

    async fn extractor(&self) -> Option<&SubdomainExtractorDispatcher> {
        Some(&self.components.extractor)
    }

    async fn run(&mut self, domain: &str) -> SubscanModuleResult {
        let mut result: SubscanModuleResult = self.name().await.into();

        let requester = &*self.components.requester.lock().await;
        let extractor = &self.components.extractor;

        let extra_params = [("num".to_string(), 100.to_string())];

        let mut query = self.param.to_search_query(domain, "site:");

        loop {
            let url = query.as_url(self.url.clone(), &extra_params);
            let response = requester.get_content(url).await;
            let news = extractor.extract(response, domain).await;

            result.extend(news.clone());

            if !query.update_many(news.clone()) {
                break;
            }
        }

        result.with_status(Finished).await
    }
}
