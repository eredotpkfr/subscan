use std::{collections::BTreeSet, net::IpAddr};

use colored::Colorize;
use derive_more::From;
use prettytable::{row, Row};
use serde::Serialize;

use super::status::SubscanModuleStatus;
use crate::{error::ModuleErrorKind, types::core::Subdomain};

/// Subscan result items data type
pub type SubscanResultItems = BTreeSet<SubscanResultItem>;

#[derive(Clone, Debug, From, PartialEq)]
#[from((&str, &Subdomain))]
pub struct SubscanModuleResultItem {
    pub module: String,
    pub subdomain: Subdomain,
}

#[derive(Clone, Debug, From, PartialEq)]
#[from((&str, ModuleErrorKind))]
#[from((&str, SubscanModuleStatus))]
#[from((&str, &str))]
pub struct SubscanModuleStatusItem {
    pub module: String,
    pub status: SubscanModuleStatus,
}

impl SubscanModuleStatusItem {
    pub async fn log(&self) {
        self.status.log(&self.module);
    }
}

/// Core scan result item, simply stores single discovered subdomain and
/// its IP address
#[derive(Clone, Debug, Default, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct SubscanResultItem {
    pub subdomain: Subdomain,
    pub ip: Option<IpAddr>,
}

impl SubscanResultItem {
    /// Returns as a txt line
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::item::SubscanResultItem;
    ///
    /// let item = SubscanResultItem {
    ///     subdomain: "baz.foo.com".into(),
    ///     ip: None
    /// };
    ///
    /// assert_eq!(item.as_txt(), "baz.foo.com\t");
    /// ```
    pub fn as_txt(&self) -> String {
        format!(
            "{}\t{}",
            self.subdomain,
            self.ip.map_or("".to_string(), |ip| ip.to_string())
        )
    }

    /// Returns as a [`Row`] instance. It can be used with table that returns
    /// from [`create_scan_result_item_table`](crate::utilities::cli::create_scan_result_item_table)
    /// function
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::utilities::cli::create_scan_result_item_table;
    /// use subscan::types::result::item::SubscanResultItem;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut table = create_scan_result_item_table().await;
    ///     let item = SubscanResultItem {
    ///         subdomain: "bar.foo.com".into(),
    ///         ip: None
    ///     };
    ///
    ///     table.add_row(item.as_table_row());
    ///
    ///     assert!(!table.is_empty());
    /// }
    /// ```
    pub fn as_table_row(&self) -> Row {
        row![
            self.subdomain,
            self.ip.map_or("".to_string(), |ip| ip.to_string())
        ]
    }

    pub async fn log(&self) {
        log::info!("{}", self.as_txt().white());
    }
}
