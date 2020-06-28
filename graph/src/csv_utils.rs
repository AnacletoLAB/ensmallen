use std::{fs::File, io::prelude::*, io::BufReader};

use rayon::prelude::*;

pub fn check_consistent_lines(path: &str, sep: &str) -> Result<(), String>{
    let file = File::open(path);
    if file.is_err() {
        return Err(format!("Cannot open file at {}", path));
    }
    let buf_reader = BufReader::new(file.unwrap());
    let mut expected_length: Option<usize> = None;

    for (counter, line) in buf_reader.lines().enumerate() {
        if line.is_err() {
            return Err(String::from("The file is empty!"));
        }
        let current_line = line.unwrap();
        let separators_number = current_line.matches(sep).count();
        if *expected_length.get_or_insert(separators_number) != separators_number {
            return Err(
                format!(
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
            )
        };
    }
    Ok(())
}

pub fn get_headers(path: &str, sep: &str) -> Vec<String> {
    let file = File::open(path).expect("Cannot open file.");
    let mut buf_reader = BufReader::new(file);
    let mut line = String::new();
    buf_reader
        .read_line(&mut line)
        .expect("Cannot read from file.");
    line.trim().split(sep).map(String::from).collect()
}

fn render_columns<'a>(columns: &[&'a str], optional_columns: &[&Option<&'a str>]) -> Vec<&'a str> {
    let mut rendered_columns: Vec<&str> = optional_columns
        .par_iter()
        .filter(|x| x.is_some())
        .map(|x| x.unwrap())
        .collect();
    rendered_columns.extend(columns.iter());
    rendered_columns
}

pub fn has_columns(
    path: &str,
    sep: &str,
    columns: &[&str],
    optional_columns: &[&Option<&str>],
) -> Result<(), String> {
    let rendered_columns = render_columns(columns, optional_columns);
    let candidate_columns = get_headers(path, sep);

    for column in rendered_columns {
        if !candidate_columns.contains(&String::from(column)) {
            return Err(
                format!(
                    concat!(
                        "Provided file hasn't the required columns.\n",
                        "Specifically, the given column {column} was not found ",
                        "within the available set of columns {columns:?}.\n",
                        "The given file is at path {path}.\n",
                    ),
                    column = column,
                    columns = candidate_columns,
                    path = path,
                )
            );
        }
    }

    Ok(())
}