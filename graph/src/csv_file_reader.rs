use indicatif::{ProgressBar, ProgressIterator, ProgressStyle};
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
/// * max_rows_number: Option<u64> -if the program should stop reading after a certain number of rows.
#[derive(Clone)]
pub struct CSVFileReader {
    pub(crate) path: String,
    pub(crate) verbose: bool,
    pub(crate) separator: String,
    pub(crate) header: bool,
    pub(crate) rows_to_skip: usize,
    pub(crate) ignore_duplicates: bool,
    pub(crate) max_rows_number: Option<u64>,
    pub(crate) comment_symbol: Option<String>,
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
                ignore_duplicates: true,
                max_rows_number: None,
                comment_symbol: None,
            }),
            Err(_) => Err(format!("Cannot open the file at {}", path)),
        }
    }

    /// Read the whole file and return how many rows it has.
    pub(crate) fn count_rows(&self) -> usize {
        BufReader::new(File::open(&self.path).unwrap())
            .lines()
            .count()
    }

    /// Return list of components of the header.
    pub fn get_header(&self) -> Result<Vec<String>, String> {
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
    pub fn get_elements_per_line(&self) -> Result<usize, String> {
        let first_line = BufReader::new(File::open(&self.path).unwrap())
            .lines()
            .filter(|line|{
                match (line, &self.comment_symbol){
                    (Ok(l), Some(cs)) => !l.starts_with(cs),
                    _ => true
                }
            })
            .nth(self.rows_to_skip);
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
    ) -> Result<impl Iterator<Item = Result<Vec<String>, String>> + '_, String> {
        let pb = if self.verbose {
            let number_of_rows = self.count_rows() as u64;
            let rows_to_skip = match (self.rows_to_skip as u64).checked_add(self.header as u64) {
                Some(v) => Ok(v),
                None => Err(concat!(
                    "This overflow was caused because rows to skip = 2**64 - 1",
                    "and header is setted to true which causes to skip one extra line.",
                    "Do you **really** want to skip 18446744073709551615 lines? Bad person. Bad."
                )),
            }?;
            if number_of_rows < rows_to_skip {
                return Err(format!(
                    concat!(
                        "The given file has {} lines but it was asked to skip",
                        "{} rows. This is not possible."
                    ),
                    number_of_rows, rows_to_skip
                ));
            }

            let rows_number = number_of_rows - rows_to_skip;
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
            .enumerate()
            // skip empty lines
            .filter_map(move |(i, line)| match line {
                Ok(l) => {
                    if l.is_empty() || self.max_rows_number.unwrap_or(u64::MAX) <= i as u64 {
                        return None;
                    }
                    if let Some(cs) = &self.comment_symbol{
                        if l.starts_with(cs){
                            return None;
                        }
                    }
                    let line_components = l
                        .split(&self.separator)
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>();
                    if line_components.len() != number_of_elements_per_line {
                        return Some(Err(format!(
                            concat!(
                                "Found line {i} with different number",
                                " ({found}) of separator from the expected",
                                " one {expected}.\n",
                                "Specifically, the line is: {line}\n",
                                "And the line components is {line_components:?}"
                            ),
                            i=i,
                            found=line_components.len(),
                            expected=number_of_elements_per_line,
                            line_components=line_components,
                            line=l
                        )));
                    }
                    Some(Ok(line_components))
                },
                Err(_) => Some(Err("There might have been an I/O error or the line could contains bytes that are not valid UTF-8".to_string()))
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
