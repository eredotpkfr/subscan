use std::fmt::Display;

/// Module skip reasons
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum SkipReason {
    /// If could not authenticated, this reason can be used
    NotAuthenticated,
}

impl Display for SkipReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SkipReason::NotAuthenticated => write!(f, "not authenticated"),
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

impl Display for SubscanModuleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubscanModuleStatus::Skipped(_) => write!(f, "SKIPPED"),
            SubscanModuleStatus::Started => write!(f, "STARTED"),
            SubscanModuleStatus::Finished => write!(f, "FINISHED"),
            SubscanModuleStatus::Failed(_) => write!(f, "FAILED"),
        }
    }
}

impl SubscanModuleStatus {
    /// Returns as a log line representation
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::enums::module::SkipReason;
    /// use subscan::enums::module::SubscanModuleStatus;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let skipped: SubscanModuleStatus = SkipReason::NotAuthenticated.into();
    ///     let started = SubscanModuleStatus::Started;
    ///     let failed = SubscanModuleStatus::Failed("foo".into());
    ///
    ///     assert_eq!(skipped.as_log().await, "[not authenticated SKIPPED]");
    ///     assert_eq!(started.as_log().await, "[STARTED]");
    ///     assert_eq!(failed.as_log().await, "[foo FAILED]");
    /// }
    /// ```
    pub async fn as_log(&self) -> String {
        match self {
            SubscanModuleStatus::Skipped(reason) => {
                format!("[{} {}]", reason, self)
            }
            SubscanModuleStatus::Started | SubscanModuleStatus::Finished => {
                format!("[{}]", self)
            }
            SubscanModuleStatus::Failed(reason) => {
                format!("[{} {}]", reason, self)
            }
        }
    }
}
