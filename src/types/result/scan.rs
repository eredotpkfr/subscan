use super::module::SubscanModuleResult;
use crate::{
    enums::module::SubscanModuleStatus,
    types::{
        core::Subdomain,
        result::{metadata::SubscanScanResultMetadata, stats::SubscanModuleStatistics},
    },
};
use chrono::Utc;
use serde::Serialize;
use std::collections::BTreeSet;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

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
    pub fn save(&self, output: &str, domain: &str) {
        let data_dir = Path::new("data");
        if !data_dir.exists() {
            fs::create_dir_all(data_dir).expect("Couldn't create data directory");
        }

        let now = Utc::now().format("%Y-%m-%d");
        let filename = format!("{}_{}.{}", now, domain, output.to_lowercase());
        let filepath = data_dir.join(filename);

        match output.to_uppercase().as_str() {
            "JSON" => self
                .save_json(filepath.to_str().unwrap())
                .expect("Failed to save JSON"),
            "TXT" => self.save_txt(filepath.to_str().unwrap()),
            "CSV" => self.save_csv(filepath.to_str().unwrap()),
            _ => panic!("Unsupported format"),
        }
    }

    fn save_json(&self, path: &str) -> std::io::Result<()> {
        let json_content = serde_json::to_string_pretty(&self).expect("Failed to serialize JSON");
        let mut file = File::create(path)?;
        file.write_all(json_content.as_bytes())?;
        Ok(())
    }

    pub fn save_txt(&self, path: &str) {
        let mut file = File::create(path).expect("Failed to create TXT file");
        for subdomain in &self.results {
            writeln!(file, "{}", subdomain).expect("Failed to write to TXT file");
        }
    }

    fn save_csv(&self, path: &str) {
        let mut file = File::create(path).expect("Failed to create CSV file");
        // CSV formatında veriyi kaydet
        writeln!(file, "Data not implemented for CSV yet.").expect("Failed to write to CSV file");
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
