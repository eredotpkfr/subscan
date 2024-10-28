/// Module skip reasons
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum SkipReason {
    /// If could not authenticated, this reason can be used
    NotAuthenticated,
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for SkipReason {
    fn to_string(&self) -> String {
        match self {
            SkipReason::NotAuthenticated => "not authenticated".into(),
        }
    }
}

/// Subscan module statuses
#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub enum SubscanModuleStatus {
    /// Uses when module execution skipped for any [`SkipReason`]
    Skipped(SkipReason),
    /// Indicates that module starts
    #[default]
    Started,
    /// Indicates that module finished successfully
    Finished,
    /// Uses when module failed for any reason with error message
    Failed(String),
}

impl From<SkipReason> for SubscanModuleStatus {
    fn from(reason: SkipReason) -> Self {
        Self::Skipped(reason)
    }
}

#[allow(clippy::to_string_trait_impl)]
impl ToString for SubscanModuleStatus {
    fn to_string(&self) -> String {
        match self {
            SubscanModuleStatus::Skipped(reason) => format!("[{} SKIPPED]", reason.to_string()),
            SubscanModuleStatus::Started => "[STARTED]".into(),
            SubscanModuleStatus::Finished => "[FINISHED]".into(),
            SubscanModuleStatus::Failed(reason) => format!("[{reason} FAILED]"),
        }
    }
}
