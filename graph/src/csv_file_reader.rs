use indicatif::ProgressIterator;
use itertools::Itertools;
use std::{fs::File, io::prelude::*, io::BufReader};

use crate::utils::get_loading_bar;

/// Structure that saves the common parameters for reading csv files.
///
/// # Attributes
///
/// * `path`: String - The of the file to read. E.g. "/tmp/test.csv"
/// * `verbose`: bool - If the progress bars and logging must be displayed.
/// * `separator`: String - The separator to use, usually, this is "\t" for tsv and "," for csv.
/// * `header`: bool - If the file (will / must) have the header with the titles of the columns.
/// * `rows_to_skip`: usize - When reading, how many lines to skip before starting to read the file.
/// * `ignore_duplicates`: bool - Whether the program should raise an exception or not when the file contains duplicated edges / nodes.
/// * `csv_is_correct`: bool - Pinky promise that the file is well made.
/// * `max_rows_number`: Option<u64> -if the program should stop reading after a certain number of rows.
/// * `list_name`: String - The name of the list that is being loaded.
/// * `graph_name`: String - The name of graph that is being loaded.
///
#[derive(Clone)]
pub struct CSVFileReader {
    pub(crate) path: String,
    pub(crate) verbose: bool,
    pub(crate) separator: String,
    pub(crate) header: bool,
    pub(crate) rows_to_skip: usize,
    pub(crate) ignore_duplicates: bool,
    pub(crate) csv_is_correct: bool,
    pub(crate) max_rows_number: Option<u64>,
    pub(crate) comment_symbol: Option<String>,
    pub(crate) list_name: String,
    pub(crate) graph_name: String,
}

/// # Builder methods
impl CSVFileReader {
    /// Return new CSVFileReader object.
    ///
    /// # Arguments
    ///
    /// * path: String - Path where to store/load the file.
    /// * list_name: String - Name of the list that is being loaded.
    ///
    pub fn new<S: Into<String>>(path: S, list_name: String) -> Result<CSVFileReader, String> {
        let path = path.into();
        // check file existance
        match File::open(&path) {
            Ok(_) => Ok(CSVFileReader {
                path,
                verbose: true,
                separator: "\t".to_string(),
                header: true,
                rows_to_skip: 0,
                ignore_duplicates: true,
                csv_is_correct: false,
                max_rows_number: None,
                comment_symbol: None,
                list_name,
                graph_name: "Graph".to_string(),
            }),
            Err(_) => Err(format!("Cannot open the file at {}", path)),
        }
    }

    /// Read the whole file and return how many rows it has.
    pub(crate) fn count_rows(&self) -> usize {
        std::cmp::min(
            BufReader::new(File::open(&self.path).unwrap())
                .lines()
                .count(),
            self.max_rows_number.unwrap_or(u64::MAX) as usize,
        )
    }

    /// Return list of components of the header.
    pub fn get_header(&self) -> Result<Vec<String>, String> {
        if let Some(first_line) = self.get_lines_iterator(false)?.next() {
            Ok(first_line?
                .split(&self.separator)
                .map(|s| s.to_string())
                .collect::<Vec<String>>())
        } else {
            Err("The given file has no lines!".to_string())
        }
    }

    pub fn get_lines_iterator(
        &self,
        skip_header: bool,
    ) -> Result<impl Iterator<Item = Result<String, String>> + '_, String> {
        let rows_to_skip = match skip_header {
            true => match (self.rows_to_skip as u64).checked_add(self.header as u64) {
                Some(v) => Ok(v),
                None => Err(concat!(
                    "This overflow was caused because rows to skip = 2**64 - 1",
                    "and header is setted to true which causes to skip one extra line.",
                    "Do you **really** want to skip 18446744073709551615 lines? Bad person. Bad."
                )),
            }?,
            false => self.rows_to_skip as u64,
        } as usize;
        Ok(BufReader::new(File::open(&self.path).unwrap())
            .lines()
            .map(|line| match line {
                Ok(l)=>Ok(l),
                Err(_)=>Err("There might have been an I/O error or the line could contains bytes that are not valid UTF-8".to_string()),
            })
            .filter_ok(move |line| !line.is_empty() && match &self.comment_symbol {
                Some(cs) => !line.starts_with(cs),
                _ => true,
            })
            .skip(rows_to_skip))
    }

    /// Return elements of the first line not to be skipped.
    pub fn get_elements_per_line(&self) -> Result<usize, String> {
        let first_line = self.get_lines_iterator(true)?.next();
        match first_line {
            Some(fl) => {
                match fl {
                    Ok(f) => {
                        Ok(f.matches(&self.separator).count() + 1)
                    },
                    Err(_) => Err("There might have been an I/O error or the line could contains bytes that are not valid UTF-8".to_string())
                }
            },
            None => Err(concat!(
                "Unable to read the first non skipped line of the file.\n",
                "The file has possibly less than the expected amount of lines"
            ).to_string())
        }
    }

    /// Return iterator that read a CSV file rows.
    pub(crate) fn read_lines(
        &self,
    ) -> Result<impl Iterator<Item = Result<Vec<Option<String>>, String>> + '_, String> {
        let pb = get_loading_bar(
            self.verbose,
            format!("Reading {}'s {}", self.graph_name, self.list_name).as_ref(),
            if self.verbose { self.count_rows() } else { 0 },
        );

        let number_of_elements_per_line = self.get_elements_per_line()?;
        Ok(self
            .get_lines_iterator(true)?
            .progress_with(pb)
            // skip empty lines
            .take(self.max_rows_number.unwrap_or(u64::MAX) as usize)
            // Handling NaN values and padding them to the number of rows
            .map_ok(move |line| {
                let mut elements: Vec<Option<String>> = line
                    .split(&self.separator)
                    .map(|element| match element.is_empty() {
                        true => None,
                        false => Some(element.to_string()),
                    })
                    .collect();
                elements.resize(number_of_elements_per_line, None);
                elements
            }))
    }

    /// Return number of the given column in header.
    ///
    /// # Arguments
    ///
    /// * column_name: String - Column to get the number of.
    ///
    pub fn get_column_number(&self, column_name: String) -> Result<usize, String> {
        let header = self.get_header()?;

        match header.iter().position(|x| *x == column_name) {
            Some(column_number) => Ok(column_number),
            None => Err(format!(
                "The column '{}' is not present in the header\n{:?}",
                column_name, header
            )),
        }
    }
}
