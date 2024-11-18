use crate::types::result::{module::SubscanModuleResult, stats::SubscanModuleStatistics};
use chrono::{TimeDelta, Utc};
use colored::Colorize;
use serde::Serialize;
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum SubscanError {
    ModuleError(ModuleErrorKind),
    ModuleErrorWithResult(SubscanModuleResult),
}

impl Display for SubscanError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubscanError::ModuleError(kind) => write!(f, "{kind}"),
            SubscanError::ModuleErrorWithResult(_) => write!(f, "failed with result"),
        }
    }
}

impl From<ModuleErrorKind> for SubscanError {
    fn from(err: ModuleErrorKind) -> Self {
        Self::ModuleError(err)
    }
}

impl From<SkipReason> for SubscanError {
    fn from(reason: SkipReason) -> Self {
        Self::ModuleError(reason.into())
    }
}

impl SubscanError {
    pub async fn status(&self) -> SubscanModuleStatus {
        match self {
            SubscanError::ModuleError(kind) => kind.status(),
            SubscanError::ModuleErrorWithResult(_) => SubscanModuleStatus::FailedWithResult,
        }
    }

    pub async fn stats(&self, module: &str) -> SubscanModuleStatistics {
        SubscanModuleStatistics {
            module: module.to_string(),
            status: self.status().await,
            count: 0,
            started_at: Utc::now(),
            finished_at: Utc::now(),
            elapsed: TimeDelta::zero(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum ModuleErrorKind {
    HTMLExtract,
    JSONExtract,
    RegexExtract,
    GetContent,
    Skip(SkipReason),
    Custom(String),
}

impl ModuleErrorKind {
    pub fn status(&self) -> SubscanModuleStatus {
        match self {
            ModuleErrorKind::HTMLExtract
            | ModuleErrorKind::JSONExtract
            | ModuleErrorKind::RegexExtract
            | ModuleErrorKind::GetContent => SubscanModuleStatus::Failed(self.clone()),
            ModuleErrorKind::Skip(reason) => SubscanModuleStatus::Skipped(reason.clone()),
            ModuleErrorKind::Custom(_) => SubscanModuleStatus::Failed(self.clone()),
        }
    }

    pub fn with_msg(&self) -> String {
        match self {
            ModuleErrorKind::HTMLExtract
            | ModuleErrorKind::JSONExtract
            | ModuleErrorKind::RegexExtract
            | ModuleErrorKind::GetContent => format!("[{self} {}]", self.status()),
            ModuleErrorKind::Skip(reason) => format!("[{reason} {}]", self.status()),
            ModuleErrorKind::Custom(msg) => format!("[{msg} {}]", self.status()),
        }
    }
}

impl From<SkipReason> for ModuleErrorKind {
    fn from(reason: SkipReason) -> Self {
        Self::Skip(reason)
    }
}

impl Display for ModuleErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleErrorKind::HTMLExtract => write!(f, "html extract error"),
            ModuleErrorKind::JSONExtract => write!(f, "json extract error"),
            ModuleErrorKind::RegexExtract => write!(f, "regex extract error"),
            ModuleErrorKind::GetContent => write!(f, "get content error"),
            ModuleErrorKind::Custom(msg) => write!(f, "{msg}"),
            ModuleErrorKind::Skip(reason) => write!(f, "{reason}"),
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
    /// use subscan::error::SubscanModuleStatus;
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
                log::error!("{:.<25}{:.>35}", module.red(), err.with_msg().red(),)
            }
            SubscanModuleStatus::FailedWithResult => log::warn!(
                "{:.<25}{:.>35}",
                module.yellow(),
                self.with_reason().yellow()
            ),
        }
    }
}
