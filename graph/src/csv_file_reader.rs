use super::*;
use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::{fs::File, io::prelude::*, io::BufReader};

use crate::utils::get_loading_bar;

/// Structure that saves the common parameters for reading csv files.
#[derive(Clone)]
pub struct CSVFileReader {
    /// The of the file to read. E.g. "/tmp/test.csv"
    pub(crate) path: String,

    /// If the progress bars and logging must be displayed.
    pub(crate) verbose: bool,

    /// The separator to use, usually, this is "\t" for tsv and "," for csv.
    pub(crate) separator: String,

    /// If the file (will / must) have the header with the titles of the columns
    pub(crate) header: bool,

    /// When reading, how many lines to skip before starting to read the file.
    pub(crate) rows_to_skip: usize,

    /// Whether the program should raise an exception or not when the file contains duplicated edges / nodes.
    pub(crate) ignore_duplicates: bool,

    /// Whether the user pinky promises that the csv is not malformed and thus it
    /// can be loaded without additional checks, hence going faster.
    pub(crate) csv_is_correct: bool,

    /// Pinky promise that the file is well made.
    pub(crate) max_rows_number: Option<u64>,

    /// if the program should stop reading after a certain number of rows.
    pub(crate) comment_symbol: Option<String>,

    /// The name of the list that is being loaded.
    pub(crate) list_name: String,

    /// The name of graph that is being loaded.
    pub(crate) graph_name: String,

    /// Whether the CSV may contain or not duplicate entries
    pub(crate) may_have_duplicates: Option<bool>
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
    pub fn new<S: Into<String>>(path: S, list_name: String) -> Result<CSVFileReader> {
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
                may_have_duplicates: None
            }),
            Err(_) => Err(format!("Cannot open the file at {}", path)),
        }
    }

    fn get_buffer_reader(&self) -> Result<BufReader<File>> {
        let file = File::open(&self.path);
        file.map_or_else(
            |_| Err(format!("Cannot open the file at {}", self.path)),
            |file| Ok(BufReader::new(file)),
        )
    }

    /// Read the whole file and return how many rows it has.
    pub(crate) fn count_rows(&self) -> Result<usize> {
        Ok(std::cmp::min(
            self.get_buffer_reader()?.lines().count(),
            self.max_rows_number.unwrap_or(u64::MAX) as usize,
        ))
    }

    /// Return list of components of the header.
    pub fn get_header(&self) -> Result<Vec<String>> {
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
    ) -> Result<impl Iterator<Item = Result<String>> + '_> {
        let rows_to_skip = match skip_header {
            true => match (self.rows_to_skip as u64).checked_add(self.header as u64) {
                Some(v) => Ok(v),
                None => Err(concat!(
                    "This overflow was caused because rows to skip = 2**64 - 1",
                    "and header is set to true which causes to skip one extra line.",
                    "Do you **really** want to skip 18446744073709551615 lines? Bad person. Bad."
                )),
            }?,
            false => self.rows_to_skip as u64,
        } as usize;
        Ok(self.get_buffer_reader()?
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
    pub fn get_elements_per_line(&self) -> Result<usize> {
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
    ) -> Result<impl ParallelIterator<Item = Result<(usize, Vec<Option<String>>)>> + '_> {
        let pb = get_loading_bar(
            self.verbose,
            format!("Reading {}'s {}", self.graph_name, self.list_name).as_ref(),
            if self.verbose { self.count_rows()? } else { 0 },
        );

        let number_of_elements_per_line = self.get_elements_per_line()?;
        Ok(self
            .get_lines_iterator(true)?
            // Reading only the requested amount of lines.
            .take(self.max_rows_number.unwrap_or(u64::MAX) as usize)
            .enumerate()
            .par_bridge()
            .progress_with(pb)
            // Handling NaN values and padding them to the number of rows
            .map(move |(line_number, line)| match line {
                Ok(line) => {
                    let mut elements: Vec<Option<String>> = vec![None; number_of_elements_per_line];
                    for (i, term) in line.split(&self.separator).enumerate() {
                        if i >= number_of_elements_per_line {
                            return Err(format!(
                                concat!(
                                    "Line number {} contains more elements ",
                                    "separated by the provided separator {:?} ",
                                    "the expected number of elements {}."
                                ),
                                line_number, self.separator, number_of_elements_per_line
                            ));
                        }
                        elements[i] = if term.is_empty(){
                            None
                        } else {
                            Some(term.to_owned())
                        };
                    }
                    Ok((line_number, elements))
                }
                Err(e) => Err(e),
            }))
    }

    /// Return number of the given column in header.
    ///
    /// # Arguments
    ///
    /// * column_name: String - Column to get the number of.
    ///
    pub fn get_column_number(&self, column_name: String) -> Result<usize> {
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
