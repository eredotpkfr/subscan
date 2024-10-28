use crate::{
    enums::dispatchers::SubscanModuleDispatcher, interfaces::module::SubscanModuleInterface,
};
use prettytable::{format::consts::FORMAT_NO_LINESEP_WITH_TITLE, row, Row, Table};

/// Creates table for module representation
///
/// # Examples
///
/// ```
/// use subscan::utilities::cli;
///
/// #[tokio::main]
/// async fn main() {
///     let table = cli::create_module_table().await;
///
///     assert!(table.is_empty());
/// }
/// ```
pub async fn create_module_table() -> Table {
    let mut table = Table::new();

    let titles = row![
        FdBwbd -> "Name",
        FdBwbd -> "Requester",
        FdBwbd -> "Extractor",
        FdBwbd -> "Is Generic?",
    ];

    table.set_format(*FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(titles);
    table
}

/// Converts module object to module table row representation
///
/// # Examples
///
/// ```
/// use subscan::utilities::cli;
/// use subscan::modules::engines::google::Google;
///
/// #[tokio::main]
/// async fn main() {
///     let module = Google::dispatcher();
///     let mut table = cli::create_module_table().await;
///
///     table.add_row(cli::module_as_table_row(&module).await);
///
///     assert!(!table.is_empty());
/// }
/// ```
pub async fn module_as_table_row(module: &SubscanModuleDispatcher) -> Row {
    let requester = if let Some(instance) = module.requester().await {
        instance.lock().await.to_string()
    } else {
        "None".into()
    };

    let extractor = if let Some(instance) = module.extractor().await {
        instance.to_string()
    } else {
        "None".into()
    };

    row![
        Fw -> module.name().await,
        Fw -> requester,
        Fw -> extractor,
        Fw -> module.is_generic().await
    ]
}
