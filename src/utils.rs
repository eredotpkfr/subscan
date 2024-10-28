/// JSON serializer utilities
pub mod serializers {
    use chrono::{DateTime, TimeDelta, Utc};
    use serde::Serializer;

    pub fn dt_to_string_method<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&dt.to_string())
    }

    pub fn td_num_seconds_method<S>(td: &TimeDelta, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(td.num_seconds())
    }
}

/// Helpful functions that uses CLI related things
pub mod cli {
    use crate::{
        enums::dispatchers::SubscanModuleDispatcher, interfaces::module::SubscanModuleInterface,
    };
    use prettytable::{format::consts::FORMAT_NO_LINESEP_WITH_TITLE, row, Row, Table};

    /// Creates table for module representation
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::utils::cli;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let table = cli::create_module_table().await;
    ///
    ///     assert!(table.is_empty());
    /// }
    /// ```
    pub async fn create_module_table() -> Table {
        let mut table = Table::new();

        let titles = row![
            FdBwbd -> "Name",
            FdBwbd -> "Requester",
            FdBwbd -> "Extractor",
            FdBwbd -> "Is Generic?",
        ];

        table.set_format(*FORMAT_NO_LINESEP_WITH_TITLE);
        table.set_titles(titles);
        table
    }

    /// Converts module object to module table row representation
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::utils::cli;
    /// use subscan::modules::engines::google::Google;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let module = Google::dispatcher();
    ///     let mut table = cli::create_module_table().await;
    ///
    ///     table.add_row(cli::module_as_table_row(&module).await);
    ///
    ///     assert!(!table.is_empty());
    /// }
    /// ```
    pub async fn module_as_table_row(module: &SubscanModuleDispatcher) -> Row {
        let requester = if let Some(instance) = module.requester().await {
            instance.lock().await.to_string()
        } else {
            "None".into()
        };

        let extractor = if let Some(instance) = module.extractor().await {
            instance.to_string()
        } else {
            "None".into()
        };

        row![
            Fw -> module.name().await,
            Fw -> requester,
            Fw -> extractor,
            Fw -> module.is_generic().await
        ]
    }
}

/// Helpful regex utilities listed in this module
pub mod regex {
    use core::result::Result;
    use regex::{Error, Regex};

    /// Helper function that generates dynamically regex statement
    /// by given domain address to parse subdomains
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::utils::regex::generate_subdomain_regex;
    ///
    /// let regex_stmt = generate_subdomain_regex("foo.com").unwrap();
    ///
    /// assert_eq!(regex_stmt.as_str(), "(?:[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?\\.)+(foo\\.com)");
    ///
    /// assert!(regex_stmt.find("bar.foo.com").is_some());
    /// assert!(regex_stmt.find("foo").is_none());
    /// ```
    pub fn generate_subdomain_regex(domain: &str) -> Result<Regex, Error> {
        let formatted = format!(
            r"(?:[a-z0-9](?:[a-z0-9-]{{0,61}}[a-z0-9])?\.)+({domain})",
            domain = domain.replace(".", r"\.")
        );

        Regex::new(&formatted)
    }
}

/// Utilities about project environments
pub mod env {
    use crate::config::SUBSCAN_ENV_NAMESPACE;

    /// Formats given module name and environment variable name with [`SUBSCAN_ENV_NAMESPACE`]
    /// prefix, returns fully generated environment variable name
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::utils::env::format_env;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     assert_eq!(format_env("foo", "apikey"), "SUBSCAN_FOO_APIKEY");
    ///     assert_eq!(format_env("foo", "username"), "SUBSCAN_FOO_USERNAME");
    ///     assert_eq!(format_env("bar", "password"), "SUBSCAN_BAR_PASSWORD");
    /// }
    /// ```
    pub fn format_env(name: &str, env: &str) -> String {
        format!(
            "{}_{}_{}",
            SUBSCAN_ENV_NAMESPACE,
            name.to_uppercase(),
            env.to_uppercase(),
        )
    }
}

/// Helpful HTTP utilities
pub mod http {
    use reqwest::Url;

    /// Update query params without remove old query params. If the
    /// given parameter name non-exists it will append end of the
    /// query otherwise it's value will be updated
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::utils::http::update_url_query;
    /// use reqwest::Url;
    ///
    /// let mut url: Url = "https://foo.com".parse().unwrap();
    ///
    /// update_url_query(&mut url, "a".into(), "b".into());
    /// assert_eq!(url.to_string(), "https://foo.com/?a=b");
    ///
    /// // does not override old `a` parameter
    /// update_url_query(&mut url, "x".into(), "y".into());
    /// assert_eq!(url.to_string(), "https://foo.com/?a=b&x=y");
    ///
    /// update_url_query(&mut url, "a".into(), "c".into());
    /// assert_eq!(url.to_string(), "https://foo.com/?x=y&a=c");
    /// ```
    pub fn update_url_query(url: &mut Url, name: &str, value: &str) {
        let binding = url.clone();
        let pairs = binding.query_pairs();
        let old = pairs.filter(|item| item.0.to_lowercase() != name.to_lowercase());

        url.query_pairs_mut()
            .clear()
            .extend_pairs(old)
            .append_pair(name, value)
            .finish();
    }
}
