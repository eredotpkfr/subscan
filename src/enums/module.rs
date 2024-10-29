use serde::Serialize;
use std::fmt::Display;

/// Module skip reasons
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize)]
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

impl Serialize for SubscanModuleStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
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
    ///     assert_eq!(skipped.with_reason().await, "[not authenticated SKIPPED]");
    ///     assert_eq!(started.with_reason().await, "[STARTED]");
    ///     assert_eq!(failed.with_reason().await, "[foo FAILED]");
    /// }
    /// ```
    pub async fn with_reason(&self) -> String {
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
