use derive_more::Deref;

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
#[derive(Clone, Debug, PartialEq)]
pub enum SubscanModuleResult {
    SubscanModuleResultItem(SubscanModuleResultItem),
    SubscanModuleStatus(SubscanModuleStatus),
}

/// Optional subscan module result type
#[derive(Clone, Debug, Deref)]
pub struct OptionalSubscanModuleResult(pub Option<SubscanModuleResult>);

impl From<(&str, &Subdomain)> for OptionalSubscanModuleResult {
    fn from(values: (&str, &Subdomain)) -> Self {
        Self(Some(SubscanModuleResult::SubscanModuleResultItem(
            values.into(),
        )))
    }
}

impl From<&str> for OptionalSubscanModuleResult {
    fn from(err_msg: &str) -> Self {
        Self(Some(SubscanModuleResult::SubscanModuleStatus(
            ModuleErrorKind::Custom(err_msg.to_owned()).into(),
        )))
    }
}

impl From<SubscanModuleStatus> for OptionalSubscanModuleResult {
    fn from(status: SubscanModuleStatus) -> Self {
        Self(Some(SubscanModuleResult::SubscanModuleStatus(status)))
    }
}

impl From<SkipReason> for OptionalSubscanModuleResult {
    fn from(reason: SkipReason) -> Self {
        Self(Some(SubscanModuleResult::SubscanModuleStatus(
            reason.into(),
        )))
    }
}

impl From<ModuleErrorKind> for OptionalSubscanModuleResult {
    fn from(kind: ModuleErrorKind) -> Self {
        Self(Some(SubscanModuleResult::SubscanModuleStatus(
            kind.status(),
        )))
    }
}
