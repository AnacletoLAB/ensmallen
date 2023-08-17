use super::*;
use indicatif::ProgressIterator;
use itertools::Itertools;

#[cfg(target_os = "linux")]
use nix::fcntl::*;
#[cfg(target_os = "linux")]
use std::os::unix::io::AsRawFd;

use num_traits::Zero;
use rayon::iter::ParallelIterator;
use std::{collections::HashMap, fs::File, io::prelude::*, io::BufReader};

use crate::utils::get_loading_bar;

const TYPES_OF_SEPARATORS: &'static [char] = &['\t', ',', ';', ' '];

/// Structure that saves the common parameters for reading csv files.
#[derive(Clone)]
#[no_binding]
pub struct CSVFileReader {
    /// The of the file to read. E.g. "/tmp/test.csv"
    pub(crate) path: String,

    /// If the progress bars and logging must be displayed.
    /// Note that this is ony used when running without parallelism
    /// because otherwise the bar synchronization ovehead is too massive.
    pub(crate) verbose: bool,

    /// The separator to use, usually, this is '\t' for tsv and "," for csv.
    pub(crate) separator: char,

    /// Boolean to check consistency when calling the builder methods.
    pub(crate) separator_was_set: bool,

    /// The number of lines to read in order to automatically detect the
    /// separator to be used.
    pub(crate) number_of_lines_to_automatically_detect_separator: usize,

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
    pub(crate) max_rows_number: Option<usize>,

    /// if the program should stop reading after a certain number of rows.
    pub(crate) comment_symbol: Option<String>,

    /// The name of the list that is being loaded.
    pub(crate) list_name: String,

    /// The name of graph that is being loaded.
    pub(crate) graph_name: String,

    /// Whether the CSV may contain or not duplicate entries
    pub(crate) may_have_duplicates: Option<bool>,

    /// Whether to read the file sequentially or in parallel
    pub(crate) parallel: bool,

    /// Whether to support reading of balanced quotes, which will significantly slow down the parsing.
    pub(crate) support_balanced_quotes: bool,

    /// Whether to trim chevrons from the elements, that is change read value from `<VALUE>` to `VALUE`
    pub(crate) remove_chevrons: bool,

    /// Whether to trim spaces from the elements, that is change read value from `  VALUE ` to `VALUE`
    pub(crate) remove_spaces: bool,
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
    pub fn new<S1: Into<String>, S2: Into<String>>(path: S1, list_name: S2) -> Result<CSVFileReader> {
        let path = path.into();
        let list_name = list_name.into();
        // check file existance
        match File::open(&path) {
            Ok(_) => Ok({
                CSVFileReader {
                    path,
                    verbose: true,
                    separator: '\t',
                    separator_was_set: false,
                    number_of_lines_to_automatically_detect_separator: 2000,
                    header: true,
                    rows_to_skip: 0,
                    ignore_duplicates: true,
                    csv_is_correct: false,
                    max_rows_number: None,
                    comment_symbol: None,
                    list_name,
                    graph_name: "Graph".to_string(),
                    may_have_duplicates: None,
                    parallel: true,
                    support_balanced_quotes: false,
                    remove_chevrons: false,
                    remove_spaces: false,
                }
            }),
            Err(_) => Err(format!("Cannot open the file at {}", path)),
        }
    }

    /// Set whether to load the CSV using the parallel reader or sequential reader.
    ///
    /// # Arguments
    /// * parallel: Option<bool> - Whether to read the CSV using a parallel or sequential reader.
    ///
    pub fn set_parallel(mut self, parallel: Option<bool>) -> CSVFileReader {
        if let Some(parallel) = parallel {
            self.parallel = parallel;
        }
        self
    }

    /// Set whether remove chevrons while reading elements.
    ///
    /// # Arguments
    /// * remove_chevrons: Option<bool> - Whether to remove chevrons while reading elements.
    ///
    pub fn set_remove_chevrons(mut self, remove_chevrons: Option<bool>) -> CSVFileReader {
        if let Some(remove_chevrons) = remove_chevrons {
            self.remove_chevrons = remove_chevrons;
        }
        self
    }

    /// Set whether remove spaces while reading elements.
    ///
    /// # Arguments
    /// * remove_spaces: Option<bool> - Whether to remove spaces while reading elements.
    ///
    pub fn set_remove_spaces(mut self, remove_spaces: Option<bool>) -> CSVFileReader {
        if let Some(remove_spaces) = remove_spaces {
            self.remove_spaces = remove_spaces;
        }
        self
    }

    /// Set whether to support the balanced quotes while reading the CSV, operation that will significantly slow down the execution.
    ///
    /// # Arguments
    /// * support_balanced_quotes: Option<bool> - Whether to support the balanced quotes while reading the CSV.
    ///
    pub fn set_support_balanced_quotes(
        mut self,
        support_balanced_quotes: Option<bool>,
    ) -> CSVFileReader {
        if let Some(support_balanced_quotes) = support_balanced_quotes {
            self.support_balanced_quotes = support_balanced_quotes;
        }
        self
    }

    /// Set separator to the provided value.
    ///
    /// # Arguments
    /// * `separator`: Option<char> - The value to use as separator in the file.
    pub fn set_separator(mut self, separator: Option<char>) -> Result<CSVFileReader> {
        self.separator = if let Some(separator) = separator {
            separator
        } else {
            self.detect_separator()?
        };
        self.separator_was_set = true;
        Ok(self)
    }

    /// Return the separator.
    pub fn get_separator(&self) -> char {
        self.separator.clone()
    }

    /// Return whether the reader is expected to include an header.
    pub fn has_header(&self) -> bool {
        self.header
    }

    /// Set the comment symbol for this file.
    ///
    /// # Arguments
    /// * `comment_symbol`: Option<String> - Comment symbol to use for this file.
    ///
    /// # Raises
    /// * If the separator was already set before calling this method.
    pub fn set_comment_symbol(mut self, comment_symbol: Option<String>) -> Result<CSVFileReader> {
        if let Some(comment_symbol) = comment_symbol {
            self.separator_must_not_already_be_set()?;
            if comment_symbol.is_empty() {
                return Err("The given comment symbol is empty.".to_string());
            }
            self.comment_symbol = Some(comment_symbol);
        }
        Ok(self)
    }

    /// Set the maximum number of rows to be read within this file.
    ///
    /// # Arguments
    /// * `max_rows_number`: Option<usize> - Number of lines to be read from this file.
    ///
    /// # Raises
    /// * If the separator was already set before calling this method.
    pub fn set_max_rows_number(mut self, max_rows_number: Option<usize>) -> Result<CSVFileReader> {
        if let Some(max_rows_number) = max_rows_number {
            self.separator_must_not_already_be_set()?;
            self.max_rows_number = Some(max_rows_number);
        }
        Ok(self)
    }

    /// Set the number of lines to skip before starting to read this file.
    ///
    /// # Arguments
    /// * `rows_to_skip`: Option<usize> - Number of lines to skip before reading the file.
    ///
    /// # Raises
    /// * If the separator was already set before calling this method.
    pub fn set_rows_to_skip(mut self, rows_to_skip: Option<usize>) -> Result<CSVFileReader> {
        if let Some(rows_to_skip) = rows_to_skip {
            self.separator_must_not_already_be_set()?;
            self.rows_to_skip = rows_to_skip;
        }
        Ok(self)
    }

    /// Set whether the file is expected to have an header.
    ///
    /// # Arguments
    /// * `header`: Option<bool> - Whether this file is expected to have an header.
    ///
    /// # Raises
    /// * If the separator was already set before calling this method.
    pub fn set_header(mut self, header: Option<bool>) -> Result<CSVFileReader> {
        if let Some(header) = header {
            self.separator_must_not_already_be_set()?;
            self.header = header;
        }
        Ok(self)
    }

    /// Checks if separator was already set and raises an error if it was not.
    pub fn separator_must_already_be_set(&self) -> Result<()> {
        if !self.separator_was_set {
            return Err(concat!(
                "The separator for this CSV file must be set BEFORE ",
                "calling this other builder method, otherwise it may ",
                "lead to an undefined behaviour."
            )
            .to_string());
        }
        Ok(())
    }

    /// Checks if separator was already set and raises an error if it was.
    pub fn separator_must_not_already_be_set(&self) -> Result<()> {
        if self.separator_was_set {
            return Err(concat!(
                "The separator for this CSV file must be set AFTER ",
                "calling this other builder method, otherwise it may ",
                "lead to an undefined behaviour."
            )
            .to_string());
        }
        Ok(())
    }

    /// Automatically detects which separator to use among a set.
    ///
    /// Specifically, the set includes ';', ',', '\t' and empty space.
    pub fn detect_separator(&self) -> Result<char> {
        let mut counter: HashMap<char, usize> = TYPES_OF_SEPARATORS
            .iter()
            .map(|separator| (*separator, 0))
            .collect();
        let mut first_line_counter: HashMap<char, usize> = TYPES_OF_SEPARATORS
            .iter()
            .map(|separator| (*separator, 0))
            .collect();
        for (_, line) in self.get_sequential_lines_iterator(true, false)?.take(1) {
            let line = line?;
            line.chars().for_each(|character| {
                first_line_counter.entry(character).and_modify(|entry| {
                    *entry += 1;
                });
            });
        }
        for (_, line) in self
            .get_sequential_lines_iterator(true, false)?
            .take(self.number_of_lines_to_automatically_detect_separator)
        {
            let line = line?;
            let mut line_counter: HashMap<char, usize> = TYPES_OF_SEPARATORS
                .iter()
                .map(|separator| (*separator, 0))
                .collect();
            line.chars().for_each(|character| {
                line_counter.entry(character).and_modify(|entry| {
                    *entry += 1;
                });
            });
            for (key, count) in line_counter.into_iter() {
                if *first_line_counter.get(&key).unwrap() == count {
                    counter.entry(key).and_modify(|entry| {
                        *entry += count;
                    });
                }
            }
        }
        Ok(counter
            .into_iter()
            .max_by(|(_, left), (_, right)| left.cmp(right))
            .unwrap()
            .0)
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
    ///
    /// TODO: make this more efficient!
    pub(crate) fn count_rows(&self) -> Result<usize> {
        Ok(std::cmp::min(
            self.get_buffer_reader()?.lines().count(),
            self.max_rows_number.unwrap_or(usize::MAX) as usize,
        ))
    }

    /// Return list of components of the header.
    pub fn get_header(&self) -> Result<Vec<String>> {
        self.separator_must_already_be_set()?;
        if let Some((_, first_line)) = self.get_sequential_lines_iterator(false, false)?.next() {
            Ok(
                splitter(&first_line?, self.separator, self.support_balanced_quotes)
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
            )
        } else {
            Err("The given file has no lines!".to_string())
        }
    }

    /// Return whether the CSV was labelled as correct.
    pub fn is_csv_correct(&self) -> bool {
        self.csv_is_correct
    }

    fn get_parallell_lines_iterator(
        &self,
        skip_header: bool,
    ) -> Result<impl ParallelIterator<Item = (usize, Result<String>)> + '_> {
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
        let mut parallell_buffer = ParallelLinesWithIndex::new(&self.path)?;
        parallell_buffer.set_skip_rows(rows_to_skip);
        parallell_buffer.set_comment_symbol(self.comment_symbol.clone());

        Ok(parallell_buffer)
    }

    /// Returns the total number of lines to be skipped.
    ///
    /// # Arguments
    /// * `skip_header`: bool - Whether to skip the header.
    ///
    /// TODO! Add lines to skip with comments.
    pub fn get_total_lines_to_skip(&self, skip_header: bool) -> Result<usize> {
        Ok(match skip_header {
            true => match (self.rows_to_skip as u64).checked_add(self.header as u64) {
                Some(v) => Ok(v),
                None => Err(concat!(
                    "This overflow was caused because rows to skip = 2**64 - 1",
                    "and header is set to true which causes to skip one extra line.",
                    "Do you **really** want to skip 18446744073709551615 lines? Bad person. Bad."
                )),
            }?,
            false => self.rows_to_skip as u64,
        } as usize)
    }

    /// Returns a sequential lines iterator.
    ///
    /// # Arguments
    /// * `skip_header`: bool - Whether to skip the header.
    /// * `verbose`: bool - Whether to show the loading bar.
    fn get_sequential_lines_iterator(
        &self,
        skip_header: bool,
        verbose: bool,
    ) -> Result<impl Iterator<Item = (usize, Result<String>)> + '_> {
        let rows_to_skip = self.get_total_lines_to_skip(skip_header)?;

        // We create the loading bar
        // We already tested removing this and it does not appear to be a bottleneck.
        let pb = get_loading_bar(
            verbose,
            format!("Reading {}'s {}", self.graph_name, self.list_name).as_ref(),
            if verbose { self.count_rows()? } else { 0 },
        );

        Ok(self.get_buffer_reader()?
            .lines()
            .progress_with(pb)
            .map(|line| match line {
                Ok(mut l)=> {
                    if l.ends_with('\r') {
                        l.pop().unwrap();
                    }
                    Ok(l)
                },
                Err(_)=>Err("There might have been an I/O error or the line could contains bytes that are not valid UTF-8".to_string()),
            })
            .filter_ok(move |line| !line.is_empty() && match &self.comment_symbol {
                Some(cs) => !line.starts_with(cs),
                _ => true,
            })
            .skip(rows_to_skip)
            .take(self.max_rows_number.unwrap_or(usize::MAX))
            .enumerate()
        )
    }

    /// Returns a sequential lines iterator.
    ///
    /// # Arguments
    /// * `skip_header`: bool - Whether to skip the header.
    /// * `verbose`: bool - Whether to show the loading bar.
    fn get_lines_iterator(
        &self,
        skip_header: bool,
        verbose: bool,
    ) -> Result<
        ItersWrapper<
            (usize, Result<String>),
            impl Iterator<Item = (usize, Result<String>)> + '_,
            impl ParallelIterator<Item = (usize, Result<String>)> + '_,
        >,
    > {
        Ok(if self.parallel {
            ItersWrapper::Parallel(self.get_parallell_lines_iterator(skip_header)?)
        } else {
            ItersWrapper::Sequential(self.get_sequential_lines_iterator(skip_header, verbose)?)
        })
    }

    /// Return elements of the first line not to be skipped.
    pub fn get_elements_per_line(&self) -> Result<usize> {
        self.separator_must_already_be_set()?;
        let first_line = self.get_sequential_lines_iterator(true, false)?.next();
        match first_line {
            Some((_, fl)) => {
                match fl {
                    Ok(f) => {
                        Ok(f.matches(self.separator).count() + 1)
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
    pub fn read_lines(
        &self,
        columns_of_interest: Option<Vec<usize>>,
    ) -> Result<
        ItersWrapper<
            Result<(usize, Vec<Option<String>>)>,
            impl Iterator<Item = Result<(usize, Vec<Option<String>>)>> + '_,
            impl ParallelIterator<Item = Result<(usize, Vec<Option<String>>)>> + '_,
        >,
    > {
        // Retrieve the number of elements that are expected to be in each line.
        let number_of_elements_per_line = self.get_elements_per_line()?;

        let columns_of_interest =
            columns_of_interest.unwrap_or((0..number_of_elements_per_line).collect());

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

        // We check that the maximum column of interest is not higher than the
        // number of elements in the lines.
        if false && max_column_of_interest >= number_of_elements_per_line {
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
             separator: char,
             support_balanced_quotes: bool,
             _: &[(usize, usize)],
             _: usize,
             _: usize| {
                line.map(|line: String| {
                    (
                        line_number,
                        splitter(&line, separator, support_balanced_quotes)
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
             separator: char,
             support_balanced_quotes: bool,
             columns_of_interest_and_position: &[(usize, usize)],
             min_column_of_interest: usize,
             max_column_of_interest: usize| {
                line.map(|line: String| {
                    let mut elements: Vec<Option<String>> =
                        vec![None; columns_of_interest_and_position.len()];
                    let mut j = 0;
                    splitter(&line, separator, support_balanced_quotes)
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
            .get_lines_iterator(true, self.verbose)?
            .map(move |(line_number, line)| {
                let (line_number, mut elements) = parse_line(
                    line_number,
                    line,
                    self.separator,
                    self.support_balanced_quotes,
                    &columns_of_interest_and_position,
                    min_column_of_interest,
                    max_column_of_interest,
                )?;
                if self.remove_spaces {
                    elements.iter_mut().for_each(|element| {
                        element.as_mut().map(|element| {
                            *element = element.trim().to_string();
                        });
                    });
                }
                if self.remove_chevrons {
                    elements
                        .iter_mut()
                        .filter(|element| {
                            element.as_ref().map_or(false, |element| {
                                element.starts_with("<") && element.ends_with(">")
                            })
                        })
                        .for_each(|element| {
                            element.as_mut().map(|element| {
                                let mut element_chars = element.chars();
                                element_chars.next();
                                element_chars.next_back();
                                *element = element_chars.as_str().to_string();
                            });
                        });
                }
                Ok((line_number, elements))
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

        // We get the position of the column of interest in the header
        match header.iter().position(|x| *x == column_name) {
            // If the column is present we return its position
            Some(column_number) => Ok(column_number),
            // If the column is not present we return an error
            // We try to make the error as extensive as possible to help
            // the user find the issue.
            None => Err(format!(
                concat!(
                    "The column of interest \"{}\" is not present in the header ",
                    "when using as separator \"{}\".",
                    "The header is:\n{}",
                    "The path to the CSV file is:\n{}"
                ),
                column_name,
                self.separator,
                header.join(&self.separator.to_string()),
                self.path
            )),
        }
    }
}
