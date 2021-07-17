use rayon::iter::ParallelIterator;

use super::*;
/// Structure that saves the reader specific to writing and reading a nodes csv file.
///
/// # Attributes
#[derive(Clone)]
pub struct TypeFileReader<T: ToFromUsize + Sync> {
    pub(crate) reader: Option<CSVFileReader>,
    pub(crate) type_column_number: usize,
    pub(crate) types_number: Option<T>,
    pub(crate) numeric_type_ids: bool,
    pub(crate) minimum_type_id: Option<T>,
}

impl<T: ToFromUsize + Sync> TypeFileReader<T> {
    /// Return new TypeFileReader object.
    ///
    /// # Arguments
    /// * reader: CSVFilereader - Path where to store/load the file.
    ///
    pub fn new(path: Option<String>) -> Result<TypeFileReader<T>> {
        let has_path = path.is_some();
        Ok(TypeFileReader {
            reader: path.map_or(Ok::<_, String>(None), |path| {
                Ok(Some(CSVFileReader::new(path, "type list".to_owned())?))
            })?,
            type_column_number: 0,
            types_number: None,
            numeric_type_ids: !has_path,
            minimum_type_id: None,
        })
    }

    /// Raises an error if the file reader was not created.
    fn must_have_reader(&self) -> Result<()> {
        if self.reader.is_none() {
            return Err(concat!(
                "This particular instance of the ",
                "type file reader was not created with a file."
            )
            .to_string());
        }
        Ok(())
    }

    /// Set the column of the type nodes.
    ///
    /// # Arguments
    /// * types_column: Option<String> - The type nodes column to use for the file.
    ///
    pub fn set_type_column<S: Into<String>>(
        mut self,
        type_column: Option<S>,
    ) -> Result<TypeFileReader<T>> {
        if let Some(column) = type_column {
            self.must_have_reader()?;
            let column = column.into();
            if column.is_empty() {
                return Err("The given type column name is empty.".to_owned());
            }

            let column_number = self
                .reader
                .as_ref()
                .map(|reader| reader.get_column_number(column))
                .unwrap();

            match column_number {
                Ok(ecn) => {
                    self = self.set_type_column_number(Some(ecn))?;
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        Ok(self)
    }

    /// Set the column number of the types.
    ///
    /// # Arguments
    /// * type_column_number: Option<usize> - The type column number to use for the file.
    ///
    pub fn set_type_column_number(
        mut self,
        type_column_number: Option<usize>,
    ) -> Result<TypeFileReader<T>> {
        if let Some(column) = type_column_number {
            self.must_have_reader()?;
            let expected_elements = self
                .reader
                .as_ref()
                .map(|reader| reader.get_elements_per_line())
                .unwrap()?;
            if column >= expected_elements {
                return Err(format!(
                    concat!(
                        "The type column number passed was {} but ",
                        "the first parsable line has {} values."
                    ),
                    column, expected_elements
                ));
            }
            self.type_column_number = column;
        }
        Ok(self)
    }

    /// Set the total number of expected types.
    ///
    /// # Arguments
    /// * types_number: Option<usize> - The type column number to use for the file.
    ///
    pub fn set_types_number(mut self, types_number: Option<T>) -> TypeFileReader<T> {
        self.types_number = types_number;
        self
    }

    /// Set the minimum type ID.
    ///
    /// # Arguments
    /// * minimum_type_id: Option<usize> - The minimum type ID to expect when loading numeric type IDs.
    ///
    pub fn set_minimum_type_id(mut self, minimum_type_id: Option<T>) -> TypeFileReader<T> {
        self.minimum_type_id = minimum_type_id;
        self
    }

    /// Set whether the CSV is expected to be well written.
    ///
    /// # Arguments
    /// * csv_is_correct: Option<bool> - Whether you pinky swear the edge list is correct.
    ///
    pub fn set_csv_is_correct(mut self, csv_is_correct: Option<bool>) -> Result<TypeFileReader<T>> {
        if let Some(cic) = csv_is_correct {
            self.must_have_reader()?;
            self.reader
                .as_mut()
                .map(|reader| reader.csv_is_correct = cic);
        }
        Ok(self)
    }

    /// Set the comment symbol to use to skip the lines.
    ///
    /// # Arguments
    /// * comment_symbol: Option<String> - if the reader should ignore or not duplicated edges.
    ///
    pub fn set_comment_symbol(
        mut self,
        comment_symbol: Option<String>,
    ) -> Result<TypeFileReader<T>> {
        if let Some(cs) = comment_symbol {
            self.must_have_reader()?;
            if cs.is_empty() {
                return Err("The given comment symbol is empty.".to_string());
            }
            self.reader
                .as_mut()
                .map(|reader| reader.comment_symbol = Some(cs));
        }
        Ok(self)
    }

    /// Set the verbose.
    ///
    /// # Arguments
    /// * `verbose`: Option<bool> - Whether to show the loading bar or not.
    ///
    pub fn set_verbose(mut self, verbose: Option<bool>) -> TypeFileReader<T> {
        if let Some(v) = verbose {
            self.reader.as_mut().map(|reader| reader.verbose = v);
        }
        self
    }

    ///Set whether the types are to be loaded as numeric values.
    ///
    /// # Arguments
    /// * `numeric_id`: Option<bool> - Whether to convert numeric Ids to Node Id.
    ///
    pub fn set_numeric_type_ids(mut self, numeric_type_ids: Option<bool>) -> TypeFileReader<T> {
        if let Some(neti) = numeric_type_ids {
            self.numeric_type_ids = neti;
        }
        self
    }

    /// Set the ignore_duplicates.
    ///
    /// # Arguments
    /// * ignore_duplicates: Option<bool> - Whether to ignore detected duplicates or raise exception.
    ///
    pub fn set_ignore_duplicates(
        mut self,
        ignore_duplicates: Option<bool>,
    ) -> Result<TypeFileReader<T>> {
        if let Some(id) = ignore_duplicates {
            self.must_have_reader()?;
            self.reader
                .as_mut()
                .map(|reader| reader.ignore_duplicates = id);
        }
        Ok(self)
    }

    /// Set the separator.
    ///
    /// # Arguments
    /// * separator: Option<String> - The separator to use for the file.
    ///
    pub fn set_separator<S: Into<String>>(
        mut self,
        separator: Option<S>,
    ) -> Result<TypeFileReader<T>> {
        if let Some(sep) = separator {
            self.must_have_reader()?;
            let sep = sep.into();
            if sep.is_empty() {
                return Err("The separator cannot be empty.".to_owned());
            }
            self.reader.as_mut().map(|reader| reader.separator = sep);
        }
        Ok(self)
    }

    /// Set the header.
    ///
    /// # Arguments
    /// * header: Option<bool> - Whether to expect an header or not.
    ///
    pub fn set_header(mut self, header: Option<bool>) -> Result<TypeFileReader<T>> {
        if let Some(h) = header {
            self.must_have_reader()?;
            self.reader.as_mut().map(|reader| reader.header = h);
        }
        Ok(self)
    }

    /// Set the name of the graph to be loaded.
    ///
    /// # Arguments
    /// * graph_name: String - The name of the graph to be loaded.
    ///
    pub(crate) fn set_graph_name(mut self, graph_name: String) -> TypeFileReader<T> {
        self.reader
            .as_mut()
            .map(|reader| reader.graph_name = graph_name);
        self
    }

    /// Set number of rows to be skipped when starting to read file.
    ///
    /// # Arguments
    /// * rows_to_skip: Option<bool> - Whether to show the loading bar or not.
    ///
    pub fn set_rows_to_skip(mut self, rows_to_skip: Option<usize>) -> Result<TypeFileReader<T>> {
        if let Some(rows_to_skip) = rows_to_skip {
            self.must_have_reader()?;
            self.reader
                .as_mut()
                .map(|reader| reader.rows_to_skip = rows_to_skip);
        }
        Ok(self)
    }

    /// Set the maximum number of rows to load from the file
    ///
    /// # Arguments
    /// * max_rows_number: Option<u64> - The edge type to use when edge type is missing.
    ///
    pub fn set_max_rows_number(
        mut self,
        max_rows_number: Option<u64>,
    ) -> Result<TypeFileReader<T>> {
        if let Some(max_rows_number) = max_rows_number {
            self.must_have_reader()?;
            self.reader
                .as_mut()
                .map(|reader| reader.max_rows_number = Some(max_rows_number));
        }
        Ok(self)
    }

    /// Parse a single line (vector of strings already splitted)
    /// # Arguments
    /// * `line_number`: Number of the line.
    /// * `elements_in_line`: Vec<Option<String>> - Vector of the values of the line to be parsed
    fn parse_type_line(&self, line_number:usize, mut elements_in_line: Vec<Option<String>>) -> Result<String> {
        // extract the type name
        elements_in_line.pop().unwrap().map_or_else(
            || Err(format!("The type at line {} is empty.", line_number)),
            |type_name|Ok(type_name)
        )
    }

    /// Return iterator of rows of the edge file.
    pub fn read_lines(
        &self,
    ) -> Option<Result<impl ParallelIterator<Item = Result<(usize, String)>> + '_>> {
        self.reader.as_ref().map(|reader| {
            Ok(reader
                .read_lines(vec![self.type_column_number])?
                .map(move |line| match line {
                    Ok((line_number, vals)) => Ok((line_number, self.parse_type_line(line_number, vals)?)),
                    Err(e) => Err(e),
                }))
        })
    }
}
