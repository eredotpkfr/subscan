use crate::types::filters::ModuleNameFilter;

#[derive(Clone, Debug, Default, PartialEq)]
pub enum CacheFilter {
    /// Do nothing to eliminate modules from cache
    #[default]
    NoFilter,
    /// Filter modules by their names
    FilterByName(ModuleNameFilter),
}

impl CacheFilter {
    /// Check module name is filtered or non-filtered by filter type
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::enums::cache::CacheFilter;
    /// use subscan::types::filters::ModuleNameFilter;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let filter: ModuleNameFilter = (vec![], vec![]).into();
    ///
    ///     assert!(!CacheFilter::NoFilter.is_filtered("foo").await);
    ///     assert!(!CacheFilter::FilterByName(filter).is_filtered("foo").await);
    ///
    ///     let filter: ModuleNameFilter = (vec![], vec!["foo".into()]).into();
    ///     assert!(CacheFilter::FilterByName(filter).is_filtered("foo").await);
    ///
    ///     let filter: ModuleNameFilter = (vec!["bar".into()], vec![]).into();
    ///     assert!(CacheFilter::FilterByName(filter).is_filtered("foo").await);
    ///
    ///     let filter: ModuleNameFilter = (vec!["foo".into()], vec!["foo".into()]).into();
    ///     assert!(CacheFilter::FilterByName(filter).is_filtered("foo").await);
    /// }
    /// ```
    pub async fn is_filtered(&self, name: &str) -> bool {
        match self {
            CacheFilter::NoFilter => false,
            CacheFilter::FilterByName(filter) => {
                if filter.valids.is_empty() && filter.invalids.is_empty() {
                    false
                } else if filter.valids.is_empty() && !filter.invalids.is_empty() {
                    filter.invalids.contains(&name.to_lowercase())
                } else if !filter.valids.is_empty() && filter.invalids.is_empty() {
                    !filter.valids.contains(&name.to_lowercase())
                } else {
                    !filter.valids.contains(&name.to_lowercase())
                        || filter.invalids.contains(&name.to_lowercase())
                }
            }
        }
    }
}
