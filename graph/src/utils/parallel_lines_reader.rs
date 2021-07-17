use rayon::iter::plumbing::{UnindexedProducer, Producer, bridge, bridge_unindexed};
use rayon::prelude::*;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, SeekFrom};

const READER_CAPACITY: usize = 8 * 1024 * 1024;
const LINE_LENGTH_EXTIMATE: usize = 256;

#[derive(Debug, Clone)]
pub struct ParallelLines<'a>{
    path: &'a str,
    number_of_lines: Option<usize>,
}

impl<'a> ParallelLines<'a> {
    pub fn new(path: &str) -> ParallelLines {
        ParallelLines{
            path,
            number_of_lines: None,
        }
    }

    pub fn with_capacity(path: &str, number_of_lines: usize) -> ParallelLines {
        ParallelLines{
            path,
            number_of_lines: Some(number_of_lines),
        }
    }
}


impl<'a> ParallelIterator for ParallelLines<'a> {
    type Item = String;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
            C: rayon::iter::plumbing::UnindexedConsumer<Self::Item> {

        let file = File::open(self.path.clone()).unwrap();
        bridge_unindexed(ParallelLinesProducer{
            path: self.path,
            max: file.metadata().unwrap().len(),
            file: BufReader::with_capacity(READER_CAPACITY,file), 
        }, consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        self.number_of_lines
    }
}

struct ParallelLinesProducer<'a> {
    path: &'a str,
    file: BufReader<File>,
    max: u64,
}

impl<'a> Iterator for ParallelLinesProducer<'a> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.file.stream_position().unwrap() >= self.max {
            return None;
        }
        let mut result = String::with_capacity(LINE_LENGTH_EXTIMATE);
        let chars_read = self.file.read_line(&mut result).unwrap();
        if chars_read == 0 {
            None
        } else {
            if result.ends_with('\n') {
                result.pop().unwrap();
            }
            Some(result)
        }
    }
}

impl<'a> UnindexedProducer for ParallelLinesProducer<'a> {
    type Item = String;

    /// Split the file in two approximately balanced streams
    fn split(mut self) -> (Self, Option<Self>) {
        // Get the current postion in the file
        let pos = self.file.stream_position().unwrap();
        // Check if it's reasonable to split the stream
        if pos + READER_CAPACITY as u64 > self.max  {
            return (self, None);
        }

        // Compute a guess of the middle point
        let mid = (self.max - pos) / 2 + pos;
        // Create a new file pointer
        let mut new_file_ptr = BufReader::with_capacity(READER_CAPACITY, File::open(self.path.clone()).unwrap());
        // skip to the guessed position
        new_file_ptr.seek(SeekFrom::Start(mid)).unwrap();
        // get to the next line start
        let mut buffer = String::with_capacity(LINE_LENGTH_EXTIMATE);
        new_file_ptr.read_line(&mut buffer).unwrap();
        // get the **actual** mid point
        let actual_mid = new_file_ptr.stream_position().unwrap();

        (
            ParallelLinesProducer{
                path: self.path,
                file: self.file,
                max: actual_mid,
            },
            Some(ParallelLinesProducer{
                path: self.path,
                file: new_file_ptr,
                max: self.max,
            })
        )
    }

    fn fold_with<F>(self, folder: F) -> F
    where
            F: rayon::iter::plumbing::Folder<Self::Item> {
        folder.consume_iter(self)
    }
}