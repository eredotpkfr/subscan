use clap::Args;

/// Get command to fetch any module
#[derive(Args, Clone, Debug)]
pub struct ModuleGetSubCommandArgs {
    /// Module name to be fetched
    name: String,
}
