mod common;

#[cfg(test)]
mod searchengine {
    use super::common::constants::{TEST_BAR_SUBDOMAIN, TEST_BAZ_SUBDOMAIN, TEST_DOMAIN};
    use reqwest::Url;
    use subscan::{
        cache::requesters,
        enums::RequesterType,
        interfaces::module::SubscanModuleInterface,
        modules::engines::{bing, duckduckgo, google, yahoo},
    };

    #[tokio::test]
    #[stubr::mock("module/engines/google.json")]
    async fn google_run_test() {
        let mut google = google::Google::new();

        google.url = Url::parse(stubr.path("/search").as_str()).unwrap();

        let result = google.run(TEST_DOMAIN.to_string()).await;

        assert_eq!(google.name().await, "Google");
        assert_eq!(result, [TEST_BAR_SUBDOMAIN.to_string()].into());
    }

    #[tokio::test]
    #[stubr::mock("module/engines/yahoo.json")]
    async fn yahoo_run_test() {
        let mut yahoo = yahoo::Yahoo::new();

        yahoo.url = Url::parse(stubr.path("/search").as_str()).unwrap();

        let result = yahoo.run(TEST_DOMAIN.to_string()).await;

        assert_eq!(yahoo.name().await, "Yahoo");
        assert_eq!(result, [TEST_BAZ_SUBDOMAIN.to_string()].into());
    }

    #[tokio::test]
    #[stubr::mock("module/engines/bing.json")]
    async fn bin_run_test() {
        let mut bing = bing::Bing::new();

        bing.url = Url::parse(stubr.path("/search").as_str()).unwrap();

        let result = bing.run(TEST_DOMAIN.to_string()).await;

        assert_eq!(bing.name().await, "Bing");
        assert_eq!(result, [TEST_BAR_SUBDOMAIN.to_string()].into());
    }

    #[tokio::test]
    #[stubr::mock("module/engines/duckduckgo.json")]
    async fn duckduckgo_run_test() {
        let mut duckduckgo = duckduckgo::DuckDuckGo::new();

        duckduckgo.requester = requesters::get_by_type(&RequesterType::HTTPClient);
        duckduckgo.url = Url::parse(stubr.uri().as_str()).unwrap();

        let result = duckduckgo.run(TEST_DOMAIN.to_string()).await;

        assert_eq!(duckduckgo.name().await, "DuckDuckGo");
        assert_eq!(result, [TEST_BAR_SUBDOMAIN.to_string()].into());
    }
}
