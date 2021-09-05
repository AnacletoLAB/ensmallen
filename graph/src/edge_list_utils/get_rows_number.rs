use crate::{CSVFileReader, Result};

/// Return number of rows in given CSV path.
///
/// # Arguments
/// * `file_path`: &str - The path from where to load the original CSV.
///
/// # Raises
/// * If there are problems with opening the file.
///
pub fn get_rows_number(file_path: &str) -> Result<usize> {
    CSVFileReader::new(file_path, "csv to index".to_string())?.count_rows()
}
