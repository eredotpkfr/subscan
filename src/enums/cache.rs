use crate::types::filters::ModuleNameFilter;

#[derive(Clone, Debug, Default)]
pub enum CacheFilter {
    /// Do nothing to eliminate modules from cache
    #[default]
    NoFilter,
    /// Filter modules by their names
    FilterByName(ModuleNameFilter),
}

impl CacheFilter {
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
