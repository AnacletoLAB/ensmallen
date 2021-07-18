use super::*;
use indicatif::ProgressIterator;
use itertools::Itertools;

#[cfg(target_os = "linux")]
use nix::fcntl::*;
#[cfg(target_os = "linux")]
use std::os::unix::io::AsRawFd;

use num_traits::Zero;
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
    pub(crate) may_have_duplicates: Option<bool>,
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
                may_have_duplicates: None,
            }),
            Err(_) => Err(format!("Cannot open the file at {}", path)),
        }
    }

    fn get_buffer_reader(&self) -> Result<BufReader<File>> {
        let file = File::open(&self.path);

        if file.is_err() {
            return Err(format!("Cannot open the file at {}", self.path));
        }

        let file = file.unwrap();

        #[cfg(target_os = "linux")]
        let _ = posix_fadvise(
            file.as_raw_fd(),
            0,
            0,
            PosixFadviseAdvice::POSIX_FADV_SEQUENTIAL,
        );
        Ok(BufReader::with_capacity(8 * 1024 * 1024, file))
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

    pub fn get_parallell_lines_iterator(
        &self,
        skip_header: bool,
    ) -> Result<impl ParallelIterator<Item = Result<String>> + '_> {
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
        let mut parallell_buffer = ParallelLines::new(&self.path)?;
        parallell_buffer.skip_rows(rows_to_skip);
        Ok(
            parallell_buffer
            .map(|line| match line {
                Ok(l)=>Ok(l),
                Err(_)=>Err("There might have been an I/O error or the line could contains bytes that are not valid UTF-8".to_string()),
            })
            .filter(move |line| match (line, &self.comment_symbol) {
                (Ok(line), Some(cs)) => !line.is_empty() && !line.starts_with(cs),
                (Ok(line), _) => !line.is_empty(),
                _ => true
            })
            //.skip(rows_to_skip)
        )
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
        columns_of_interest: Vec<usize>,
    ) -> Result<impl ParallelIterator<Item = Result<(usize, Vec<Option<String>>)>> + '_> {
        // We create the loading bar
        // We already tested removing this and it does not appear to be a bottleneck.
        let pb = get_loading_bar(
            self.verbose,
            format!("Reading {}'s {}", self.graph_name, self.list_name).as_ref(),
            if self.verbose { self.count_rows()? } else { 0 },
        );

        // We check if the provided columns of interest
        let number_of_column_of_interest = columns_of_interest.len();
        if number_of_column_of_interest.is_zero() {
            return Err("The number of columns of interest provided was zero.".to_string());
        }
        if columns_of_interest.iter().cloned().unique().count() != number_of_column_of_interest {
            return Err("A duplicate column of interest was provided.".to_string());
        }

        // We check if the values are already sorted
        let columns_of_interest_are_sorted = columns_of_interest.is_sorted();

        // We zip the original position to the columns of interest
        // so to know where to map the extracted value.
        let mut columns_of_interest_and_position = columns_of_interest
            .into_iter()
            .enumerate()
            .collect::<Vec<(usize, usize)>>();

        // If necessary we sort these column of interest.
        if !columns_of_interest_are_sorted {
            columns_of_interest_and_position
                .sort_by(|(_, column_a), (_, column_b)| column_a.cmp(column_b));
        }

        // We get the minimum and maximum column of interest
        let min_column_of_interest = columns_of_interest_and_position.first().unwrap().1;
        let max_column_of_interest = columns_of_interest_and_position.last().unwrap().1;

        // Retrieve the number of elements that are expected to be in each line.
        let number_of_elements_per_line = self.get_elements_per_line()?;

        // We check that the maximum column of interest is not higher than the
        // number of elements in the lines.
        if max_column_of_interest >= number_of_elements_per_line {
            return Err(format!(
                concat!(
                    "The maximum column number of interest provided ({}) ",
                    "is higher or equal to the number of elements ",
                    "in the CSV lines ({})."
                ),
                max_column_of_interest, number_of_elements_per_line
            ));
        }

        // If the number of values between minimum and maximum is equal to the
        // number of columns of interest it means that these values are a dense range.
        let column_of_interest_are_dense_range =
            max_column_of_interest - min_column_of_interest == number_of_column_of_interest;
        // Check if the number of values in the CSV lines match exactly the number
        // of requested values.
        let all_elements_are_of_interest =
            number_of_elements_per_line == number_of_column_of_interest;

        let parse_line = if column_of_interest_are_dense_range
            && self.csv_is_correct
            && columns_of_interest_are_sorted
            && all_elements_are_of_interest
        {
            // If all the elements are requested and the are requested in the order
            // they are provided in from the CSV, then we can simply collect the
            // values split on the separator.
            |line_number: usize,
             line: Result<String>,
             separator: &str,
             _: &[(usize, usize)],
             _: usize,
             _: usize| {
                line.map(|line: String| {
                    (
                        line_number,
                        line.split(separator)
                            .into_iter()
                            .map(|element| {
                                if element.is_empty() {
                                    None
                                } else {
                                    Some(element.to_owned())
                                }
                            })
                            .collect::<Vec<Option<String>>>(),
                    )
                })
            }
        } else {
            // If either not all the elements are requested
            // or generally it becomes necessary to remap the values
            |line_number: usize,
             line: Result<String>,
             separator: &str,
             columns_of_interest_and_position: &[(usize, usize)],
             min_column_of_interest: usize,
             max_column_of_interest: usize| {
                line.map(|line: String| {
                    let mut elements: Vec<Option<String>> =
                        vec![None; columns_of_interest_and_position.len()];
                    let mut j = 0;
                    line.split(&separator)
                        .enumerate()
                        // We skip to the first value of interest
                        .skip(min_column_of_interest)
                        // We take at most a number of elements equal to
                        // the delta between the minimum column and maximum column.
                        // This way we can avoid having to check for out of bounds
                        // afterwards in the for each loop.
                        .take(1 + max_column_of_interest - min_column_of_interest)
                        // Empty values are left as None
                        .for_each(|(i, element)| {
                            if !element.is_empty() && i == columns_of_interest_and_position[j].1 {
                                elements[columns_of_interest_and_position[j].0] =
                                    Some(element.to_owned());
                                j += 1;
                            }
                        });
                    (line_number, elements)
                })
            }
        };

        Ok(self
            .get_parallell_lines_iterator(true)?
            // Reading only the requested amount of lines.
            //.take(self.max_rows_number.unwrap_or(u64::MAX) as usize)
            //.enumerate()
            //.progress_with(pb)
            //.par_bridge()
            .map(|line| (0, line))
            // Handling NaN values and padding them to the number of rows
            .map(move |(line_number, line)| {
                parse_line(
                    line_number,
                    line,
                    &self.separator,
                    &columns_of_interest_and_position,
                    min_column_of_interest,
                    max_column_of_interest,
                )
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
