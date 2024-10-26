use crate::{
    extractors::{html::HTMLExtractor, json::JSONExtractor, regex::RegexExtractor},
    modules::{
        generics::{engine::GenericSearchEngineModule, integration::GenericIntegrationModule},
        integrations::{
            commoncrawl::CommonCrawl, dnsdumpster::DnsDumpster, github::GitHub, netlas::Netlas,
            waybackarchive::WaybackArchive,
        },
    },
    requesters::{chrome::ChromeBrowser, client::HTTPClient},
    types::env::Credentials,
};
use enum_dispatch::enum_dispatch;
use serde_json::Value;

/// Dispatcher enumeration to decide module types
///
/// It allows to made static type dispatching instead of dynamic dispatch and speed up performance.
/// For more technical details please follow up `enum_dispatch` package
///
/// Each `Subscan` module that compatible with [`SubscanModuleInterface`](crate::interfaces::module::SubscanModuleInterface)
/// must be appeared in this dispatcher as a any enum variant c/o `enum_dispatch`
/// macro creates a sub method for [`SubscanModuleInterface`](crate::interfaces::module::SubscanModuleInterface)
/// methods and matches all of these variants with their methods
///
/// When you call any method from [`SubscanModuleDispatcher`] object this mappings
/// should be implemented otherwise you cannot access these methods like
/// `.name(`, `.requester(`, `.run(`, etc.
#[enum_dispatch(SubscanModuleInterface)]
pub enum SubscanModuleDispatcher {
    /// Enum variant of generic API integrations. It can be used for all generic API modules
    /// at the same time, for this only requirement is the module should be implemented as
    /// a [`GenericIntegrationModule`]
    GenericIntegrationModule(GenericIntegrationModule),
    /// Also another generic variant for search engines, It can be used for all generic search
    /// engine modules at the same time. Just modules should be implemented as
    /// a [`GenericSearchEngineModule`]
    GenericSearchEngineModule(GenericSearchEngineModule),
    /// Non-generic `CommonCrawl` integration variant
    CommonCrawl(CommonCrawl),
    /// Non-generic `DnsDumpster` integration variant
    DnsDumpster(DnsDumpster),
    /// Non-generic `GitHub` integration variant
    GitHub(GitHub),
    /// Non-generic `Netlas` API integration variant
    Netlas(Netlas),
    /// Non-generic `WaybackArchive` integration variant
    WaybackArchive(WaybackArchive),
}

impl SubscanModuleDispatcher {
    /// Returns [`true`] if module implemented as a generic module
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::modules::integrations::github::GitHub;
    /// use subscan::modules::engines::google::Google;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let github = GitHub::dispatcher();
    ///     let google = Google::dispatcher();
    ///
    ///     assert!(!github.is_generic().await);
    ///     assert!(google.is_generic().await);
    /// }
    ///
    ///
    /// ```
    pub async fn is_generic(&self) -> bool {
        matches!(
            self,
            SubscanModuleDispatcher::GenericIntegrationModule(_)
                | SubscanModuleDispatcher::GenericSearchEngineModule(_)
        )
    }
}

/// Dispatcher enumeration to decide extractor types
///
/// It allows to made static type dispatching instead of dynamic dispatch and speed up performance.
/// For more technical details please follow up `enum_dispatch` package
#[enum_dispatch(SubdomainExtractorInterface)]
pub enum SubdomainExtractorDispatcher {
    /// HTML extractor type to extract subdomain addresses from any HTML content.
    /// See the [`HTMLExtractor`] struct definition for examples and technical details
    HTMLExtractor(HTMLExtractor),
    /// Regex extractor type allows to extract subdomain addresses from string content with a
    /// regex pattern by given domain address. See the [`RegexExtractor`] for technical details
    /// and example usages
    RegexExtractor(RegexExtractor),
    /// JSON extractor type can extract subdomains from JSON content. In this type head up point
    /// is to know that created as a wrapper struct to be compatible with
    /// [`SubdomainExtractorInterface`](crate::interfaces::extractor::SubdomainExtractorInterface)
    /// so parser method must be implemented and gave this wrapper struct. See the
    /// [`JSONExtractor`] struct and [`InnerExtractFunc`](crate::types::func::InnerExtractFunc)
    /// type for examples and technical details
    JSONExtractor(JSONExtractor),
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for SubdomainExtractorDispatcher {
    fn to_string(&self) -> String {
        match self {
            SubdomainExtractorDispatcher::HTMLExtractor(_) => "HTMLExtractor".into(),
            SubdomainExtractorDispatcher::RegexExtractor(_) => "RegexExtractor".into(),
            SubdomainExtractorDispatcher::JSONExtractor(_) => "JSONExtractor".into(),
        }
    }
}

/// Dispatcher enumeration to decide requester types
///
/// It allows to made static type dispatching instead of dynamic dispatch and speed up performance.
/// For more technical details please follow up `enum_dispatch` package
#[enum_dispatch(RequesterInterface)]
pub enum RequesterDispatcher {
    /// Chrome browser struct definition as a enum value. On this requester type, Chrome browser
    ///  will run and all HTTP requests made with browser. It has pros according to [`HTTPClient`]
    /// requester like running Js, rendering pages, etc. See the [`ChromeBrowser`] definition to
    /// learn usage
    ChromeBrowser(ChromeBrowser),
    /// Simple HTTP client interface to make requesters, it does not allows to run Js, rendering
    ///  pages or user interface. Just send HTTP requests via [`reqwest`]. See the [`HTTPClient`]
    /// struct definition for examples and technical details
    HTTPClient(HTTPClient),
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for RequesterDispatcher {
    fn to_string(&self) -> String {
        match self {
            RequesterDispatcher::ChromeBrowser(_) => "ChromeBrowser".into(),
            RequesterDispatcher::HTTPClient(_) => "HTTPClient".into(),
        }
    }
}

/// Authentication methods for API calls or HTTP requests. [`GenericIntegrationModule`]
/// uses them to apply correct auth method. See the method descriptions to learn how it works
#[derive(PartialEq)]
pub enum AuthenticationMethod {
    /// Some APIs uses request headers to get API key. If this auth type selected API key
    /// will add in request headers with a given header key
    APIKeyAsHeader(String),
    /// This auth type uses when API require API key as a query param. If this method chose
    /// API key will be added in URL as a query param with given parameter key
    APIKeyAsQueryParam(String),
    /// Use basic HTTP authentication method. If the credentials are not provided, module
    /// tries to fetch from environment variables using pre-formatted
    /// (see [`format_env`](crate::utils::env::format_env)) variable names. Module specific
    /// variable names looks like `SUBSCAN_FOO_USERNAME`, `SUBSCAN_FOO_PASSWORD`
    BasicHTTPAuthentication(Credentials),
    /// This auth type does nothing for auth
    NoAuthentication,
}

impl From<Credentials> for AuthenticationMethod {
    fn from(credentials: Credentials) -> Self {
        Self::BasicHTTPAuthentication(credentials)
    }
}

impl AuthenticationMethod {
    /// Checks the any auth method selector or not
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::enums::AuthenticationMethod;
    ///
    /// let as_header = AuthenticationMethod::APIKeyAsHeader("X-API-Key".to_string());
    /// let no_auth = AuthenticationMethod::NoAuthentication;
    ///
    /// assert!(as_header.is_set());
    /// assert!(!no_auth.is_set());
    /// ```
    pub fn is_set(&self) -> bool {
        self != &Self::NoAuthentication
    }
}

/// Content enumeration that stores all content types in a single storage
#[derive(Clone, Default)]
pub enum Content {
    /// Stores [`String`] typed content
    String(String),
    /// Stores [`Value`] typed content
    JSON(Value),
    /// Empty content type
    #[default]
    Empty,
}

impl From<String> for Content {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Content {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<Value> for Content {
    fn from(value: Value) -> Self {
        Self::JSON(value)
    }
}

impl Content {
    /// Returns content value as a [`String`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::enums::Content;
    /// use serde_json::json;
    ///
    /// let json = json!({"foo": "bar"});
    /// let content = Content::from(json);
    /// let empty = Content::default();
    ///
    /// assert_eq!(content.as_string(), "{\"foo\":\"bar\"}");
    /// assert_eq!(empty.as_string(), "");
    /// ```
    pub fn as_string(self) -> String {
        match self {
            Self::String(content) => content,
            Self::JSON(json) => json.to_string(),
            Self::Empty => String::new(),
        }
    }
    /// Returns content value as a [`Value`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::enums::Content;
    /// use serde_json::json;
    ///
    /// let json = json!({"foo": "bar"});
    /// let content = Content::from("{\"foo\":\"bar\"}");
    ///
    /// assert_eq!(content.as_json(), json);
    /// ```
    pub fn as_json(self) -> Value {
        match self {
            Self::String(content) => serde_json::from_str(&content).unwrap_or_default(),
            Self::JSON(json) => json,
            Self::Empty => Value::Null,
        }
    }
    /// Returns [`true`] if content is empty otherwise [`false`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::enums::Content;
    /// use serde_json::Value;
    ///
    /// let empty = Content::Empty;
    /// let non_empty = Content::from("foo");
    ///
    /// assert!(empty.is_empty());
    /// assert!(!non_empty.is_empty());
    ///
    /// assert!(Content::from("").is_empty());
    /// assert!(Content::from(Value::Null).is_empty());
    /// ```
    pub fn is_empty(self) -> bool {
        match self {
            Self::String(content) => content.is_empty(),
            Self::JSON(json) => json == Value::Null,
            Self::Empty => true,
        }
    }
}
