use super::*;
use csv::{ByteRecord, WriterBuilder};
use indicatif::ProgressIterator;

/// Structure that saves the common parameters for reading csv files.
///
/// # Attributes
/// * path: String - The path where to save the file. E.g. "/tmp/test.csv"
/// * `verbose`: bool - If the progress bars and logging must be displayed.
/// * separator: String - The separator to use, usually, this is "\t" for tsv and "," for csv.
/// * header: bool - If the file (will / must) have the header with the titles of the columns.
pub struct CSVFileWriter {
    pub(crate) path: String,
    pub(crate) verbose: bool,
    pub(crate) separator: String,
    pub(crate) header: bool,
}

/// # Builder methods
impl CSVFileWriter {
    /// Return new CSVFileWriter object.
    ///
    /// # Arguments
    ///
    /// * path: String - Path where to store/load the file.
    ///
    pub fn new<S: Into<String>>(path: S) -> CSVFileWriter {
        CSVFileWriter {
            path: path.into(),
            verbose: true,
            separator: "\t".to_string(),
            header: true,
        }
    }

    /// Write given rows iterator to file.
    ///
    /// # Arguments
    ///
    /// * `lines_number`: Option<usize> - Number of lines to expect to write out.
    /// * `header`: Vec<String> - The header to write out, if so required.
    /// * `values`: impl Iterator<Item = Vec<String>> - Iterator of rows to write out.
    pub(crate) fn write_lines(
        &self,
        lines_number: Option<usize>,
        header: Vec<String>,
        values: impl Iterator<Item = Vec<String>>,
    ) -> Result<()> {
        let pb = get_loading_bar(
            self.verbose && lines_number.is_some(),
            "Writing to file",
            lines_number.unwrap_or(0),
        );

        let mut stream = WriterBuilder::new()
            .delimiter((&self.separator).as_bytes()[0])
            .from_path(self.path.clone())
            .unwrap();

        if self.header {
            match stream.write_byte_record(&ByteRecord::from(&header[..])) {
                Ok(_) => Ok(()),
                Err(_) => {
                    Err("Cannot write the header. There might have been an I/O error.".to_string())
                }
            }?;
        }

        for (i, line) in values.progress_with(pb).enumerate() {
            match stream.write_byte_record(&ByteRecord::from(&line[..])) {
                Ok(_) => Ok(()),
                Err(_) => Err(format!(
                    "Cannot write the {i} line. There might have been an I/O error.",
                    i = i
                )),
            }?;
        }

        match stream.flush() {
            Ok(_) => Ok(()),
            Err(_) => Err("Unable to close file. There might have been an I/O error.".to_string()),
        }
    }
}

/// Return formatted vector of rows.
///
/// # Arguments
///
/// * `number_of_columns`: usize - Total number of columns to renderize.
/// * `values`: Vec<String> - Vector of column values.
/// * `positions`: Vec<usize> - Vector of column numbers.
pub(crate) fn compose_lines(
    number_of_columns: usize,
    values: Vec<String>,
    positions: Vec<usize>,
) -> Vec<String> {
    let mut new_values = vec!["".to_string(); number_of_columns];
    for (name, pos) in values.into_iter().zip(positions.into_iter()) {
        new_values[pos] = name
    }
    new_values
}
