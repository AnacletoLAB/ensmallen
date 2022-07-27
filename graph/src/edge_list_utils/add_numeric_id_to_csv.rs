use crate::{utils::ItersWrapper, CSVFileReader, CSVFileWriter, Result};

/// Create a new CSV with the lines number added to it.
///
/// # Arguments
/// * `original_csv_path`: &str - The path from where to load the original CSV.
/// * `original_csv_separator`: Option<char> - Separator to use for the original CSV.
/// * `original_csv_header`: Option<bool> - Whether the original CSV has an header.
/// * `target_csv_path`: &str - The path from where to load the target CSV. This cannot be the same as the original CSV.
/// * `target_csv_separator`: Option<char> - Separator to use for the target CSV. If None, the one provided from the original CSV will be used.
/// * `target_csv_header`: Option<bool> - Whether the target CSV has an header. If None, the one provided from the original CSV will be used.
/// * `target_csv_ids_column`: Option<String> - The column name to use for the ids in the target list.
/// * `target_csv_ids_column_number`: Option<usize> - The column number to use for the ids in the target list.
/// * `comment_symbol`: Option<String> - The comment symbol to use within the original CSV.
/// * `support_balanced_quotes`: Option<bool> - Whether to support balanced quotes.
/// * `max_rows_number`: Option<usize> - The amount of rows to load from the original CSV.
/// * `rows_to_skip`: Option<usize> - The amount of rows to skip from the original CSV.
/// * `verbose`: Option<bool> - Whether to show the loading bar while processing the file.
///
/// # Raises
/// * If there are problems with opening the original or target file.
/// * If the original and target paths are identical.
///
/// TODO! add check for space on disk where possible.
pub fn add_numeric_id_to_csv(
    original_csv_path: &str,
    target_csv_path: &str,
    original_csv_separator: Option<char>,
    original_csv_header: Option<bool>,
    target_csv_separator: Option<char>,
    target_csv_header: Option<bool>,
    target_csv_ids_column: Option<String>,
    target_csv_ids_column_number: Option<usize>,
    comment_symbol: Option<String>,
    support_balanced_quotes: Option<bool>,
    max_rows_number: Option<usize>,
    rows_to_skip: Option<usize>,
    lines_number: Option<usize>,
    verbose: Option<bool>,
) -> Result<usize> {
    let target_csv_ids_column_number = target_csv_ids_column_number.unwrap_or(0);
    let target_csv_ids_column = target_csv_ids_column.unwrap_or("index".to_string());
    if original_csv_path == target_csv_path {
        return Err(concat!(
            "Both the original and the target CSV path ",
            "are set to the same path.\n",
            "It is not possible to write this file inplace, ",
            "as each line would be slightly longer ",
            "than the pre-existing one and would overwrite ",
            "a part of the successive line."
        )
        .to_string());
    }

    let file_reader = CSVFileReader::new(original_csv_path, "csv to index".to_string())?
        .set_comment_symbol(comment_symbol)?
        .set_rows_to_skip(rows_to_skip)?
        .set_max_rows_number(max_rows_number)?
        .set_header(original_csv_header)?
        .set_separator(original_csv_separator)?
        .set_support_balanced_quotes(support_balanced_quotes)
        .set_parallel(Some(false));

    let file_writer = CSVFileWriter::new(target_csv_path)
        .set_separator(target_csv_separator.or(Some(file_reader.get_separator())))?
        .set_verbose(verbose)
        .set_header(target_csv_header.or(Some(file_reader.has_header())));
    let lines_iterator = file_reader.read_lines(None)?;
    let lines_iterator = match lines_iterator {
        ItersWrapper::Parallel(_) => unreachable!("This is not meant to run in parallel."),
        ItersWrapper::Sequential(i) => i,
    };
    let mut header = file_reader.get_header()?;
    header.insert(target_csv_ids_column_number, target_csv_ids_column);
    let mut effective_lines_number = 0;
    file_writer.write_lines(
        lines_number,
        header,
        lines_iterator
            // Removing eventual errors.
            .filter_map(|line| line.ok())
            .enumerate()
            // Processing line
            .map(|(line_number, (_, mut line))| {
                effective_lines_number += 1;
                line.insert(target_csv_ids_column_number, Some(line_number.to_string()));
                line.into_iter()
                    .map(|value| value.unwrap_or("".to_string()))
                    .collect()
            }),
    )?;
    Ok(effective_lines_number)
}
