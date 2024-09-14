use super::core::Subdomain;
use reqwest::Url;
use std::collections::BTreeSet;

/// Data type to store search URL query param
/// for search engines like `Google`, `Yahoo`, `Bing`, etc.
#[derive(Debug, Clone)]
pub struct SearchQueryParam(pub String);

impl SearchQueryParam {
    /// Create query param from static str
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::types::query::SearchQueryParam;
    ///
    /// let param = SearchQueryParam::from("q");
    ///
    /// // do something with param
    /// ```
    pub fn from(param: &str) -> Self {
        Self(param.to_string())
    }

    /// Clones inner value and returns it as a [`String`]
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::types::query::SearchQueryParam;
    ///
    /// let param = SearchQueryParam::from("q");
    ///
    /// let as_string = param.as_string();
    /// ```
    pub fn as_string(&self) -> String {
        self.0.clone()
    }

    /// Get fully [`SearchQuery`] object from [`SearchQueryParam`]
    /// configured by the given `domain` and `prefix` params
    ///
    /// # Example
    ///
    /// ```
    /// use subscan::types::query::SearchQueryParam;
    ///
    /// let (domain, prefix) = (
    ///     String::from("foo.com"),
    ///     String::from("site:"),
    /// );
    ///
    /// let param = SearchQueryParam::from("q");
    /// let mut search_query = param.to_search_query(
    ///     domain.clone(),
    ///     prefix.clone()
    /// );
    ///
    /// assert_eq!(search_query.domain, domain);
    /// assert_eq!(search_query.prefix, prefix);
    /// assert_eq!(search_query.as_search_str(), "site:foo.com".to_string());
    /// ```
    pub fn to_search_query(&self, domain: String, prefix: String) -> SearchQuery {
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
    /// let param = SearchQueryParam::from("s");
    /// let domain = String::from("foo.com");
    /// let prefix = String::from("site:");
    ///
    /// let query = SearchQuery::new(param, prefix, domain);
    ///
    /// // do something with query
    /// ```
    pub fn new(param: SearchQueryParam, prefix: String, domain: String) -> Self {
        Self {
            param,
            prefix,
            domain,
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
    /// let domain = String::from("foo.com");
    /// let prefix = String::from("site:");
    ///
    /// let mut query = SearchQuery::new(param, prefix, domain);
    ///
    /// assert_eq!(query.as_search_str(), String::from("site:foo.com"));
    /// assert_eq!(query.update(Subdomain::from("api.foo.com")), true);
    /// assert_eq!(query.as_search_str(), String::from("site:foo.com -api"));
    /// assert_eq!(query.update(Subdomain::from("api.foo.com")), false);
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
    /// let domain = String::from("foo.com");
    /// let prefix = String::from("site:");
    ///
    /// let news = BTreeSet::from_iter([
    ///     Subdomain::from("api.foo.com"),
    ///     Subdomain::from("app.foo.com"),
    /// ]);
    ///
    /// let mut query = SearchQuery::new(param, prefix, domain);
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
    /// let domain = String::from("foo.com");
    /// let prefix = String::from("site:");
    ///
    /// let mut query = SearchQuery::new(param, prefix, domain);
    ///
    /// assert_eq!(query.as_search_str(), String::from("site:foo.com"));
    /// ````
    pub fn as_search_str(&mut self) -> String {
        let asvec = Vec::from_iter(self.state.clone());
        let long_prefix = format!("{}{}", self.prefix, self.domain);
        let formatted = format!("{} {}", long_prefix, asvec.join(" "));

        formatted.trim().to_string()
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
    /// use subscan::types::core::Subdomain;
    /// use reqwest::Url;
    ///
    /// let param = SearchQueryParam::from("s");
    /// let domain = String::from("foo.com");
    /// let prefix = String::from("site:");
    ///
    /// let base_url = Url::parse("https://bar.com").unwrap();
    /// let expected_url = Url::parse("https://bar.com/?s=site%3Afoo.com").unwrap();
    ///
    /// let mut query = SearchQuery::new(param, prefix, domain);
    ///
    /// assert_eq!(query.as_url(base_url, &[]), expected_url);
    /// ````
    pub fn as_url(&mut self, base_url: Url, extra_params: &[(String, String)]) -> Url {
        let params = [
            extra_params,
            &[(self.param.as_string(), self.as_search_str())],
        ]
        .concat();

        Url::parse_with_params(&base_url.to_string(), params).expect("URL parse error!")
    }
}
