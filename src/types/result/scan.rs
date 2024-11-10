use super::module::SubscanModuleResult;
use crate::{
    enums::{module::SubscanModuleStatus, output::OutputFormat},
    types::{
        core::Subdomain,
        result::{metadata::ScanResultMetadata, stats::SubscanModuleStatistics},
    },
};
use chrono::Utc;
use csv::Writer;
use prettytable::{format::consts::FORMAT_NO_LINESEP_WITH_TITLE, row, table};
use serde::Serialize;
use serde_json;
use std::{collections::BTreeSet, fs::File, io::Write};

/// `Subscan` scan result
#[derive(Clone, Default, Serialize)]
pub struct ScanResult {
    /// Scan metadata
    pub metadata: ScanResultMetadata,
    /// Module statistics
    pub statistics: Vec<SubscanModuleStatistics>,
    /// Subscans that have been discovered
    pub results: BTreeSet<Subdomain>,
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
        for subdomain in &self.results {
            writeln!(writer, "{}", subdomain).unwrap();
        }
    }

    async fn save_csv<W: Write>(&self, writer: W) {
        let mut writer = Writer::from_writer(writer);

        writer.serialize("subdomains").unwrap();

        for subdomain in &self.results {
            writer.serialize(subdomain).unwrap()
        }
    }

    async fn save_json<W: Write>(&self, mut writer: W) {
        let json_content = serde_json::to_string_pretty(&self).unwrap();

        writer.write_all(json_content.as_bytes()).unwrap()
    }

    async fn save_html<W: Write>(&self, mut writer: W) {
        let mut table = table!();

        table.set_titles(row!["subdomain"]);
        table.set_format(*FORMAT_NO_LINESEP_WITH_TITLE);

        for subdomain in &self.results {
            table.add_row(row![subdomain]);
        }

        table.print_html(&mut writer).unwrap()
    }

    pub async fn log(&self) {
        for subdomain in &self.results {
            log::info!("{}", subdomain);
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

impl Extend<Subdomain> for ScanResult {
    fn extend<T: IntoIterator<Item = Subdomain>>(&mut self, iter: T) {
        self.results.extend(iter);
    }
}

impl ScanResult {
    /// Update `finished_at`, `elapsed` and `total` fields and returns itself
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::scan::ScanResult;
    /// use std::collections::BTreeSet;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut result: ScanResult = "foo.com".into();
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
    /// use subscan::types::result::scan::ScanResult;
    /// use subscan::enums::module::{SubscanModuleStatus, SkipReason};
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut result: ScanResult = "foo.com".into();
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

    /// Add module statistics on [`ScanResult`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::scan::ScanResult;
    /// use subscan::enums::module::{SubscanModuleStatus, SkipReason};
    /// use subscan::types::result::module::SubscanModuleResult;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut scan_result: ScanResult = "foo.com".into();
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
    /// statistics into [`ScanResult`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::scan::ScanResult;
    /// use subscan::enums::module::{SubscanModuleStatus, SkipReason};
    /// use subscan::types::result::module::SubscanModuleResult;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut scan_result: ScanResult = "foo.com".into();
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
