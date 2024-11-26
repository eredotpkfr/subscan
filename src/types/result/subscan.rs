use std::{collections::BTreeSet, io::Write};

use chrono::Utc;
use colored::Colorize;
use csv::WriterBuilder;
use serde::Serialize;
use serde_json;

use super::{pool::PoolResult, statistics::SubscanResultStatistics};
use crate::{
    enums::output::OutputFormat,
    types::result::{item::SubscanResultItem, metadata::SubscanResultMetadata},
    utilities::cli,
};

/// `Subscan` scan result type
#[derive(Clone, Default, Serialize)]
pub struct SubscanResult {
    pub metadata: SubscanResultMetadata,
    pub statistics: SubscanResultStatistics,
    pub items: BTreeSet<SubscanResultItem>,
    pub total: usize,
}

impl SubscanResult {
    pub async fn save(&self, output: &OutputFormat) -> String {
        let (file, filename) = output.get_file(&self.metadata.target).await;

        match output {
            OutputFormat::TXT => self.save_txt(file).await,
            OutputFormat::CSV => self.save_csv(file).await,
            OutputFormat::JSON => self.save_json(file).await,
            OutputFormat::HTML => self.save_html(file).await,
        }

        log::info!("Scan results saved to {filename}");

        filename
    }

    async fn save_txt<W: Write>(&self, mut writer: W) {
        for item in &self.items {
            writeln!(writer, "{}", item.as_txt()).unwrap();
        }
    }

    async fn save_csv<W: Write>(&self, writer: W) {
        let mut writer = WriterBuilder::new().has_headers(true).from_writer(writer);

        for item in &self.items {
            writer.serialize(item).unwrap()
        }
    }

    async fn save_json<W: Write>(&self, mut writer: W) {
        let json_content = serde_json::to_string_pretty(&self).unwrap();

        writer.write_all(json_content.as_bytes()).unwrap()
    }

    async fn save_html<W: Write>(&self, mut writer: W) {
        let mut table = cli::create_scan_result_item_table().await;

        for item in &self.items {
            table.add_row(item.as_table_row());
        }

        table.print_html(&mut writer).unwrap()
    }

    pub async fn log(&self) {
        for item in &self.items {
            log::info!("{}", item.as_txt().white());
        }

        log::info!("Total: {}", self.items.len());
    }
}

impl From<&str> for SubscanResult {
    fn from(target: &str) -> Self {
        Self {
            metadata: target.into(),
            ..Default::default()
        }
    }
}

impl Extend<SubscanResultItem> for SubscanResult {
    fn extend<T: IntoIterator<Item = SubscanResultItem>>(&mut self, iter: T) {
        self.items.extend(iter);
    }
}

impl SubscanResult {
    /// Update fields with [`PoolResult`]
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeSet;
    /// use subscan::types::result::{
    ///     subscan::SubscanResult,
    ///     pool::PoolResult,
    /// };
    /// use subscan::types::core::Subdomain;
    /// use subscan::types::result::item::PoolResultItem;
    /// use subscan::types::result::statistics::PoolStatistics;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut result = SubscanResult::default();
    ///     let item = PoolResultItem {
    ///         subdomain: Subdomain::from("bar.foo.com"),
    ///         ip: None,
    ///     };
    ///
    ///     let poolres = PoolResult {
    ///         statistics: PoolStatistics::default(),
    ///         items: BTreeSet::from_iter([item]),
    ///     };
    ///
    ///     result.update_with_pool_result(poolres).await;
    ///
    ///     assert_eq!(result.statistics.module.len(), 0);
    ///     assert_eq!(result.items.len(), 1);
    /// }
    /// ```
    pub async fn update_with_pool_result(&mut self, result: PoolResult) {
        self.statistics.set(result.statistics).await;
        self.items.extend(result.items);
    }

    /// Update `finished_at`, `elapsed` and `total` fields and returns itself
    ///
    /// # Examples
    ///
    /// ```
    /// use std::collections::BTreeSet;
    /// use subscan::types::result::{
    ///     subscan::SubscanResult,
    ///     statistics::PoolStatistics,
    ///     pool::PoolResult,
    /// };
    /// use subscan::types::result::item::PoolResultItem;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let item = PoolResultItem {
    ///         subdomain: "bar.foo.com".to_string(),
    ///         ip: None,
    ///     };
    ///
    ///     let mut result: SubscanResult = "foo.com".into();
    ///     let poolres = PoolResult {
    ///         statistics: PoolStatistics::default(),
    ///         items: BTreeSet::from_iter([item]),
    ///     };
    ///
    ///     result.extend(poolres.items);
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
        self.total = self.items.len();

        self
    }
}
