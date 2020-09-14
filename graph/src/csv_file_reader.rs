use indicatif::{ProgressIterator, ProgressBar, ProgressStyle};
use std::{fs::File, io::prelude::*, io::BufReader};

/// Structure that saves the common parameters for reading csv files.
///
/// # Attributes
/// * path: String - The of the file to read. E.g. "/tmp/test.csv"
/// * verbose: bool - If the progress bars and logging must be displayed.
/// * separator: String - The separator to use, usually, this is "\t" for tsv and "," for csv.
/// * header: bool - If the file (will / must) have the header with the titles of the columns.
/// * rows_to_skip: usize - When reading, how many lines to skip before starting to read the file.
/// * ignore_duplicates: bool -if the program should raise an exception or not when the file contains duplicated edges / nodes.
pub struct CSVFileReader {
    pub(crate) path: String,
    pub(crate) verbose: bool,
    pub(crate) separator: String,
    pub(crate) header: bool,
    pub(crate) rows_to_skip: usize,
    pub(crate) ignore_duplicates: bool
}

/// # Builder methods
impl CSVFileReader {
    /// Return new CSVFileReader object.
    ///
    /// # Arguments
    ///
    /// * path: String - Path where to store/load the file.
    ///
    pub fn new(path: String) -> Result<CSVFileReader, String> {
        // check file existance
        match File::open(&path) {
            Ok(_) => Ok(CSVFileReader {
                path,
                verbose: true,
                separator: "\t".to_string(),
                header: true,
                rows_to_skip: 0,
                ignore_duplicates: true
            }),
            Err(_) => Err(format!("Cannot open the file at {}", path)),
        }
    }

    /// Set the verbose.
    ///
    /// # Arguments
    ///
    /// * verbose: Option<bool> - Wethever to show the loading bar or not.
    ///
    pub fn set_verbose(mut self, verbose: Option<bool>) -> CSVFileReader {
        if let Some(v) = verbose {
            self.verbose = v;
        }
        self
    }

    /// Set the ignore_duplicates.
    ///
    /// # Arguments
    ///
    /// * ignore_duplicates: Option<bool> - Wethever to ignore detected duplicates or raise exception.
    ///
    pub fn set_ignore_duplicates(mut self, ignore_duplicates: Option<bool>) -> CSVFileReader {
        if let Some(v) = ignore_duplicates {
            self.ignore_duplicates = v;
        }
        self
    }

    /// Set the separator.
    ///
    /// # Arguments
    ///
    /// * separator: Option<String> - The separator to use for the file.
    ///
    pub fn set_separator(mut self, separator: Option<String>) -> CSVFileReader {
        if let Some(v) = separator {
            self.separator = v;
        }
        self
    }

    /// Set the header.
    ///
    /// # Arguments
    ///
    /// * header: Option<bool> - Wethever to expect an header or not.
    ///
    pub fn set_header(mut self, header: Option<bool>) -> CSVFileReader {
        if let Some(v) = header {
            self.header = v;
        }
        self
    }

    /// Set number of rows to be skipped when starting to read file.
    ///
    /// # Arguments
    ///
    /// * rows_to_skip: Option<bool> - Wethever to show the loading bar or not.
    ///
    pub fn set_rows_to_skip(mut self, rows_to_skip: Option<usize>) -> CSVFileReader {
        if let Some(v) = rows_to_skip {
            self.rows_to_skip = v;
        }
        self
    }
    /// Read the whole file and return how many rows it has.
    pub(crate) fn count_rows(&self) -> usize {
        BufReader::new(File::open(&self.path).unwrap())
            .lines()
            .count()
    }

    /// Return list of components of the header.
    pub(crate) fn get_header(&self) -> Result<Vec<String>, String> {
        let file = File::open(&self.path).unwrap();
        let node_buf_reader = BufReader::new(file);
        let mut lines = node_buf_reader.lines().skip(self.rows_to_skip);
        // read the first line

        if let Some(lt) = lines.next() {
            match lt {
                Ok(line) => Ok(line
                    .split(&self.separator)
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()),
                Err(_) => Err("Something went wrong reading the line from the file".to_string()),
            }
        } else {
            Err("The given file has no lines!".to_string())
        }
    }

    /// Return elements of the first line not to be skipped.
    pub(crate) fn get_elements_per_line(&self) -> Result<usize, String> {
        let first_line = BufReader::new(File::open(&self.path).unwrap())
            .lines()
            .nth(self.rows_to_skip);
        match first_line {
            Some(fl) => {
                match fl {
                    Ok(f) => {
                        Ok(f.matches(&self.separator).count())
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
    pub(crate) fn read_lines(&self) -> Result<impl Iterator<Item = Result<Vec<String>, String>> + '_, String> {
        let pb = if self.verbose {
            let rows_number =
                self.count_rows() as u64 - self.rows_to_skip as u64 - self.header as u64;
            let pb = ProgressBar::new(rows_number);
            pb.set_draw_delta(rows_number / 100);
            pb.set_style(ProgressStyle::default_bar().template(
                "Reading csv {spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] ({pos}/{len}, ETA {eta})",
            ));
            pb
        } else {
            ProgressBar::hidden()
        };

        let number_of_elements_per_line = self.get_elements_per_line()?;
        Ok(BufReader::new(File::open(&self.path).unwrap())
            .lines()
            .skip(self.rows_to_skip + self.header as usize)
            .progress_with(pb)
            // unwrap the line and remove tralings new line chars
            .map(|line| match line {
                Ok(mut l) => {      
                    if l.ends_with('\n') {
                        l.pop();
                        if l.ends_with('\r') {
                            l.pop();
                        }
                    }
                    Ok(l)
                },
                Err(_) => Err("There might have been an I/O error or the line could contains bytes that are not valid UTF-8".to_string())
            })
            .enumerate()
            // skip empty lines
            .filter(|(_, line)| match line {
                Ok(l) => {
                    l != "" 
                },
                Err(_) => true
            })
            // split and validate the values
            .map(move |(i, line)| {
                match line {
                    Ok(l) => {
                        let line_components = l
                        .split(&self.separator)
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>();
                        if line_components.len() != number_of_elements_per_line {
                            return Err(format!(
                                concat!(
                                    "Found line {i} with different number",
                                    " ({found}) of separator from the expected",
                                    " one {expected}.\n",
                                    "Specifically, the line is: {line}"
                                ),
                                i=i,
                                found=line_components.len(),
                                expected=number_of_elements_per_line,
                                line=l
                            ));
                        }
                        Ok(line_components)
                    }
                    Err(e) => Err(e)
                }
            }))
    }

    /// Return number of the given column in header.
    /// 
    /// # Arguments
    /// 
    /// * column_name: String - Column to get the number of.
    /// 
    pub(crate) fn get_column_number(&self, column_name: String) -> Result<usize, String> {
        let header = self.get_header()?;

        match header.iter().position(|x| *x == column_name) {
            Some(column_number) => {
                Ok(column_number)
            }
            None => Err(format!(
                "The column '{}' is not present in the header\n{:?}",
                column_name, header
            )),
        }
    }
}