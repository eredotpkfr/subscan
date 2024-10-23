use crate::{enums::SubscanModuleDispatcher, utils::cli};
use clap::Args;
use tokio::sync::Mutex;

/// Get command to fetch any module
#[derive(Args, Clone, Debug)]
pub struct ModuleGetSubCommandArgs {
    /// Module name to be fetched
    pub name: String,
}

impl ModuleGetSubCommandArgs {
    pub async fn as_table(&self, module: &Mutex<SubscanModuleDispatcher>) {
        let mut table = cli::create_module_table().await;

        table.add_row(cli::module_as_table_row(&*module.lock().await).await);

        table.printstd();
    }
}
