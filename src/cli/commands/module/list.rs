use crate::{enums::SubscanModuleDispatcher, utils::cli};
use clap::Args;
use tokio::sync::Mutex;

/// List command to list implemented modules
#[derive(Args, Clone, Debug)]
pub struct ModuleListSubCommandArgs {}

impl ModuleListSubCommandArgs {
    pub async fn as_table(&self, modules: &Vec<Mutex<SubscanModuleDispatcher>>) {
        let mut table = cli::create_module_table().await;

        for module in modules {
            table.add_row(cli::module_as_table_row(&*module.lock().await).await);
        }

        table.printstd();
    }
}
