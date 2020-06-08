extern crate csv;

use std::{
    io::prelude::*,
    fs::File,
    io::BufReader
};


pub fn check_consistent_lines(path: &str, sep: &str){
    let file = File::open(path).expect("Cannot open file.");
    let buf_reader = BufReader::new(file);
    let mut expected_length: Option<usize> = None;

    for (counter, line) in buf_reader.lines().enumerate() {
        let current_line = line.unwrap();
        let separators_number = current_line.matches(sep).count();
         if *expected_length.get_or_insert(separators_number) != separators_number {
            panic!(
                concat!(
                    "Provided nodes file has malformed lines. ",
                    "The provided lines have different numbers ",
                    "of the given separator.\n",
                    "The expected number of separators was {expected_length}, ",
                    "but a line with {separators_number} separators was found. \n",
                    "The line is the number {counter}.\n",
                    "The given file is at path {path}.\n",
                    "The line in question is: '{line}'\n",
                ),
                expected_length=expected_length.unwrap(),
                separators_number=separators_number,
                counter=counter,
                path=path,
                line=current_line
            )
        };
    }
}