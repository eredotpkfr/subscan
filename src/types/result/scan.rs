use super::{pool::SubscanModulePoolResult, stats::ScanResultStatistics};
use crate::{
    enums::output::OutputFormat,
    types::result::{item::ScanResultItem, metadata::ScanResultMetadata},
    utilities::cli,
};
use chrono::Utc;
use colored::Colorize;
use csv::WriterBuilder;
use serde::Serialize;
use serde_json;
use std::{collections::BTreeSet, fs::File, io::Write};

/// `Subscan` scan result
#[derive(Clone, Default, Serialize)]
pub struct ScanResult {
    /// Scan metadata
    pub metadata: ScanResultMetadata,
    /// Module statistics
    pub statistics: ScanResultStatistics,
    /// Subdomains that have been discovered
    pub results: BTreeSet<ScanResultItem>,
    /// Total count of discovered subdomains
    pub total: usize,
}

impl ScanResult {
    pub async fn save(&self, output: &OutputFormat) -> String {
        let (file, filename) = self.get_output_file(output).await;

        match output {
            OutputFormat::TXT => self.save_txt(file).await,
            OutputFormat::CSV => self.save_csv(file).await,
            OutputFormat::JSON => self.save_json(file).await,
            OutputFormat::HTML => self.save_html(file).await,
        }

        log::info!("Scan results saved to {filename}");

        filename
    }

    async fn get_output_file(&self, output: &OutputFormat) -> (File, String) {
        let now = Utc::now().timestamp();
        let filename = match output {
            OutputFormat::TXT => format!("{}_{}.txt", self.metadata.target, now),
            OutputFormat::CSV => format!("{}_{}.csv", self.metadata.target, now),
            OutputFormat::JSON => format!("{}_{}.json", self.metadata.target, now),
            OutputFormat::HTML => format!("{}_{}.html", self.metadata.target, now),
        };

        (File::create(filename.clone()).unwrap(), filename)
    }

    async fn save_txt<W: Write>(&self, mut writer: W) {
        for item in &self.results {
            writeln!(writer, "{}", item.as_txt()).unwrap();
        }
    }

    async fn save_csv<W: Write>(&self, writer: W) {
        let mut writer = WriterBuilder::new().has_headers(true).from_writer(writer);

        for item in &self.results {
            writer.serialize(item).unwrap()
        }
    }

    async fn save_json<W: Write>(&self, mut writer: W) {
        let json_content = serde_json::to_string_pretty(&self).unwrap();

        writer.write_all(json_content.as_bytes()).unwrap()
    }

    async fn save_html<W: Write>(&self, mut writer: W) {
        let mut table = cli::create_scan_result_item_table().await;

        for item in &self.results {
            table.add_row(item.as_table_row());
        }

        table.print_html(&mut writer).unwrap()
    }

    pub async fn log(&self) {
        for item in &self.results {
            log::info!("{}", item.as_txt().white());
        }

        log::info!("Total: {}", self.results.len());
    }
}

impl From<&str> for ScanResult {
    fn from(target: &str) -> Self {
        Self {
            metadata: target.into(),
            ..Default::default()
        }
    }
}

impl Extend<ScanResultItem> for ScanResult {
    fn extend<T: IntoIterator<Item = ScanResultItem>>(&mut self, iter: T) {
        self.results.extend(iter);
    }
}

impl ScanResult {
    /// Update `finished_at`, `elapsed` and `total` fields and returns itself
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeSet;
    /// use subscan::types::result::{
    ///     scan::ScanResult,
    ///     stats::SubscanModulePoolStatistics,
    ///     pool::SubscanModulePoolResult,
    /// };
    /// use subscan::types::result::item::SubscanModulePoolResultItem;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut result: ScanResult = "foo.com".into();
    ///     let pool_result = SubscanModulePoolResult {
    ///         statistics: SubscanModulePoolStatistics::default(),
    ///         results: BTreeSet::from_iter([
    ///             SubscanModulePoolResultItem {
    ///                 subdomain: "bar.foo.com".to_string(),
    ///                 ip: None,
    ///             }
    ///         ])
    ///     };
    ///
    ///     result.update_with_pool_result(pool_result).await;
    ///
    ///     let finished = result.clone().with_finished().await;
    ///
    ///     assert_eq!(finished.metadata.target, "foo.com");
    ///     assert_eq!(finished.total, 1);
    /// }
    /// ```
    pub async fn with_finished(mut self) -> Self {
        self.metadata.finished_at = Utc::now();
        self.metadata.elapsed = self.metadata.finished_at - self.metadata.started_at;
        self.total = self.results.len();

        self
    }

    /// Update scan results with any module result, that merges all subdomains and
    /// statistics into [`ScanResult`]
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeSet;
    /// use subscan::types::result::scan::ScanResult;
    /// use subscan::enums::module::{SubscanModuleStatus, SkipReason};
    /// use subscan::types::result::{
    ///     pool::SubscanModulePoolResult,
    ///     item::SubscanModulePoolResultItem,
    ///     stats::SubscanModulePoolStatistics
    /// };
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut scan_result: ScanResult = "foo.com".into();
    ///     let pool_result = SubscanModulePoolResult {
    ///         statistics: SubscanModulePoolStatistics::default(),
    ///         results: BTreeSet::from_iter([
    ///             SubscanModulePoolResultItem {
    ///                 subdomain: "bar.foo.com".to_string(),
    ///                 ip: None,
    ///             }
    ///         ])
    ///     };
    ///
    ///     scan_result.update_with_pool_result(pool_result).await;
    ///
    ///     assert_eq!(scan_result.statistics.module.len(), 0);
    ///     assert_eq!(scan_result.results.len(), 1);
    ///     assert_eq!(scan_result.total, 1);
    /// }
    /// ```
    pub async fn update_with_pool_result(&mut self, result: SubscanModulePoolResult) {
        self.statistics = result.statistics;
        self.results = result.results;
        self.total = self.results.len();
    }
}
