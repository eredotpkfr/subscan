use super::module::SubscanModuleResult;
use crate::{
    enums::{module::SubscanModuleStatus, output::OutputFormat},
    types::{
        core::Subdomain,
        result::{metadata::SubscanScanResultMetadata, stats::SubscanModuleStatistics},
    },
};
use chrono::Utc;
use csv::Writer;
use serde::Serialize;
use serde_json;
use std::{
    collections::BTreeSet,
    fs::File,
    io::{self, Write},
};

/// `Subscan` scan result
#[derive(Clone, Default, Serialize)]
pub struct SubscanScanResult {
    /// Scan metadata
    pub metadata: SubscanScanResultMetadata,
    /// Module statistics
    pub statistics: Vec<SubscanModuleStatistics>,
    /// Subscans that have been discovered
    pub results: BTreeSet<Subdomain>,
    /// Total count of discovered subdomains
    pub total: usize,
}

impl SubscanScanResult {
    pub fn save(&self, output: &OutputFormat) {
        let now = Utc::now().format("%Y-%m-%d");
        let filename = match output {
            OutputFormat::TXT => format!("{}_{}.txt", now, self.metadata.target),
            OutputFormat::CSV => format!("{}_{}.csv", now, self.metadata.target),
            OutputFormat::JSON => format!("{}_{}.json", now, self.metadata.target),
        };

        let file = self
            .get_output_file(&filename)
            .expect("Failed to create output file");

        match output {
            OutputFormat::TXT => self.save_txt(file),
            OutputFormat::CSV => self.save_csv(file),
            OutputFormat::JSON => self.save_json(file),
        }
    }

    fn get_output_file(&self, filename: &str) -> io::Result<File> {
        File::create(filename)
    }

    fn save_json<W: Write>(&self, mut writer: W) {
        let json_content = serde_json::to_string_pretty(&self).expect("Failed to serialize JSON");
        writer
            .write_all(json_content.as_bytes())
            .expect("Failed to write JSON");
    }

    fn save_txt<W: Write>(&self, mut writer: W) {
        for subdomain in &self.results {
            writeln!(writer, "{}", subdomain).expect("Failed to write TXT line");
        }
    }

    fn save_csv<W: Write>(&self, writer: W) {
        let mut wtr = Writer::from_writer(writer);
        wtr.write_record(&["subdomains"])
            .expect("Failed to write CSV header");

        for subdomain in &self.results {
            wtr.write_record(&[subdomain])
                .expect("Failed to write CSV record");
        }
        wtr.flush().expect("Failed to flush CSV writer");
    }
}

impl From<&str> for SubscanScanResult {
    fn from(target: &str) -> Self {
        Self {
            metadata: target.into(),
            ..Default::default()
        }
    }
}

impl Extend<Subdomain> for SubscanScanResult {
    fn extend<T: IntoIterator<Item = Subdomain>>(&mut self, iter: T) {
        self.results.extend(iter);
    }
}

impl SubscanScanResult {
    /// Update `finished_at`, `elapsed` and `total` fields and returns itself
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::scan::SubscanScanResult;
    /// use std::collections::BTreeSet;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut result: SubscanScanResult = "foo.com".into();
    ///
    ///     result.extend(BTreeSet::from_iter(["bar.foo.com".into()]));
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

    /// Group modules by their statuses
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::scan::SubscanScanResult;
    /// use subscan::enums::module::{SubscanModuleStatus, SkipReason};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut result: SubscanScanResult = "foo.com".into();
    ///
    ///     result.add_status("one", &SubscanModuleStatus::Started).await;
    ///     result.add_status("two", &SkipReason::NotAuthenticated.into()).await;
    ///     result.add_status("three", &SubscanModuleStatus::Finished).await;
    ///     result.add_status("four", &SubscanModuleStatus::Failed("bar".into())).await;
    ///
    ///     assert_eq!(result.metadata.started.len(), 1);
    ///     assert_eq!(result.metadata.skipped.len(), 1);
    ///     assert_eq!(result.metadata.finished.len(), 1);
    ///     assert_eq!(result.metadata.failed.len(), 1);
    /// }
    /// ```
    pub async fn add_status(&mut self, module: &str, status: &SubscanModuleStatus) -> bool {
        match status {
            SubscanModuleStatus::Started => self.metadata.started.insert(module.to_string()),
            SubscanModuleStatus::Skipped(_) => self.metadata.skipped.insert(module.to_string()),
            SubscanModuleStatus::Finished => self.metadata.finished.insert(module.to_string()),
            SubscanModuleStatus::Failed(_) => self.metadata.failed.insert(module.to_string()),
        }
    }

    /// Add module statistics on [`SubscanScanResult`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::scan::SubscanScanResult;
    /// use subscan::enums::module::{SubscanModuleStatus, SkipReason};
    /// use subscan::types::result::module::SubscanModuleResult;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut scan_result: SubscanScanResult = "foo.com".into();
    ///     let module_result: SubscanModuleResult = "foo".into();
    ///
    ///     scan_result.add_statistic(module_result.into()).await;
    ///
    ///     assert_eq!(scan_result.statistics.len(), 1);
    /// }
    /// ```
    pub async fn add_statistic(&mut self, stats: SubscanModuleStatistics) {
        self.statistics.push(stats);
    }

    /// Update scan results with any module result, that merges all subdomains and
    /// statistics into [`SubscanScanResult`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::scan::SubscanScanResult;
    /// use subscan::enums::module::{SubscanModuleStatus, SkipReason};
    /// use subscan::types::result::module::SubscanModuleResult;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut scan_result: SubscanScanResult = "foo.com".into();
    ///     let module_result: SubscanModuleResult = "foo".into();
    ///
    ///     scan_result.update_with_module_result(module_result).await;
    ///
    ///     assert_eq!(scan_result.statistics.len(), 1);
    ///     assert_eq!(scan_result.metadata.started.len(), 1);
    ///     assert_eq!(scan_result.results.len(), 0);
    ///     assert_eq!(scan_result.total, 0);
    /// }
    /// ```
    pub async fn update_with_module_result(&mut self, result: SubscanModuleResult) {
        self.add_status(&result.module, &result.status).await;
        self.add_statistic(result.stats()).await;
        self.extend(result.subdomains);
    }
}
