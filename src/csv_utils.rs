use std::{fs::File, io::prelude::*, io::BufReader};

use std::collections::HashMap;

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

fn get_headers(path: &str, sep: &str) -> Vec<String> {
    let file = File::open(path).expect("Cannot open file.");
    let mut buf_reader = BufReader::new(file);
    let mut line = String::new();
    buf_reader
        .read_line(&mut line)
        .expect("Cannot read from file.");
    line.trim().split(sep).map(String::from).collect()
}

fn render_columns(columns: &[String], optional_columns: &[Option<String>]) -> Vec<String> {
    let mut rendered_columns: Vec<String> = optional_columns
        .iter().cloned()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();
    rendered_columns.extend(columns.iter().cloned());
    rendered_columns
}

pub fn has_columns(
    path: &str,
    sep: &str,
    columns: &[String],
    optional_columns: &[Option<String>],
) {
    let rendered_columns = render_columns(columns, optional_columns);
    let candidate_columns = get_headers(path, sep);

    for column in rendered_columns {
        if !candidate_columns.contains(&String::from(&column)) {
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

pub fn read_csv(
    path: &str,
    sep: &str,
    required_columns: &[String],
    optional_columns: &[Option<String>]
) -> HashMap<String, Vec<String>> {
    let columns = render_columns(required_columns, optional_columns);
    let headers = get_headers(path, sep);
    let mut result: HashMap<String, Vec<String>> =
        columns.iter().map(|x| (x.clone(), Vec::new())).collect();

    // open the file
    let file = File::open(path).expect("Cannot open file.");
    let mut buf_reader = BufReader::new(file);
    // Skip header
    let mut line = String::new();
    buf_reader.read_line(&mut line).unwrap();
    // convert the csv to a dict of lists
    for line in buf_reader.lines() {
        for (value, column) in line.unwrap().trim().split(sep).zip(headers.iter()) {
            if result.contains_key(column) {
                result.get_mut(column).unwrap().push(String::from(value));
            }
        }
    }

    result
}