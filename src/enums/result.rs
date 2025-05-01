use crate::{
    error::ModuleErrorKind,
    types::{
        core::Subdomain,
        result::{
            item::SubscanModuleResultItem,
            status::{SkipReason, SubscanModuleStatus},
        },
    },
};

/// Subscan module result variants
#[derive(Debug, PartialEq)]
pub enum SubscanModuleResult {
    SubscanModuleResultItem(SubscanModuleResultItem),
    SubscanModuleStatus(SubscanModuleStatus),
}

impl From<(&str, &Subdomain)> for SubscanModuleResult {
    fn from(values: (&str, &Subdomain)) -> Self {
        Self::SubscanModuleResultItem(values.into())
    }
}

impl From<SubscanModuleStatus> for Option<SubscanModuleResult> {
    fn from(status: SubscanModuleStatus) -> Self {
        Some(SubscanModuleResult::SubscanModuleStatus(status))
    }
}

impl From<SkipReason> for Option<SubscanModuleResult> {
    fn from(reason: SkipReason) -> Self {
        Some(SubscanModuleResult::SubscanModuleStatus(reason.into()))
    }
}

impl From<ModuleErrorKind> for Option<SubscanModuleResult> {
    fn from(kind: ModuleErrorKind) -> Self {
        Some(SubscanModuleResult::SubscanModuleStatus(kind.status()))
    }
}
