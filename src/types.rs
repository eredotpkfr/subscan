pub mod core {
    pub type Subdomain = String;
}

pub mod config {
    use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
    use std::time::Duration;

    #[derive(Debug, Clone)]
    pub struct RequesterConfig {
        pub http_headers: HeaderMap,
        pub http_timeout: Duration,
        pub http_proxy: Option<String>,
    }

    impl RequesterConfig {
        pub fn new() -> Self {
            Self {
                http_headers: HeaderMap::new(),
                http_timeout: Duration::from_secs(10),
                http_proxy: None,
            }
        }

        pub fn add_header(&mut self, name: HeaderName, value: HeaderValue) {
            self.http_headers.insert(name, value);
        }
    }
}

pub mod query {
    use reqwest::Url;
    use std::collections::BTreeSet;

    #[derive(Debug, Clone)]
    pub struct QueryParam(pub String);

    impl QueryParam {
        pub fn from(param: &str) -> Self {
            QueryParam(param.to_string())
        }

        pub fn as_string(&self) -> String {
            self.0.clone()
        }

        pub fn to_search_query(&self, domain: String, prefix: String) -> SearchQuery {
            SearchQuery::new(self.clone(), prefix, domain)
        }
    }

    #[derive(Debug)]
    pub struct SearchQuery {
        pub param: QueryParam,
        pub prefix: String,
        pub domain: String,
        pub state: BTreeSet<String>,
    }

    impl SearchQuery {
        fn new(param: QueryParam, prefix: String, domain: String) -> Self {
            Self {
                param: param,
                prefix: prefix,
                domain: domain,
                state: BTreeSet::new(),
            }
        }

        pub fn update(&mut self, sub: super::core::Subdomain) -> bool {
            let formatted = format!(".{}", self.domain);

            if let Some(stripped) = sub.strip_suffix(&formatted) {
                self.state.insert(format!("-{}", stripped))
            } else {
                false
            }
        }

        pub fn update_many(&mut self, subs: BTreeSet<super::core::Subdomain>) -> bool {
            subs.into_iter()
                .map(|item| self.update(item))
                .collect::<Vec<bool>>()
                .into_iter()
                .any(|item| item == true)
        }

        pub fn as_search_str(&mut self) -> String {
            let asvec = Vec::from_iter(self.state.clone());
            let long_prefix = format!("{}{}", self.prefix, self.domain);
            let formatted = format!("{} {}", long_prefix, asvec.join(" "));

            formatted.trim().to_string()
        }

        pub fn as_url(&mut self, base_url: Url, extra_params: &[(String, String)]) -> Url {
            let params = [
                extra_params,
                &[(self.param.as_string(), self.as_search_str())],
            ]
            .concat();

            Url::parse_with_params(&base_url.to_string(), params).expect("URL parse error!")
        }
    }
}
