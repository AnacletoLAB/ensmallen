use rayon::iter::ParallelIterator;

use super::*;
/// Structure that saves the reader specific to read a types csv file.
///
/// # Attributes
#[no_binding]
#[derive(Clone)]
pub struct TypeFileReader<T: ToFromUsize + Sync> {
    pub(crate) reader: Option<CSVFileReader>,
    pub(crate) type_ids_column_number: Option<usize>,
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
            type_ids_column_number: None,
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

    /// Set the column of the type IDs.
    ///
    /// # Arguments
    /// * type_ids_column: Option<String> - The name of the type id column to use for the file.
    ///
    pub fn set_type_ids_column<S: Into<String>>(
        mut self,
        type_ids_column: Option<S>,
    ) -> Result<TypeFileReader<T>> {
        if let Some(column) = type_ids_column {
            self.must_have_reader()?;
            let column = column.into();
            if column.is_empty() {
                return Err("The given type types column is empty.".to_owned());
            }
            let column_number = self
                .reader
                .as_ref()
                .map_or(Ok::<_, String>(None), |reader| {
                    Ok(Some(reader.get_column_number(column)?))
                })?;
            self = self.set_type_ids_column_number(column_number)?;
        }
        Ok(self)
    }

    /// Set the type id type column number.
    ///
    /// # Arguments
    /// * `type_ids_column_number`: Option<usize> - The type id column number to use for the file.
    ///
    pub fn set_type_ids_column_number(
        mut self,
        type_ids_column_number: Option<usize>,
    ) -> Result<TypeFileReader<T>> {
        if let Some(column) = type_ids_column_number {
            self.must_have_reader()?;
            if let Some(reader) = self.reader.as_mut() {
                let expected_elements = reader.get_elements_per_line()?;
                if column >= expected_elements {
                    return Err(format!(
                        concat!(
                            "The type id column number passed was {} but ",
                            "the first parsable line has {} values."
                        ),
                        column, expected_elements
                    ));
                }
            }
            self.type_ids_column_number = Some(column);
        }
        Ok(self)
    }

    /// Set the column of the type.
    ///
    /// # Arguments
    /// * types_column: Option<String> - The type column to use for the file.
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

    /// Get the minimum type ID.
    pub fn get_minimum_type_id(&self) -> Option<T> {
        self.minimum_type_id
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

    /// Return whether the CSV was labelled to have numeric type IDs.
    pub fn has_numeric_type_ids(&self) -> bool {
        self.numeric_type_ids
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
        if let Some(comment_symbol) = comment_symbol {
            self.must_have_reader()?;
            self.reader = self.reader.map_or(Ok::<_, String>(None), |reader| {
                Ok(Some(reader.set_comment_symbol(Some(comment_symbol))?))
            })?;
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
    ///
    /// * separator: Option<String> - The separator to use for the file.
    ///
    pub fn set_separator(mut self, separator: Option<String>) -> Result<TypeFileReader<T>> {
        if separator.is_some() {
            self.must_have_reader()?;
        }
        self.reader = self.reader.map_or(Ok::<_, String>(None), |reader| {
            Ok(Some(reader.set_separator(separator)?))
        })?;
        Ok(self)
    }

    /// Set the header.
    ///
    /// # Arguments
    ///
    /// * header: Option<bool> - Whether to expect an header or not.
    ///
    pub fn set_header(mut self, header: Option<bool>) -> Result<TypeFileReader<T>> {
        if header.is_some() {
            self.must_have_reader()?;
        }
        self.reader = self.reader.map_or(Ok::<_, String>(None), |reader| {
            Ok(Some(reader.set_header(header)?))
        })?;
        Ok(self)
    }

    /// Set number of rows to be skipped when starting to read file.
    ///
    /// # Arguments
    ///
    /// * rows_to_skip: Option<bool> - Whether to show the loading bar or not.
    ///
    pub fn set_rows_to_skip(mut self, rows_to_skip: Option<usize>) -> Result<TypeFileReader<T>> {
        if rows_to_skip.is_some() {
            self.must_have_reader()?;
        }
        self.reader = self.reader.map_or(Ok::<_, String>(None), |reader| {
            Ok(Some(reader.set_rows_to_skip(rows_to_skip)?))
        })?;
        Ok(self)
    }

    /// Set the maximum number of rows to load from the file
    ///
    /// # Arguments
    /// * max_rows_number: Option<usize> - The edge type to use when edge type is missing.
    ///
    pub fn set_max_rows_number(
        mut self,
        max_rows_number: Option<usize>,
    ) -> Result<TypeFileReader<T>> {
        if max_rows_number.is_some() {
            self.must_have_reader()?;
        }
        self.reader = self.reader.map_or(Ok::<_, String>(None), |reader| {
            Ok(Some(reader.set_max_rows_number(max_rows_number)?))
        })?;
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

    /// Set whether to load the type list in sequential or in parallel.
    ///
    /// # Arguments
    /// * parallel: Option<bool> - Whether to load the type list in sequential or parallel.
    ///
    pub fn set_parallel(mut self, parallel: Option<bool>) -> Result<TypeFileReader<T>> {
        if let Some(parallel) = parallel {
            self.must_have_reader()?;
            self.reader
                .as_mut()
                .map(|reader| reader.parallel = parallel);
        }
        Ok(self)
    }

    /// Parse a single line (vector of strings already splitted)
    /// # Arguments
    /// * `line_number`: Number of the line.
    /// * `elements_in_line`: Vec<Option<String>> - Vector of the values of the line to be parsed
    fn parse_type_line(
        &self,
        line_number: usize,
        mut elements_in_line: Vec<Option<String>>,
    ) -> Result<(usize, String)> {
        // extract the type name
        let type_name = elements_in_line.pop().unwrap().map_or_else(
            || Err(format!("The type at line {} is empty.", line_number)),
            |type_name| Ok(type_name),
        )?;

        // Finally we check if the type ID was provided.
        let line_number = if self.type_ids_column_number.is_some() {
            let maybe_type_id = elements_in_line
                .pop()
                // We can unwrap because the check always happens in the CSV reader
                .unwrap();
            if maybe_type_id.is_none() {
                return Err("The type id cannot be undefined.".to_owned());
            }
            let type_id = maybe_type_id.unwrap();
            match type_id.as_str().parse::<usize>() {
                Ok(type_id) => Ok(type_id),
                Err(_) => Err(format!(
                    concat!(
                        "Unable to pass the type ID `{:?}` to ",
                        "a numeric value while reading line {}."
                    ),
                    type_id, line_number
                )),
            }?
        } else {
            line_number
        };

        Ok((line_number, type_name))
    }

    /// Return iterator of rows of the edge file.
    pub fn read_lines(
        &self,
    ) -> Option<
        Result<
            ItersWrapper<
                Result<(usize, String)>,
                impl Iterator<Item = Result<(usize, String)>> + '_,
                impl ParallelIterator<Item = Result<(usize, String)>> + '_,
            >,
        >,
    > {
        self.reader.as_ref().map(|reader| {
            Ok(reader
                .read_lines(Some(
                    vec![self.type_ids_column_number, Some(self.type_column_number)]
                        .iter()
                        .filter_map(|&e| e)
                        .collect(),
                ))?
                .map(move |line| match line {
                    Ok((line_number, vals)) => self.parse_type_line(line_number, vals),
                    Err(e) => Err(e),
                }))
        })
    }
}
