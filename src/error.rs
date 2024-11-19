use crate::types::result::{
    module::SubscanModuleResult,
    statistics::SubscanModuleStatistics,
    status::{SkipReason, SubscanModuleStatus},
};
use chrono::{TimeDelta, Utc};
use std::fmt::Display;

/// Subscan error variants
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum SubscanError {
    /// Module error, see [`ModuleErrorKind`] for generic error definitions
    ModuleError(ModuleErrorKind),
    /// This error type uses for the make graceful returns from module `.run(`
    /// method. If the module has already discovered a subdomains and encountered
    /// an error during runtime we need to save already discovered subdomains. So
    /// implemented this error type to ensure this
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
    /// Get [`SubscanModuleStatus`] type
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::error::{SubscanError, ModuleErrorKind::Custom};
    /// use subscan::types::result::{
    ///     status::SubscanModuleStatus::FailedWithResult,
    ///     module::SubscanModuleResult
    /// };
    ///
    /// let result = SubscanModuleResult::default();
    ///
    /// let failed = SubscanError::from(Custom("foo".into()));
    /// let failed_with_result = SubscanError::ModuleErrorWithResult(result);
    ///
    /// assert_eq!(failed.status(), Custom("foo".into()).into());
    /// assert_eq!(failed_with_result.status(), FailedWithResult);
    ///
    /// assert_eq!(format!("{failed}"), "foo");
    /// assert_eq!(format!("{failed_with_result}"), "failed with result");
    /// ```
    pub fn status(&self) -> SubscanModuleStatus {
        match self {
            SubscanError::ModuleError(kind) => kind.status(),
            SubscanError::ModuleErrorWithResult(_) => SubscanModuleStatus::FailedWithResult,
        }
    }

    /// Get [`SubscanModuleStatistics`] from any [`SubscanError`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::error::{SubscanError, ModuleErrorKind::Custom};
    /// use subscan::types::result::{
    ///     status::SkipReason::SkippedByUser,
    ///     module::SubscanModuleResult
    /// };
    ///
    /// let failed = SubscanError::from(SkippedByUser);
    /// let stats = failed.stats("foo");
    ///
    /// assert_eq!(stats.module, "foo");
    /// assert_eq!(stats.status, SkippedByUser.into());
    /// assert_eq!(stats.count, 0);
    /// assert_eq!(stats.elapsed.num_seconds(), 0);
    /// ```
    pub fn stats(&self, module: &str) -> SubscanModuleStatistics {
        SubscanModuleStatistics {
            module: module.to_string(),
            status: self.status(),
            count: 0,
            started_at: Utc::now(),
            finished_at: Utc::now(),
            elapsed: TimeDelta::zero(),
        }
    }
}

/// Kind of [`SubscanError::ModuleError`]
#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum ModuleErrorKind {
    /// Indicates an error when extracting subdomains from any HTML content
    HTMLExtract,
    /// Indicates an error when extracting subdomains from any JSON content
    JSONExtract,
    /// Indicates an error when extracting subdomains by using regex pattern
    RegexExtract,
    /// Indicates an error when getting content from URL
    GetContent,
    /// Indicates that the module was skipped for any [`SkipReason`]
    Skip(SkipReason),
    /// Indicates that the module encountered a error with a custom error message
    Custom(String),
}

impl ModuleErrorKind {
    /// Wrap [`ModuleErrorKind`] with a [`SubscanModuleStatus`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::error::ModuleErrorKind;
    /// use subscan::types::result::status::{
    ///     SkipReason::SkippedByUser,
    ///     SubscanModuleStatus::{Failed, Skipped},
    /// };
    ///
    /// let failed = ModuleErrorKind::RegexExtract;
    /// let skipped = ModuleErrorKind::from(SkippedByUser);
    /// let custom = ModuleErrorKind::Custom("foo".into());
    ///
    /// assert_eq!(failed.status(), Failed(failed.clone()));
    /// assert_eq!(skipped.status(), SkippedByUser.into());
    /// assert_eq!(custom.status(), Failed(custom));
    /// ```
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

    /// Return [`ModuleErrorKind`] type with a error message
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::error::ModuleErrorKind;
    /// use subscan::types::result::status::{SkipReason::SkippedByUser};
    ///
    /// let html = ModuleErrorKind::HTMLExtract;
    /// let json = ModuleErrorKind::JSONExtract;
    /// let regex = ModuleErrorKind::RegexExtract;
    /// let skipped = ModuleErrorKind::from(SkippedByUser);
    /// let custom = ModuleErrorKind::Custom("foo".into());
    ///
    /// assert_eq!(html.with_msg(), "[html extract error FAILED]");
    /// assert_eq!(json.with_msg(), "[json extract error FAILED]");
    /// assert_eq!(regex.with_msg(), "[regex extract error FAILED]");
    /// assert_eq!(skipped.with_msg(), "[skipped by user SKIPPED]");
    /// assert_eq!(custom.with_msg(), "[foo FAILED]");
    /// ```
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
