#[derive(Clone, Debug)]
pub struct ModuleNameFilter {
    pub valids: Vec<String>,
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
