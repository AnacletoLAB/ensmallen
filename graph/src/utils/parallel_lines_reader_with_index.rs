#[cfg(target_os = "linux")]
use nix::fcntl::*;
use rayon::iter::plumbing::{bridge_unindexed, UnindexedProducer};
use rayon::prelude::*;
use std::fs::File;
use std::io::{prelude::*, BufReader, SeekFrom};
#[cfg(target_os = "linux")]
use std::os::unix::io::AsRawFd;
use std::sync::{Arc, Mutex};

pub const READER_CAPACITY: usize = 1 << 17;

type IterType = (usize, Result<String, String>);

pub struct ParallelLinesWithIndex<'a> {
    path: &'a str,
    file: File,
    comment_symbol: Option<String>,
    number_of_lines: Option<usize>,
    number_of_rows_to_skip: Option<usize>,
    buffer_size: usize,
    max_producers: usize,
}

impl<'a> ParallelLinesWithIndex<'a> {
    pub fn new(path: &'a str) -> Result<ParallelLinesWithIndex<'a>, String> {
        let file = match File::open(path.clone()) {
            Ok(file) => Ok(file),
            Err(_) => Err(format!("Cannot open file {}", path)),
        }?;

        Ok(ParallelLinesWithIndex {
            path: path,
            file,
            number_of_lines: None,
            comment_symbol: None,
            number_of_rows_to_skip: None,
            buffer_size: READER_CAPACITY,
            max_producers: num_cpus::get(),
        })
    }

    pub fn set_buffer_size(&mut self, buffer_size: usize) {
        self.buffer_size = buffer_size;
    }

    pub fn set_max_producers(&mut self, max_producers: usize) {
        self.max_producers = max_producers;
    }

    pub fn set_skip_rows(&mut self, number_of_rows_to_skip: usize) {
        self.number_of_rows_to_skip = Some(number_of_rows_to_skip);
    }

    pub fn set_comment_symbol(&mut self, comment_symbol: Option<String>) {
        self.comment_symbol = comment_symbol;
    }
}

impl<'a> ParallelIterator for ParallelLinesWithIndex<'a> {
    type Item = IterType;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: rayon::iter::plumbing::UnindexedConsumer<Self::Item>,
    {
        #[cfg(target_os = "linux")]
        let _ = posix_fadvise(
            self.file.as_raw_fd(),
            0,
            0,
            PosixFadviseAdvice::POSIX_FADV_SEQUENTIAL,
        );

        // Create the file reader
        let mut reader = BufReader::with_capacity(self.buffer_size, self.file);
        // Skip the first rows (as specified by the user)
        if let Some(rts) = self.number_of_rows_to_skip {
            for _ in 0..rts {
                let mut _buffer = String::new();
                let result_bytes_read = reader.read_line(&mut _buffer);
                match result_bytes_read {
                    Ok(bytes_read) => {
                        // Reached End Of File
                        if bytes_read == 0 {
                            break;
                        }
                    }
                    Err(_errot) => {}
                }
            }
        }

        // Create the first producer
        let producer = ParalellLinesProducerWithIndex {
            path: self.path,
            file: reader,
            line_count: 0,
            modulus: 0,
            remainder: 0,
            max: self.max_producers,
            prod_count: Arc::new(Mutex::new(1)),
            buffer_size: self.buffer_size,
            comment_symbol: self.comment_symbol,
        };
        bridge_unindexed(producer, consumer)
    }

    fn opt_len(&self) -> Option<usize> {
        self.number_of_lines
    }
}

#[derive(Debug)]
struct ParalellLinesProducerWithIndex<'a> {
    path: &'a str,
    file: BufReader<File>,
    line_count: usize,
    modulus: usize,
    remainder: usize,
    max: usize,
    prod_count: Arc<Mutex<usize>>,
    buffer_size: usize,
    comment_symbol: Option<String>,
}

impl<'a> Drop for ParalellLinesProducerWithIndex<'a> {
    fn drop(&mut self) {
        let mut count = self.prod_count.lock().unwrap();
        *count -= 1;
    }
}

impl<'a> Iterator for ParalellLinesProducerWithIndex<'a> {
    type Item = IterType;

    fn next(&mut self) -> Option<Self::Item> {
        let mut line = Vec::with_capacity(128);

        loop {
            line.clear();

            // read a line
            let result_bytes_read = self.file.read_until(b'\n', &mut line);

            // check if it's ok, if we reached EOF, and if it's a comment
            if let Ok(bytes_read) = result_bytes_read {
                // EOF
                if bytes_read == 0 {
                    return None;
                }
                // Comment
                if let Some(cs) = self.comment_symbol.as_ref() {
                    if line.starts_with(cs.as_bytes()) {
                        continue;
                    }
                }
            };

            // increase the line count only on non-comment
            self.line_count += 1;

            // check if we are at the line we want to return
            if (self.line_count & self.modulus) == self.remainder {
                if line.ends_with(&[b'\n']) {
                    line.pop();
                    if line.ends_with(&[b'\r']) {
                        line.pop();
                    }
                }

                return Some((
                    self.line_count - 1,
                    match result_bytes_read {
                        Ok(_) => Ok(unsafe{String::from_utf8_unchecked(line)}),
                        Err(error) => Err(error.to_string()),
                    },
                ));
            }
        }
    }
}

fn upper_power_of_two(mut v: usize) -> usize {
    v -= 1;
    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    v + 1
}

impl<'a> UnindexedProducer for ParalellLinesProducerWithIndex<'a> {
    type Item = IterType;

    /// Split the file in two approximately balanced streams
    fn split(mut self) -> (Self, Option<Self>) {
        // Check if it's reasonable to split the stream
        let cond = {
            let mut count = self.prod_count.lock().unwrap();
            if *count > self.max || (self.modulus + 1) > upper_power_of_two(self.max) {
                true
            } else {
                *count += 1;
                false
            }
        };

        if cond {
            return (self, None);
        }

        let file =
            File::open(self.path.clone()).expect(&format!("Could not open the file {}", self.path));

        #[cfg(target_os = "linux")]
        let _ = posix_fadvise(
            file.as_raw_fd(),
            0,
            0,
            PosixFadviseAdvice::POSIX_FADV_SEQUENTIAL,
        );

        // Create a copy of the file reader of the father
        let mut new_file = BufReader::with_capacity(self.buffer_size, file);

        // Updated its position to the same byte in the file as the father.
        new_file
            .seek(SeekFrom::Start(self.file.stream_position().expect(
                "Could not read the file pointer position in the file.",
            )))
            .expect("Could seek the new file to the position of the old one.");

        // Create the child
        let new = ParalellLinesProducerWithIndex {
            path: self.path.clone(),
            line_count: self.line_count,
            file: new_file,
            modulus: (self.modulus << 1) | 1,
            remainder: self.modulus + self.remainder + 1,
            max: self.max,
            buffer_size: self.buffer_size,
            prod_count: self.prod_count.clone(),
            comment_symbol: self.comment_symbol.clone(),
        };

        // Update the father modulus so that the lines are equally splitted
        self.modulus = (self.modulus << 1) | 1;

        // Returns the two new producers
        (self, Some(new))
    }

    fn fold_with<F>(self, folder: F) -> F
    where
        F: rayon::iter::plumbing::Folder<Self::Item>,
    {
        folder.consume_iter(self)
    }
}
