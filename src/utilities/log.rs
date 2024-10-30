use crate::{enums::module::SubscanModuleStatus, types::result::module::SubscanModuleResult};
use colored::Colorize;

/// Log any [`SubscanModuleResult`] object by its status
pub async fn result(result: SubscanModuleResult) {
    match result.status {
        SubscanModuleStatus::Started => {
            log::info!(
                "{:.<25}{:.>35}",
                result.module.white(),
                result.status.with_reason().await.white()
            )
        }
        SubscanModuleStatus::Finished => {
            log::info!(
                "{:.<25}{:.>35}",
                result.module.white(),
                result.status.with_reason().await.white()
            )
        }
        SubscanModuleStatus::Skipped(_) => {
            log::warn!(
                "{:.<25}{:.>35}",
                result.module.yellow(),
                result.status.with_reason().await.yellow()
            )
        }
        SubscanModuleStatus::Failed(_) => {
            log::error!(
                "{:.<25}{:.>35}",
                result.module.red(),
                result.status.with_reason().await.red()
            )
        }
    }
}

/// Log any status with any module
pub async fn status(module: &str, status: SubscanModuleStatus) {
    match status {
        SubscanModuleStatus::Skipped(_) => log::warn!(
            "{:.<25}{:.>35}",
            module.yellow(),
            status.with_reason().await.yellow()
        ),
        SubscanModuleStatus::Started => log::info!(
            "{:.<25}{:.>35}",
            module.white(),
            status.with_reason().await.white()
        ),
        SubscanModuleStatus::Finished => log::info!(
            "{:.<25}{:.>35}",
            module.white(),
            status.with_reason().await.white()
        ),
        SubscanModuleStatus::Failed(_) => log::error!(
            "{:.<25}{:.>35}",
            module.red(),
            status.with_reason().await.red()
        ),
    }
}
