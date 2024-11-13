use prettytable::{format::consts::FORMAT_NO_LINESEP_WITH_TITLE, row, table, Table};

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
    let mut table = table!();

    let titles = row!["Name", "Requester", "Extractor", "Is Generic?"];

    table.set_format(*FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(titles);

    table
}

/// Creates table for module representation
///
/// # Examples
///
/// ```
/// use subscan::utilities::cli;
///
/// #[tokio::main]
/// async fn main() {
///     let table = cli::create_scan_result_item_table().await;
///
///     assert!(table.is_empty());
/// }
/// ```
pub async fn create_scan_result_item_table() -> Table {
    let mut table = table!();

    let titles = row!["Subdomain", "IP"];

    table.set_format(*FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(titles);

    table
}
