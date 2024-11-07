use super::core::Subdomain;
use itertools::Itertools;
use reqwest::Url;
use std::collections::BTreeSet;

/// Data type to store search URL query param
/// for search engines like `Google`, `Yahoo`, `Bing`, etc.
#[derive(Debug, Clone)]
pub struct SearchQueryParam(String);

impl From<&str> for SearchQueryParam {
    /// Create query param from static str
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::types::query::SearchQueryParam;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let param = SearchQueryParam::from("q");
    ///
    ///     // do something with param
    /// }
    /// ```
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for SearchQueryParam {
    /// Clones inner value and returns it as a [`String`]
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::types::query::SearchQueryParam;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let param = SearchQueryParam::from("q");
    ///
    ///     let as_string = param.to_string();
    ///
    ///     // do something with string query param
    /// }
    /// ```
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

impl SearchQueryParam {
    /// Get fully [`SearchQuery`] object from [`SearchQueryParam`]
    /// configured by the given `domain` and `prefix` params
    ///
    /// # Example
    ///
    /// ```
    /// use subscan::types::query::SearchQueryParam;
    ///
    /// let domain = "foo.com";
    /// let prefix = "site:";
    ///
    /// let param = SearchQueryParam::from("q");
    /// let mut search_query = param.to_search_query(domain, prefix);
    ///
    /// assert_eq!(search_query.domain, domain);
    /// assert_eq!(search_query.prefix, prefix);
    /// assert_eq!(search_query.as_search_str(), "site:foo.com".to_string());
    /// ```
    pub fn to_search_query(&self, domain: &str, prefix: &str) -> SearchQuery {
        SearchQuery::new(self.clone(), prefix, domain)
    }
}

/// To store and manage full search query string for
/// search engines. Uses while enumerating subdomains.
/// End of the day, the query looks like
/// `site:foo.com -www -api -app`
#[derive(Debug)]
pub struct SearchQuery {
    /// URL query param while used the full query
    pub param: SearchQueryParam,
    /// If available query prefix like google dorks
    /// `site:`, `inurl:`, `intext:`, etc.
    pub prefix: String,
    /// Target domain to be included in query
    pub domain: String,
    /// Query state, already founded subdomains listed
    /// in this state and creates a new query by using these
    /// subdomains. These values adds end of the query with
    /// dash (`-`) prefix, so search engines does not list
    /// these subdomains anymore
    pub state: BTreeSet<String>,
}

impl SearchQuery {
    /// Create a new [`SearchQuery`] instance with `prefix` and `domain` values
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::types::query::{SearchQuery, SearchQueryParam};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let param = SearchQueryParam::from("s");
    ///     let query = SearchQuery::new(param, "site:", "foo.com");
    ///
    ///     // do something with query
    /// }
    /// ```
    pub fn new(param: SearchQueryParam, prefix: &str, domain: &str) -> Self {
        Self {
            param,
            prefix: prefix.to_string(),
            domain: domain.to_string(),
            state: BTreeSet::new(),
        }
    }

    /// Update query state with a single [`Subdomain`] value
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::query::{SearchQuery, SearchQueryParam};
    /// use subscan::types::core::Subdomain;
    ///
    /// let param = SearchQueryParam::from("s");
    /// let mut query = SearchQuery::new(param, "site:", "foo.com");
    ///
    /// assert_eq!(query.as_search_str(), String::from("site:foo.com"));
    /// assert_eq!(query.update(Subdomain::from("api.foo.com")), true);
    /// assert_eq!(query.as_search_str(), String::from("site:foo.com -api"));
    /// assert_eq!(query.update(Subdomain::from("api.foo.com")), false);
    /// assert_eq!(query.update(Subdomain::from("bar")), false);
    /// ```
    pub fn update(&mut self, sub: Subdomain) -> bool {
        let formatted = format!(".{}", self.domain);

        if let Some(stripped) = sub.strip_suffix(&formatted) {
            self.state.insert(format!("-{}", stripped))
        } else {
            false
        }
    }

    /// Update query state with many [`Subdomain`] value
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeSet;
    /// use subscan::types::query::{SearchQuery, SearchQueryParam};
    /// use subscan::types::core::Subdomain;
    ///
    /// let param = SearchQueryParam::from("s");
    ///
    /// let news = BTreeSet::from_iter([
    ///     Subdomain::from("api.foo.com"),
    ///     Subdomain::from("app.foo.com"),
    /// ]);
    ///
    /// let mut query = SearchQuery::new(param, "site:", "foo.com");
    ///
    /// assert_eq!(query.as_search_str(), String::from("site:foo.com"));
    /// assert_eq!(query.update_many(news.clone()), true);
    /// assert_eq!(query.as_search_str(), String::from("site:foo.com -api -app"));
    /// assert_eq!(query.update_many(news), false);
    /// ```
    pub fn update_many(&mut self, subs: BTreeSet<Subdomain>) -> bool {
        let filter_stmt = |item: &&String| self.update(item.to_string());

        subs.iter().filter(filter_stmt).count() > 0
    }

    /// Returns fully query as a searchable on search engine
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::query::{SearchQuery, SearchQueryParam};
    /// use subscan::types::core::Subdomain;
    ///
    /// let param = SearchQueryParam::from("s");
    /// let mut query = SearchQuery::new(param, "site:", "foo.com");
    ///
    /// assert_eq!(query.as_search_str(), "site:foo.com");
    ///
    /// query.update("bar.foo.com".into());
    ///
    /// assert_eq!(query.as_search_str(), "site:foo.com -bar")
    /// ````
    pub fn as_search_str(&mut self) -> String {
        let suffix = self.state.iter().join(" ");

        if suffix.is_empty() {
            format!("{}{}", self.prefix, self.domain)
        } else {
            format!("{}{} {}", self.prefix, self.domain, suffix)
        }
    }

    /// According to given `base_url` returns searchable
    /// [`reqwest::Url`] that includes fully search query
    /// with current query state. Also extra URL query
    /// parameters configurable with `extra_params` parameter
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::query::{SearchQuery, SearchQueryParam};
    /// use reqwest::Url;
    ///
    /// let param = SearchQueryParam::from("s");
    /// let base_url = Url::parse("https://bar.com").unwrap();
    /// let extra_params = &[("bar".to_string(), "baz".to_string())];
    ///
    /// let expected_url = Url::parse("https://bar.com/?bar=baz&s=site%3Afoo.com").unwrap();
    ///
    /// let mut query = SearchQuery::new(param, "site:", "foo.com");
    ///
    /// assert_eq!(query.as_url(base_url, extra_params), expected_url);
    /// ````
    pub fn as_url(&mut self, base_url: Url, extra_params: &[(String, String)]) -> Url {
        let query_param = &[(self.param.to_string(), self.as_search_str())];
        let params = [extra_params, query_param].concat();

        Url::parse_with_params(base_url.as_ref(), params).expect("URL parse error!")
    }
}
