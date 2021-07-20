use crate::{EdgeFileReader, Result};

/// Sort given numeric edge list in place using the sort command.
///
/// # Implementative details
/// Note that the header and the rows to skip in general will
/// be removed from the file before being sorted, hence they will
/// not appear in the sorted result.
///
/// # Arguments
/// * `path`: &str - The path from where to load the edge list.
/// * `target_path`: &str - The where to store the edge list.
/// * `separator`: Option<String> - The separator for the rows in the edge list.
/// * `header`: Option<bool> - Whether the edge list has an header.
/// * `sources_column`: Option<String> - The column name to use for the source nodes.
/// * `sources_column_number`: Option<usize> - The column number to use for the source nodes.
/// * `destinations_column`: Option<String> - The column name to use for the destination nodes.
/// * `destinations_column_number`: Option<usize> - The column number to use for the destination nodes.
/// * `edge_types_column`: Option<String> - The column name to use for the edge types.
/// * `edge_types_column_number`: Option<usize> - The column number to use for the edge types.
/// * `rows_to_skip`: Option<usize> - Number of rows to skip in the edge list.
/// * `skip_edge_types_if_unavailable`: Option<bool> - Whether to automatically skip the edge types if they are not available.
///
pub fn sort_numeric_edge_list(
    path: &str,
    target_path: &str,
    separator: Option<String>,
    header: Option<bool>,
    sources_column: Option<String>,
    sources_column_number: Option<usize>,
    destinations_column: Option<String>,
    destinations_column_number: Option<usize>,
    edge_types_column: Option<String>,
    edge_types_column_number: Option<usize>,
    rows_to_skip: Option<usize>,
    skip_edge_types_if_unavailable: Option<bool>,
) -> Result<()> {
    if cfg!(target_os = "windows") {
        return Err(concat!(
            "This utility is based on the sort command, which is ",
            "not available on Windows, but only macOS and Linux. ",
        )
        .to_string());
    }

    let file_reader = EdgeFileReader::new(path)?
        .set_separator(separator)?
        .set_destinations_column(destinations_column)?
        .set_destinations_column_number(destinations_column_number)?
        .set_sources_column(sources_column)?
        .set_sources_column_number(sources_column_number)?
        .set_skip_edge_types_if_unavailable(skip_edge_types_if_unavailable)
        .set_edge_types_column(edge_types_column)?
        .set_edge_types_column_number(edge_types_column_number)?
        .set_rows_to_skip(rows_to_skip)
        .set_header(header);

    let sed_command_status = std::process::Command::new("sed")
        .args(&[
            format!("1,{}d", file_reader.get_total_lines_to_skip(true)? + 1,).as_ref(),
            path,
        ])
        .stdout(std::process::Stdio::piped())
        .spawn();

    // We check if the operation went fine.
    let sed_command = match sed_command_status {
        Ok(command) => Ok(command),
        Err(_) => Err(concat!(
            "Could not execute sed to skip headers ",
            "before sorting inplace on the provided edge list."
        )
        .to_owned()),
    }?;

    let arguments = vec![
        // We specify the separator of the fields
        format!("--field-separator={}", file_reader.get_separator()),
        // We want to sort over the source and destination columns
        // and also the edge types if they are available, in order
        // to avoid dropping duplicate-like lines that are actually
        // edges from a multigraph.
        format!(
            "--key={key},{key}",
            key = file_reader.get_sources_column_number() + 1
        ),
        format!(
            "--key={key},{key}",
            key = file_reader.get_destinations_column_number() + 1
        ),
        if let Some(edge_types_column) = file_reader.get_edge_types_column_number() {
            format!("--key={key},{key}", key = edge_types_column + 1)
        } else {
            "".to_owned()
        },
        // The values in the keys are numeric
        "--numeric-sort".to_owned(),
        // We want to sort the file inplace
        format!("--output={}", target_path),
    ]
    .into_iter()
    .filter(|arg| !arg.is_empty())
    .collect::<Vec<String>>();

    let sort_command_status = std::process::Command::new("sort")
        .args(arguments)
        .stdin(sed_command.stdout.unwrap())
        .status();

    // We check if the operation went fine.
    match sort_command_status {
        Ok(_) => Ok(()),
        Err(_) => Err("Could not execute sort inplace on the provided edge list.".to_owned()),
    }
}

/// Sort given numeric edge list in place using the sort command.
///
/// # Implementative details
/// Note that the header and the rows to skip in general will
/// be removed from the file before being sorted, hence they will
/// not appear in the sorted result.
///
/// # Arguments
/// * `path`: &str - The path from where to load the edge list.
/// * `separator`: Option<String> - The separator for the rows in the edge list.
/// * `header`: Option<bool> - Whether the edge list has an header.
/// * `sources_column`: Option<String> - The column name to use for the source nodes.
/// * `sources_column_number`: Option<usize> - The column number to use for the source nodes.
/// * `destinations_column`: Option<String> - The column name to use for the destination nodes.
/// * `destinations_column_number`: Option<usize> - The column number to use for the destination nodes.
/// * `edge_types_column`: Option<String> - The column name to use for the edge types.
/// * `edge_types_column_number`: Option<usize> - The column number to use for the edge types.
/// * `rows_to_skip`: Option<usize> - Number of rows to skip in the edge list.
/// * `skip_edge_types_if_unavailable`: Option<bool> - Whether to automatically skip the edge types if they are not available.
///
pub fn sort_numeric_edge_list_inplace(
    path: &str,
    separator: Option<String>,
    header: Option<bool>,
    sources_column: Option<String>,
    sources_column_number: Option<usize>,
    destinations_column: Option<String>,
    destinations_column_number: Option<usize>,
    edge_types_column: Option<String>,
    edge_types_column_number: Option<usize>,
    rows_to_skip: Option<usize>,
    skip_edge_types_if_unavailable: Option<bool>,
) -> Result<()> {
    sort_numeric_edge_list(
        path,
        path,
        separator,
        header,
        sources_column,
        sources_column_number,
        destinations_column,
        destinations_column_number,
        edge_types_column,
        edge_types_column_number,
        rows_to_skip,
        skip_edge_types_if_unavailable,
    )
}
