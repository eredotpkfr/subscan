use derive_more::Deref;

use crate::types::{
    core::Subdomain,
    result::{
        item::{SubscanModuleResultItem, SubscanModuleStatusItem},
        status::SubscanModuleStatus,
    },
};

/// Subscan module result variants
#[derive(Clone, Debug, PartialEq)]
pub enum SubscanModuleResult {
    SubscanModuleResultItem(SubscanModuleResultItem),
    SubscanModuleStatusItem(SubscanModuleStatusItem),
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

impl From<(&str, SubscanModuleStatus)> for OptionalSubscanModuleResult {
    fn from(values: (&str, SubscanModuleStatus)) -> Self {
        Self(Some(SubscanModuleResult::SubscanModuleStatusItem(
            values.into(),
        )))
    }
}

impl From<(&str, &str)> for OptionalSubscanModuleResult {
    fn from(values: (&str, &str)) -> Self {
        Self(Some(SubscanModuleResult::SubscanModuleStatusItem(
            values.into(),
        )))
    }
}
