use crate::types::result::{
    module::SubscanModuleResult,
    statistics::SubscanModuleStatistics,
    status::{SkipReason, SubscanModuleStatus},
};
use chrono::{TimeDelta, Utc};
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
