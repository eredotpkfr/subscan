use crate::types::core::Subdomain;
use prettytable::{row, Row};
use serde::Serialize;
use std::net::IpAddr;

pub type SubscanModulePoolResultItem = ScanResultItem;

#[derive(Clone, Default, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ScanResultItem {
    /// Discovered subdomain address
    pub subdomain: Subdomain,
    /// IP address of subdomain
    pub ip: Option<IpAddr>,
}

impl ScanResultItem {
    pub fn as_txt(&self) -> String {
        format!(
            "{} {}",
            self.subdomain,
            self.ip.map_or("".to_string(), |ip| ip.to_string())
        )
    }

    pub fn as_table_row(&self) -> Row {
        row![
            self.subdomain,
            self.ip.map_or("".to_string(), |ip| ip.to_string())
        ]
    }
}
