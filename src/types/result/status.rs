use crate::error::ModuleErrorKind;
use colored::Colorize;
use serde::Serialize;
use std::fmt::Display;

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub enum SubscanModuleStatus {
    #[default]
    Started,
    Finished,
    Skipped(SkipReason),
    Failed(ModuleErrorKind),
    FailedWithResult,
}

impl Display for SubscanModuleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubscanModuleStatus::Started => write!(f, "STARTED"),
            SubscanModuleStatus::Finished => write!(f, "FINISHED"),
            SubscanModuleStatus::Failed(_) | SubscanModuleStatus::FailedWithResult => {
                write!(f, "FAILED")
            }
            SubscanModuleStatus::Skipped(_) => write!(f, "SKIPPED"),
        }
    }
}

impl From<SkipReason> for SubscanModuleStatus {
    fn from(reason: SkipReason) -> Self {
        Self::Skipped(reason)
    }
}

impl Serialize for SubscanModuleStatus {
    /// Serialize object to string for JSON outputs
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::status::SubscanModuleStatus;
    /// use serde_json::json;
    ///
    /// let json = json!({
    ///     "status": SubscanModuleStatus::Finished,
    /// });
    ///
    /// assert_eq!(serde_json::to_string(&json).unwrap(), "{\"status\":\"FINISHED\"}");
    /// ```
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl SubscanModuleStatus {
    pub fn with_reason(&self) -> String {
        match self {
            SubscanModuleStatus::Started => format!("[{self}]"),
            SubscanModuleStatus::Finished => format!("[{self}]"),
            SubscanModuleStatus::Failed(err) => format!("[{err} {self}]"),
            SubscanModuleStatus::FailedWithResult => format!("[failed with result {self}]"),
            SubscanModuleStatus::Skipped(reason) => format!("[{reason} {self}]"),
        }
    }

    pub fn log(&self, module: &str) {
        match self {
            SubscanModuleStatus::Started | SubscanModuleStatus::Finished => {
                log::info!("{:.<25}{:.>35}", module.white(), self.with_reason().white())
            }
            SubscanModuleStatus::Skipped(_) => log::warn!(
                "{:.<25}{:.>35}",
                module.yellow(),
                self.with_reason().yellow()
            ),
            SubscanModuleStatus::Failed(err) => {
                log::error!("{:.<25}{:.>35}", module.red(), err.with_msg().red())
            }
            SubscanModuleStatus::FailedWithResult => log::warn!(
                "{:.<25}{:.>35}",
                module.yellow(),
                self.with_reason().yellow()
            ),
        }
    }
}

/// Module skip reasons
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize)]
pub enum SkipReason {
    /// Indicates that if authentication requires by module but API key, HTTP credentials
    /// or other any authentication method not provided
    AuthenticationNotProvided,
    /// Indicates that the module skipped by user
    SkippedByUser,
}

impl Display for SkipReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SkipReason::AuthenticationNotProvided => write!(f, "auth not provided"),
            SkipReason::SkippedByUser => write!(f, "skipped by user"),
        }
    }
}
