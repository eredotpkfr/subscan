use crate::{cli::Cli, types::env::Credentials};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, USER_AGENT};
use std::{collections::HashMap, time::Duration};

/// Default HTTP timeout as a [`Duration`]
pub const DEFAULT_HTTP_TIMEOUT: Duration = Duration::from_secs(10);

/// Type definition for store [`RequesterInterface`](crate::interfaces::requester::RequesterInterface)
/// configurations in a struct. Also it has helpful
/// methods to manage configs
#[derive(Debug, Clone, PartialEq)]
pub struct RequesterConfig {
    /// Stores header values for HTTP requests as a [`HeaderMap`]
    pub headers: HeaderMap,
    /// HTTP request timeout value as a secs (default: `10`)
    pub timeout: Duration,
    /// Proxy server for HTTP requests
    pub proxy: Option<String>,
    /// Basic HTTP authentication credentials
    pub credentials: Credentials,
}

impl Default for RequesterConfig {
    fn default() -> Self {
        Self {
            headers: HeaderMap::new(),
            timeout: DEFAULT_HTTP_TIMEOUT,
            proxy: None,
            credentials: Credentials::default(),
        }
    }
}

impl From<&Cli> for RequesterConfig {
    /// Build [`RequesterConfig`] object from given
    /// [`Cli`] object
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    /// use reqwest::header::USER_AGENT;
    /// use subscan::cli::Cli;
    /// use subscan::types::config::RequesterConfig;
    ///
    /// let cli = Cli {
    ///     domain: String::from("foo.com"),
    ///     timeout: 120,
    ///     user_agent: String::from("bar"),
    ///     proxy: None,
    /// };
    ///
    /// let config = RequesterConfig::from(&cli);
    /// let user_agent = config.headers.get(USER_AGENT).unwrap();
    ///
    /// assert_eq!(config.timeout.as_secs(), cli.timeout);
    /// assert_eq!(config.proxy, cli.proxy);
    /// assert_eq!(user_agent.to_str().unwrap(), cli.user_agent);
    /// ```
    fn from(cli: &Cli) -> Self {
        Self {
            headers: HeaderMap::from_iter([(
                USER_AGENT,
                HeaderValue::from_str(&cli.user_agent).unwrap(),
            )]),
            timeout: Duration::from_secs(cli.timeout),
            proxy: cli.proxy.clone(),
            ..Default::default()
        }
    }
}

impl RequesterConfig {
    /// Converts [`HeaderMap`] headers to [`HashMap`] mappings and returns them
    ///
    /// # Examples
    ///
    /// ```
    /// use std::time::Duration;
    /// use reqwest::header::{USER_AGENT, HeaderValue, HeaderMap};
    /// use subscan::types::config::RequesterConfig;
    ///
    /// let user_agent = HeaderValue::from_str(&String::from("foo")).unwrap();
    ///
    /// let config = RequesterConfig {
    ///     headers: HeaderMap::from_iter([
    ///         (USER_AGENT, user_agent)
    ///     ]),
    ///     ..Default::default()
    /// };
    ///
    /// let headers = config.headers_as_hashmap();
    ///
    /// assert_eq!(headers.get("user-agent").unwrap(), &"foo");
    /// ```
    pub fn headers_as_hashmap(&self) -> HashMap<&str, &str> {
        let cast_to_str: for<'a, 'b> fn((&'a HeaderName, &'b HeaderValue)) -> (&'a str, &'b str) =
            |item| (item.0.as_str(), item.1.to_str().unwrap());

        HashMap::from_iter(self.headers.iter().map(cast_to_str))
    }

    /// Append a new default HTTP header in [`RequesterConfig`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::config::RequesterConfig;
    /// use reqwest::header::{USER_AGENT, HeaderValue};
    ///
    /// let mut config = RequesterConfig::default();
    /// let user_agent = HeaderValue::from_str("foo").expect("Value error!");
    ///
    /// assert_eq!(config.headers.len(), 0);
    ///
    /// config.add_header(USER_AGENT, user_agent);
    ///
    /// assert!(config.headers.contains_key(USER_AGENT));
    ///
    /// assert_eq!(config.headers.len(), 1);
    /// assert_eq!(config.headers.get(USER_AGENT).unwrap(), "foo");
    /// ```
    pub fn add_header(&mut self, name: HeaderName, value: HeaderValue) {
        self.headers.insert(name, value);
    }

    /// Set basic HTTP authentication credentials for requester
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::config::RequesterConfig;
    /// use subscan::types::env::{Env, Credentials};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut config = RequesterConfig::default();
    ///
    ///     let credentials = Credentials {
    ///         username: Env { name: "USERNAME".into(), value: Some("foo".to_string())},
    ///         password: Env { name: "PASSWORD".into(), value: Some("bar".to_string())},
    ///     };
    ///
    ///     assert!(!config.credentials.is_ok());
    ///     assert!(config.credentials.username.value.is_none());
    ///     assert!(config.credentials.password.value.is_none());
    ///
    ///     config.set_credentials(credentials);
    ///
    ///     assert!(config.credentials.is_ok());
    ///
    ///     assert_eq!(config.credentials.username.name, "USERNAME");
    ///     assert_eq!(config.credentials.password.name, "PASSWORD");
    ///     assert_eq!(config.credentials.username.value, Some("foo".to_string()));
    ///     assert_eq!(config.credentials.password.value, Some("bar".to_string()));
    /// }
    /// ```
    pub fn set_credentials(&mut self, credentials: Credentials) {
        self.credentials = credentials;
    }
}
