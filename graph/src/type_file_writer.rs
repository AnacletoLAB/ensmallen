use super::*;
use indicatif::ProgressIterator;
use std::{fs::File, io::BufWriter};

/// Structure that saves the writer specific to writing and reading a types csv file.
#[no_binding]
pub struct TypeFileWriter {
    pub(crate) writer: CSVFileWriter,
    pub(crate) type_ids_column: Option<String>,
    pub(crate) type_ids_column_number: Option<usize>,
    pub(crate) types_column: String,
    pub(crate) types_column_number: usize,
    number_of_columns: usize,
    columns_are_dense: bool,
}

impl TypeFileWriter {
    /// Return new TypeFileWriter object.
    ///
    /// # Arguments
    ///
    /// * path: String - Path where to store/load the file.
    ///
    pub fn new<S: Into<String>>(path: S) -> TypeFileWriter {
        TypeFileWriter {
            writer: CSVFileWriter::new(path),
            type_ids_column: None,
            type_ids_column_number: None,
            types_column: "type_names".to_string(),
            types_column_number: 0,
            number_of_columns: 1,
            columns_are_dense: true,
        }
    }

    // Return whether the columns are currently dense.
    fn are_columns_dense(&self) -> bool {
        let mut offset = 0;
        if self
            .type_ids_column_number
            .map_or(false, |type_ids_column_number| type_ids_column_number != 0)
        {
            return false;
        }
        if self.type_ids_column_number.is_some() {
            offset += 1;
        }
        self.types_column_number == offset
    }

    /// Set the column of the nodes.
    ///
    /// # Arguments
    ///
    /// * types_column: Option<String> - The nodes column to use for the file.
    ///
    pub fn set_types_column<S: Into<String>>(mut self, types_column: Option<S>) -> TypeFileWriter {
        if let Some(column) = types_column {
            self.types_column = column.into();
        }
        self
    }

    /// Set the column_number of the nodes.
    ///
    /// # Arguments
    ///
    /// * types_column_number: Option<usize> - The nodes column_number to use for the file.
    ///
    pub fn set_types_column_number(mut self, types_column_number: Option<usize>) -> TypeFileWriter {
        if let Some(column_number) = types_column_number {
            self.types_column_number = column_number;
            self.number_of_columns = self.number_of_columns.max(column_number + 1);
            self.columns_are_dense = self.are_columns_dense();
        }
        self
    }

    /// Set the column of the node IDs.
    ///
    /// # Arguments
    /// * type_ids_column: Option<String> - The node IDs column to use for the file.
    ///
    pub fn set_type_ids_column(mut self, type_ids_column: Option<String>) -> TypeFileWriter {
        self.type_ids_column = type_ids_column;
        self
    }

    /// Set the column number of the node IDs.
    ///
    /// # Arguments
    /// * type_ids_column_number: Option<usize> - The node types column to use for the file.
    ///
    pub fn set_type_ids_column_number(
        mut self,
        type_ids_column_number: Option<usize>,
    ) -> TypeFileWriter {
        if let Some(column_number) = type_ids_column_number {
            self.type_ids_column_number = Some(column_number);
            self.number_of_columns = self.number_of_columns.max(column_number + 1);
            self.columns_are_dense = self.are_columns_dense();
        }
        self
    }

    /// Set the verbose.
    ///
    /// # Arguments
    ///
    /// * `verbose`: Option<bool> - Whether to show the loading bar or not.
    ///
    pub fn set_verbose(mut self, verbose: Option<bool>) -> TypeFileWriter {
        self.writer = self.writer.set_verbose(verbose);
        self
    }

    /// Set the separator.
    ///
    /// # Arguments
    /// * separator: Option<char> - The separator to use for the file.
    ///
    pub fn set_separator(mut self, separator: Option<char>) -> Result<TypeFileWriter> {
        self.writer = self.writer.set_separator(separator)?;
        Ok(self)
    }

    /// Set the header.
    ///
    /// # Arguments
    ///
    /// * header: Option<bool> - Whether to write out an header or not.
    ///
    pub fn set_header(mut self, header: Option<bool>) -> TypeFileWriter {
        self.writer = self.writer.set_header(header);
        self
    }

    fn build_header(&self) -> (Vec<String>, Vec<usize>) {
        // build the header
        let mut header_values = vec![];
        let mut header_positions = vec![];

        if let (Some(type_ids_column), Some(type_ids_column_number)) =
            (&self.type_ids_column, self.type_ids_column_number)
        {
            header_values.push(type_ids_column.clone());
            header_positions.push(type_ids_column_number);
        }

        header_positions.push(self.types_column_number.clone());
        header_values.push(self.types_column.clone());

        (header_values, header_positions)
    }

    /// Parses provided line into a vector of strings writable by the CSVFileWriter.
    fn parse_line<T: ToFromUsize>(&self, type_id: T, type_name: String) -> Vec<String> {
        let mut line = vec![];

        let mut positions = vec![];

        if let Some(type_ids_column_number) = &self.type_ids_column_number {
            line.push(type_id.to_string());
            if !self.columns_are_dense {
                positions.push(*type_ids_column_number);
            }
        }

        line.push(type_name);

        if !self.columns_are_dense {
            positions.push(self.types_column_number);
        }

        if self.columns_are_dense {
            line
        } else {
            compose_lines(self.number_of_columns, line, positions)
        }
    }

    pub(crate) fn start_writer(&self) -> Result<BufWriter<File>> {
        let (header_values, header_positions) = self.build_header();
        self.writer.start_writer(compose_lines(
            self.number_of_columns,
            header_values,
            header_positions,
        ))
    }

    /// Write the provided set of line elements to file.
    ///
    /// # Arguments
    /// `stream`: BufWriter<File> - The stream where to write the line
    /// `type_id`: T - The type of the element to be written to disk.
    /// `type_name`: String - The name of the element to be writter to disk.
    ///
    /// # Raises
    /// * If some I/O error is encountered.
    pub(crate) fn write_line<T: ToFromUsize>(
        &self,
        stream: &mut BufWriter<File>,
        type_id: T,
        type_name: String,
    ) -> Result<()> {
        self.writer
            .write_line(stream, self.parse_line(type_id, type_name))
    }

    pub(crate) fn close_writer(&self, stream: BufWriter<File>) -> Result<()> {
        self.writer.close_writer(stream)
    }

    /// Write edge list iterator to file.
    ///  
    /// # Arguments
    /// * `lines_number`: Option<usize> - The number of lines in the file.
    /// * `iterator`: impl Iterator<Item=_> - The iterator with the edge list to write to file.
    pub fn dump_iterator<T: ToFromUsize>(
        &self,
        lines_number: Option<usize>,
        iterator: impl Iterator<Item = (T, String)>,
    ) -> Result<()> {
        let pb = get_loading_bar(
            self.writer.verbose && lines_number.is_some(),
            "Writing to type list",
            lines_number.unwrap_or(0),
        );
        
        let mut stream = self.start_writer()?;
        for (type_id, type_name) in iterator.progress_with(pb) {
            self.write_line(&mut stream, type_id, type_name)?;
        }
        self.close_writer(stream)
    }
}
