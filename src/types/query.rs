use super::core::Subdomain;
use reqwest::Url;
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub struct SearchQueryParam(pub String);

impl SearchQueryParam {
    pub fn from(param: &str) -> Self {
        Self(param.to_string())
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
    pub param: SearchQueryParam,
    pub prefix: String,
    pub domain: String,
    pub state: BTreeSet<String>,
}

impl SearchQuery {
    fn new(param: SearchQueryParam, prefix: String, domain: String) -> Self {
        Self {
            param,
            prefix,
            domain,
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

    pub fn as_url(&mut self, base_url: Url, extra_params: &[(String, String)]) -> Url {
        let params = [
            extra_params,
            &[(self.param.as_string(), self.as_search_str())],
        ]
        .concat();

        Url::parse_with_params(&base_url.to_string(), params).expect("URL parse error!")
    }
}
