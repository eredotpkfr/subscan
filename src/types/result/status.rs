use colored::Colorize;
use derive_more::{Display, From};
use serde::Serialize;

use crate::error::ModuleErrorKind;

/// Subscan module states
#[derive(Clone, Debug, Default, Display, Eq, From, Ord, PartialEq, PartialOrd)]
pub enum SubscanModuleStatus {
    #[default]
    #[display("STARTED")]
    Started,
    #[display("FINISHED")]
    Finished,
    #[from]
    #[display("SKIPPED")]
    Skipped(SkipReason),
    #[from]
    #[display("FAILED")]
    Failed(ModuleErrorKind),
    #[display("FAILED")]
    FailedWithResult,
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
    /// Return status with a reason text
    ///
    /// # Examples
    ///
    /// ```
    /// use subscan::types::result::status::{
    ///     SubscanModuleStatus::{
    ///         Started,
    ///         Finished,
    ///         Failed,
    ///         FailedWithResult,
    ///         Skipped
    ///     },
    ///     SkipReason::AuthenticationNotProvided,
    /// };
    /// use subscan::error::ModuleErrorKind::Custom;
    ///
    /// assert_eq!(Started.with_reason(), "[STARTED]");
    /// assert_eq!(Finished.with_reason(), "[FINISHED]");
    ///
    /// assert_eq!(
    ///     Failed(Custom("foo".into())).with_reason(),
    ///     "[foo FAILED]"
    /// );
    /// assert_eq!(
    ///     FailedWithResult.with_reason(),
    ///     "[failed with result FAILED]"
    /// );
    /// assert_eq!(
    ///     Skipped(AuthenticationNotProvided.into()).with_reason(),
    ///     "[auth not provided SKIPPED]"
    /// );
    /// ```
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
#[derive(Clone, Debug, Display, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub enum SkipReason {
    /// Indicates that if authentication requires by module but API key, HTTP credentials
    /// or other any authentication method not provided
    #[display("auth not provided")]
    AuthenticationNotProvided,
    /// Indicates that the module skipped by user
    #[display("skipped by user")]
    SkippedByUser,
}
