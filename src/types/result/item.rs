use crate::types::core::Subdomain;
use prettytable::{row, Row};
use serde::Serialize;
use std::net::IpAddr;

/// Module pool item, alias for [`ScanResultItem`]
pub type SubscanModulePoolResultItem = ScanResultItem;

/// Core scan result item, simply stores single discovered subdomain and
/// its IP address
#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ScanResultItem {
    pub subdomain: Subdomain,
    pub ip: Option<IpAddr>,
}

impl ScanResultItem {
    /// Returns as a txt line
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::item::ScanResultItem;
    ///
    /// let item = ScanResultItem {
    ///     subdomain: "bar.foo.com".into(),
    ///     ip: None
    /// };
    ///
    /// assert_eq!(item.as_txt(), "bar.foo.com\t");
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
    /// use subscan::types::result::item::ScanResultItem;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut table = create_scan_result_item_table().await;
    ///     let item = ScanResultItem {
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
}