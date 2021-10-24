use super::*;
use indicatif::ProgressIterator;
use std::{fs::OpenOptions, io::prelude::*, io::BufWriter};

/// Structure that saves the common parameters for reading csv files.
///
/// # Attributes
/// * path: String - The path where to save the file. E.g. "/tmp/test.csv"
/// * `verbose`: bool - If the progress bars and logging must be displayed.
/// * `separator`: char - The separator to use, usually, this is '\t' for tsv and "," for csv.
/// * `header`: bool - If the file (will / must) have the header with the titles of the columns.
#[no_binding]
pub struct CSVFileWriter {
    pub(crate) path: String,
    verbose: bool,
    separator: char,
    header: bool,
}

/// # Builder methods
impl CSVFileWriter {
    /// Return new CSVFileWriter object.
    ///
    /// # Arguments
    /// * `path`: String - Path where to store/load the file.
    ///
    pub fn new<S: Into<String>>(path: S) -> CSVFileWriter {
        CSVFileWriter {
            path: path.into(),
            verbose: true,
            separator: '\t',
            header: true,
        }
    }

    /// Set whether the file is expected to have an header.
    ///
    /// # Arguments
    /// * `header`: Option<bool> - Whether this file is expected to have an header.
    ///
    /// # Raises
    /// * If the separator was already set before calling this method.
    pub fn set_header(mut self, header: Option<bool>) -> CSVFileWriter {
        if let Some(header) = header {
            self.header = header;
        }
        self
    }

    /// Set whether to show a loading bar.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show the loading bar.
    ///
    /// # Raises
    /// * If the separator was already set before calling this method.
    pub fn set_verbose(mut self, verbose: Option<bool>) -> CSVFileWriter {
        if let Some(verbose) = verbose {
            self.verbose = verbose;
        }
        self
    }

    /// Set separator to the provided value.
    ///
    /// # Arguments
    /// * `separator`: Option<char> - The value to use as separator in the file.
    pub fn set_separator(mut self, separator: Option<char>) -> Result<CSVFileWriter> {
        if let Some(separator) = separator {
            self.separator = separator
        };
        Ok(self)
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

        // Create file in such a way it supports also rewrite inplace
        let mut file = match OpenOptions::new().append(true).open(self.path.clone()) {
            Ok(f) => Ok(f),
            Err(_) => Err(format!("Cannot open in writing the file {}", self.path)),
        }?;

        // Move the pointer back to the beginning of the file.
        match file.seek(std::io::SeekFrom::Start(0)) {
            Ok(f) => Ok(()),
            Err(_) => Err(format!("Unable to move file pointer to beginning of the file {}", self.path)),
        }?;

        let mut stream = BufWriter::with_capacity(8 * 1024 * 1024, file);

        if self.header {
            let mut line = header.join(self.separator.to_string().as_str());
            line.push('\n');
            match stream.write(line.as_bytes()) {
                Ok(_) => Ok(()),
                Err(_) => {
                    Err("Cannot write the header. There might have been an I/O error.".to_string())
                }
            }?;
        }

        for (i, value) in values.progress_with(pb).enumerate() {
            let mut line = value.join(self.separator.to_string().as_str());
            line.push('\n');
            match stream.write(line.as_bytes()) {
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
        }?;

        // Get the file ownership back from the stream.
        let mut file = match stream.into_inner() {
            Ok(f) => Ok(f),
            Err(_) => Err("Cannot recover ownership of file pointer from stream.".to_string()),
        }?;

        // Reading the file size of the current stream position.
        let file_size = match file.stream_position() {
            Ok(file_size) => Ok(file_size),
            Err(_) => Err("Cannot read file size.".to_string()),
        }?;

        // Truncate the possible remainder of the file.
        match file.set_len(file_size) {
            Ok(_) => Ok(()),
            Err(_) => Err("Cannot truncate the file.".to_string()),
        }?;

        Ok(())
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
