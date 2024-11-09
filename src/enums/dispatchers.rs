use crate::{
    extractors::{html::HTMLExtractor, json::JSONExtractor, regex::RegexExtractor},
    interfaces::module::SubscanModuleInterface,
    modules::{
        generics::{engine::GenericSearchEngineModule, integration::GenericIntegrationModule},
        integrations::{
            commoncrawl::CommonCrawl, dnsdumpster::DnsDumpster, github::GitHub, netlas::Netlas,
            waybackarchive::WaybackArchive,
        },
        zonetransfer::ZoneTransfer,
    },
    requesters::{chrome::ChromeBrowser, client::HTTPClient},
};
use enum_dispatch::enum_dispatch;
use prettytable::{row, Row};

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
    /// Non-generic `ZoneTransfer` module variant
    ZoneTransfer(ZoneTransfer),
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

    /// Converts module object to module table row representation
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::utilities::cli;
    /// use subscan::modules::engines::{
    ///     google::Google,
    ///     duckduckgo::DuckDuckGo
    /// };
    /// use subscan::modules::zonetransfer::ZoneTransfer;
    /// use subscan::modules::integrations::{
    ///     alienvault::AlienVault,
    ///     commoncrawl::CommonCrawl
    /// };
    /// use prettytable::table;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let modules = vec![
    ///         Google::dispatcher(),
    ///         DuckDuckGo::dispatcher(),
    ///         ZoneTransfer::dispatcher(),
    ///         AlienVault::dispatcher(),
    ///         CommonCrawl::dispatcher()
    ///     ];
    ///
    ///     let mut table = cli::create_module_table().await;
    ///
    ///     for module in modules {
    ///         table.add_row(module.as_table_row().await);
    ///     }
    ///
    ///     assert!(!table.is_empty());
    /// }
    /// ```
    pub async fn as_table_row(&self) -> Row {
        let requester = if let Some(instance) = self.requester().await {
            instance.lock().await.to_string()
        } else {
            "None".into()
        };

        let extractor = if let Some(instance) = self.extractor().await {
            instance.to_string()
        } else {
            "None".into()
        };

        row![
            self.name().await,
            requester,
            extractor,
            self.is_generic().await
        ]
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
