use crate::{enums::dispatchers::SubscanModuleDispatcher, utilities::cli};
use clap::Args;
use std::io::Write;
use tokio::sync::Mutex;

/// Get command to fetch any module
#[derive(Args, Clone, Debug)]
pub struct ModuleGetSubCommandArgs {
    /// Module name to be fetched
    pub name: String,
}

impl ModuleGetSubCommandArgs {
    pub async fn as_table<W: Write>(&self, module: &Mutex<SubscanModuleDispatcher>, out: &mut W) {
        let mut table = cli::create_module_table().await;

        table.add_row(module.lock().await.as_table_row().await);
        table.print(out).unwrap();
    }
}
