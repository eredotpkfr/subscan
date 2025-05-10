use chrono::{TimeDelta, Utc};
use derive_more::{Display, From};
use scraper::error::SelectorErrorKind;

use crate::types::result::{
    statistics::SubscanModuleStatistic,
    status::{SkipReason, SubscanModuleStatus},
};

/// Subscan error variants
#[derive(Clone, Debug, Display, Eq, From, Ord, PartialEq, PartialOrd)]
pub enum SubscanError {
    /// Module error, see [`ModuleErrorKind`] for generic error definitions
    #[from(ModuleErrorKind, SkipReason, SelectorErrorKind<'_>, regex::Error, reqwest::Error, url::ParseError)]
    #[display("{_0}")]
    ModuleError(ModuleErrorKind),
}

impl SubscanError {
    /// Get [`SubscanModuleStatus`] type
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::error::{SubscanError, ModuleErrorKind::Custom};
    ///
    /// let failed = SubscanError::from(Custom("foo".into()));
    ///
    /// assert_eq!(failed.status(), Custom("foo".into()).into());
    /// assert_eq!(format!("{failed}"), "foo");
    /// ```
    pub fn status(&self) -> SubscanModuleStatus {
        match self {
            SubscanError::ModuleError(kind) => kind.status(),
        }
    }

    /// Get [`SubscanModuleStatistic`] from any [`SubscanError`]
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::error::{SubscanError, ModuleErrorKind::Custom};
    /// use subscan::types::result::status::SkipReason::SkippedByUser;
    ///
    /// let failed = SubscanError::from(SkippedByUser);
    /// let stats = failed.stats();
    ///
    /// assert_eq!(stats.status, SkippedByUser.into());
    /// assert_eq!(stats.count, 0);
    /// assert_eq!(stats.elapsed.num_seconds(), 0);
    /// ```
    pub fn stats(&self) -> SubscanModuleStatistic {
        SubscanModuleStatistic {
            status: self.status(),
            count: 0,
            started_at: Utc::now(),
            finished_at: Utc::now(),
            elapsed: TimeDelta::zero(),
        }
    }
}

/// Kind of [`SubscanError::ModuleError`]
#[derive(Clone, Debug, Display, Eq, From, Ord, PartialEq, PartialOrd)]
pub enum ModuleErrorKind {
    /// Indicates an error when extracting subdomains from any HTML content
    #[from(SelectorErrorKind<'_>)]
    #[display("html extract error")]
    HTMLExtract,
    /// Indicates an error when extracting subdomains from any JSON content
    #[display("json extract error")]
    JSONExtract,
    /// Indicates an error when extracting subdomains by using regex pattern
    #[from(regex::Error)]
    #[display("regex extract error")]
    RegexExtract,
    /// Indicates an error when getting content from URL
    #[from(reqwest::Error)]
    #[display("get content error")]
    GetContent,
    /// Indicates an error while parsing URL
    #[from(url::ParseError)]
    #[display("url parse error")]
    UrlParse,
    /// Indicates that the module was skipped for any [`SkipReason`]
    #[from]
    #[display("{_0}")]
    Skip(SkipReason),
    /// Indicates that the module encountered a error with a custom error message
    #[from(String, &str)]
    #[display("{_0}")]
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
            | ModuleErrorKind::UrlParse
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
    /// let url = ModuleErrorKind::UrlParse;
    /// let skipped = ModuleErrorKind::from(SkippedByUser);
    /// let custom = ModuleErrorKind::Custom("foo".into());
    ///
    /// assert_eq!(html.with_msg(), "[html extract error FAILED]");
    /// assert_eq!(json.with_msg(), "[json extract error FAILED]");
    /// assert_eq!(regex.with_msg(), "[regex extract error FAILED]");
    /// assert_eq!(url.with_msg(), "[url parse error FAILED]");
    /// assert_eq!(skipped.with_msg(), "[skipped by user SKIPPED]");
    /// assert_eq!(custom.with_msg(), "[foo FAILED]");
    /// ```
    pub fn with_msg(&self) -> String {
        match self {
            ModuleErrorKind::HTMLExtract
            | ModuleErrorKind::JSONExtract
            | ModuleErrorKind::RegexExtract
            | ModuleErrorKind::UrlParse
            | ModuleErrorKind::GetContent => format!("[{self} {}]", self.status()),
            ModuleErrorKind::Skip(reason) => format!("[{reason} {}]", self.status()),
            ModuleErrorKind::Custom(msg) => format!("[{msg} {}]", self.status()),
        }
    }
}
