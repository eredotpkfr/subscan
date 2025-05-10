use derive_more::From;

/// This filter allows to filter modules by their names
#[derive(Clone, Debug, From, PartialEq)]
#[from((Vec<String>, Vec<String>))]
pub struct ModuleNameFilter {
    /// Valid [`SubscanModule`](crate::types::core::SubscanModule) names list
    pub modules: Vec<String>,
    /// Invalid [`SubscanModule`](crate::types::core::SubscanModule) names list
    pub skips: Vec<String>,
}

impl ModuleNameFilter {
    /// Check module name is filtered or non-filtered by this filter
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::filters::ModuleNameFilter;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let filter: ModuleNameFilter = (vec![], vec![]).into();
    ///     assert!(!filter.is_filtered("foo").await);
    ///
    ///     let filter: ModuleNameFilter = (vec![], vec!["foo".into()]).into();
    ///     assert!(filter.is_filtered("foo").await);
    ///
    ///     let filter: ModuleNameFilter = (vec!["bar".into()], vec![]).into();
    ///     assert!(filter.is_filtered("foo").await);
    ///
    ///     let filter: ModuleNameFilter = (vec!["foo".into()], vec!["foo".into()]).into();
    ///     assert!(filter.is_filtered("foo").await);
    /// }
    /// ```
    pub async fn is_filtered(&self, name: &str) -> bool {
        if self.modules.is_empty() && self.skips.is_empty() {
            false
        } else if self.modules.is_empty() && !self.skips.is_empty() {
            self.skips.contains(&name.to_lowercase())
        } else if !self.modules.is_empty() && self.skips.is_empty() {
            !self.modules.contains(&name.to_lowercase())
        } else {
            !self.modules.contains(&name.to_lowercase())
                || self.skips.contains(&name.to_lowercase())
        }
    }
}
