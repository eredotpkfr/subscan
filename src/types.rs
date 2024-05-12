pub mod core {
    use std::collections::BTreeSet;

    pub type Subdomain = String;

    #[derive(Debug)]
    pub struct QueryParam(pub String);

    impl QueryParam {
        pub fn from(param: &str) -> Self {
            QueryParam(param.to_string())
        }

        pub fn as_string(&self) -> String {
            self.0.as_str().to_string()
        }

        pub fn as_search_query(&self, domain: String, prefix: String) -> SearchQuery {
            SearchQuery::new(QueryParam(self.as_string()), prefix, domain)
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

        pub fn update(&mut self, sub: Subdomain) -> bool {
            let formatted = format!(".{}", self.domain);

            if let Some(stripped) = sub.strip_suffix(&formatted) {
                self.state.insert(format!("-{}", stripped))
            } else {
                false
            }
        }

        pub fn update_many(&mut self, subs: BTreeSet<Subdomain>) -> bool {
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
    }
}
