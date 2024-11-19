use std::io::Write;

use clap::Args;

use crate::{types::core::SubscanModule, utilities::cli};

/// List command to list implemented modules
#[derive(Args, Clone, Debug)]
pub struct ModuleListSubCommandArgs {}

impl ModuleListSubCommandArgs {
    pub async fn as_table<W: Write>(&self, modules: &Vec<SubscanModule>, out: &mut W) {
        let mut table = cli::create_module_table().await;

        for module in modules {
            table.add_row(module.lock().await.as_table_row().await);
        }

        table.print(out).unwrap();
    }
}
