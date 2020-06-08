extern crate csv;

use std::{fs::File, io::prelude::*, io::BufReader};

pub fn check_consistent_lines(path: &str, sep: &str) {
    let file = File::open(path).expect("Cannot open file.");
    let buf_reader = BufReader::new(file);
    let mut expected_length: Option<usize> = None;

    for (counter, line) in buf_reader.lines().enumerate() {
        let current_line = line.unwrap();
        let separators_number = current_line.matches(sep).count();
        if *expected_length.get_or_insert(separators_number) != separators_number {
            panic!(
                concat!(
                    "Provided file has malformed lines. ",
                    "The provided lines have different numbers ",
                    "of the given separator.\n",
                    "The expected number of separators was {expected_length}, ",
                    "but a line with {separators_number} separators was found. \n",
                    "The line is the number {counter}.\n",
                    "The given file is at path {path}.\n",
                    "The line in question is: '{line}'\n",
                ),
                expected_length = expected_length.unwrap(),
                separators_number = separators_number,
                counter = counter,
                path = path,
                line = current_line
            )
        };
    }
}

pub fn has_columns(path: &str, sep: &str, columns: Vec<&str>) {
    let file = File::open(path).expect("Cannot open file.");
    let mut buf_reader = BufReader::new(file);
    let mut line = String::new();
    buf_reader
        .read_line(&mut line)
        .expect("Cannot read from file.");
    let candidate_columns: Vec<&str> = line.trim().split(sep).collect();
    for column in columns {
        if !candidate_columns.contains(&column) {
            panic!(
                concat!(
                    "Provided file hasn't the required columns.\n",
                    "Specifically, the given column {column} was not found ",
                    "within the available set of columns {columns:?}.",
                    "The given file is at path {path}.\n",
                ),
                column = column,
                columns = candidate_columns,
                path = path,
            )
        }
    }
}
