use crate::extractors::{html::HTMLExtractor, json::JSONExtractor, regex::RegexExtractor};
use crate::modules::generics::api_integration::GenericAPIIntegrationModule;
use crate::modules::generics::search_engine::GenericSearchEngineModule;
use crate::requesters::{chrome::ChromeBrowser, client::HTTPClient};
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
    /// a [`GenericAPIIntegrationModule`]
    GenericAPIIntegrationModule(GenericAPIIntegrationModule),
    /// Also another generic variant for search engines, It can be used for all generic search
    /// engine modules at the same time. Just modules should be implemented as
    /// a [`GenericSearchEngineModule`]
    GenericSearchEngineModule(GenericSearchEngineModule),
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
    /// and examples usages
    RegexExtractor(RegexExtractor),
    /// JSON extractor type can extract subdomains from JSON content. In this type head up point
    /// is to know that created as a wrapper struct to be compatible with
    /// [`SubdomainExtractorInterface`](crate::interfaces::extractor::SubdomainExtractorInterface)
    /// so parser method must be implemented and gave this wrapper struct. See the
    /// [`JSONExtractor`] struct and [`InnerExtractMethod`](crate::types::core::InnerExtractMethod)
    /// type for examples and technical details
    JSONExtractor(JSONExtractor),
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

/// Authentication methods for API calls.
/// [`GenericAPIIntegrationModule`] uses them to apply
/// correct auth method. See the method descriptions to
/// learn how it works
#[derive(PartialEq)]
pub enum APIAuthMethod {
    /// Some APIs uses request headers to get API key. If this auth type selected API key
    /// will add in request headers with a given header key
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::enums::APIAuthMethod;
    ///
    /// let auth = APIAuthMethod::APIKeyAsHeader("X-API-Key".into());
    /// ```
    APIKeyAsHeader(String),
    /// This auth type uses when API require API key as a query param. If this method chose
    /// API key will be added in URL as a query param with given parameter key
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::enums::APIAuthMethod;
    ///
    /// let auth = APIAuthMethod::APIKeyAsQueryParam("q".into());
    /// ```
    APIKeyAsQueryParam(String),
    /// This auth type does nothing for auth
    NoAuth,
}

impl APIAuthMethod {
    /// Checks the any auth method selector or not
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::enums::APIAuthMethod;
    ///
    /// let as_header = APIAuthMethod::APIKeyAsHeader("X-API-Key".to_string());
    /// let no_auth = APIAuthMethod::NoAuth;
    ///
    /// assert!(as_header.is_set());
    /// assert!(!no_auth.is_set());
    /// ```
    pub fn is_set(&self) -> bool {
        self != &Self::NoAuth
    }
}

/// Content enumeration that stores all content types in a single storage
#[derive(Clone, Default)]
pub enum Content {
    /// Stores [`String`] typed content
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::enums::Content;
    ///
    /// let content = Content::String(String::from("foo"));
    /// ```
    String(String),
    /// Stores [`Value`] typed content
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use subscan::enums::Content;
    /// use serde_json::json;
    ///
    /// let json = Content::JSON(json!({"foo": "bar"}));
    /// ```
    JSON(Value),
    /// Empty content type
    #[default]
    Empty,
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
    ///
    /// assert_eq!(content.as_string(), "{\"foo\":\"bar\"}");
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
    /// Returns `true` if content is empty otherwise `false`
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
