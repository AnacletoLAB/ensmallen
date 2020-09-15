use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
use std::{fs::File, io::prelude::*};

/// Structure that saves the common parameters for reading csv files.
///
/// # Attributes
/// * path: String - The path where to save the file. E.g. "/tmp/test.csv"
/// * verbose: bool - If the progress bars and logging must be displayed.
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
    pub fn new(path: String) -> CSVFileWriter {
        CSVFileWriter {
            path,
            verbose: true,
            separator: "\t".to_string(),
            header: true,
        }
    }

    /// Set the verbose.
    ///
    /// # Arguments
    ///
    /// * verbose: Option<bool> - Wethever to show the loading bar or not.
    ///
    pub fn set_verbose(mut self, verbose: Option<bool>) -> CSVFileWriter {
        if let Some(v) = verbose {
            self.verbose = v;
        }
        self
    }

    /// Set the separator.
    ///
    /// # Arguments
    ///
    /// * separator: Option<String> - The separator to use for the file.
    ///
    pub fn set_separator(mut self, separator: Option<String>) -> CSVFileWriter {
        if let Some(v) = separator {
            self.separator = v;
        }
        self
    }

    /// Set the header.
    ///
    /// # Arguments
    ///
    /// * header: Option<bool> - Wethever to write out an header or not.
    ///
    pub fn set_header(mut self, header: Option<bool>) -> CSVFileWriter {
        if let Some(v) = header {
            self.header = v;
        }
        self
    }

    /// Write given rows iterator to file.
    ///
    /// # Arguments
    ///
    /// * `lines_number`: u64 - Number of lines to expect to write out.
    /// * `header`: Vec<String> - The header to write out, if so required.
    /// * `values`: impl Iterator<Item = Vec<String>> - Iterator of rows to write out.
    pub(crate) fn write_lines(
        &self,
        lines_number: u64,
        header: Vec<String>,
        values: impl Iterator<Item = Vec<String>>,
    ) -> Result<(), String> {
        let pb = if self.verbose {
            let pb = ProgressBar::new(lines_number);
            pb.set_draw_delta(lines_number / 100);
            pb.set_style(ProgressStyle::default_bar().template(
                "Writing csv {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
            ));
            pb
        } else {
            ProgressBar::hidden()
        };

        let mut file = match File::create(self.path.clone()) {
            Ok(f) => Ok(f),
            Err(_) => Err(format!("Cannot open in writing the file {}", self.path)),
        }?;

        if self.header {
            let mut line = header.join(&self.separator);
            line.push('\n');
            match file.write_all(line.as_bytes()) {
                Ok(_) => Ok(()),
                Err(_) => {
                    Err("Cannot write the header. There might have been an I/O error.".to_string())
                }
            }?;
        }

        for (i, value) in values.progress_with(pb).enumerate() {
            let mut line = value.join(&self.separator);
            line.push('\n');
            match file.write_all(line.as_bytes()) {
                Ok(_) => Ok(()),
                Err(_) => Err(format!(
                    "Cannot write the {i} line. There might have been an I/O error.",
                    i = i
                )),
            }?;
        }

        match file.sync_all() {
            Ok(_) => Ok(()),
            Err(_) => Err(
                "Unable to close file. There might have been an I/O error.".to_string()
            )
        }
    }
}

/// Return formatted vector of rows.
///
/// # Arguments
///
/// * `number_of_columns`: usize - Total number of columns to renderize.
/// * `pairs`: Vec<(String, usize)> - Vector of tuples of column names and their position.
pub(crate) fn compose_lines(number_of_columns: usize, pairs: Vec<(String, usize)>) -> Vec<String> {
    let mut values = vec!["".to_string(); number_of_columns];
    for (name, pos) in pairs {
        values[pos] = name
    }
    values
}
