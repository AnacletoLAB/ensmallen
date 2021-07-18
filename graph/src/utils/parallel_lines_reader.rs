use super::*;
use rayon::iter::plumbing::{UnindexedProducer, bridge_unindexed};
use rayon::prelude::*;
use std::fs::File;
use std::io::{prelude::*, BufReader, SeekFrom};

const READER_CAPACITY: usize = 8 * 1024 * 1024;
const LINE_LENGTH_EXTIMATE: usize = 256;

#[derive(Debug, Clone)]
pub struct ParallelLines<'a>{
    path: &'a str,
    number_of_lines: Option<usize>,
    number_of_rows_to_skip: Option<usize>,
}

impl<'a> ParallelLines<'a> {
    pub fn new(path: &str) -> Result<ParallelLines> {

        match File::open(path.clone()) {
            Ok(_) => Ok(()),
            Err(_) => Err(format!("Cannot open file {}", path)),
        }?;

        Ok(ParallelLines{
            path,
            number_of_lines: None,
            number_of_rows_to_skip: None,
        })
    }

    pub fn with_capacity(path: &str, number_of_lines: usize) -> Result<ParallelLines>  {
        
        match File::open(path.clone()) {
            Ok(_) => Ok(()),
            Err(_) => Err(format!("Cannot open file {}", path)),
        }?;

        Ok(ParallelLines{
            path,
            number_of_lines: Some(number_of_lines),
            number_of_rows_to_skip: None,
        })
    }

    pub fn skip_rows(&mut self, number_of_rows_to_skip: usize) {
        self.number_of_rows_to_skip = Some(number_of_rows_to_skip);
    }
}


impl<'a> ParallelIterator for ParallelLines<'a> {
    type Item = std::io::Result<String>;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
            C: rayon::iter::plumbing::UnindexedConsumer<Self::Item> {

        let file = File::open(self.path.clone()).unwrap();

        let file_len  =file.metadata().unwrap().len();
        let mut buff_reader = BufReader::with_capacity(READER_CAPACITY,file);

        if let Some(rts) = self.number_of_rows_to_skip {
            for _ in 0..rts {
                let mut _buffer = String::new();
                buff_reader.read_line(&mut _buffer).unwrap();
            }
        }

        bridge_unindexed(ParallelLinesProducer{
            path: self.path,
            max: file_len,
            file: buff_reader, 
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
    type Item = std::io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let pos = match self.file.stream_position() {
            Ok(pos) => pos,
            Err(e) => return Some(Err(e))
        };
        if pos >= self.max {
            return None;
        }
        let mut result = String::with_capacity(LINE_LENGTH_EXTIMATE);
        let chars_read = match self.file.read_line(&mut result) {
            Ok(result) => result,
            Err(e) => return Some(Err(e)),
        };
        if chars_read == 0 {
            None
        } else {
            if result.ends_with('\n') {
                result.pop().unwrap();
            }
            Some(Ok(result))
        }
    }
}

impl<'a> UnindexedProducer for ParallelLinesProducer<'a> {
    type Item = std::io::Result<String>;

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