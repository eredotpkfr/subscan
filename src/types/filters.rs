/// This filter allows to filter modules by their names
#[derive(Clone, Debug, PartialEq)]
pub struct ModuleNameFilter {
    /// Valid [`SubscanModule`](crate::types::core::SubscanModule) names list
    pub valids: Vec<String>,
    /// Invalid [`SubscanModule`](crate::types::core::SubscanModule) names list
    pub invalids: Vec<String>,
}

impl From<(Vec<String>, Vec<String>)> for ModuleNameFilter {
    /// Create [`ModuleNameFilter`] from tuple
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::filters::ModuleNameFilter;
    ///
    /// let filter = ModuleNameFilter::from((vec![], vec!["foo".into()]));
    ///
    /// assert!(filter.valids.is_empty());
    /// assert!(!filter.invalids.is_empty());
    /// ```
    fn from(tuple: (Vec<String>, Vec<String>)) -> Self {
        Self {
            valids: tuple.0,
            invalids: tuple.1,
        }
    }
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
        if self.valids.is_empty() && self.invalids.is_empty() {
            false
        } else if self.valids.is_empty() && !self.invalids.is_empty() {
            self.invalids.contains(&name.to_lowercase())
        } else if !self.valids.is_empty() && self.invalids.is_empty() {
            !self.valids.contains(&name.to_lowercase())
        } else {
            !self.valids.contains(&name.to_lowercase())
                || self.invalids.contains(&name.to_lowercase())
        }
    }
}
