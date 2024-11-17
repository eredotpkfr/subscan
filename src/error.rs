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
    pub fn status(&self) -> SubscanModuleStatus {
        match self {
            SubscanError::ModuleError(kind) => kind.status(),
            SubscanError::ModuleErrorWithResult(_) => SubscanModuleStatus::Failed,
        }
    }

    pub fn with_msg(&self) -> String {
        format!("[{self} {}]", self.status())
    }

    pub async fn stats(&self, module: &str) -> SubscanModuleStatistics {
        SubscanModuleStatistics {
            module: module.to_string(),
            status: self.status(),
            count: 0,
            started_at: Utc::now(),
            finished_at: Utc::now(),
            elapsed: TimeDelta::zero(),
        }
    }

    pub async fn log(&self, module: &str) {
        match self {
            SubscanError::ModuleError(kind) => kind.log(module),
            SubscanError::ModuleErrorWithResult(_) => {
                log::warn!("{:.<25}{:.>35}", module.yellow(), self.with_msg().yellow())
            }
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
pub enum SubscanModuleStatus {
    #[default]
    Started,
    Finished,
    Failed,
    FailedWithResult,
    Skipped(SkipReason),
}

impl Display for SubscanModuleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubscanModuleStatus::Started => write!(f, "STARTED"),
            SubscanModuleStatus::Finished => write!(f, "FINISHED"),
            SubscanModuleStatus::Failed | SubscanModuleStatus::FailedWithResult => {
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
            SubscanModuleStatus::Failed => format!("[{self}]"),
            SubscanModuleStatus::FailedWithResult => format!("[failed with result {self}]"),
            SubscanModuleStatus::Skipped(reason) => format!("[{reason} {self}]"),
        }
    }

    pub async fn log(&self, module: &str) {
        match self {
            SubscanModuleStatus::Started => {
                log::info!("{:.<25}{:.>35}", module.white(), self.with_reason().white())
            }
            SubscanModuleStatus::Finished => {
                log::info!("{:.<25}{:.>35}", module.white(), self.with_reason().white())
            }
            SubscanModuleStatus::FailedWithResult => log::warn!(
                "{:.<25}{:.>35}",
                module.yellow(),
                self.with_reason().yellow()
            ),
            SubscanModuleStatus::Failed => {
                log::error!("{:.<25}{:.>35}", module.red(), self.with_reason().red())
            }
            SubscanModuleStatus::Skipped(_) => log::warn!(
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

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum ModuleErrorKind {
    HTMLExtractError,
    JSONExtractError,
    RegexExtractError,
    GetContentError,
    SkipError(SkipReason),
    CustomError(String),
}

impl From<SkipReason> for ModuleErrorKind {
    fn from(reason: SkipReason) -> Self {
        Self::SkipError(reason)
    }
}

impl Display for ModuleErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleErrorKind::HTMLExtractError => write!(f, "html extract error"),
            ModuleErrorKind::JSONExtractError => write!(f, "json extract error"),
            ModuleErrorKind::RegexExtractError => write!(f, "regex extract error"),
            ModuleErrorKind::GetContentError => write!(f, "get content error"),
            ModuleErrorKind::CustomError(msg) => write!(f, "{msg}"),
            ModuleErrorKind::SkipError(reason) => write!(f, "{reason}"),
        }
    }
}

impl ModuleErrorKind {
    pub fn status(&self) -> SubscanModuleStatus {
        match self {
            ModuleErrorKind::HTMLExtractError
            | ModuleErrorKind::JSONExtractError
            | ModuleErrorKind::RegexExtractError
            | ModuleErrorKind::GetContentError => SubscanModuleStatus::Failed,
            ModuleErrorKind::SkipError(reason) => SubscanModuleStatus::Skipped(reason.clone()),
            ModuleErrorKind::CustomError(_) => SubscanModuleStatus::Failed,
        }
    }

    pub fn with_msg(&self) -> String {
        format!("[{self} {}]", self.status())
    }

    pub fn log(&self, module: &str) {
        match self {
            ModuleErrorKind::HTMLExtractError
            | ModuleErrorKind::JSONExtractError
            | ModuleErrorKind::RegexExtractError
            | ModuleErrorKind::GetContentError => {
                log::error!("{:.<25}{:.>35}", module.red(), self.with_msg().red())
            }
            ModuleErrorKind::SkipError(_) => {
                log::warn!("{:.<25}{:.>35}", module.yellow(), self.with_msg().yellow())
            }
            ModuleErrorKind::CustomError(_) => {
                log::error!("{:.<25}{:.>35}", module.red(), self.with_msg().red())
            }
        }
    }
}
